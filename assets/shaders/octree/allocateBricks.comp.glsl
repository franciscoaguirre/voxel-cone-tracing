#version 460 core

#include "./_constants.glsl"
#include "./_helpers.glsl"

layout (local_size_x = WORKING_GROUP_SIZE, local_size_y = 1, local_size_z = 1) in;

uniform layout(binding = 0, r32ui) uimageBuffer nodePool;
uniform layout(binding = 1, r32ui) uimageBuffer nodePoolBrickPointers;
uniform layout(binding = 0, offset = 0) atomic_uint nextFreeBrickCounter;

uniform uint brickPoolResolution;

void allocate_3x3x3_texture_brick(int nodeID) {
    uint nextFreeBrick = atomicCounterIncrement(nextFreeBrickCounter);
    memoryBarrier();
    uvec3 textureAddress = uvec3(0);
    uint brickPoolResolutionBricks = brickPoolResolution / 3;
    textureAddress.x = nextFreeBrick % brickPoolResolutionBricks;
    textureAddress.y = (nextFreeBrick / brickPoolResolutionBricks) % brickPoolResolutionBricks;
    textureAddress.z = nextFreeBrick / (brickPoolResolutionBricks * brickPoolResolutionBricks);
    textureAddress *= 3;

    imageStore(nodePoolBrickPointers, nodeID, uvec4(vec3ToUintXYZ10(textureAddress), 0, 0, 0));
}

void main() {
    int nodeID = int(gl_GlobalInvocationID.x);
    allocate_3x3x3_texture_brick(nodeID);
}
