#version 460 core

layout (location = 0) in uint nodeID;

#include "assets/shaders/octree/_constants.glsl"

out vec4 geom_nodePosition;
out float geom_halfNodeSize;

uniform uint voxelDimension;
uniform uint maxOctreeLevel;

uniform layout(binding = 0, rgb10_a2ui) uimageBuffer nodePositions;
uniform layout(binding = 1, r32ui) uimageBuffer nodePool;
uniform layout(binding = 2, r32ui) readonly uimageBuffer levelStartIndices;

void main() {
    uint octreeLevel = 0;
    bool foundLevel = false;

    for (uint level = 0; level < maxOctreeLevel; level++) {
        uint levelStartIndex = imageLoad(levelStartIndices, int(level)).r;

        if (levelStartIndex > nodeID) {
            octreeLevel = level - 1;
            foundLevel = true;
            break;
        }
    }

    if (!foundLevel) {
        octreeLevel = maxOctreeLevel - 1;
    }
    
    vec4 nodePosition = imageLoad(nodePositions, int(nodeID));
    vec3 normalizedNodePosition = nodePosition.xyz / float(voxelDimension);

    float halfNodeSize = (0.5 / float(pow(2.0, float(octreeLevel))));
    geom_nodePosition = vec4((normalizedNodePosition.xyz) * 2.0 - vec3(1.0), 1.0);
    float normalizedHalfNodeSize = halfNodeSize * 2.0;
    geom_nodePosition.xyz += normalizedHalfNodeSize;
    geom_halfNodeSize = normalizedHalfNodeSize;
}
