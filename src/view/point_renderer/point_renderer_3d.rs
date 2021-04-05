// Third party
use kiss3d::{
    camera::Camera,
    context::Context,
    renderer::Renderer,
    resource::{
        AllocationType, BufferType, Effect, GPUVec, ShaderAttribute, ShaderUniform, Texture,
    },
};
use na::{Matrix4, Point3, Point4};

// Internal
use super::{texture_creation::load_texture, PointRendererInteraction, RenderMode};

pub struct PointRenderer3D {
    // The shader itself
    shader: Effect,
    /// Data allocation
    point_data: Vec<PointData>,
    gpu_vec: GPUVec<Point3<f32>>,
    normal_gpu_vec: GPUVec<Point4<f32>>,
    /// Shader attributes
    pos_attribute: ShaderAttribute<Point3<f32>>,
    color_attribute: ShaderAttribute<Point3<f32>>,
    normal_attribute: ShaderAttribute<Point4<f32>>,
    // Shader uniform
    proj_uniform: ShaderUniform<Matrix4<f32>>,
    view_uniform: ShaderUniform<Matrix4<f32>>,
    eye_uniform: ShaderUniform<Point3<f32>>,
    alpha_texture_uniform: ShaderUniform<i32>,
    render_mode_uniform: ShaderUniform<i32>,
    size_uniform: ShaderUniform<f32>,
    shading_intensity_uniform: ShaderUniform<f32>,
    gamma_uniform: ShaderUniform<f32>,
    normal_enabled_uniform: ShaderUniform<i32>,
    // Normal variables
    alpha_texture: Texture,
    gamma: f32,
    // Tuple that in the first element stores the current value and in the second the initial value.
    point_size: (f32, f32),
    blob_size: (f32, f32),
    shading_intensity: f32,
    visible: bool,
    pub render_mode: RenderMode,
    normal_enabled: bool,
    // last transform and dirty bool. Used to determine
    // if the points needs to be resorted on the z axis
    last_transform: Matrix4<f32>,
    dirty: bool,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct PointData {
    pub point: Point3<f32>,
    pub color: Point3<f32>,
    // 3D vector with eccentricity encoded into the 4th 'w' value
    pub normal: Option<Point4<f32>>,
    pub projected_z: f32,
}

impl PointRenderer3D {
    pub fn new(default_point_size: f32, default_blob_size: f32) -> PointRenderer3D {
        let mut shader = Effect::new_from_str(VERTEX_SHADER_SRC_3D, FRAGMENT_SHADER_SRC_3D);
        shader.use_program();

        PointRenderer3D {
            // Points and their color interleaved. note that each point in the cloud will have 6 points here as it defines
            // 2 triangles in the continous render mode
            point_data: Vec::new(),
            gpu_vec: GPUVec::new(Vec::new(), BufferType::Array, AllocationType::StreamDraw),
            normal_gpu_vec: GPUVec::new(Vec::new(), BufferType::Array, AllocationType::StreamDraw),
            // Shader variables
            pos_attribute: shader
                .get_attrib::<Point3<f32>>("position")
                .expect("Failed to get 'position' shader attribute."),
            color_attribute: shader
                .get_attrib::<Point3<f32>>("color")
                .expect("Failed to get 'color' shader attribute."),
            normal_attribute: shader
                .get_attrib::<Point4<f32>>("normal")
                .expect("Failed to get 'normal' shader attribute"),
            proj_uniform: shader
                .get_uniform::<Matrix4<f32>>("proj")
                .expect("Failed to get 'proj' shader attribute."),
            view_uniform: shader
                .get_uniform::<Matrix4<f32>>("view")
                .expect("Failed to get 'view' shader attribute."),
            eye_uniform: shader
                .get_uniform::<Point3<f32>>("eye")
                .expect("Failed to get 'eye' shader attribute."),
            size_uniform: shader
                .get_uniform("size")
                .expect("Failed to get 'size' uniform shader attribute"),
            shading_intensity_uniform: shader
                .get_uniform("shading_intensity")
                .expect("Failed to get 'shading_intensity' uniform shader attribute"),
            alpha_texture_uniform: shader
                .get_uniform("alphaTexture")
                .expect("Failed to get 'alphaTexture' uniform shader attribute"),
            render_mode_uniform: shader
                .get_uniform("renderMode")
                .expect("Failed to get 'renderMode' uniform shader attribute"),
            normal_enabled_uniform: shader
                .get_uniform("normalEnabled")
                .expect("Failed to get 'normalEnabled' uniform shader attribute"),
            gamma_uniform: shader
                .get_uniform("gamma")
                .expect("Failed to get 'gamma' uniform shader attribute"),
            // Shader itself
            shader,
            // GL variables with default values
            gamma: 2.0f32,
            point_size: (default_point_size, default_point_size),
            blob_size: (default_blob_size, default_blob_size),
            shading_intensity: 1.0f32,
            // Variable to set when skipping all rendering while keeping data loaded.
            visible: true,
            alpha_texture: load_texture(),
            render_mode: RenderMode::Continuous,
            normal_enabled: false,
            last_transform: Matrix4::identity(),
            dirty: false,
        }
    }

    /// Insert a single point with a color
    /// Here the normal is a 3D vector with the eccentricity encoded into the 4th value
    pub fn push(&mut self, point: Point3<f32>, normal: Option<Point4<f32>>, color: Point3<f32>) {
        self.point_data.push(PointData {
            point,
            color,
            normal,
            projected_z: 0.0f32,
        });
        self.dirty = true;
    }

    /// Clear all the points and their colors
    pub fn clear(&mut self) {
        self.point_data.clear();
        for points in self.gpu_vec.data_mut().iter_mut() {
            points.clear()
        }
    }

    /// Indicates whether some points have to be drawn.
    pub fn needs_rendering(&self) -> bool {
        self.gpu_vec.len() != 0 && self.visible
    }

    pub fn set_shading(&mut self, val: bool) {
        self.normal_enabled = val;
        self.dirty = true;
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
        self.gpu_vec.len() / 2
    }

    pub fn sort_point_if_needed(&mut self, camera: &dyn Camera) {
        if self.last_transform == camera.transformation() && !self.dirty {
            return;
        }
        self.sort_z_buffer(camera);
        self.sync_gpu_vector();
        self.dirty = false;
    }

    /// Sync the gpu vector back up with the data contained in `point_data`
    pub fn sync_gpu_vector(&mut self) {
        for points in self.gpu_vec.data_mut().iter_mut() {
            // Clear the GPU data array
            points.clear();
            // Reinsert all the points
            for point_data in &self.point_data {
                for _ in 0..6 {
                    points.push(point_data.point);
                    points.push(point_data.color);
                }
            }
        }

        if self.normal_enabled {
            for normals in self.normal_gpu_vec.data_mut().iter_mut() {
                normals.clear();
                for point_data in &self.point_data {
                    let normal = point_data
                        .normal
                        .expect("Normal not present while normals have been enabled");
                    for _ in 0..6 {
                        normals.push(normal);
                    }
                }
            }
        }
    }

    fn sort_z_buffer(&mut self, camera: &dyn Camera) {
        // Update the z value for each point using the current camera
        for point in self.point_data.iter_mut() {
            point.projected_z = PointRenderer3D::get_projected_z_value(&point.point, camera);
        }

        // Sort the array using this new information
        self.point_data
            .sort_by(|a, b| b.projected_z.partial_cmp(&a.projected_z).unwrap())
    }

    fn get_projected_z_value(point: &Point3<f32>, camera: &dyn Camera) -> f32 {
        let h_world_coord = point.to_homogeneous();
        let h_camera_point = camera.transformation() * h_world_coord;
        match Point3::from_homogeneous(h_camera_point) {
            Some(projected_point) => projected_point.z,
            None => f32::NEG_INFINITY,
        }
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

    /// Get the shading intensity
    fn get_shading_intensity(&self) -> f32 {
        self.shading_intensity
    }

    /// Set the shading intensity
    fn set_shading_intensity(&mut self, intensity: f32) {
        self.shading_intensity = intensity
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

        if self.normal_enabled {
            self.normal_attribute.enable();
        }

        // Load the current camera position to the shader
        camera.upload(pass, &mut self.proj_uniform, &mut self.view_uniform);
        self.eye_uniform.upload(&camera.eye());

        // Set the texture
        self.alpha_texture_uniform.upload(&1);

        // Set the blob size and point size
        self.size_uniform.upload(&self.get_blob_size());

        // Set the gamma
        self.gamma_uniform.upload(&self.gamma);

        // Set the shading intensity
        self.shading_intensity_uniform
            .upload(&self.shading_intensity);

        let ctxt = Context::get();

        // Enable gl blending
        verify!(ctxt.enable(Context::BLEND));
        verify!(ctxt.blend_func(Context::SRC_ALPHA, Context::ONE_MINUS_SRC_ALPHA));

        // Manually enable GL_VERTEX_PROGRAM_POINT_SIZE -> 8642_16 -> 34370_10
        verify!(ctxt.enable(34370u32));

        // Set if the normals need to be used
        self.normal_enabled_uniform
            .upload(match self.normal_enabled {
                true => &1,
                false => &0,
            });

        match self.render_mode {
            RenderMode::Discreet => {
                // set the correct render mode in the shader.
                self.render_mode_uniform.upload(&0);

                // Set the point size
                self.size_uniform.upload(&self.get_point_size());

                self.pos_attribute.bind_sub_buffer(&mut self.gpu_vec, 11, 0);
                self.color_attribute
                    .bind_sub_buffer(&mut self.gpu_vec, 11, 1);
                if self.normal_enabled {
                    self.normal_attribute
                        .bind_sub_buffer(&mut self.normal_gpu_vec, 5, 0);
                }
                verify!(ctxt.draw_arrays(Context::POINTS, 0, self.num_points() as i32 / 6));
            }
            RenderMode::Continuous => {
                // set the correct render mode in the shader.
                self.render_mode_uniform.upload(&1);

                // Set the blob size
                self.size_uniform.upload(&self.get_blob_size());

                // Sort the points
                self.sort_point_if_needed(camera);

                // The points and colours are interleaved in the same buffer
                self.pos_attribute.bind_sub_buffer(&mut self.gpu_vec, 1, 0);
                self.color_attribute
                    .bind_sub_buffer(&mut self.gpu_vec, 1, 1);

                if self.normal_enabled {
                    self.normal_attribute
                        .bind_sub_buffer(&mut self.normal_gpu_vec, 0, 0);
                }

                // Set the correct drawing method of the polygons
                let _ = verify!(ctxt.polygon_mode(Context::FRONT_AND_BACK, Context::FILL));

                // Upload the alpha texture to the shader
                verify!(ctxt.bind_texture(Context::TEXTURE_2D, Some(&self.alpha_texture)));

                // Actually draw the textured polygons. Each point is split represented by
                // 2 textured triangles.
                verify!(ctxt.draw_arrays(Context::TRIANGLES, 0, self.num_points() as i32));
            }
        }

        self.last_transform = camera.transformation();

        // Disable the blending again.
        verify!(ctxt.disable(Context::BLEND));

        self.pos_attribute.disable();
        self.color_attribute.disable();
        if self.normal_enabled {
            self.normal_attribute.disable();
        }
    }
}

/// Vertex shader used by the point renderer
const VERTEX_SHADER_SRC_3D: &str = r#"#version 150
    // Input to this shader
    in vec3 position;
    in vec3 color;
    // 3D vector with the eccentricity encoded into the w
    in vec4 normal;

    // Uniform variables for all vertices.
    uniform mat4 proj;
    uniform mat4 view;
    uniform vec3 eye;
    uniform float size;
    uniform float shading_intensity;
    uniform int renderMode;
    uniform int normalEnabled;

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

    // Get the color after the point was shaded
    // here the camera position is used as light position.
    vec3 get_shaded_color(vec3 point_position) {
        // We check the eccentricity before calculating the shading.
        // Only if we are confident that the local area is like a plane
        // we will shade it.
        if (normal.w < 0.5) {
            vec3 dir = normalize(eye - point_position);
            vec3 norm = normalize(normal.xyz);
            float shading = abs(dot(norm, -dir));
            float z = mix(color.z * shading, color.z, 1 - shading_intensity);
            return vec3(color.xy, z);
        } else {
            float shading = 0.5;
            float z = mix(color.z * shading, color.z, 1 - shading_intensity);
            return vec3(color.xy, z);
        }
    }

    // Render method used when using the discreet representation
    void render_discreet() {
        // Transform the world coordinate to a screen coordinate.
        gl_Position = proj * view * vec4(position, 1.0);

        // Set the size of the points. This needs to be done in the
        // shader because of a bug for intel igpus.
        gl_PointSize = size;

        // Make the (shaded) color available to the fragment shader
        if (normalEnabled == 0) {
            PointColor = color;
        } else {
            PointColor = get_shaded_color(position);
        }
    }

    // Render method used when using the continous representation
    void render_continuos() {
        // Get the offset position to one of the triangle corners
        vec4 offset = vec4(getOffset(), 0.0, 1.0);
        vec4 view_position = view * vec4(position, 1.0);
        vec4 offset_position = view_position + offset;

        // Transform the world coordinate to a screen coordinate.
        gl_Position = proj * offset_position;

        // Make the (shaded) color available to the fragment shader
        if (normalEnabled == 0) {
            PointColor = color;
        } else {
            PointColor = get_shaded_color(position);
        }

        // Make the texture coordinate available to the fragment shader.
        TextureCoordinate = getTextureCoordinate();
    }

    void main() {
        if (renderMode == 0) {
            render_discreet();
        } else {
            render_continuos();
        }
    }"#;

/// Fragment shader used by the point renderer
const FRAGMENT_SHADER_SRC_3D: &str = r#"#version 150
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
    }"#;
