#version 460 core

#include "./_constants.glsl"

layout (local_size_x = 32, local_size_y = 32, local_size_z = 1) in;

uniform layout(binding = 0, r32ui) readonly uimageBuffer nodePool; // TODO: Could be a texture
uniform layout(binding = 1, r32ui) uimage3D brickPoolPhotons;
uniform layout(binding = 2, r32ui) uimageBuffer totalPhotonHits;

uniform usampler2D lightViewMap;

uniform uint octreeLevel;
uniform uint voxelDimension;

#include "./_helpers.glsl"
#include "./_traversalHelpers.glsl"
#include "./_octreeTraversal.glsl"
#include "./_brickCoordinates.glsl"

void main() {
    uvec3 queryCoordinates = texelFetch(
        lightViewMap,
        ivec2(gl_GlobalInvocationID.xy),
        0 // Mipmap level is always 0
    ).xyz;

    if (queryCoordinates == uvec3(0)) {
        return;
    }
    vec3 normalizedQueryCoordinates = normalizedFromIntCoordinates(queryCoordinates, float(voxelDimension));

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

    ivec3 brickCoordinates = calculateBrickCoordinates(nodeID);
    uint offset = calculateChildLocalID(nodeCoordinates, halfNodeSize, normalizedQueryCoordinates);
    ivec3 brickOffset = 2 * ivec3(CHILD_OFFSETS[offset]);

    // Write a photon in brick's corners
    imageStore(brickPoolPhotons, brickCoordinates + brickOffset, uvec4(1, 0, 0, 0));
}
