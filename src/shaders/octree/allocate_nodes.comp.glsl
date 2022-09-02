#version 430 core

layout (local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

uniform int starting_node_in_level;
uniform int starting_free_space;

uniform layout(binding = 0, r32ui) uimageBuffer u_nodePoolBuff;

uniform layout(binding = 0, offset = 0) atomic_uint allocated_tiles_counter;

const uint NODES_PER_TILE = 8;
const int NODE_FLAG_VALUE = 0x80000000;

bool is_node_flagged(uint node) {
    return (node & NODE_FLAG_VALUE) != 0;
}

void main()
{
    uint allocated_tile_index;
    uint thread_index = gl_GlobalInvocationID.x;
    uint parent_node = imageLoad(u_nodePoolBuff, starting_node_in_level + int(thread_index)).r;

    if (is_node_flagged(parent_node)) {
        allocated_tile_index = atomicCounterIncrement(allocated_tiles_counter);
        allocated_tile_index *= NODES_PER_TILE;
        allocated_tile_index += starting_free_space;
        allocated_tile_index |= NODE_FLAG_VALUE; // Keep flag

        imageStore(u_nodePoolBuff, starting_node_in_level + int(thread_index), uvec4(allocated_tile_index, 0, 0, 0));
    }
}
