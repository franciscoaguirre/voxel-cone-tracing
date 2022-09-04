#version 430 core

layout (local_size_x = 64, local_size_y = 1, local_size_z = 1) in;

uniform int first_tile_in_level;
uniform int first_free_tile;

uniform layout(binding = 0, r32ui) uimageBuffer u_nodePoolBuff;

uniform layout(binding = 0, offset = 0) atomic_uint allocated_tiles_counter;

const int NODES_PER_TILE = 8;
const int NODE_FLAG_VALUE = 0x80000000;

bool is_node_flagged(uint node) {
    return (node & NODE_FLAG_VALUE) != 0;
}

void main()
{
    uint allocated_tile_index;
    uint thread_index = gl_GlobalInvocationID.x;
    int parent_node_index = first_tile_in_level * NODES_PER_TILE + int(thread_index);
    uint parent_node = imageLoad(u_nodePoolBuff, parent_node_index).r;

    if (is_node_flagged(parent_node)) {
        allocated_tile_index = atomicCounterIncrement(allocated_tiles_counter);
        allocated_tile_index += first_free_tile;

        imageStore(u_nodePoolBuff, parent_node_index, uvec4(allocated_tile_index, 0, 0, 0));
    }
}
