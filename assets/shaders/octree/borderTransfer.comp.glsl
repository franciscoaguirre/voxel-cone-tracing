#version 460 core

#include "./_constants.glsl"
#include "./_helpers.glsl"

layout (local_size_x = WORKING_GROUP_SIZE, local_size_y = 1, local_size_z = 1) in;

uniform layout(binding = 0, r32ui) uimageBuffer nodePoolNeighbors; // Will have different axis
uniform layout(binding = 1, rgba8) image3D brickPoolValues; // Will be different
uniform layout(binding = 2, r32ui) uimageBuffer levelStartIndices;

struct Direction {
    int axis;
    int sign;
};

uniform Direction direction;
uniform uint axis;
uniform uint octreeLevel;
uniform uint voxelDimension;

#include "./_threadNodeUtil.glsl"
#include "./_averageHelpers.glsl"
#include "assets/shaders/octree/_brickCoordinates.glsl"

void main() {
    int nodeAddress = getThreadNode();

    if (nodeAddress == NODE_NOT_FOUND) {
        return;
    }

    int neighborAddress = int(imageLoad(nodePoolNeighbors, nodeAddress).r);
    if (neighborAddress == 0) {
        return;
    }
    
    ivec3 brickAddress = calculateBrickCoordinates(nodeAddress);
    ivec3 neighborBrickAddress = calculateBrickCoordinates(neighborAddress);

    if (axis == 0) { // X axis
        for (int y = 0; y <= 2; y++) {
            for (int z = 0; z <= 2; z++) {
                ivec3 offset = ivec3(2, y, z);
                ivec3 neighborOffset = ivec3(0, y, z);

                vec4 borderValue = imageLoad(brickPoolValues, brickAddress + offset);
                vec4 neighborBorderValue = imageLoad(brickPoolValues, neighborBrickAddress + neighborOffset);
                memoryBarrier();

                vec4 finalValue = borderValue; // We copy the value to the neighbor
                imageStore(brickPoolValues, brickAddress + offset, finalValue);
                imageStore(brickPoolValues, neighborBrickAddress + neighborOffset, finalValue);
            }
        }
    }

    if (axis == 1) { // Y axis
        for (int x = 0; x <= 2; x++) {
            for (int z = 0; z <= 2; z++) {
                ivec3 offset = ivec3(x, 2, z);
                ivec3 neighborOffset = ivec3(x, 0, z);

                vec4 borderValue = imageLoad(brickPoolValues, brickAddress + offset);
                vec4 neighborBorderValue = imageLoad(brickPoolValues, neighborBrickAddress + neighborOffset);
                memoryBarrier();

                vec4 finalValue = borderValue + neighborBorderValue; // We hold partial values on each
                imageStore(brickPoolValues, brickAddress + offset, finalValue);
                imageStore(brickPoolValues, neighborBrickAddress + neighborOffset, finalValue);
            }
        }
    }

    if (axis == 2) { // Z axis
        for (int x = 0; x <= 2; x++) {
            for (int y = 0; y <= 2; y++) {
                ivec3 offset = ivec3(x, y, 2);
                ivec3 neighborOffset = ivec3(x, y, 0);

                vec4 borderValue = imageLoad(brickPoolValues, brickAddress + offset);
                vec4 neighborBorderValue = imageLoad(brickPoolValues, neighborBrickAddress + neighborOffset);
                memoryBarrier();

                vec4 finalValue = borderValue + neighborBorderValue; // We hold partial values on each
                imageStore(brickPoolValues, brickAddress + offset, finalValue);
                imageStore(brickPoolValues, neighborBrickAddress + neighborOffset, finalValue);
            }
        }
    }
}
