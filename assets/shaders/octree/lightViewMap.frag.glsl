#version 460 core

layout (location = 0) out vec4 FragColor;

in vec4 frag_position;

uniform layout(binding = 0, r32ui) uimageBuffer nodePool;

uniform uint voxelDimension;
uniform uint octreeLevel;

#include "./_constants.glsl"
#include "./_traversalHelpers.glsl"
#include "./_octreeTraversal.glsl"

void main() {
    FragColor = vec4(1.0, 1.0, 1.0, 1.0);

    vec3 nodeCoordinates;
    float halfNodeSize;
    vec3 normalizedPosition = frag_position.xyz / float(voxelDimension);
    int nodeID = traverseOctree(
        normalizedPosition,
        octreeLevel,
        nodeCoordinates,
        halfNodeSize
    );

    if (nodeID != NODE_NOT_FOUND) {
        // TODO: Store photon in brick
    }
}
