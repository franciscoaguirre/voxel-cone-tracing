#version 460 core

#include "./_constants.glsl"

layout (local_size_x = 32, local_size_y = 32, local_size_z = 1) in;

uniform layout(binding = 0, r32ui) readonly uimageBuffer nodePool;
uniform layout(binding = 1, r32ui) readonly uimageBuffer nodePoolBrickPointers;
uniform layout(binding = 2, r32ui) uimage3D brickPoolPhotons;

uniform sampler2D lightViewMap;
uniform uint octreeLevel;

#include "./_helpers.glsl"
#include "./_traversalHelpers.glsl"
#include "./_octreeTraversal.glsl"

void main() {
    vec3 queryCoordinates = texelFetch(
        lightViewMap,
        ivec2(gl_GlobalInvocationID.x, gl_GlobalInvocationID.y),
        0
    ).xyz;

    if (queryCoordinates.xyz == vec3(0)) {
        return;
    }

    float halfNodeSize;
    vec3 nodeCoordinates;
    int nodeID = traverseOctree(
        queryCoordinates,
        octreeLevel,
        nodeCoordinates,
        halfNodeSize
    );

    if (nodeID == NODE_NOT_FOUND) {
        return;
    }

    uint brickCoordinatesCompact = imageLoad(nodePoolBrickPointers, nodeID).r;
    ivec3 brickCoordinates = ivec3(uintXYZ10ToVec3(brickCoordinatesCompact));
    // uint offset = calculateChildLocalID(nodeCoordinates, halfNodeSize, queryCoordinates);
    imageStore(
        brickPoolPhotons,
        brickCoordinates,
        uvec4(1, 0, 0, 0) // TODO: Add photons
    );
}
