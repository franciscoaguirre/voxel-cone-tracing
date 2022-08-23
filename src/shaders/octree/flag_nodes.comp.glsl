#version 430 core

layout (local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

uniform int number_of_voxel_fragments;
uniform int octree_level;
uniform int voxel_dimension;

uniform layout(binding = 0, rgb10_a2ui) uimageBuffer u_voxelPos;
uniform layout(binding = 1, r32ui) uimageBuffer u_octreeBuf;

void main()
{
    uint thread_index = gl_GlobalInvocationID.x; // TODO: Make grid bigger
    uvec4 voxel_position = imageLoad(u_voxelPos, int(thread_index));
    int current_voxel_dimension = voxel_dimension;
    int child_index = 0;
    uint node = imageLoad(u_octreeBuf, child_index).r;
    
    for (int i = 0; i < octree_level; i++)
    {
        current_voxel_dimension /= 2; // i.e. 256 / 2 = 128
    }
}
