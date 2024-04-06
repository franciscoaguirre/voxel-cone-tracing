use c_str_macro::c_str;
use engine::prelude::*;

#[derive(Pausable)]
pub struct Visualizer {
    visualization_shader: Shader,
    world_positions_shader: Shader,
    back_framebuffer: Framebuffer<1>,
    front_framebuffer: Framebuffer<1>,
    cube_renderer: Cube,
    quad_renderer: Quad,
    paused: bool,
}

impl Visualizer {
    pub unsafe fn new() -> Self {
        Self {
            world_positions_shader: compile_shaders!(
                "assets/shaders/voxel_fragment/worldPositions.glsl"
            ),
            cube_renderer: Cube::new(),
            quad_renderer: Quad::new(),
            back_framebuffer: Framebuffer::<1>::new_floating_point(),
            front_framebuffer: Framebuffer::<1>::new_floating_point(),
            visualization_shader: compile_shaders!(
                "assets/shaders/voxel_fragment/visualizeVoxel3DTexture.glsl"
            ),
            paused: false,
        }
    }
}

impl Kernel for Visualizer {
    /// Initializes the visualizer.
    unsafe fn setup(&mut self, _assets: &mut AssetRegistry) {}

    /// Runs the ray marching code against the voxels 3D texture.
    unsafe fn update(&mut self, scene: &Scene, assets: &AssetRegistry) {
        let active_camera = &scene.cameras[scene.active_camera.unwrap_or(0)].borrow();

        // Use world positions shader
        self.world_positions_shader.use_program();

        // Upload camera
        self.world_positions_shader
            .set_mat4(c_str!("projection"), &active_camera.get_projection_matrix());
        self.world_positions_shader
            .set_mat4(c_str!("view"), &active_camera.transform.get_view_matrix());

        // Settings
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
        gl::Enable(gl::CULL_FACE);
        gl::Enable(gl::DEPTH_TEST);

        // Back
        gl::CullFace(gl::FRONT);
        gl::BindFramebuffer(gl::FRAMEBUFFER, self.back_framebuffer.fbo());
        let (width, height) = common::get_framebuffer_size();
        gl::Viewport(0, 0, width, height);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        gl::BindVertexArray(self.cube_renderer.get_vao());
        gl::DrawElements(
            gl::TRIANGLES,
            self.cube_renderer.get_num_indices() as i32,
            gl::UNSIGNED_INT,
            std::ptr::null(),
        );
        gl::BindVertexArray(0);

        // Front
        gl::CullFace(gl::BACK);
        gl::BindFramebuffer(gl::FRAMEBUFFER, self.front_framebuffer.fbo());
        let (width, height) = common::get_framebuffer_size();
        gl::Viewport(0, 0, width, height);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        gl::BindVertexArray(self.cube_renderer.get_vao());
        gl::DrawElements(
            gl::TRIANGLES,
            self.cube_renderer.get_num_indices() as i32,
            gl::UNSIGNED_INT,
            std::ptr::null(),
        );
        gl::BindVertexArray(0);

        self.visualization_shader.use_program();

        // Upload camera
        self.visualization_shader
            .set_mat4(c_str!("projection"), &active_camera.get_projection_matrix());
        self.visualization_shader
            .set_mat4(c_str!("view"), &active_camera.transform.get_view_matrix());
        self.visualization_shader.set_vec3(
            c_str!("cameraPosition"),
            active_camera.transform.position.x,
            active_camera.transform.position.y,
            active_camera.transform.position.z,
        );
        // self.visualization_shader
        //     .set_int(c_str!("level"), inputs.mipmap_level); // TODO: How do I handle these uniforms specific to each kernel?

        // Unbind framebuffer.
        // Very important because we are clearing the framebuffer later,
        // we don't want to clear this one.
        gl::BindRenderbuffer(gl::RENDERBUFFER, 0);
        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

        gl::Disable(gl::DEPTH_TEST);
        gl::Disable(gl::CULL_FACE);

        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, self.back_framebuffer.textures()[0].1);
        self.visualization_shader.set_int(c_str!("textureBack"), 0);

        gl::ActiveTexture(gl::TEXTURE1);
        gl::BindTexture(gl::TEXTURE_2D, self.front_framebuffer.textures()[0].1);
        self.visualization_shader.set_int(c_str!("textureFront"), 1);

        gl::ActiveTexture(gl::TEXTURE2);
        gl::BindTexture(
            gl::TEXTURE_3D,
            *assets.get_texture("voxels_texture").unwrap(),
        );
        self.visualization_shader
            .set_int(c_str!("voxelsTexture"), 2);

        let (width, height) = common::get_framebuffer_size();
        gl::Viewport(0, 0, width, height);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

        gl::BindVertexArray(self.quad_renderer.get_vao());
        gl::DrawElements(
            gl::TRIANGLES,
            self.quad_renderer.get_num_indices() as i32,
            gl::UNSIGNED_INT,
            std::ptr::null(),
        );
        gl::BindVertexArray(0);
    }
}
