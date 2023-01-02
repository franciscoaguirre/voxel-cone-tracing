#version 460 core

// layout (location = 0) in vec3 a_position;

out vec4 voxel_position;
out vec4 voxel_color;
out int vertex_id;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;
uniform uint voxel_dimension;
uniform layout(binding = 0, rgb10_a2ui) uimageBuffer voxel_position_texture;
uniform layout(binding = 1, rgba8) imageBuffer voxel_diffuse_texture;

void main() {
    /* gl_Position = projection * view * model * voxel_position; */

    voxel_position = vec4(imageLoad(voxel_position_texture, gl_VertexID).xyz, 1.0);
    // Voxel structure maps points from 0 to 1, transform the from -1 to 1
    voxel_position.xyz = (voxel_position.xyz / voxel_dimension) * 2 - vec3(1.0,1.0,1.0);

    // Move point to middle of voxel (instead of bottom right of voxel)
    float half_pixel = 1.0 / voxel_dimension;
    voxel_position.xyz = voxel_position.xyz + vec3(half_pixel, half_pixel, half_pixel);

    gl_Position = voxel_position;
    gl_PointSize = 25.0;
    vertex_id = gl_VertexID;
    voxel_color = imageLoad(voxel_diffuse_texture, gl_VertexID);

}
