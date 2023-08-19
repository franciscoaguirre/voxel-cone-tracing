#version 460 core

#include "./_constants.glsl"
#include "./_helpers.glsl"

layout (local_size_x = WORKING_GROUP_SIZE, local_size_y = 1, local_size_z = 1) in;

uniform layout(binding = 0, rgb10_a2ui) uimageBuffer voxelPositions;
uniform layout(binding = 1, rgba8) imageBuffer voxelColors;
uniform layout(binding = 2, r32ui) uimage3D brickPoolColorsRaw;
uniform layout(binding = 3, r32ui) uimageBuffer nodePool;
uniform layout(binding = 4, rgba32f) imageBuffer voxelNormals;
uniform layout(binding = 5, rgba32f) image3D brickPoolNormals;

uniform uint voxelDimension;
uniform uint octreeLevel;
uniform uint numberOfVoxelFragments;

#include "./_traversalHelpers.glsl"
#include "./_octreeTraversal.glsl"
#include "assets/shaders/octree/_brickCoordinates.glsl"

void imageAtomicR32UIAvg(ivec3 coordinates, vec4 val) {
    val.rgb *= 255.0f; // Optimize following calculations
    uint newVal = convVec4ToR32UI(val);
    uint previousStoredValue = 0;
    uint currentStoredValue;

    // Loop as long as destination value gets changed by other threads
    while ((currentStoredValue = imageAtomicCompSwap(brickPoolColorsRaw, coordinates, previousStoredValue, newVal)) != previousStoredValue) {
        previousStoredValue = currentStoredValue;
        vec4 rval = convR32UIToVec4(currentStoredValue);
        rval.rgb = rval.rgb * rval.a; // Denormalize
        vec4 curValF = rval + val; // Add new value
        curValF.rgb /= curValF.a; // Renormalize
        newVal = convVec4ToR32UI(curValF);
    }
}

void storeInLeaf(
    vec3 voxelPosition,
    int nodeID,
    vec4 voxelColor,
    float halfNodeSize,
    vec3 nodeCoordinates,
    vec4 voxelNormal
) {
    ivec3 brickCoordinates = calculateBrickCoordinates(nodeID);
    uint offset = calculateChildLocalID(nodeCoordinates, halfNodeSize, voxelPosition);

    // Premultiply alpha for better interpolation
    // voxelColor.rgb *= voxelColor.a;

    imageAtomicR32UIAvg(
        brickCoordinates + 2 * ivec3(CHILD_OFFSETS[offset]),
        voxelColor
    );

    imageStore(
        brickPoolNormals,
        brickCoordinates + 2 * ivec3(CHILD_OFFSETS[offset]),
        voxelNormal
    );
}

void main() {
    // Get voxel attributes from voxel fragment list
    const uint threadIndex = gl_GlobalInvocationID.x;
    if (threadIndex < numberOfVoxelFragments) {
        // We need to traverse the tree to get the node because we
        // need the voxel attributes (color, normal, etc)
        uvec3 voxelPosition = imageLoad(voxelPositions, int(threadIndex)).xyz;
        vec4 voxelColor = imageLoad(voxelColors, int(threadIndex));
        vec4 voxelNormal = imageLoad(voxelNormals, int(threadIndex));

        memoryBarrier();

        vec3 normalizedVoxelPosition = normalizedFromIntCoordinates(voxelPosition, float(voxelDimension));

        float halfNodeSize;
        vec3 nodeCoordinates;
        // We send the voxel position to traverse the octree and find the leaf
        int nodeID = traverseOctree(
          normalizedVoxelPosition,
          octreeLevel,
          nodeCoordinates,
          halfNodeSize
        );

        storeInLeaf(normalizedVoxelPosition, nodeID, voxelColor, halfNodeSize, nodeCoordinates, voxelNormal);
    }
}

