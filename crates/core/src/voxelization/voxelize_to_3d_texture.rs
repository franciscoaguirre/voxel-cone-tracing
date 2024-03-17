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
    helpers::bind_3d_image_texture(0, voxels_texture, gl::READ_WRITE, gl::RGBA8);
    let model_normalization_matrix = scene_aabb.normalization_matrix();
    for object in objects.iter_mut() {
        object.draw(&voxelization_shader, &model_normalization_matrix);
    }
    gl::ColorMask(gl::TRUE, gl::TRUE, gl::TRUE, gl::TRUE);
    voxels_texture
}

/// Visualizes the voxel 3D texture.
/// In order to do this, creates a front and back textures with the world positions of a unit cube.
/// Then we render a quad where we ray march our voxels 3D texture.
pub unsafe fn visualize(camera: &Camera, voxels_texture: Texture3D) {
    // let world_positions_shader =
    //     compile_shaders!("assets/shaders/voxel_fragment/worldPositions.glsl");
    // world_positions_shader.use_program();

    // Upload camera
    // world_positions_shader.set_mat4(c_str!("projection"), &camera.get_projection_matrix());
    // world_positions_shader.set_mat4(c_str!("view"), &camera.transform.get_view_matrix());

    // Settings
    // gl::ClearColor(0.0, 0.0, 0.0, 1.0);
    // gl::Enable(gl::CULL_FACE);
    // gl::Enable(gl::DEPTH_TEST);

    // let cube = Cube::new();

    // Back
    // let back_framebuffer = Framebuffer::<1>::new();
    // gl::CullFace(gl::FRONT);
    // gl::BindFramebuffer(gl::FRAMEBUFFER, back_framebuffer.fbo());
    // let (width, height) = common::get_framebuffer_size();
    // gl::Viewport(0, 0, width, height);
    // gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    // gl::BindVertexArray(cube.get_vao());
    // gl::DrawElements(
    //     gl::TRIANGLES,
    //     cube.get_num_indices() as i32,
    //     gl::UNSIGNED_INT,
    //     std::ptr::null(),
    // );
    // gl::BindVertexArray(0);

    // Front
    // let front_framebuffer = Framebuffer::<1>::new();
    // gl::CullFace(gl::BACK);
    // gl::BindFramebuffer(gl::FRAMEBUFFER, front_framebuffer.fbo());
    // let (width, height) = common::get_framebuffer_size();
    // gl::Viewport(0, 0, width, height);
    // gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    // gl::BindVertexArray(cube.get_vao());
    // gl::DrawElements(
    //     gl::TRIANGLES,
    //     cube.get_num_indices() as i32,
    //     gl::UNSIGNED_INT,
    //     std::ptr::null(),
    // );
    // gl::BindVertexArray(0);

    let visualization_shader =
        compile_shaders!("assets/shaders/voxel_fragment/visualizeVoxel3DTexture.glsl");
    visualization_shader.use_program();

    // Upload camera
    visualization_shader.set_mat4(c_str!("projection"), &camera.get_projection_matrix());
    visualization_shader.set_mat4(c_str!("view"), &camera.transform.get_view_matrix());
    visualization_shader.set_vec3(
        c_str!("cameraPosition"),
        camera.transform.position.x,
        camera.transform.position.y,
        camera.transform.position.z,
    );

    // Unbind framebuffer
    // gl::BindRenderbuffer(gl::RENDERBUFFER, 0);
    // gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

    // gl::Disable(gl::DEPTH_TEST);
    // gl::Enable(gl::CULL_FACE);

    // gl::ActiveTexture(gl::TEXTURE0);
    // gl::BindTexture(gl::TEXTURE_2D, back_framebuffer.textures()[0]);
    // visualization_shader.set_int(c_str!("textureBack"), 0);
    // gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
    // gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

    // gl::ActiveTexture(gl::TEXTURE1);
    // gl::BindTexture(gl::TEXTURE_2D, front_framebuffer.textures()[0]);
    // visualization_shader.set_int(c_str!("textureFront"), 1);
    // gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
    // gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

    gl::ActiveTexture(gl::TEXTURE0);
    gl::BindTexture(gl::TEXTURE_3D, voxels_texture);
    visualization_shader.set_int(c_str!("voxelsTexture"), 0);
    gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
    gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
    gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_WRAP_R, gl::REPEAT as i32);
    gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
    gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

    // gl::Viewport(0, 0, width, height);
    gl::Enable(gl::DEPTH_TEST);
    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

    let quad = Quad::new();
    gl::BindVertexArray(quad.get_vao());
    gl::DrawElements(
        gl::TRIANGLES,
        quad.get_num_indices() as i32,
        gl::UNSIGNED_INT,
        std::ptr::null(),
    );
    gl::BindVertexArray(0);
}
