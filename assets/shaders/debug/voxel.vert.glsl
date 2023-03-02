#version 460 core

layout (location = 0) in uint voxelIndex;

out vec4 geom_voxelPosition;
out float geom_halfNodeSize;

uniform uint voxelDimension;
uniform uint octreeLevels;

uniform layout(binding = 0, rgb10_a2ui) uimageBuffer voxelPositions;
uniform layout(binding = 1, r32ui) uimageBuffer nodePool;

#include "assets/shaders/octree/_constants.glsl"
#include "assets/shaders/octree/_traversalHelpers.glsl"
#include "assets/shaders/octree/_octreeTraversal.glsl"

void main() {
    vec4 voxelFragmentPosition = imageLoad(voxelPositions, int(voxelIndex));

    float halfNodeSize;
    vec3 nodeCoordinates;
    int _nodeID = traverseOctree(
        vec3(voxelFragmentPosition) / float(voxelDimension),
        octreeLevels,
        nodeCoordinates,
        halfNodeSize
    );

    geom_voxelPosition = vec4((nodeCoordinates.xyz) * 2.0 - vec3(1.0), 1.0);
    float normalizedHalfNodeSize = halfNodeSize * 2.0;
    geom_voxelPosition.xyz += normalizedHalfNodeSize;
    geom_halfNodeSize = normalizedHalfNodeSize;
}
