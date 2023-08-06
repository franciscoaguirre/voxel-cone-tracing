#version 460 core

#include "./_constants.glsl"
#include "./_helpers.glsl"

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

uint getFinalValue(uint borderPhotons, uint neighborBorderPhotons) {
    return borderPhotons + neighborBorderPhotons;
}

void main() {
    // TODO: Optimize with the 2D node map
    int nodeID = getThreadNode();
    // uvec3 queryCoordinates = texelFetch(
    //     lightViewMap,
    //     ivec2(gl_GlobalInvocationID.xy),
    //     0
    // ).xyz;
    // if (queryCoordinates == uvec3(0)) {
    //     return;
    // }
    // vec3 normalizedQueryCoordinates = vec3(queryCoordinates.xyz / (float(voxelDimension) * 2.0));

    // float halfNodeSize;
    // vec3 nodeCoordinates;
    // int nodeID = traverseOctree(
    //     normalizedQueryCoordinates,
    //     octreeLevel,
    //     nodeCoordinates,
    //     halfNodeSize
    // );
    // if (nodeID == NODE_NOT_FOUND) {
    //     return;
    // }

    int neighborID = int(imageLoad(nodePoolNeighbors, nodeID).r);
    if (neighborID == 0) {
        return;
    }

    ivec3 brickAddress = calculateBrickCoordinates(nodeID);
    ivec3 neighborBrickAddress = calculateBrickCoordinates(neighborID);
    for (int i = 0; i <= 2; i++) {
       for (int j = 0; j <= 2; j++) {
         ivec3 neighborOffset;
         ivec3 offset;
         if(axis == 0) {
           neighborOffset = ivec3(0, i, j);
           offset = ivec3(2, i, j);
         } else if (axis == 1) {
           neighborOffset = ivec3(i, 0, j);
           offset = ivec3(i, 2, j);
         } else {
           neighborOffset = ivec3(i, j, 0);
           offset = ivec3(i, j, 2);
         }

         uint borderPhotons = imageLoad(photonValues, brickAddress + offset).r;
         uint neighborBorderPhotons = imageLoad(photonValues, neighborBrickAddress + neighborOffset).r;
         memoryBarrier();

         uint photonsFinalValue = getFinalValue(borderPhotons, neighborBorderPhotons);
         imageStore(photonValues, brickAddress + offset, uvec4(photonsFinalValue, 0, 0, 0));
         imageStore(photonValues, neighborBrickAddress + neighborOffset, uvec4(photonsFinalValue, 0, 0, 0));
       }
    }
}
