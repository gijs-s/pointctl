extern crate kiss3d;
extern crate nalgebra as na;

// Third party
use gl;
use kiss3d::{
    camera::Camera,
    context::Context,
    renderer::Renderer,
    resource::{
        AllocationType, BufferType, Effect, GPUVec, ShaderAttribute, ShaderUniform, Texture,
    },
};
use na::{Matrix3, Matrix4, Point2, Point3};

// Internal
use super::{marcos, texture_creation::load_texture, RenderMode};
use crate::view::color_map::ColorMap;

/// 3D
pub struct PointRenderer3D {
    // The shader itself
    shader: Effect,
    /// Shader attributes
    pos_attribute: ShaderAttribute<Point3<f32>>,
    color_attribute: ShaderAttribute<Point3<f32>>,
    // Shader uniform
    proj_uniform: ShaderUniform<Matrix4<f32>>,
    view_uniform: ShaderUniform<Matrix4<f32>>,
    alpha_texture_uniform: ShaderUniform<i32>,
    render_mode_uniform: ShaderUniform<i32>,
    blob_size_uniform: ShaderUniform<f32>,
    // Data allocation
    points: GPUVec<Point3<f32>>,
    // Normal variables
    alpha_texture: Texture,
    gamma: f32,
    point_size: f32,
    blob_size: f32,
    visible: bool,
    pub render_mode: RenderMode,
}

impl PointRenderer3D {
    pub fn new() -> PointRenderer3D {
        let mut shader = Effect::new_from_str(VERTEX_SHADER_SRC_3D, FRAGMENT_SHADER_SRC_3D);
        shader.use_program();

        PointRenderer3D {
            // Points and their color interleaved. note that each point in the cloud will have 6 points here as it defines
            // 2 triangles in the continous render mode
            points: GPUVec::new(Vec::new(), BufferType::Array, AllocationType::StreamDraw),
            // Shader variables
            pos_attribute: shader
                .get_attrib::<Point3<f32>>("position")
                .expect("Failed to get 'position' shader attribute."),
            color_attribute: shader
                .get_attrib::<Point3<f32>>("color")
                .expect("Failed to get 'color' shader attribute."),
            proj_uniform: shader
                .get_uniform::<Matrix4<f32>>("proj")
                .expect("Failed to get 'proj' shader attribute."),
            view_uniform: shader
                .get_uniform::<Matrix4<f32>>("view")
                .expect("Failed to get 'view' shader attribute."),
            blob_size_uniform: shader
                .get_uniform("blobSize")
                .expect("Failed to get 'blobSize' uniform shader attribute"),
            alpha_texture_uniform: shader
                .get_uniform("alphaTexture")
                .expect("Failed to get 'alphaTexture' uniform shader attribute"),
            render_mode_uniform: shader
                .get_uniform("renderMode")
                .expect("Failed to get 'renderMode' uniform shader attribute"),
            // Shader itself
            shader,
            // GL variables with default values
            gamma: 2.0,
            point_size: 4.0,
            blob_size: 1.0,
            // Variable to set when skipping all rendering while keeping data loaded.
            visible: true,
            alpha_texture: load_texture(),
            render_mode: RenderMode::Discreet,
        }
    }

    /// Insert a single point with a color
    pub fn push(&mut self, point: Point3<f32>, color: Point3<f32>) {
        for points_buffer in self.points.data_mut().iter_mut() {
            for _ in 0..6 {
                points_buffer.push(point);
                points_buffer.push(color);
            }
        }
    }

    /// Insert a large number of points with colors at once
    pub fn batch_insert(&mut self, points_x_colors: Vec<(Point3<f32>, Point3<f32>)>) {
        for points_buffer in self.points.data_mut().iter_mut() {
            for &(point, color) in points_x_colors.iter() {
                for _ in 0..6 {
                    points_buffer.push(point);
                    points_buffer.push(color);
                }
            }
        }
    }

    /// Clear all the points and their colors
    pub fn clear(&mut self) {
        for points in self.points.data_mut().iter_mut() {
            points.clear()
        }
    }

    /// Indicates whether some points have to be drawn.
    pub fn needs_rendering(&self) -> bool {
        self.points.len() != 0 && self.visible
    }

    // Turn off the rendering for this renderer and clear the screen.
    pub fn hide(&mut self) {
        self.visible = false;

        // Clear the screen
        let ctxt = Context::get();
        verify!(ctxt.active_texture(Context::TEXTURE0));
    }

    // Turn on the rendering for this renderer
    pub fn show(&mut self) {
        self.visible = true;
    }

    // Set the point size
    pub fn set_point_size(&mut self, point_size: f32) {
        self.point_size = point_size;
    }

    /// Set the gamma which will be used to next render loop
    pub fn set_gamma(&mut self, gamma: f32) {
        self.gamma = gamma;
    }

    /// Get the gamma which will be used to next render loop
    pub fn get_gamma(&self) -> f32 {
        self.gamma
    }

    /// Set the splat size
    pub fn set_blob_size(&mut self, size: f32) {
        self.blob_size = size;
    }

    /// Get the blob size used for the continous rendering
    pub fn get_blob_size(&self) -> f32 {
        self.blob_size
    }

    // Retrieve the number of points
    pub fn num_points(&self) -> usize {
        // Points and colours are interleaved so we divide by 2
        self.points.len() / 2
    }

    pub fn switch_rendering_mode(&mut self) {
        self.render_mode = match self.render_mode {
            RenderMode::Discreet => RenderMode::Continuous,
            RenderMode::Continuous => RenderMode::Discreet,
        };
    }
}

impl Renderer for PointRenderer3D {
    /// Actually draws the points.
    fn render(&mut self, pass: usize, camera: &mut dyn Camera) {
        // If there are no points to draw or the render is not set to visible to not do anything
        if !self.needs_rendering() {
            return;
        }

        self.shader.use_program();

        // Enable the attributes
        self.pos_attribute.enable();
        self.color_attribute.enable();

        // Load the current camera position to the shader
        camera.upload(pass, &mut self.proj_uniform, &mut self.view_uniform);

        // Set the texture
        self.alpha_texture_uniform.upload(&1);

        // Set the blob size and point size
        self.blob_size_uniform.upload(&self.blob_size);

        let ctxt = Context::get();

        // Enable gl blending
        verify!(ctxt.enable(Context::BLEND));
        verify!(ctxt.blend_func(Context::SRC_ALPHA, Context::ONE_MINUS_SRC_ALPHA));

        match self.render_mode {
            RenderMode::Discreet => {
                // set the correct render mode in the shader.
                self.render_mode_uniform.upload(&0);

                // Set the point size
                ctxt.point_size(self.point_size);

                // The points and colours are interleaved in the same buffer
                self.pos_attribute.bind_sub_buffer(&mut self.points, 11, 0);
                self.color_attribute
                    .bind_sub_buffer(&mut self.points, 11, 1);

                ctxt.draw_arrays(Context::POINTS, 0, self.num_points() as i32 / 6);
            }
            RenderMode::Continuous => {
                // set the correct render mode in the shader.
                self.render_mode_uniform.upload(&1);

                // Set the point size to 1
                ctxt.point_size(1.0f32);

                // The points and colours are interleaved in the same buffer
                self.pos_attribute.bind_sub_buffer(&mut self.points, 1, 0);
                self.color_attribute.bind_sub_buffer(&mut self.points, 1, 1);

                // Set the correct drawing method of the polygons
                let _ = verify!(ctxt.polygon_mode(Context::FRONT_AND_BACK, Context::FILL));

                // Upload the alpha texture to the shader
                verify!(ctxt.bind_texture(Context::TEXTURE_2D, Some(&self.alpha_texture)));

                // Actually draw the textured polygons. Each point is split represented by
                // 2 textured triangles.
                verify!(ctxt.draw_arrays(Context::TRIANGLES, 0, self.num_points() as i32));
            }
        }

        // Disable the blending again.
        verify!(ctxt.disable(Context::BLEND));

        self.pos_attribute.disable();
        self.color_attribute.disable();
    }
}

/// The continous rendering needs work, if found the following resources that might be interesting:
/// - https://community.khronos.org/t/geometry-shader-point-sprite-to-sphere/63015
/// - http://www.opengl-tutorial.org/intermediate-tutorials/billboards-particles/billboards/
/// - https://solarianprogrammer.com/2013/05/17/opengl-101-textures/

/// Vertex shader used by the point renderer
const VERTEX_SHADER_SRC_3D: &'static str = "#version 460
    // Input to this shader
    layout (location = 0) in vec3 position;
    layout (location = 1) in vec3 color;

    // Uniform variables for all vertices.
    uniform mat4 proj;
    uniform mat4 view;
    uniform float blobSize;
    uniform int renderMode;

    // Passed on to the rest of the shader pipeline
    out vec2 TextureCoordinate;
    out vec3 PointColor;

    // Get the texture coordinate
    vec2 getTextureCoordinate() {
        float index = mod(gl_VertexID, 3);
        if (index == 0.0) {
            return vec2(1.0, 0.0);
        }
        if (index == 1.0) {
            return vec2(0.0, 0.0);
        }
        if (index == 2.0) {
            return vec2(0.0, 1.0);
        }
    }

    // Get the offset vector.
    vec2 getOffset() {
        float scale = blobSize;
        float negScale = -1.0 * blobSize;

        float index = mod(gl_VertexID, 6);
        if (index == 0.0) {
            return vec2(negScale, 0.0);
        }
        if (index == 1.0) {
            return vec2(0.0, scale);
        }
        if (index == 2.0) {
            return vec2(scale, 0.0);
        }
        if (index == 3.0) {
            return vec2(negScale, 0.0);
        }
        if (index == 4.0) {
            return vec2(0.0, negScale);
        }
        if (index == 5.0) {
            return vec2(scale, 0.0);
        }
    }

    // Render method used when using the discreet representation
    void render_discreet() {
        // Transform the world coordinate to a screen coordinate.
        gl_Position = proj * view * vec4(position, 1.0);

        // Make the color and tex coordinate available to the fragment shader.
        PointColor = color;
    }

    // Render method used when using the continous representation
    void render_continuos() {
        // Get the offset position to one of the triangle cornors
        // TODO: This now offset is on a 2D plane
        vec3 offset_position = position + vec3(getOffset(), 0.0);

        // Transform the world coordinate to a screen coordinate.
        gl_Position = proj * view * vec4(offset_position, 1.0);

        // Make the color and tex coordinate available to the fragment shader.
        PointColor = color;
        TextureCoordinate = getTextureCoordinate();
    }

    void main() {
        if (renderMode == 0) {
            render_discreet();
            return;
        } else {
            render_continuos();
            return;
        }
    }";

/// Fragment shader used by the point renderer
const FRAGMENT_SHADER_SRC_3D: &'static str = "#version 460
#ifdef GL_FRAGMENT_PRECISION_HIGH
   precision highp float;
#else
   precision mediump float;
#endif

    // input color
    in vec3 PointColor;
    in vec2 TextureCoordinate;

    // Uniform variables over all the inputs
    uniform int renderMode;
    uniform sampler2D alphaTexture;

    // output color
    layout( location = 0 ) out vec4 FragColor;

    // Transfrom a HSV color to an RGB color (0..1)
    vec3 hsv2rgb(vec3 c)
    {
        vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
        vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
        return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
    }

    // Discreet render mode, just draw the point.
    void render_discreet() {
        vec3 rgb_color = hsv2rgb(PointColor);
        FragColor = vec4(rgb_color, 1.0);
    }

    // Continous render mode, here we need to use the texture for the alpha
    void render_continuos() {
        // Get alpha from the texture
        float alpha = texture(alphaTexture, TextureCoordinate).a;

        // Don't even draw the point if the alpha is 0
        if(alpha == 0.0)
            discard;

        vec3 rgb_color = hsv2rgb(PointColor);
        FragColor = vec4(rgb_color, alpha);
    }

    void main() {
        if (renderMode == 0){
            render_discreet();
        } else {
            render_continuos();
        }
    }";
