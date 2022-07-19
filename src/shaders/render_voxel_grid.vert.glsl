#version 430 core

// layout (location = 0) in vec3 a_position;

out vec4 voxel_position;
out int vertex_id;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;
uniform int voxel_dimension;
uniform layout(binding = 0, rgb10_a2ui) uimageBuffer voxel_position_texture;
uniform int voxel_fragment_count;

void main() {
    voxel_position = vec4(imageLoad(voxel_position_texture, gl_VertexID).xy, -1.0, 1.0);
    // Voxel structure maps points from 0 to 1, transform the from -1 to 1
    voxel_position.xy = (voxel_position.xy / voxel_dimension) * 2 - vec2(1.0,1.0);

    // Move point to middle of voxel (instead of bottom right of voxel)
    voxel_position.xy = voxel_position.xy + vec2(1.0/voxel_dimension,1.0/voxel_dimension);
    gl_Position = voxel_position;
    gl_PointSize = 4.0;
    vertex_id = gl_VertexID;
}
