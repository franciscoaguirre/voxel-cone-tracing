#version 460 core

#include "./_constants.glsl"
#include "./_helpers.glsl"

// layout (local_size_x = 32, local_size_y = 32, local_size_z = 1) in;
layout (local_size_x = WORKING_GROUP_SIZE, local_size_y = 1, local_size_z = 1) in;

uniform layout(binding = 0, r32ui) uimageBuffer nodePoolNeighbors;
uniform layout(binding = 1, r32ui) uimage3D photonValues;
uniform layout(binding = 2, r32ui) uimageBuffer nodePool;
uniform layout(binding = 3, r32ui) uimageBuffer levelStartIndices;

uniform usampler2D lightViewMap;

uniform uint axis;
uniform uint octreeLevel;
uniform uint voxelDimension;

#include "./_threadNodeUtil.glsl"
#include "assets/shaders/octree/_brickCoordinates.glsl"

// Doing it this way can have concurrency problems if we run all three axis concurrently (not sure if posible, but paper does it in two passes)

void main() {
    int nodeID = getThreadNode();

    int neighborID = int(imageLoad(nodePoolNeighbors, nodeID).r);
    if (neighborID == 0) {
        return;
    }

    ivec3 brickAddress = calculateBrickCoordinates(nodeID);
    ivec3 neighborBrickAddress = calculateBrickCoordinates(neighborID);

    if (axis == 0) {
        for (int y = 0; y <= 2; y++) {
            for (int z = 0; z <= 2; z++) {
                ivec3 neighborOffset = ivec3(0, y, z);
                ivec3 offset = ivec3(2, y, z);

                uint borderPhotons = imageLoad(photonValues, brickAddress + offset).r;
                uint neighborBorderPhotons = imageLoad(photonValues, neighborBrickAddress + neighborOffset).r;
                memoryBarrier();

                uint photonsFinalValue = borderPhotons + neighborBorderPhotons;

                imageStore(photonValues, brickAddress + offset, uvec4(photonsFinalValue, 0, 0, 0));
                imageStore(photonValues, neighborBrickAddress + neighborOffset, uvec4(photonsFinalValue, 0, 0, 0));
            }
        }
    }

    if (axis == 1) {
        for (int x = 0; x <= 2; x++) {
            for (int z = 0; z <= 2; z++) {
                ivec3 neighborOffset = ivec3(x, 0, z);
                ivec3 offset = ivec3(x, 2, z);

                uvec4 borderPhotons = imageLoad(photonValues, brickAddress + offset);
                uvec4 neighborBorderPhotons = imageLoad(photonValues, neighborBrickAddress + neighborOffset);
                memoryBarrier();

                uvec4 photonsFinalValue = borderPhotons + neighborBorderPhotons;
                imageStore(photonValues, brickAddress + offset, photonsFinalValue);
                imageStore(photonValues, neighborBrickAddress + neighborOffset, photonsFinalValue);
            }
        }
    }

    if (axis == 2) {
        for (int x = 0; x <= 2; x++) {
            for (int y = 0; y <= 2; y++) {
                ivec3 neighborOffset = ivec3(x, y, 0);
                ivec3 offset = ivec3(x, y, 2);

                uvec4 borderPhotons = imageLoad(photonValues, brickAddress + offset);
                uvec4 neighborBorderPhotons = imageLoad(photonValues, neighborBrickAddress + neighborOffset);
                memoryBarrier();

                uvec4 photonsFinalValue = borderPhotons + neighborBorderPhotons;
                imageStore(photonValues, brickAddress + offset, photonsFinalValue);
                imageStore(photonValues, neighborBrickAddress + neighborOffset, photonsFinalValue);
            }
        }
    }
}
