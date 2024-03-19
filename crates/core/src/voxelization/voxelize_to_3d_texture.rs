use c_str_macro::c_str;
use engine::prelude::*;

pub unsafe fn voxelize(
    objects: &mut [Object],
    scene_aabb: &Aabb,
    camera: &Camera,
    light: &Light,
) -> Texture3D {
    let voxelization_shader =
        compile_shaders!("assets/shaders/voxel_fragment/voxelizeTo3DTexture.glsl");
    voxelization_shader.use_program();
    gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
    gl::Viewport(0, 0, 256, 256);
    gl::ColorMask(gl::FALSE, gl::FALSE, gl::FALSE, gl::FALSE);
    gl::Disable(gl::CULL_FACE);
    gl::Disable(gl::DEPTH_TEST);
    gl::Disable(gl::BLEND);
    voxelization_shader.set_mat4(c_str!("projection"), &camera.get_projection_matrix());
    voxelization_shader.set_mat4(c_str!("view"), &camera.transform.get_view_matrix());
    voxelization_shader.set_vec3(
        c_str!("pointLight.position"),
        light.transform().position.x,
        light.transform().position.y,
        light.transform().position.z,
    );
    voxelization_shader.set_vec3(c_str!("pointLight.color"), 1.0, 1.0, 1.0); // TODO: Do not hardcode to white.
    let voxels_texture = helpers::generate_3d_rgba_texture(256); // 256 voxels
    gl::BindTexture(gl::TEXTURE_3D, voxels_texture);
    helpers::bind_3d_image_texture(0, voxels_texture, gl::READ_WRITE, gl::RGBA8);
    let model_normalization_matrix = scene_aabb.normalization_matrix();
    for object in objects.iter_mut() {
        object.draw(&voxelization_shader, &model_normalization_matrix);
    }
    gl::GenerateMipmap(gl::TEXTURE_3D);
    gl::ColorMask(gl::TRUE, gl::TRUE, gl::TRUE, gl::TRUE);
    voxels_texture
}

pub struct Visualizer {
    visualization_shader: Shader,
    world_positions_shader: Shader,
    back_framebuffer: Framebuffer<1>,
    front_framebuffer: Framebuffer<1>,
    cube_renderer: Cube,
    quad_renderer: Quad,
}

pub trait GpuKernel {
    type InitInputs<'a>: 'a;
    type RunInputs<'b>: 'b;

    unsafe fn init<'a>(inputs: Self::InitInputs<'a>) -> Self;
    unsafe fn run<'b>(&self, inputs: Self::RunInputs<'b>);
}

pub struct VisualizerRunInputs<'a> {
    pub camera: &'a Camera,
    pub voxels_texture: Texture3D,
    pub mipmap_level: i32,
}

impl GpuKernel for Visualizer {
    type InitInputs<'a> = ();
    type RunInputs<'a> = VisualizerRunInputs<'a>;

    /// Initializes the visualizer.
    unsafe fn init(_: ()) -> Self {
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
        }
    }

    /// Runs the ray marching code against the voxels 3D texture.
    unsafe fn run<'a>(&self, inputs: Self::RunInputs<'a>) {
        // Use world positions shader
        self.world_positions_shader.use_program();

        // Upload camera
        self.world_positions_shader
            .set_mat4(c_str!("projection"), &inputs.camera.get_projection_matrix());
        self.world_positions_shader
            .set_mat4(c_str!("view"), &inputs.camera.transform.get_view_matrix());

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
        // TODO: Descomentar esto hace que no se vea nada.
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
            .set_mat4(c_str!("projection"), &inputs.camera.get_projection_matrix());
        self.visualization_shader
            .set_mat4(c_str!("view"), &inputs.camera.transform.get_view_matrix());
        self.visualization_shader.set_vec3(
            c_str!("cameraPosition"),
            inputs.camera.transform.position.x,
            inputs.camera.transform.position.y,
            inputs.camera.transform.position.z,
        );
        self.visualization_shader
            .set_int(c_str!("mipmapLevel"), inputs.mipmap_level);

        // Unbind framebuffer.
        // Very important because we are clearing the framebuffer later,
        // we don't want to clear this one.
        gl::BindRenderbuffer(gl::RENDERBUFFER, 0);
        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

        gl::Disable(gl::DEPTH_TEST);
        gl::Disable(gl::CULL_FACE);

        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, self.back_framebuffer.textures()[0]);
        self.visualization_shader.set_int(c_str!("textureBack"), 0);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        gl::ActiveTexture(gl::TEXTURE1);
        gl::BindTexture(gl::TEXTURE_2D, self.front_framebuffer.textures()[0]);
        self.visualization_shader.set_int(c_str!("textureFront"), 1);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        gl::ActiveTexture(gl::TEXTURE2);
        gl::BindTexture(gl::TEXTURE_3D, inputs.voxels_texture);
        self.visualization_shader
            .set_int(c_str!("voxelsTexture"), 2);
        gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_WRAP_R, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

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
