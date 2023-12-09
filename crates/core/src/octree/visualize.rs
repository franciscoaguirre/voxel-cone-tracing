use std::{ffi::c_void, mem::size_of};

use c_str_macro::c_str;
use cgmath::{vec3, Matrix4, Vector3};
use serde::{Serialize, Deserialize};
use engine::prelude::*;

use crate::config::Config;
use super::{NodeData, Octree};

impl Octree {
    pub unsafe fn render(
        &self,
        model: &Matrix4<f32>,
        view: &Matrix4<f32>,
        projection: &Matrix4<f32>,
        octree_level: u32,
        color_direction: Vector3<f32>,
        should_show_normals: bool,
        brick_attribute: BrickAttribute,
        brick_padding: f32, // Between 0 and 1
        node_data: &NodeData,
    ) {
        if should_show_normals {
            self.show_normals(octree_level, projection, view, model);
        }

        if self.renderer.bricks_to_show.at_least_one() {
            self.show_bricks(
                octree_level,
                projection,
                view,
                model,
                color_direction,
                brick_attribute,
                brick_padding,
            );
        } else {
            self.renderer.shader.use_program();
            self.renderer.shader.bind_image_texture_with_format(0, self.textures.node_positions, TextureFormat::Rgb10A2Ui);
            self.renderer.shader.bind_image_texture(1, node_data.level_start_indices);

            let config = Config::instance();

            self.renderer
                .shader
                .set_uint(c_str!("octreeLevel"), octree_level);
            self.renderer
                .shader
                .set_uint(c_str!("voxelDimension"), config.voxel_dimension());

            self.renderer
                .shader
                .set_mat4(c_str!("projection"), projection);
            self.renderer.shader.set_mat4(c_str!("view"), view);
            self.renderer.shader.set_mat4(c_str!("model"), model);

            let vao = Vao::new();
            vao.draw_points(
                // Use necessary per level
                node_data.nodes_per_level[octree_level as usize] as i32,
            );
        }
    }

    unsafe fn show_normals(
        &self,
        octree_level: u32,
        projection: &Matrix4<f32>,
        view: &Matrix4<f32>,
        model: &Matrix4<f32>,
    ) {
        self.renderer.normals_shader.use_program();

        self.renderer
            .normals_shader
            .set_mat4(c_str!("projection"), projection);
        self.renderer.normals_shader.set_mat4(c_str!("view"), view);
        self.renderer
            .normals_shader
            .set_mat4(c_str!("model"), model);

        let config = Config::instance();

        self.renderer
            .normals_shader
            .set_uint(c_str!("voxelDimension"), config.voxel_dimension());
        self.renderer
            .normals_shader
            .set_uint(c_str!("octreeLevel"), octree_level);

        self.renderer
            .normals_shader
            .bind_image_texture(0, self.geometry_data.level_start_indices, TextureAccess::ReadOnly);

        self.renderer
            .normals_shader
            .bind_image_texture_with_format(1, self.textures.node_positions, TextureAccess::ReadOnly, TextureFormat::Rgb10A2Ui);

        self.renderer
            .normals_shader
            .bind_3d_image_texture(2, self.textures.brick_pool_normals, TextureAccess::ReadOnly, TextureFormat::Rgba32f);

        let vao = Vao::new();

        let all_bricks_to_show: u32 = self.renderer.bricks_to_show.into();
        if (all_bricks_to_show & 1) > 0 {
            self.renderer
                .normals_shader
                .set_uint(c_str!("bricksToShow"), all_bricks_to_show & 1);

            vao.draw_points(
                self.geometry_data.node_data.nodes_per_level[octree_level as usize] as i32,
            );
        }

        if (all_bricks_to_show & 2) > 0 {
            self.renderer
                .normals_shader
                .set_uint(c_str!("bricksToShow"), all_bricks_to_show & 2);

            vao.draw_points(
                self.geometry_data.node_data.nodes_per_level[octree_level as usize] as i32,
            );
        }

        if (all_bricks_to_show & 4) > 0 {
            self.renderer
                .normals_shader
                .set_uint(c_str!("bricksToShow"), all_bricks_to_show & 4);

            vao.draw_points(
                self.geometry_data.node_data.nodes_per_level[octree_level as usize] as i32,
            );
        }
    }

    unsafe fn show_bricks(
        &self,
        octree_level: u32,
        projection: &Matrix4<f32>,
        view: &Matrix4<f32>,
        model: &Matrix4<f32>,
        color_direction: Vector3<f32>,
        brick_attribute: BrickAttribute,
        brick_padding: f32, // Between 0 and 1
    ) {
        self.renderer.bricks_shader.use_program();

        self.renderer
            .bricks_shader
            .set_mat4(c_str!("projection"), projection);
        self.renderer.bricks_shader.set_mat4(c_str!("view"), view);
        self.renderer.bricks_shader.set_mat4(c_str!("model"), model);

        let config = Config::instance();

        self.renderer
            .bricks_shader
            .set_uint(c_str!("voxelDimension"), config.voxel_dimension());
        self.renderer
            .bricks_shader
            .set_uint(c_str!("octreeLevel"), octree_level);
        self.renderer
            .bricks_shader
            .set_uint(c_str!("maxOctreeLevel"), config.octree_levels());
        self.renderer
            .bricks_shader
            .set_uint(c_str!("mode"), brick_attribute.into());
        self.renderer
            .bricks_shader
            .set_float(c_str!("brickPadding"), brick_padding);

        let color_direction = if octree_level == config.last_octree_level() {
            vec3(1.0, 0.0, 0.0)
        } else {
            color_direction
        };

        self.renderer.bricks_shader.set_vec3(
            c_str!("colorDirection"),
            color_direction.x,
            color_direction.y,
            color_direction.z,
        );

        self.renderer.bricks_shader.bind_image_texture_with_format(
            0,
            self.textures.node_positions,
            TextureAccess::ReadOnly,
            TextureFormat::Rgb10A2Ui,
        );
        self.renderer.bricks_shader.bind_image_texture(
            1,
            self.geometry_data.node_data.level_start_indices,
            TextureAccess::ReadOnly,
        );

        let color_textures = vec![
            (
                c_str!("brickPoolColorsX"),
                self.textures.brick_pool_colors[0],
            ),
            (
                c_str!("brickPoolColorsXNeg"),
                self.textures.brick_pool_colors[1],
            ),
            (
                c_str!("brickPoolColorsY"),
                self.textures.brick_pool_colors[2],
            ),
            (
                c_str!("brickPoolColorsYNeg"),
                self.textures.brick_pool_colors[3],
            ),
            (
                c_str!("brickPoolColorsZ"),
                self.textures.brick_pool_colors[4],
            ),
            (
                c_str!("brickPoolColorsZNeg"),
                self.textures.brick_pool_colors[5],
            ),
            // Irradiance textures
            (
                c_str!("brickPoolIrradianceX"),
                self.textures.brick_pool_irradiance[0],
            ),
            (
                c_str!("brickPoolIrradianceXNeg"),
                self.textures.brick_pool_irradiance[1],
            ),
            (
                c_str!("brickPoolIrradianceY"),
                self.textures.brick_pool_irradiance[2],
            ),
            (
                c_str!("brickPoolIrradianceYNeg"),
                self.textures.brick_pool_irradiance[3],
            ),
            (
                c_str!("brickPoolIrradianceZ"),
                self.textures.brick_pool_irradiance[4],
            ),
            (
                c_str!("brickPoolIrradianceZNeg"),
                self.textures.brick_pool_irradiance[5],
            ),
        ];

        for &(texture_name, texture) in color_textures.iter() {
            self.renderer
                .bricks_shader
                .bind_3d_texture(texture_name, texture, false);
        }

        let vao = Vao::new();
        let all_bricks_to_show: u32 = self.renderer.bricks_to_show.into();
        for z_layer in 0..3 {
            let mask = 2u32.pow(z_layer);
            let brick_layer_to_show: u32 = self.renderer.bricks_to_show.into();
            if brick_layer_to_show & mask != 0 {
                for x_layer in 0..3 {
                    self.renderer
                        .bricks_shader
                        .set_uint(c_str!("bricksToShow"), z_layer * 3 + x_layer);

                    vao.draw_points(
                        self.geometry_data.node_data.nodes_per_level[octree_level as usize] as i32,
                    );
                }
            }
        }
    }

    pub fn set_bricks_to_show(&mut self, bricks_to_show: BricksToShow) {
        self.renderer.bricks_to_show = bricks_to_show;
    }

    pub unsafe fn set_node_indices(&mut self, node_indices: &Vec<u32>) {
        if node_indices.is_empty() {
            let vao = Vao::new();
            vao.bind(); // TODO: Why do we do this?
            self.renderer.node_count = 0;
            self.renderer.vao = vao;
            return;
        }

        let mut vao = 0;
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        let mut vbo = 0;
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (node_indices.len() * size_of::<u32>()) as isize,
            &node_indices[0] as *const u32 as *const c_void,
            gl::DYNAMIC_DRAW,
        );
        gl::VertexAttribIPointer(
            0,
            1,
            gl::UNSIGNED_INT,
            size_of::<u32>() as i32,
            0 as *const c_void,
        );
        gl::EnableVertexAttribArray(0);

        self.renderer.node_count = node_indices.len() as u32;
        self.renderer.vao = vao;
    }

    pub unsafe fn run_node_positions_shader(
        &self,
        projection: &Matrix4<f32>,
        view: &Matrix4<f32>,
        model: &Matrix4<f32>,
    ) {
        if self.renderer.node_count == 0 {
            return;
        }

        self.renderer.node_positions_shader.use_program();

        let config = Config::instance();

        self.renderer
            .node_positions_shader
            .set_uint(c_str!("voxelDimension"), config.voxel_dimension());
        self.renderer
            .node_positions_shader
            .set_uint(c_str!("maxOctreeLevel"), config.octree_levels() + 1);

        self.renderer
            .node_positions_shader
            .set_mat4(c_str!("projection"), &projection);
        self.renderer
            .node_positions_shader
            .set_mat4(c_str!("view"), &view);
        self.renderer
            .node_positions_shader
            .set_mat4(c_str!("model"), &model);

        self.renderer
            .node_positions_shader
            .bind_image_texture_with_format(
                0,
                self.textures.node_positions,
                TextureAccess::ReadOnly,
                TextureFormat::Rgb10A2Ui,
            );

        self.renderer
            .node_positions_shader
            .bind_image_texture(
                1,
                self.geometry_data.node_data.level_start_indices,
                TextureAccess::ReadOnly,
            );

        self.renderer
            .node_positions_shader
            .bind_image_texture(
                2,
                self.border_data.node_data.level_start_indices,
                TextureAccess::ReadOnly,
            );

        self.renderer.vao.draw_points(self.renderer.node_count as i32);
    }

    pub unsafe fn run_node_neighbors_shader(
        &self,
        projection: &Matrix4<f32>,
        view: &Matrix4<f32>,
        model: &Matrix4<f32>,
    ) {
        if self.renderer.node_count == 0 {
            return;
        }

        let config = Config::instance();

        self.renderer.node_neighbors_shader.use_program();

        self.renderer
            .node_neighbors_shader
            .set_uint(c_str!("voxelDimension"), config.voxel_dimension());
        self.renderer
            .node_neighbors_shader
            .set_uint(c_str!("maxOctreeLevel"), config.octree_levels() + 1);

        self.renderer
            .node_neighbors_shader
            .set_mat4(c_str!("projection"), &projection);
        self.renderer
            .node_neighbors_shader
            .set_mat4(c_str!("view"), &view);
        self.renderer
            .node_neighbors_shader
            .set_mat4(c_str!("model"), &model);

        self.renderer
            .node_neighbors_shader
            .bind_image_texture_with_format(
                0,
                self.textures.node_positions,
                TextureAccess::ReadOnly,
                TextureFormat::Rgb10A2Ui,
            );
        self.renderer
            .node_neighbors_shader
            .bind_image_texture(
                1,
                self.textures.node_pool,
                TextureAccess::ReadOnly,
            );
        self.renderer
            .node_neighbors_shader
            .bind_image_texture(
                2,
                self.geometry_data.node_data.level_start_indices,
                TextureAccess::ReadOnly,
            );
        self.renderer
            .node_neighbors_shader
            .bind_image_texture(
                3,
                self.border_data.node_data.level_start_indices,
                TextureAccess::ReadOnly,
            );

        for texture_offset in 0..(self.textures.neighbors.len() / 2) {
            self.renderer
                .node_neighbors_shader
                .bind_image_texture(
                    3 + texture_offset as u32,
                    self.textures.neighbors[texture_offset as usize],
                    TextureAccess::ReadOnly,
                );
        }
        self.renderer.vao.draw_points(self.renderer.node_count as i32);

        for texture_offset in 0..(self.textures.neighbors.len() / 2) {
            self.renderer
                .node_neighbors_shader
                .bind_image_texture(
                    3 + texture_offset as u32,
                    self.textures.neighbors[(texture_offset + 3) as usize],
                    TextureAccess::ReadOnly,
                );
        }
        self.renderer.vao.draw_points(self.renderer.node_count as i32);
    }

    pub unsafe fn run_get_photons_shader(&self, node_index: u32) {
        self.renderer.get_photons_shader.use_program();

        let config = Config::instance();

        self.renderer
            .get_photons_shader
            .set_uint(c_str!("nodeID"), node_index);
        self.renderer
            .get_photons_shader
            .set_uint(c_str!("voxelDimension"), config.voxel_dimension());

        self.renderer
            .get_photons_shader
            .bind_image_texture(
                0,
                self.textures.photons_buffer,
                TextureAccess::WriteOnly
            );
        self.renderer
            .get_photons_shader
            .bind_3d_image_texture(
                1,
                self.textures.brick_pool_photons,
                TextureAccess::ReadOnly,
                TextureFormat::R32Ui,
            );

        self.renderer.get_photons_shader.dispatch(1);
        self.renderer.get_photons_shader.wait();
    }

    pub unsafe fn run_get_children_shader(&self, node_index: u32) {
        self.renderer.get_children_shader.use_program();

        self.renderer
            .get_children_shader
            .set_uint(c_str!("nodeID"), node_index);

        self.renderer
            .get_children_shader
            .bind_image_texture(
                0,
                self.textures.children_buffer,
                TextureAccess::WriteOnly,
            );
        self.renderer
            .get_children_shader
            .bind_image_texture(
                1,
                self.textures.node_pool,
                TextureAccess::ReadOnly,
            );

        self.renderer.get_children_shader.dispatch(1);
        self.renderer.get_children_shader.wait();
    }

    pub unsafe fn run_node_bricks_shader(
        &self,
        projection: &Matrix4<f32>,
        view: &Matrix4<f32>,
        model: &Matrix4<f32>,
        color_direction: Vector3<f32>,
        brick_attribute: BrickAttribute,
        brick_padding: f32, // Between 0 and 1
    ) {
        if self.renderer.node_count == 0 {
            return;
        }

        self.renderer.node_bricks_shader.use_program();

        self.renderer
            .node_bricks_shader
            .set_float(c_str!("brickPadding"), brick_padding);

        self.renderer.node_bricks_shader.set_vec3(
            c_str!("colorDirection"),
            color_direction.x,
            color_direction.y,
            color_direction.z,
        );

        let config = Config::instance();

        self.renderer
            .node_bricks_shader
            .set_uint(c_str!("voxelDimension"), config.voxel_dimension());
        self.renderer
            .node_bricks_shader
            .set_uint(c_str!("maxOctreeLevel"), config.octree_levels() + 1);

        self.renderer
            .node_bricks_shader
            .set_mat4(c_str!("projection"), &projection);
        self.renderer
            .node_bricks_shader
            .set_mat4(c_str!("view"), &view);
        self.renderer
            .node_bricks_shader
            .set_mat4(c_str!("model"), &model);

        self.renderer
            .node_bricks_shader
            .set_uint(c_str!("mode"), brick_attribute.into());

        self.renderer
            .node_bricks_shader
            .bind_image_texture_with_format(
                0,
                self.textures.node_positions,
                TextureAccess::ReadOnly,
                TextureFormat::Rgb10A2Ui,
            );
        self.renderer
            .node_bricks_shader
            .bind_image_texture(
                1,
                self.geometry_data.node_data.level_start_indices,
                TextureAccess::ReadOnly,
            );
        self.renderer
            .node_bricks_shader
            .bind_3d_image_texture(
                2,
                self.textures.brick_pool_colors[0 as usize], // TODO: Use `color_direction`
                TextureAccess::ReadOnly,
                TextureFormat::Rgba8,
            );

        let color_textures = vec![
            (
                c_str!("brickPoolColorsX"),
                self.textures.brick_pool_colors[0],
            ),
            (
                c_str!("brickPoolColorsXNeg"),
                self.textures.brick_pool_colors[1],
            ),
            (
                c_str!("brickPoolColorsY"),
                self.textures.brick_pool_colors[2],
            ),
            (
                c_str!("brickPoolColorsYNeg"),
                self.textures.brick_pool_colors[3],
            ),
            (
                c_str!("brickPoolColorsZ"),
                self.textures.brick_pool_colors[4],
            ),
            (
                c_str!("brickPoolColorsZNeg"),
                self.textures.brick_pool_colors[5],
            ),
            // Irradiance textures
            (
                c_str!("brickPoolIrradianceX"),
                self.textures.brick_pool_irradiance[0],
            ),
            (
                c_str!("brickPoolIrradianceXNeg"),
                self.textures.brick_pool_irradiance[1],
            ),
            (
                c_str!("brickPoolIrradianceY"),
                self.textures.brick_pool_irradiance[2],
            ),
            (
                c_str!("brickPoolIrradianceYNeg"),
                self.textures.brick_pool_irradiance[3],
            ),
            (
                c_str!("brickPoolIrradianceZ"),
                self.textures.brick_pool_irradiance[4],
            ),
            (
                c_str!("brickPoolIrradianceZNeg"),
                self.textures.brick_pool_irradiance[5],
            ),
        ];

        for &(texture_name, texture) in color_textures.iter() {
            self.renderer
                .node_bricks_shader
                .bind_3d_texture(texture_name, texture, false);
        }

        self.renderer
            .node_bricks_shader
            .bind_3d_image_texture(
                3,
                self.textures.brick_pool_photons,
                TextureAccess::ReadOnly,
                TextureFormat::R32Ui,
            );
        self.renderer
            .node_bricks_shader
            .bind_3d_image_texture(
                4,
                self.textures.brick_pool_normals,
                TextureAccess::ReadOnly,
                TextureFormat::Rgba32f,
            );
        self.renderer
            .node_bricks_shader
            .bind_image_texture(
                5,
                self.border_data.node_data.level_start_indices,
                TextureAccess::ReadOnly,
            );

        for z_layer in 0..3 {
            let mask = 2u32.pow(z_layer);
            let brick_layer_to_show: u32 = self.renderer.bricks_to_show.into();
            if brick_layer_to_show & mask != 0 {
                for x_layer in 0..3 {
                    self.renderer
                        .node_bricks_shader
                        .set_uint(c_str!("bricksToShow"), z_layer * 3 + x_layer);

                    self.renderer.vao.draw_points(self.renderer.node_count as i32);
                }
            }
        }
    }

    pub unsafe fn run_colors_quad_shader(&self, node_index: u32) {
        self.renderer.get_colors_quad_shader.use_program();

        let config = Config::instance();

        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        self.renderer
            .get_colors_quad_shader
            .set_uint(c_str!("nodeID"), node_index);
        self.renderer.get_colors_quad_shader.set_float(
            c_str!("brickPoolResolutionf"),
            config.brick_pool_resolution as f32,
        );

        let mut fbo = 0;
        gl::GenFramebuffers(1, &mut fbo);
        gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);

        let (viewport_width, viewport_height) = config.viewport_dimensions();

        let mut rbo = 0;
        gl::GenRenderbuffers(1, &mut rbo);
        gl::BindRenderbuffer(gl::RENDERBUFFER, rbo);
        gl::RenderbufferStorage(
            gl::RENDERBUFFER,
            gl::DEPTH24_STENCIL8,
            viewport_width,
            viewport_height,
        );
        gl::FramebufferRenderbuffer(
            gl::FRAMEBUFFER,
            gl::DEPTH_STENCIL_ATTACHMENT,
            gl::RENDERBUFFER,
            rbo,
        );
        gl::BindRenderbuffer(gl::RENDERBUFFER, 0);

        gl::FramebufferTexture2D(
            gl::FRAMEBUFFER,
            gl::COLOR_ATTACHMENT0,
            gl::TEXTURE_2D,
            self.textures.color_quad_textures[0].0,
            0,
        );

        gl::FramebufferTexture2D(
            gl::FRAMEBUFFER,
            gl::COLOR_ATTACHMENT1,
            gl::TEXTURE_2D,
            self.textures.color_quad_textures[1].0,
            0,
        );

        gl::DrawBuffers(2, [gl::COLOR_ATTACHMENT0, gl::COLOR_ATTACHMENT1].as_ptr());

        if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
            println!("ERROR::FRAMEBUFFER: Framebuffer is not complete!");
        }

        let quad = Quad::new();

        let (debug, buffer) = helpers::generate_texture_buffer(3, gl::R32F, 69f32);
        helpers::bind_image_texture(0, debug, gl::WRITE_ONLY, gl::R32F);

        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_3D, self.textures.brick_pool_colors[0]); // TODO: Visualize other directions
        self.renderer
            .get_colors_quad_shader
            .set_int(c_str!("brickPoolColors"), 0);
        gl::BindVertexArray(quad.get_vao());

        gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_WRAP_R, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        gl::Enable(gl::DEPTH_TEST);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        self.renderer
            .get_colors_quad_shader
            .set_bool(c_str!("isNeighbor"), false);
        gl::DrawElements(
            gl::TRIANGLES,
            quad.get_num_indices() as i32,
            gl::UNSIGNED_INT,
            std::ptr::null(),
        );

        self.renderer
            .get_colors_quad_shader
            .set_bool(c_str!("isNeighbor"), true);
        gl::DrawElements(
            gl::TRIANGLES,
            quad.get_num_indices() as i32,
            gl::UNSIGNED_INT,
            std::ptr::null(),
        );

        gl::BindVertexArray(0);
        gl::BindTexture(gl::TEXTURE_3D, 0);
        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum BrickAttribute {
    None,
    Color,
    Photons,
}

impl Default for BrickAttribute {
    fn default() -> Self {
        Self::None
    }
}

impl Into<u32> for BrickAttribute {
    fn into(self) -> u32 {
        use BrickAttribute::*;
        match self {
            None => 0,
            Color => 1,
            Photons => 2,
        }
    }
}

impl BrickAttribute {
    pub fn next(self) -> Self {
        use BrickAttribute::*;
        match self {
            None => Color,
            Color => Photons,
            Photons => None,
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy)]
pub struct BricksToShow {
    show_z0: bool,
    show_z1: bool,
    show_z2: bool,
}

impl BricksToShow {
    pub fn toggle_z0(&mut self) {
        self.show_z0 = !self.show_z0;
    }

    pub fn toggle_z1(&mut self) {
        self.show_z1 = !self.show_z1;
    }

    pub fn toggle_z2(&mut self) {
        self.show_z2 = !self.show_z2;
    }

    pub fn at_least_one(&self) -> bool {
        self.show_z0 || self.show_z1 || self.show_z2
    }

    pub fn z0(&self) -> bool {
        self.show_z0
    }

    pub fn z1(&self) -> bool {
        self.show_z1
    }

    pub fn z2(&self) -> bool {
        self.show_z2
    }
}

impl Into<u32> for BricksToShow {
    fn into(self) -> u32 {
        self.show_z0 as u32 | (self.show_z1 as u32) << 1 | (self.show_z2 as u32) << 2 as u32
    }
}
