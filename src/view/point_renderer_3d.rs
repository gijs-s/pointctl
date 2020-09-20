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
use super::{marcos, texture_creation::load_texture, PointRendererInteraction, RenderMode};
use crate::view::color_map::ColorMap;
pub struct PointRenderer3D {
    // The shader itself
    shader: Effect,
    /// Data allocation
    points_vec: GPUVec<Point3<f32>>,
    colors_vec: GPUVec<Point3<f32>>,
    /// Shader attributes
    pos_attribute: ShaderAttribute<Point3<f32>>,
    color_attribute: ShaderAttribute<Point3<f32>>,
    // Shader uniform
    proj_uniform: ShaderUniform<Matrix4<f32>>,
    view_uniform: ShaderUniform<Matrix4<f32>>,
    alpha_texture_uniform: ShaderUniform<i32>,
    render_mode_uniform: ShaderUniform<i32>,
    size_uniform: ShaderUniform<f32>,
    gamma_uniform: ShaderUniform<f32>,
    // Normal variables
    alpha_texture: Texture,
    gamma: f32,
    // Tuple that in the first element stores the current value and in the second the initial value.
    point_size: (f32, f32),
    blob_size: (f32, f32),
    visible: bool,
    pub render_mode: RenderMode,
}

impl PointRenderer3D {
    pub fn new(default_point_size: f32, default_blob_size: f32) -> PointRenderer3D {
        let mut shader = Effect::new_from_str(VERTEX_SHADER_SRC_3D, FRAGMENT_SHADER_SRC_3D);
        shader.use_program();

        PointRenderer3D {
            // Points and their color interleaved. note that each point in the cloud will have 6 points here as it defines
            // 2 triangles in the continous render mode
            points_vec: GPUVec::new(Vec::new(), BufferType::Array, AllocationType::StreamDraw),
            colors_vec: GPUVec::new(Vec::new(), BufferType::Array, AllocationType::StreamDraw),
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
            size_uniform: shader
                .get_uniform("size")
                .expect("Failed to get 'size' uniform shader attribute"),
            alpha_texture_uniform: shader
                .get_uniform("alphaTexture")
                .expect("Failed to get 'alphaTexture' uniform shader attribute"),
            render_mode_uniform: shader
                .get_uniform("renderMode")
                .expect("Failed to get 'renderMode' uniform shader attribute"),
            gamma_uniform: shader
                .get_uniform("gamma")
                .expect("Failed to get 'gamma' uniform shader attribute"),
            // Shader itself
            shader,
            // GL variables with default values
            gamma: 2.0,
            point_size: (default_point_size, default_point_size),
            blob_size: (default_blob_size, default_blob_size),
            // Variable to set when skipping all rendering while keeping data loaded.
            visible: true,
            alpha_texture: load_texture(),
            render_mode: RenderMode::Discreet,
        }
    }

    /// Insert a single point with a color
    pub fn push(&mut self, point: Point3<f32>, color: Point3<f32>) {
        for points in self.points_vec.data_mut().iter_mut() {
            for _ in 0..6 {
                points.push(point);
            }
        }
        for colors in self.colors_vec.data_mut().iter_mut() {
            for _ in 0..6 {
                colors.push(color);
            }
        }
    }

    /// Clear all the points and their colors
    pub fn clear(&mut self) {
        for points in self.points_vec.data_mut().iter_mut() {
            points.clear()
        }
        for points in self.colors_vec.data_mut().iter_mut() {
            points.clear()
        }
    }

    /// Indicates whether some points have to be drawn.
    pub fn needs_rendering(&self) -> bool {
        self.points_vec.len() != 0 && self.visible
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

    // Retrieve the number of points
    pub fn num_points(&self) -> usize {
        self.points_vec.len()
    }
}

impl PointRendererInteraction for PointRenderer3D {
    /// Switch the rendering mode.
    fn switch_render_mode(&mut self) {
        self.render_mode = self.render_mode.inverse();
    }

    /// Retrieve the current render mode
    fn get_current_render_mode(&self) -> RenderMode {
        self.render_mode
    }

    /// Set the gamma which will be used to next render loop
    fn set_gamma(&mut self, gamma: f32) {
        self.gamma = gamma;
    }

    /// Get the gamma which will be used to next render loop
    fn get_gamma(&self) -> f32 {
        self.gamma
    }

    /// Reset the gamma value
    fn reset_gamma(&mut self) {
        self.gamma = self.get_default_gamma();
    }

    /// Set the point size
    fn set_point_size(&mut self, size: f32) {
        self.point_size.0 = size;
    }

    /// Set the point size
    fn get_point_size(&self) -> f32 {
        self.point_size.0
    }

    /// Reset the point size to its initial value
    fn reset_point_size(&mut self) {
        self.point_size.0 = self.point_size.1;
    }

    /// Get the initial point size
    fn get_default_point_size(&self) -> f32 {
        self.point_size.1
    }

    /// Set the blob size used for continous rendering
    fn set_blob_size(&mut self, size: f32) {
        self.blob_size.0 = size;
    }

    /// Get the blob size used for the continous rendering
    fn get_blob_size(&self) -> f32 {
        self.blob_size.0
    }

    /// Reset the blob size used for continous rendering
    fn reset_blob_size(&mut self) {
        self.blob_size.0 = self.blob_size.1;
    }

    /// Get initial blob size used for continous rendering
    fn get_default_blob_size(&self) -> f32 {
        self.blob_size.1
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

        // Set the gamma
        self.gamma_uniform.upload(&self.gamma);

        let ctxt = Context::get();

        // Enable gl blending
        verify!(ctxt.enable(Context::BLEND));
        verify!(ctxt.blend_func(Context::SRC_ALPHA, Context::ONE_MINUS_SRC_ALPHA));

        // Bind the buffers
        self.pos_attribute
            .bind_sub_buffer(&mut self.points_vec, 0, 0);
        self.color_attribute
            .bind_sub_buffer(&mut self.colors_vec, 0, 0);

        match self.render_mode {
            RenderMode::Discreet => {
                // set the correct render mode in the shader.
                self.render_mode_uniform.upload(&0);

                // Set the point size
                self.size_uniform.upload(&(self.get_point_size() / 30.0f32));
            }
            RenderMode::Continuous => {
                // set the correct render mode in the shader.
                self.render_mode_uniform.upload(&1);

                // Set the blob size
                self.size_uniform.upload(&self.get_blob_size());

                // Upload the alpha texture to the shader
                verify!(ctxt.bind_texture(Context::TEXTURE_2D, Some(&self.alpha_texture)));
            }
        }

        // Set the correct drawing method of the polygons
        let _ = verify!(ctxt.polygon_mode(Context::FRONT_AND_BACK, Context::FILL));

        // Actually draw the textured polygons. Each point is split represented by
        // 2 textured triangles.
        verify!(ctxt.draw_arrays(Context::TRIANGLES, 0, self.num_points() as i32));

        // Disable the blending again.
        verify!(ctxt.disable(Context::BLEND));

        self.pos_attribute.disable();
        self.color_attribute.disable();
    }
}

/// The continous rendering needs work. The points are being drawn in an arbirary order, this causes trouble when blending.

/// Vertex shader used by the point renderer
const VERTEX_SHADER_SRC_3D: &'static str = "#version 460
    // Input to this shader
    layout (location = 0) in vec3 position;
    layout (location = 1) in vec3 color;

    // Uniform variables for all vertices.
    uniform mat4 proj;
    uniform mat4 view;
    uniform float size;

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
        float neg_size = -1.0 * size;

        float index = mod(gl_VertexID, 6);
        if (index == 0.0) {
            return vec2(neg_size, 0.0);
        }
        if (index == 1.0) {
            return vec2(0.0, size);
        }
        if (index == 2.0) {
            return vec2(size, 0.0);
        }
        if (index == 3.0) {
            return vec2(neg_size, 0.0);
        }
        if (index == 4.0) {
            return vec2(0.0, neg_size);
        }
        if (index == 5.0) {
            return vec2(size, 0.0);
        }
    }

    void main() {
        // Get the offset position to one of the triangle corners
        vec4 offset = vec4(getOffset(), 0.0, 1.0);
        vec4 position = view * vec4(position, 1.0);
        vec4 offset_position = position + offset;

        // Transform the world coordinate to a screen coordinate.
        gl_Position = proj * offset_position;

        // Make the color and tex coordinate available to the fragment shader.
        PointColor = color;
        TextureCoordinate = getTextureCoordinate();
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
    uniform float gamma;
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
        // Convert color to rgb
        vec3 rgb_color = hsv2rgb(PointColor);

        // Gamma normalization
        rgb_color.rgb = pow(rgb_color.rgb, vec3(1.0/gamma));

        // Set the color output
        FragColor = vec4(rgb_color, 1.0);
    }

    // Continous render mode, here we need to use the texture for the alpha
    void render_continuos() {
        // Get alpha from the texture
        float alpha = texture(alphaTexture, TextureCoordinate).a;

        // Don't even draw the point if the alpha is 0
        if(alpha == 0.0)
            discard;

        // Convert color to rgb
        vec3 rgb_color = hsv2rgb(PointColor);

        // Gamma normalization
        rgb_color.rgb = pow(rgb_color.rgb, vec3(1.0/gamma));

        // Set the color output
        FragColor = vec4(rgb_color, alpha);
    }

    void main() {
        if (renderMode == 0){
            render_discreet();
        } else {
            render_continuos();
        }
    }";
