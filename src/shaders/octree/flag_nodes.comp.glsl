#version 430 core

layout (local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

uniform int number_of_voxel_fragments;
uniform int octree_level;
uniform int voxel_dimension;

uniform layout(binding = 0, rgb10_a2ui) uimageBuffer u_voxelPos;
uniform layout(binding = 1, r32ui) uimageBuffer u_nodePoolBuff;

const uint NODES_PER_TILE = 8;
const int NODE_FLAG_VALUE = 0x80000000;

bool within_second_half(uint min, uint half_node_size, uint coordinate_position) {
 return coordinate_position > min + half_node_size;
}

// Each node is divided into 8 subsections/children, for each coordinate the node is divided in two.
// Given the borders of the node, for each coordinate we calculate if the fragment is within the first or second
// half of the node.
bvec3 calculate_node_subsection(uvec3 node_coordinates, uint half_node_size, uvec3 fragment_position) {
  bvec3 subsection;
  subsection.x = within_second_half(node_coordinates.x, half_node_size, fragment_position.x);
  subsection.y = within_second_half(node_coordinates.y, half_node_size, fragment_position.y);
  subsection.z = within_second_half(node_coordinates.z, half_node_size, fragment_position.z);
  return subsection;
}

// As we have one pointer for all 2x2x2 children, we calculate the index of the child this voxel fragment falls into
uint calculate_node_index(uint tile_index, bvec3 subsection) {
  return (tile_index * NODES_PER_TILE) + 
         uint(subsection[0]) +
         uint(subsection[1]) * 2 +
         uint(subsection[2]) * 4; // binary -> base10, this gives a unique index per subsection. Then add it to the tile_index
}

void update_node_coordinates(
  uvec3 current_node_coordinates,
  bvec3 subsection,
  uint current_half_node_size
) {
  if (subsection.x) {
    current_node_coordinates.x += current_half_node_size;
  }
  if (subsection.y) {
    current_node_coordinates.y += current_half_node_size;
  }
  if (subsection.z) {
    current_node_coordinates.z += current_half_node_size;
  }
}

void main()
{
    uint thread_index = gl_GlobalInvocationID.x; // TODO: Make grid bigger
    uvec4 voxel_fragment_position = imageLoad(u_voxelPos, int(thread_index));
    uint current_half_node_size = voxel_dimension / 2;

    // Global indices
    uint current_tile_index = 0;
    uint current_node_index = 0;

    // Each node's coordinates are the coordinates of the point with lower (x, y, z)
    // within the node.
    uvec3 current_node_coordinates = uvec3(0, 0, 0);
    
    for (uint i = 0; i < octree_level; i++)
    {
        current_tile_index = imageLoad(u_nodePoolBuff, int(current_node_index)).r;

        bvec3 subsection = calculate_node_subsection(current_node_coordinates,
                                                     current_half_node_size,
                                                     voxel_fragment_position.xyz);

        current_node_index = calculate_node_index(current_tile_index, subsection);

        // TODO: Does it mutate current_node_coordinates?
        update_node_coordinates(
          current_node_coordinates,
          subsection,
          current_half_node_size
        );

        current_half_node_size /= 2;
    }

    imageStore(u_nodePoolBuff, int(current_node_index), uvec4(NODE_FLAG_VALUE, 0, 0, 0));
}
