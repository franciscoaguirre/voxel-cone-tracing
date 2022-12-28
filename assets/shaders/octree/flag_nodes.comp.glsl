#version 460 core

#include "./_constants.glsl"
#include "./_traversal_helpers.glsl"
#include "./_octree_traversal.glsl"

layout (local_size_x = WORKING_GROUP_SIZE, local_size_y = 1, local_size_z = 1) in;

uniform int octree_level;
uniform int number_of_voxel_fragments;
uniform int voxel_dimension;

uniform layout(binding = 0, rgb10_a2ui) uimageBuffer voxel_positions;
uniform layout(binding = 1, r32ui) uimageBuffer node_pool;

void main()
{
    const uint thread_index = gl_GlobalInvocationID.x;

    if (thread_index >= number_of_voxel_fragments) {
      return;
    }

    uvec4 voxel_fragment_position = imageLoad(voxel_positions, int(thread_index));
    
    int node_index = traverse_octree(
        vec3(voxel_fragment_position) / float(voxel_dimension),
        octree_level,
        node_pool
    );

    imageStore(node_pool, node_index, uvec4(NODE_FLAG_VALUE, 0, 0, 0));
}
