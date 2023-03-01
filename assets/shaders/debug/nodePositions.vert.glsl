#version 460 core

layout (location = 0) in uint nodeIndex;

#include "assets/shaders/octree/_constants.glsl"

out vec4 geom_nodePosition;
out float geom_halfNodeSize;

uniform uint voxelDimension;
uniform uint maxOctreeLevel;

uniform layout(binding = 0, rgb10_a2ui) uimageBuffer nodePositions;
uniform layout(binding = 1, r32ui) uimageBuffer nodePool;
uniform layout(binding = 2, r32ui) readonly uimageBuffer levelStartIndices;

void main() {
    uint octreeLevel = 1;
    for (uint level = 0; level < maxOctreeLevel; level++) {
        uint levelStartIndex = imageLoad(levelStartIndices, int(level)).r;

        // TODO: levelStartIndices works by tile, so should we multiply by NODES_PER_TILE?
        if (levelStartIndex * NODES_PER_TILE > nodeIndex) {
            octreeLevel = level - 1;
            break;
        }
    }
    
    vec4 nodePosition = imageLoad(nodePositions, int(nodeIndex));
    vec3 normalizedNodePosition = nodePosition.xyz / float(voxelDimension);

    float halfNodeSize = (1.0 / float(pow(2.0, float(octreeLevel))));
    geom_nodePosition = vec4((normalizedNodePosition.xyz) * 2.0 - vec3(1.0), 1.0);
    float normalizedHalfNodeSize = halfNodeSize * 2.0;
    geom_nodePosition.xyz += normalizedHalfNodeSize;
    geom_halfNodeSize = normalizedHalfNodeSize;
}
