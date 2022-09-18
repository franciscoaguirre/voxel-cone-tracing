#version 460 core

#include "./_constants.glsl"
#include "./_octree_traversal.glsl"

layout (local_size_x = WORKING_GROUP_SIZE, local_size_y = 1, local_size_z = 1) in;

uniform int number_of_voxel_fragments;
uniform int octree_level;
uniform int voxel_dimension;

uniform layout(binding = 0, rgb10_a2ui) uimageBuffer u_voxelPos;
uniform layout(binding = 1, r32ui) uimageBuffer u_nodePoolBuff;

void main()
{
    const uint thread_index = gl_GlobalInvocationID.x; // TODO: Make grid bigger

    if (thread_index >= number_of_voxel_fragments) {
      return;
    }

    uvec4 voxel_fragment_position = imageLoad(u_voxelPos, int(thread_index));
    uint current_half_node_size = voxel_dimension / 2;

    // Start journey in first tile, in node calculated via coordinates
    uint current_tile_index = 0;

    // Each node's coordinates are the coordinates of the point with lower (x, y, z)
    // within the node.
    uvec3 current_node_coordinates = uvec3(0, 0, 0);

    bvec3 subsection = calculate_node_subsection(current_node_coordinates,
                                                 current_half_node_size,
                                                 voxel_fragment_position.xyz);

    uint current_node_index = calculate_node_index(current_tile_index, subsection);
    
    current_node_coordinates = update_node_coordinates(
      current_node_coordinates,
      subsection,
      current_half_node_size
    );

    current_half_node_size /= 2;

    for (uint i = 0; i < octree_level; i++)
    {
        current_tile_index = imageLoad(u_nodePoolBuff, int(current_node_index)).r;

        bvec3 subsection = calculate_node_subsection(current_node_coordinates,
                                                     current_half_node_size,
                                                     voxel_fragment_position.xyz);

        current_node_index = calculate_node_index(current_tile_index, subsection);

        current_node_coordinates = update_node_coordinates(
          current_node_coordinates,
          subsection,
          current_half_node_size
        );

        current_half_node_size /= 2;
    }

    imageStore(u_nodePoolBuff, int(current_node_index), uvec4(NODE_FLAG_VALUE, 0, 0, 0));
}
