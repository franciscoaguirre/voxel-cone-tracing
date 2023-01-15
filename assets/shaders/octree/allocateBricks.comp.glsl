#version 460 core

#include "./_constants.glsl"
#include "./_helpers.glsl"

layout (local_size_x = WORKING_GROUP_SIZE, local_size_y = 1, local_size_z = 1) in;

uniform layout(binding = 0, r32ui) uimageBuffer nodePool;
uniform layout(binding = 1, r32ui) uimageBuffer nodePoolBrickPointers;
uniform layout(binding = 0, offset = 0) atomic_uint nextFreeBrickCounter;

uniform uint brickPoolResolution;

void allocate_3x3x3_texture_brick(in int nodeAddress) {
    uint nextFreeBrick = atomicCounterIncrement(nextFreeBrickCounter);
    memoryBarrier();
    uvec3 textureAddress = uvec3(0);
    uint brickPoolResolutionBricks = brickPoolResolution / 3;
    textureAddress.x = nextFreeBrick % brickPoolResolutionBricks;
    textureAddress.y = (nextFreeBrick / brickPoolResolutionBricks) % brickPoolResolutionBricks;
    textureAddress.z = nextFreeBrick / (brickPoolResolutionBricks * brickPoolResolutionBricks);
    textureAddress *= 3;

    imageStore(nodePoolBrickPointers, nodeAddress, uvec4(vec3ToUintXYZ10(textureAddress), 0, 0, 0));
}

void main() {
    uint tileAddress = gl_GlobalInvocationID.x * 8;

    for (int i = 0; i < NODES_PER_TILE; i++) {
        int nodeAddress = int(tileAddress + i);
        allocate_3x3x3_texture_brick(nodeAddress);

        // TODO: Brick flag?
        // uint nodeNextU = imageLoad(nodePool_next, address).x;
        // imageStore(nodePool_next, address,
        // uvec4(NODE_MASK_BRICK | nodeNextU, 0, 0, 0));
    }
}
