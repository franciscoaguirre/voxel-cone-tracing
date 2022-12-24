#version 460 core

#include "./_constants.glsl"

layout (local_size_x = WORKING_GROUP_SIZE, local_size_y = 1, local_size_z = 1) in;

uniform layout(binding = 0, rgb10_a2ui) uimageBuffer voxel_positions;

uniform uint voxel_dimension;

void main() {
    const uint thread_index = gl_GlobalInvocationID.x;
}
