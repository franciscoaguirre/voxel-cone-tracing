#version 460 core

#include "./_constants.glsl"

layout (local_size_x = WORKING_GROUP_SIZE, local_size_y = 1, local_size_z = 1) in;

uniform uint octree_level;
uniform uint number_of_voxel_fragments;
uniform uint voxel_dimension;

uniform layout(binding = 0, rgb10_a2ui) uimageBuffer voxel_positions;
uniform layout(binding = 1, r32ui) uimageBuffer nodePool;

#include "./_traversalHelpers.glsl"
#include "./_octreeTraversal.glsl"

void main()
{
    const uint threadIndex = gl_GlobalInvocationID.x;

    if (threadIndex >= number_of_voxel_fragments) {
      return;
    }

    uvec4 voxel_fragment_position = imageLoad(voxel_positions, int(threadIndex));
    
    int node_index = traverseOctree(
        vec3(voxel_fragment_position) / float(voxel_dimension),
        octree_level
    );

    imageStore(nodePool, node_index, uvec4(NODE_FLAG_VALUE, 0, 0, 0));
}
