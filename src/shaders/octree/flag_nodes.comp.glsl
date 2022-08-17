#version 430 core

layout (local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

uniform int number_of_voxel_fragments;
uniform int octree_level;
uniform int voxel_dimension;

uniform layout(binding = 0, rgb10_a2ui) uimageBuffer u_voxelPos;
uniform layout(binding = 1, r32ui) uimageBuffer u_octreeBuf;

void main()
{
    
}
