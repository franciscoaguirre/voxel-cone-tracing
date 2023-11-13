#version 460 core

layout (local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

uniform layout(binding = 0, r32ui) readonly uimageBuffer nodePool;
uniform layout(binding = 1, r32ui) writeonly uimageBuffer outputBuffer;

uniform uvec3 queryCoordinates;
uniform uint voxelDimension;
uniform uint octreeLevel;

#include "assets/shaders/octree/_constants.glsl"
#include "assets/shaders/octree/_helpers.glsl"
#include "assets/shaders/octree/_traversalHelpers.glsl"
#include "assets/shaders/octree/_octreeTraversal.glsl"

void main() {
    vec3 normalizedQueryCoordinates = normalizedFromIntCoordinates(queryCoordinates, float(voxelDimension));
    vec3 nodeCoordinates;
    float halfNodeSize;
    int nodeID = traverseOctree(
        normalizedQueryCoordinates,
        octreeLevel,
        nodeCoordinates,
        halfNodeSize
    );
    imageStore(outputBuffer, 0, uvec4(uint(nodeID), 0, 0, 0));
}
