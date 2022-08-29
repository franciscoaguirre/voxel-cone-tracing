#version 430 core

layout (local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

uniform int number_of_voxel_fragments;
uniform int octree_level;
uniform int voxel_dimension;

uniform layout(binding = 0, rgb10_a2ui) uimageBuffer u_voxelPos;
uniform layout(binding = 1, r32ui) uimageBuffer u_nodePoolBuff;

bool within_second_half(uint min, uint half_node_size, uint coordinate_position) {
 return coordinate_position > min + half_node_size;
}

// Each node is divided into 8 subsections/children, for each coordinate the node is divided in two.
// Given the borders of the node, for each coordinate we calculate if the fragment is within the first or second
// half of the node.
bvec3 calculate_node_subsection(uvec3 borders_start) {
  bvec3 subsection;
  subsection.x = within_second_half(borders_min.x, half_node_size, fragment_position.x);
  subsection.y = within_second_half(borders_min.y, half_node_size, fragment_position.y);
  subsection.z = within_second_half(borders_min.z, half_node_size, fragment_position.z);
  return subsection;
}

// As we have one pointer for all 2x2x2 children, we calculate the index of the child this voxel fragment falls into
int calculate_child_index(uint tile_index, bvec3 subsection) {
  return tile_index + 
         int(subsection[0]) +
         int(subsection[1]) * 2 +
         int(subsection[2]) * 4; // binary -> base10, this gives a unique index per subsection. Then add it to the tile_index
}

void main()
{
    uint node_flag_value = 0x80000000;

    uint thread_index = gl_GlobalInvocationID.x; // TODO: Make grid bigger
    uvec4 voxel_fragment_position = imageLoad(u_voxelPos, int(thread_index));
    int current_half_node_size = voxel_dimension / 2;
    int current_node_tile_index = 0;
    int current_child_index = 0;

    uvec3 current_node_mins = uvec3(0, 0, 0); // Node borders per coordinate

    
    for (int i = 0; i < octree_level; i++)
    {
        current_node_tile_index = imageLoad(u_octreeBuf, current_child_index).r;
        bvec3 subsection = calculate_node_subsection(current_node_mins,
                                                     current_half_node_size,
                                                     voxel_fragment_position);

        current_child_index = calculate_child_index(current_node_tile_index, subsection);

        current_half_node_size /= 2;
    }
}
