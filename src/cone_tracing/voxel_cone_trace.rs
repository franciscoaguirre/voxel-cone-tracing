use c_str_macro::c_str;
use cgmath::vec3;

use crate::{
    config::CONFIG,
    helpers,
    octree::OctreeTextures,
    rendering::{camera::Camera, light::SpotLight, quad::Quad, shader::Shader},
};

pub unsafe fn voxel_cone_trace(
    shader: &Shader,
    should_show_color: bool,
    should_show_direct: bool,
    should_show_indirect: bool,
    should_show_indirect_specular: bool,
    should_show_ambient_occlusion: bool,
    light: &SpotLight,
    cone_angle: f32,
    textures: &OctreeTextures,
    camera_view_map_positions: u32,
    camera_view_map_normals: u32,
    camera_view_map_colors: u32,
    light_maps: (u32, u32, u32),
    quad: &Quad,
    camera: &Camera,
    should_show_final_image_quad: bool,
) {
    shader.use_program();

    shader.set_uint(c_str!("voxelDimension"), CONFIG.voxel_dimension);
    shader.set_uint(c_str!("maxOctreeLevel"), CONFIG.last_octree_level);
    shader.set_bool(c_str!("shouldShowColor"), should_show_color);
    shader.set_bool(c_str!("shouldShowDirect"), should_show_direct);
    shader.set_bool(c_str!("shouldShowIndirect"), should_show_indirect);
    shader.set_bool(
        c_str!("shouldShowIndirectSpecular"),
        should_show_indirect_specular,
    );
    shader.set_bool(
        c_str!("shouldShowAmbientOcclusion"),
        should_show_ambient_occlusion,
    );
    shader.set_vec3(
        c_str!("eyePosition"),
        camera.transform.position.x,
        camera.transform.position.y,
        camera.transform.position.z,
    );
    let light_direction = vec3(
        light.transform.position.x,
        light.transform.position.y,
        light.transform.position.z,
    );
    shader.set_vec3(
        c_str!("lightDirection"),
        light_direction.x,
        light_direction.y,
        light_direction.z,
    );
    shader.set_float(c_str!("shininess"), 30.0);
    shader.set_mat4(
        c_str!("lightViewMatrix"),
        &light.transform.get_view_matrix(),
    );
    shader.set_mat4(
        c_str!("lightProjectionMatrix"),
        &light.get_projection_matrix(),
    );
    shader.set_float(c_str!("coneAngle"), cone_angle as f32);
    helpers::bind_image_texture(0, textures.node_pool.0, gl::READ_ONLY, gl::R32UI);

    let brick_pool_textures = vec![
        (
            c_str!("brickPoolColorsX"),
            textures.brick_pool_colors[0],
            gl::LINEAR as i32,
        ),
        (
            c_str!("brickPoolColorsXNeg"),
            textures.brick_pool_colors[1],
            gl::LINEAR as i32,
        ),
        (
            c_str!("brickPoolColorsY"),
            textures.brick_pool_colors[2],
            gl::LINEAR as i32,
        ),
        (
            c_str!("brickPoolColorsYNeg"),
            textures.brick_pool_colors[3],
            gl::LINEAR as i32,
        ),
        (
            c_str!("brickPoolColorsZ"),
            textures.brick_pool_colors[4],
            gl::LINEAR as i32,
        ),
        (
            c_str!("brickPoolColorsZNeg"),
            textures.brick_pool_colors[5],
            gl::LINEAR as i32,
        ),
        (
            c_str!("brickPoolNormals"),
            textures.brick_pool_normals,
            gl::NEAREST as i32,
        ),
        // Irradiance textures
        (
            c_str!("brickPoolIrradianceX"),
            textures.brick_pool_irradiance[0],
            gl::NEAREST as i32,
        ),
        (
            c_str!("brickPoolIrradianceXNeg"),
            textures.brick_pool_irradiance[1],
            gl::NEAREST as i32,
        ),
        (
            c_str!("brickPoolIrradianceY"),
            textures.brick_pool_irradiance[2],
            gl::NEAREST as i32,
        ),
        (
            c_str!("brickPoolIrradianceYNeg"),
            textures.brick_pool_irradiance[3],
            gl::NEAREST as i32,
        ),
        (
            c_str!("brickPoolIrradianceZ"),
            textures.brick_pool_irradiance[4],
            gl::NEAREST as i32,
        ),
        (
            c_str!("brickPoolIrradianceZNeg"),
            textures.brick_pool_irradiance[5],
            gl::NEAREST as i32,
        ),
    ];

    let mut texture_counter = 0;

    for &(texture_name, texture, sample_interpolation) in brick_pool_textures.iter() {
        gl::ActiveTexture(gl::TEXTURE0 + texture_counter);
        gl::BindTexture(gl::TEXTURE_3D, texture);
        shader.set_int(texture_name, texture_counter as i32);
        gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_WRAP_R, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_MIN_FILTER, sample_interpolation);
        gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_MAG_FILTER, sample_interpolation);
        texture_counter += 1;
    }

    // let g_buffer_textures = vec![
    //     (c_str!("gBufferColors"), eye_view_map_colors),
    //     (c_str!("gBufferPositions"), eye_view_map),
    //     (c_str!("gBufferNormals"), eye_view_map_normals),
    // ];
    let g_buffer_textures = vec![
        (c_str!("gBufferColors"), camera_view_map_colors),
        (c_str!("gBufferPositions"), camera_view_map_positions),
        (c_str!("gBufferNormals"), camera_view_map_normals),
    ];

    for &(texture_name, texture) in g_buffer_textures.iter() {
        gl::ActiveTexture(gl::TEXTURE0 + texture_counter);
        gl::BindTexture(gl::TEXTURE_2D, texture);
        shader.set_int(texture_name, texture_counter as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        texture_counter += 1;
    }

    gl::ActiveTexture(gl::TEXTURE0 + texture_counter);
    gl::BindTexture(gl::TEXTURE_2D, light_maps.2);
    shader.set_int(c_str!("shadowMap"), texture_counter as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

    let quad_vao = quad.get_vao();

    if should_show_final_image_quad {
        gl::BindVertexArray(quad_vao);
        gl::DrawElements(
            gl::TRIANGLES,
            quad.get_num_indices() as i32,
            gl::UNSIGNED_INT,
            std::ptr::null(),
        );
        gl::BindVertexArray(0);
    }

    // let (debug, buffer) = helpers::generate_texture_buffer(100, gl::R32F, 69f32);
    // helpers::bind_image_texture(4, debug, gl::WRITE_ONLY, gl::R32F);
    // our_model.draw(&shader);
    // let debug_values = helpers::get_values_from_texture_buffer(buffer, 100, 420f32);
    // dbg!(&debug_values[..20]);

    // Show normals
    // render_normals_shader.use_program();
    // render_normals_shader.set_mat4(c_str!("projection"), &projection);
    // render_normals_shader.set_mat4(c_str!("view"), &view);
    // render_normals_shader.set_mat4(c_str!("model"), &model_normalization_matrix);
    // our_model.draw(&render_normals_shader);
}
