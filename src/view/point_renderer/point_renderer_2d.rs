// Third party
use kiss3d::{
    context::Context,
    planar_camera::PlanarCamera,
    renderer::PlanarRenderer,
    resource::{
        AllocationType, BufferType, Effect, GPUVec, ShaderAttribute, ShaderUniform, Texture,
    },
};
use na::{Matrix3, Point2, Point3};

// Internal
use super::{texture_creation::load_texture, PointRendererInteraction, RenderMode};

pub struct PointRenderer2D {
    shader: Effect,
    pos_attribute: ShaderAttribute<Point2<f32>>,
    color_attribute: ShaderAttribute<Point3<f32>>,
    // Uniform attributes
    proj_uniform: ShaderUniform<Matrix3<f32>>,
    view_uniform: ShaderUniform<Matrix3<f32>>,
    alpha_texture_uniform: ShaderUniform<i32>,
    render_mode_uniform: ShaderUniform<i32>,
    size_uniform: ShaderUniform<f32>,
    gamma_uniform: ShaderUniform<f32>,
    // GPU vecs
    points_vec: GPUVec<Point2<f32>>,
    colors_vec: GPUVec<Point3<f32>>,
    // Normal variables
    alpha_texture: Texture,
    gamma: f32,
    // Tuple that in the first element stores the current value and in the second the initial value.
    point_size: (f32, f32),
    blob_size: (f32, f32),
    visible: bool,
    pub render_mode: RenderMode,
}

impl PointRenderer2D {
    pub fn new(default_point_size: f32, default_blob_size: f32) -> PointRenderer2D {
        let mut shader = Effect::new_from_str(VERTEX_SHADER_SRC_2D, FRAGMENT_SHADER_SRC_2D);

        shader.use_program();

        PointRenderer2D {
            points_vec: GPUVec::new(Vec::new(), BufferType::Array, AllocationType::StreamDraw),
            colors_vec: GPUVec::new(Vec::new(), BufferType::Array, AllocationType::StreamDraw),
            // Shader variables
            pos_attribute: shader
                .get_attrib::<Point2<f32>>("position")
                .expect("Failed to get 'position' shader attribute."),
            color_attribute: shader
                .get_attrib::<Point3<f32>>("color")
                .expect("Failed to get 'color' shader attribute."),
            proj_uniform: shader
                .get_uniform::<Matrix3<f32>>("proj")
                .expect("Failed to get 'proj' uniform shader attribute"),
            view_uniform: shader
                .get_uniform::<Matrix3<f32>>("view")
                .expect("Failed to get 'view' uniform shader attribute"),
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
            // GL variables
            gamma: 2.0,
            point_size: (default_point_size, default_point_size),
            blob_size: (default_blob_size, default_blob_size),
            // Variable to set when skipping all rendering while keeping data loaded.
            visible: true,
            alpha_texture: load_texture(),
            render_mode: RenderMode::Continuous,
        }
    }

    /// Insert a single point with a color
    pub fn push(&mut self, point: Point2<f32>, color: Point3<f32>) {
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

    /// Clear all the points
    pub fn clear(&mut self) {
        for points in self.points_vec.data_mut().iter_mut() {
            points.clear()
        }
        for color in self.colors_vec.data_mut().iter_mut() {
            color.clear()
        }
    }

    /// Indicates whether some points have to be drawn.
    pub fn needs_rendering(&self) -> bool {
        self.num_points() != 0 && self.visible
    }

    // Turn of the rendering for this renderer and clear the screen.
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

impl PointRendererInteraction for PointRenderer2D {
    /// Retrieve the current render mode
    fn get_current_render_mode(&self) -> RenderMode {
        self.render_mode
    }

    /// Switch the rendering mode.
    fn switch_render_mode(&mut self) {
        self.render_mode = self.render_mode.inverse();
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

impl PlanarRenderer for PointRenderer2D {
    /// Actually draws the lines.
    fn render(&mut self, camera: &mut dyn PlanarCamera) {
        // If there are no points we do not even need to attempt the render
        if !self.needs_rendering() {
            return;
        }

        self.shader.use_program();

        // Enable the attributes
        self.pos_attribute.enable();
        self.color_attribute.enable();

        // Load the current camera position to the shader
        camera.upload(&mut self.proj_uniform, &mut self.view_uniform);

        // Set the texture
        self.alpha_texture_uniform.upload(&1);

        // Set the gamma
        self.gamma_uniform.upload(&self.gamma);

        // Dive into gl calls!
        let ctxt = Context::get();

        // Enable blending with alpha
        verify!(ctxt.enable(Context::BLEND));
        verify!(ctxt.blend_func(Context::SRC_ALPHA, Context::ONE_MINUS_SRC_ALPHA));

        // Manually enable GL_VERTEX_PROGRAM_POINT_SIZE -> 8642_16 -> 34370_10
        // TODO: Is this required? Added because of a bug in the intel igpu where
        // setting the point size is broken.
        verify!(ctxt.enable(34370u32));

        match self.render_mode {
            RenderMode::Discreet => {
                // set the correct render mode in the shader.
                self.render_mode_uniform.upload(&0);

                // Set the point size
                self.size_uniform.upload(&self.get_point_size());

                // Draw the first point of all the triangle sets, hence the stride of 5
                self.pos_attribute
                    .bind_sub_buffer(&mut self.points_vec, 5, 0);
                self.color_attribute
                    .bind_sub_buffer(&mut self.colors_vec, 5, 0);

                verify!(ctxt.draw_arrays(Context::POINTS, 0, self.num_points() as i32 / 6));
            }
            RenderMode::Continuous => {
                // set the correct render mode in the shader.
                self.render_mode_uniform.upload(&1);

                // Set the blob size
                self.size_uniform.upload(&self.get_blob_size());

                // Bind the buffers with data to all the shader attributes
                self.pos_attribute
                    .bind_sub_buffer(&mut self.points_vec, 0, 0);
                self.color_attribute
                    .bind_sub_buffer(&mut self.colors_vec, 0, 0);

                // Set the correct drawing method of the polygons
                let _ = verify!(ctxt.polygon_mode(Context::FRONT_AND_BACK, Context::FILL));

                // Ensure we use the correct texture
                verify!(ctxt.bind_texture(Context::TEXTURE_2D, Some(&self.alpha_texture)));

                // Actually draw the textured polygons. Each point is split represented by
                // 2 textured triangles.
                verify!(ctxt.draw_arrays(Context::TRIANGLES, 0, self.num_points() as i32));
            }
        }

        // Disable the blending again.
        verify!(ctxt.disable(Context::BLEND));

        // Disable the attributes again.
        self.pos_attribute.disable();
        self.color_attribute.disable();
    }
}

const VERTEX_SHADER_SRC_2D: &str = "#version 150
    // Input to this shader
    in vec2 position;
    in vec3 color;

    // Uniform variables over all the inputs
    uniform mat3 proj;
    uniform mat3 view;
    uniform float size;
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
        float scale = size;
        float negScale = -1.0 * size;

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

    void render_discreet() {
        // Transform the world coordinate to a screen coordinate
        vec3 projected_pos = proj * view * vec3(position, 1.0);
        // We set the z to 0 to draw all points along a plane.
        projected_pos.z = 0.0;

        // Set the screenspace position
        gl_Position = vec4(projected_pos, 1.0);

        // Set the size of the points. This needs to be done in the
        // shader because of a bug for intel igpus.
        gl_PointSize = size;

        // Make the color and tex coordinate available to the fragment shader.
        PointColor = color;
    }

    void render_continuos() {
        // Get the offset to one of the triangle corners
        vec2 offset_position = position + getOffset();

        // Get the projected triangle corner position
        vec3 projected_pos = proj * view * vec3(offset_position, 1.0);

        // We set the z to 0 to draw all points along a plane.
        projected_pos.z = 0.0;

        // Set the screenspace position
        gl_Position = vec4(projected_pos, 1.0);

        // Make the color and tex coordinate available to the fragment shader.
        PointColor = color;
        TextureCoordinate = getTextureCoordinate();
    }

    void main() {
        if (renderMode == 0) {
            render_discreet();
        } else {
            render_continuos();
        }
    }";

/// Fragment shader used by the point renderer
const FRAGMENT_SHADER_SRC_2D: &str = "#version 150
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
    out vec4 FragColor;

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
