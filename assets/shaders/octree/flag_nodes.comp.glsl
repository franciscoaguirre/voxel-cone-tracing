#version 460 core

#include "./_constants.glsl"
#include "./_traversal_helpers.glsl"
#include "./_octree_traversal.glsl"

layout (local_size_x = WORKING_GROUP_SIZE, local_size_y = 1, local_size_z = 1) in;

uniform int voxel_dimension;
uniform int octree_level;
uniform int number_of_voxel_fragments;

uniform layout(binding = 0, rgb10_a2ui) uimageBuffer u_voxelPos;
uniform layout(binding = 1, r32ui) uimageBuffer u_nodePoolBuff;

void main()
{
    const uint thread_index = gl_GlobalInvocationID.x;

    if (thread_index >= number_of_voxel_fragments) {
      return;
    }

    uvec4 voxel_fragment_position = imageLoad(u_voxelPos, int(thread_index));
    
    int node_index = traverse_octree(
        uvec3(voxel_fragment_position),
        voxel_dimension,
        octree_level,
        u_nodePoolBuff
    );

    imageStore(u_nodePoolBuff, node_index, uvec4(NODE_FLAG_VALUE, 0, 0, 0));
}
