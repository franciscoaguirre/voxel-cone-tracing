#version 460 core

#include "./_constants.glsl"

layout (local_size_x = WORKING_GROUP_SIZE, local_size_y = 1, local_size_z = 1) in;

uniform layout(binding = 0, r32ui) uimageBuffer nodePool;
uniform layout(binding = 1, rgba8) image3D brickPoolValues;
uniform layout(binding = 2, r32ui) uimageBuffer levelStartIndices;

uniform uint octreeLevel;

#include "./_helpers.glsl"
#include "./_mipmapUtil.glsl"
#include "./_threadNodeUtil.glsl"

void main() {
    int nodeAddress = getThreadNode();

    if (nodeAddress == NODE_NOT_FOUND) {
        return;
    }

    loadChildNodeIDs(nodeAddress);
    vec4 left = mipmapIsotropic(ivec3(0, 2, 2));
    vec4 right = mipmapIsotropic(ivec3(4, 2, 2));
    vec4 bottom = mipmapIsotropic(ivec3(2, 0, 2));
    vec4 top = mipmapIsotropic(ivec3(2, 4, 2));
    vec4 near = mipmapIsotropic(ivec3(2, 2, 0));
    vec4 far = mipmapIsotropic(ivec3(2, 2, 4));

    memoryBarrier();

    ivec3 brickAddress = calculateBrickCoordinates(nodeAddress);
    imageStore(brickPoolValues, brickAddress + ivec3(0,1,1), left);
    imageStore(brickPoolValues, brickAddress + ivec3(2,1,1), right);
    imageStore(brickPoolValues, brickAddress + ivec3(1,0,1), bottom);
    imageStore(brickPoolValues, brickAddress + ivec3(1,2,1), top);
    imageStore(brickPoolValues, brickAddress + ivec3(1,1,0), near);
    imageStore(brickPoolValues, brickAddress + ivec3(1,1,2), far);
}
