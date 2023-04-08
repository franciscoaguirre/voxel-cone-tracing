#version 460 core

#include "./_constants.glsl"

layout (local_size_x = 32, local_size_y = 32, local_size_z = 1) in;

uniform layout(binding = 0, r32ui) readonly uimageBuffer nodePool;
uniform layout(binding = 1, r32ui) readonly uimageBuffer nodePoolBrickPointers;
uniform layout(binding = 2, r32ui) uimage3D brickPoolPhotons;

uniform usampler2D lightViewMap;
uniform uint octreeLevel;
uniform uint voxelDimension;

#include "./_helpers.glsl"
#include "./_traversalHelpers.glsl"
#include "./_octreeTraversal.glsl"

void main() {
    uvec4 queryCoordinates = texelFetch(
        lightViewMap,
        ivec2(gl_GlobalInvocationID.xy),
        0
    );

    if (queryCoordinates.xyz == uvec3(0)) {
        return;
    }
    vec3 normalizedQueryCoordinates = vec3(queryCoordinates.xyz / float(voxelDimension));

    float halfNodeSize;
    vec3 nodeCoordinates;
    int nodeID = traverseOctree(
        normalizedQueryCoordinates,
        octreeLevel,
        nodeCoordinates,
        halfNodeSize
    );

    if (nodeID == NODE_NOT_FOUND) {
        return;
    }

    uint brickCoordinatesCompact = imageLoad(nodePoolBrickPointers, nodeID).r;
    ivec3 brickCoordinates = ivec3(uintXYZ10ToVec3(brickCoordinatesCompact));
    ivec3 brickOffset = ivec3(calculateBrickVoxel(nodeCoordinates, halfNodeSize, normalizedQueryCoordinates));

    imageAtomicAdd(brickPoolPhotons, brickCoordinates + brickOffset, uint(1));
}
