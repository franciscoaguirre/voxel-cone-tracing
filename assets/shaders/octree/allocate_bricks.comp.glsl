#version 460 core

#include "./_constants.glsl"
#include "./_helpers.glsl"

layout (local_size_x = WORKING_GROUP_SIZE, local_size_y = 1, local_size_z = 1) in;

uniform layout(binding = 0, r32ui) uimageBuffer node_pool;
uniform layout(binding = 1, r32ui) uimageBuffer node_pool_brick_pointers;
uniform layout(binding = 0, offset = 0) atomic_uint next_free_brick_counter;

uniform uint brick_pool_resolution;

void allocate_3x3x3_texture_brick(in int node_address) {
    uint next_free_brick = atomicCounterIncrement(next_free_brick_counter);
    memoryBarrier();
    uvec3 texture_address = uvec3(0);
    uint brick_pool_resolution_bricks = brick_pool_resolution / 3;
    texture_address.x = next_free_brick % brick_pool_resolution_bricks;
    texture_address.y = (next_free_brick / brick_pool_resolution_bricks) % brick_pool_resolution_bricks;
    texture_address.z = next_free_brick / (brick_pool_resolution_bricks * brick_pool_resolution_bricks);
    texture_address *= 3;

    /* imageStore(node_pool_brick_pointers, node_address, uvec4(vec3ToUintXYZ10(texture_address), 0, 0, 0)); */
    /* imageStore(node_pool_brick_pointers, 0, uvec4(255, 0, 0, 0)); */
}

void main() {
    uint tile_address = gl_GlobalInvocationID.x; // TODO: Multiply by 8?

    imageStore(node_pool_brick_pointers, 0, uvec4(255, 0, 0, 0));

    for (int i = 0; i < NODES_PER_TILE; i++) {
        int node_address = int(tile_address + i);
        allocate_3x3x3_texture_brick(node_address);

        // TODO: Brick flag?
    }
}
