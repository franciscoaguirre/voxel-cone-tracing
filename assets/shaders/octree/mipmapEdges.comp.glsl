#version 460 core

#include "assets/shaders/octree/_constants.glsl"

layout (local_size_x = WORKING_GROUP_SIZE, local_size_y = 1, local_size_z = 1) in;

uniform layout(binding = 0, r32ui) uimageBuffer nodePool;
uniform layout(binding = 1, rgba8) writeonly image3D brickPoolValues;
uniform layout(binding = 2, r32ui) uimageBuffer levelStartIndices;
uniform layout(binding = 3, r32ui) uimageBuffer directionalNeighbors;
uniform layout(binding = 4, rgba8) readonly image3D brickPoolValuesRead;

uniform uint octreeLevel;
uniform uint voxelDimension;

#include "assets/shaders/octree/_helpers.glsl"
#include "assets/shaders/octree/_threadNodeUtil.glsl"
#include "assets/shaders/octree/_brickCoordinates.glsl"
#include "assets/shaders/octree/_mipmapAnisotropic.glsl"

void main() {
    int nodeAddress = getThreadNode();

    if (nodeAddress == NODE_NOT_FOUND) {
        return;
    }

    setup(nodeAddress);
    vec4 nearBottom = mipmapAnisotropic(ivec3(2, 0, 0));
    vec4 nearRight = mipmapAnisotropic(ivec3(4, 2, 0));
    vec4 nearTop = mipmapAnisotropic(ivec3(2, 4, 0));
    vec4 nearLeft = mipmapAnisotropic(ivec3(0, 2, 0));
    vec4 farBottom = mipmapAnisotropic(ivec3(2, 0, 4));
    vec4 farRight = mipmapAnisotropic(ivec3(4, 2, 4));
    vec4 farTop = mipmapAnisotropic(ivec3(2, 4, 4));
    vec4 farLeft = mipmapAnisotropic(ivec3(0, 2, 4));
    vec4 leftBottom = mipmapAnisotropic(ivec3(0, 0, 2));
    vec4 leftTop = mipmapAnisotropic(ivec3(0, 4, 2));
    vec4 rightBottom = mipmapAnisotropic(ivec3(4, 0, 2));
    vec4 rightTop = mipmapAnisotropic(ivec3(4, 4, 2));

    memoryBarrier();

    ivec3 brickAddress = calculateBrickCoordinates(nodeAddress);
    imageStore(brickPoolValues, brickAddress + ivec3(1,0,0), nearBottom);
    imageStore(brickPoolValues, brickAddress + ivec3(2,1,0), nearRight);
    imageStore(brickPoolValues, brickAddress + ivec3(1,2,0), nearTop);
    imageStore(brickPoolValues, brickAddress + ivec3(0,1,0), nearLeft);
    imageStore(brickPoolValues, brickAddress + ivec3(1,0,2), farBottom);
    imageStore(brickPoolValues, brickAddress + ivec3(2,1,2), farRight);
    imageStore(brickPoolValues, brickAddress + ivec3(1,2,2), farTop);
    imageStore(brickPoolValues, brickAddress + ivec3(0,1,2), farLeft);
    imageStore(brickPoolValues, brickAddress + ivec3(0, 0, 1), leftBottom);
    imageStore(brickPoolValues, brickAddress + ivec3(2, 0, 1), rightBottom);
    imageStore(brickPoolValues, brickAddress + ivec3(0, 2, 1), leftTop);
    imageStore(brickPoolValues, brickAddress + ivec3(2, 2, 1), rightTop);
}
