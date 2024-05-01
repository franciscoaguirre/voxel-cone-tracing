use std::mem::size_of;

use c_str_macro::c_str;
use cgmath::point3;
use engine::prelude::*;
use gl::types::*;

use crate::config::Config;

#[derive(Pausable)]
pub struct Voxelizer {
    voxelization_shader: Shader,
    atomic_counter: u32,
    voxel_positions: (GLuint, GLuint),
    voxel_colors: (GLuint, GLuint),
    voxel_normals: (GLuint, GLuint),
    paused: bool,
}

impl Voxelizer {
    pub unsafe fn new() -> Self {
        Self {
            voxelization_shader: compile_shaders!(
                "assets/shaders/voxel_fragment/voxelize.vert.glsl",
                "assets/shaders/voxel_fragment/voxelize.frag.glsl",
                "assets/shaders/voxel_fragment/voxelize.geom.glsl",
            ),
            atomic_counter: helpers::generate_atomic_counter_buffer(),
            voxel_positions: helpers::initialize_texture_buffer(gl::R32UI),
            voxel_colors: helpers::initialize_texture_buffer(gl::RGBA8),
            voxel_normals: helpers::initialize_texture_buffer(gl::RGBA32F),
            paused: false,
        }
    }

    /// Gets the number of voxel fragments that would be generated by the
    /// voxelization algorithm.
    unsafe fn calculate_voxel_fragment_list_length(&self, inputs: &SystemInputs) {
        self.voxelization_shader.use_program();
        self.voxelization_shader
            .set_bool(c_str!("shouldStore"), false);
        self.voxelization_shader.set_bool(c_str!("hasBump"), false);
        self.voxelize_scene(inputs);
    }

    /// Runs the voxelization shader.
    unsafe fn voxelize_scene(&self, inputs: &SystemInputs) {
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        let config = Config::instance();
        gl::Viewport(
            0,
            0,
            config.voxel_dimension() as i32,
            config.voxel_dimension() as i32,
        );
        let model_normalization_matrix = inputs.scene.aabb.normalization_matrix();
        self.voxelization_shader.set_mat4(
            c_str!("modelNormalizationMatrix"),
            &model_normalization_matrix,
        );
        self.voxelization_shader
            .set_int(c_str!("voxelDimension"), config.voxel_dimension() as i32);
        gl::BindBufferBase(gl::ATOMIC_COUNTER_BUFFER, 0, self.atomic_counter);
        self.voxelization_shader
            .set_vec3(c_str!("fallbackColor"), 1.0, 1.0, 1.0);
        let ortho = cgmath::ortho(-1.0, 1.0, -1.0, 1.0, 0.0001, 10_000.0);

        let mut right_camera = Transform::default();
        right_camera.position = point3(-2.0, 0.0, 0.0);
        right_camera.set_rotation_y(0.0);
        let right_view_matrix = ortho * right_camera.get_view_matrix();

        let mut top_camera = Transform::default();
        top_camera.position = point3(0.0, 2.0, 0.0);
        top_camera.set_rotation_x(-90.0);
        top_camera.set_rotation_y(90.0);
        let top_view_matrix = ortho * top_camera.get_view_matrix();

        let mut far_camera = Transform::default();
        far_camera.position = point3(0.0, 0.0, 2.0);
        far_camera.set_rotation_y(-90.0);
        let far_view_matrix = ortho * far_camera.get_view_matrix();
        self.voxelization_shader.set_mat4_array(
            c_str!("axisProjections"),
            &[&right_view_matrix, &top_view_matrix, &far_view_matrix],
        );
        gl::Disable(gl::CULL_FACE);
        gl::Disable(gl::DEPTH_TEST);
        for object in inputs.scene.objects.iter() {
            object.borrow_mut().draw(
                &self.voxelization_shader,
                &model_normalization_matrix,
                inputs.assets,
            );
        }
        let (viewport_width, viewport_height) = config.viewport_dimensions();
        gl::ColorMask(gl::TRUE, gl::TRUE, gl::TRUE, gl::TRUE);
        gl::Viewport(0, 0, viewport_width, viewport_height);
    }

    /// Populates the voxel fragment list, comprised of the following textures:
    /// - voxel_positions
    /// - voxel_colors
    /// - voxel_normals
    unsafe fn populate_voxel_fragment_list(&self, inputs: &SystemInputs) {
        self.voxelization_shader.use_program();
        self.voxelization_shader
            .set_bool(c_str!("shouldStore"), true);
        self.voxelization_shader.set_bool(c_str!("hasBump"), false);

        helpers::bind_image_texture(0, self.voxel_positions.0, gl::WRITE_ONLY, gl::RGB10_A2UI);
        helpers::bind_image_texture(1, self.voxel_colors.0, gl::WRITE_ONLY, gl::RGBA8);
        helpers::bind_image_texture(2, self.voxel_normals.0, gl::WRITE_ONLY, gl::RGBA32F);

        self.voxelize_scene(inputs);
    }
}

impl System for Voxelizer {
    unsafe fn setup(&mut self, assets: &mut AssetRegistry) {
        assets.register_uniform("number_of_voxel_fragments", Uniform::Uint(0));
        assets.register_texture("voxel_positions", self.voxel_positions.0);
        assets.register_texture("voxel_colors", self.voxel_colors.0);
        assets.register_texture("voxel_normals", self.voxel_normals.0);
    }

    unsafe fn update(&mut self, inputs: SystemInputs) {
        helpers::reset_atomic_counter(self.atomic_counter);
        self.calculate_voxel_fragment_list_length(&inputs);
        gl::MemoryBarrier(gl::ATOMIC_COUNTER_BUFFER);
        gl::BindBuffer(gl::ATOMIC_COUNTER_BUFFER, self.atomic_counter);
        let count = gl::MapBufferRange(
            gl::ATOMIC_COUNTER_BUFFER,
            0,
            size_of::<GLuint>() as isize,
            gl::MAP_READ_BIT | gl::MAP_WRITE_BIT,
        ) as *mut GLuint;
        let number_of_voxel_fragments = *count;
        helpers::fill_texture_buffer_with_data(
            self.voxel_positions.1,
            &vec![0u32; number_of_voxel_fragments as usize],
            gl::STATIC_DRAW,
        );
        helpers::fill_texture_buffer_with_data(
            self.voxel_colors.1,
            &vec![0u32; number_of_voxel_fragments as usize],
            gl::STATIC_DRAW,
        );
        helpers::fill_texture_buffer_with_data(
            self.voxel_normals.1,
            &vec![0u32; number_of_voxel_fragments as usize],
            gl::STATIC_DRAW,
        );
        *count = 0;
        gl::UnmapBuffer(gl::ATOMIC_COUNTER_BUFFER);
        gl::BindBuffer(gl::ATOMIC_COUNTER_BUFFER, 0);
        self.populate_voxel_fragment_list(&inputs);
        gl::MemoryBarrier(gl::SHADER_IMAGE_ACCESS_BARRIER_BIT);
        // TODO: Cannot borrow `&'a AssetRegistry` as mutable.
        *inputs
            .assets
            .get_uniform_mut("number_of_voxel_fragments")
            .unwrap() = Uniform::Uint(number_of_voxel_fragments);
    }

    fn get_info(&self) -> SystemInfo {
        SystemInfo {
            name: "SVOVoxelizer",
        }
    }
}
