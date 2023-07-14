#version 460 core

#include "./_constants.glsl"
#include "./_helpers.glsl"

layout (local_size_x = WORKING_GROUP_SIZE, local_size_y = 1, local_size_z = 1) in;

uniform layout(binding = 0, rgb10_a2ui) uimageBuffer voxelPositions;
uniform layout(binding = 1, rgba8) imageBuffer voxelColors;
uniform layout(binding = 2, rgba8) image3D brickPoolColors;
uniform layout(binding = 3, r32ui) uimageBuffer nodePool;
uniform layout(binding = 4, rgba32f) imageBuffer voxelNormals;
uniform layout(binding = 5, rgba32f) image3D brickPoolNormals;

uniform uint voxelDimension;
uniform uint octreeLevel;
uniform uint numberOfVoxelFragments;

#include "./_traversalHelpers.glsl"
#include "./_octreeTraversal.glsl"
#include "assets/shaders/octree/_brickCoordinates.glsl"

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
    voxelColor.rgb *= voxelColor.a;

    imageStore(
        brickPoolColors,
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
