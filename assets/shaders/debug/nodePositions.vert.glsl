#version 460 core

layout (location = 0) in uint nodeID;

out vec4 geom_nodePosition;
out float geom_halfNodeSize;
out uint geom_nodeID;

uniform uint voxelDimension;
uniform uint maxOctreeLevel;

uniform layout(binding = 0, rgb10_a2ui) readonly uimageBuffer nodePositions;
uniform layout(binding = 1, r32ui) readonly uimageBuffer levelStartIndices;
uniform layout(binding = 2, r32ui) readonly uimageBuffer borderLevelStartIndices;

#include "assets/shaders/octree/_findOctreeLevel.glsl"
#include "assets/shaders/octree/_constants.glsl"
#include "assets/shaders/octree/_helpers.glsl"

void main() {
    uint octreeLevel = findOctreeLevel(nodeID, maxOctreeLevel);
    float halfNodeSize = calculateHalfNodeSize(octreeLevel);
    float normalizedHalfNodeSize = halfNodeSize * 2.0;
    geom_halfNodeSize = normalizedHalfNodeSize;
    
    uvec3 nodePosition = imageLoad(nodePositions, int(nodeID)).xyz;
    vec3 normalizedNodePosition = nodePosition / float(voxelDimension);
    geom_nodePosition = vec4((normalizedNodePosition.xyz) * 2.0 - vec3(1.0), 1.0);
    geom_nodePosition.xyz += normalizedHalfNodeSize;

    geom_nodeID = nodeID;
}
