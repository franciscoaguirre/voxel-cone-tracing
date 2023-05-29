#version 460 core

#include "./_constants.glsl"

layout (local_size_x = 32, local_size_y = 32, local_size_z = 1) in;

uniform layout(binding = 0, r32ui) uimageBuffer nodePool;
uniform layout(binding = 1, r32ui) uimage3D brickPoolPhotons;

uniform usampler2D lightViewMap;
uniform uint octreeLevel;
uniform uint voxelDimension;

#include "./_helpers.glsl"
#include "./_umipmapUtil.glsl"
#include "./_traversalHelpers.glsl"
#include "./_octreeTraversal.glsl"

void main() {
    uvec3 queryCoordinates = texelFetch(
        lightViewMap,
        ivec2(gl_GlobalInvocationID.xy),
        0
    ).xyz;
    if (queryCoordinates == uvec3(0)) {
        return;
    }
    vec3 normalizedQueryCoordinates = normalizedFromIntCoordinates(queryCoordinates, (float(voxelDimension) * 1.5));
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

    loadChildNodeIDs(nodeID);
    uint nearRightTop = mipmapIsotropic(ivec3(4, 4, 0));
    uint nearRightBottom = mipmapIsotropic(ivec3(4, 0, 0));
    uint nearLeftTop = mipmapIsotropic(ivec3(0, 4, 0));
    uint nearLeftBottom = mipmapIsotropic(ivec3(0, 0, 0));
    uint farRightTop = mipmapIsotropic(ivec3(4, 4, 4));
    uint farRightBottom = mipmapIsotropic(ivec3(4, 0, 4));
    uint farLeftTop = mipmapIsotropic(ivec3(0, 4, 4));
    uint farLeftBottom = mipmapIsotropic(ivec3(0, 0, 4));
  
    memoryBarrier();

    ivec3 brickAddress = calculateBrickCoordinates(nodeID);
    imageStore(brickPoolPhotons, brickAddress + ivec3(2, 2, 0), uvec4(nearRightTop, 0, 0, 0));
    imageStore(brickPoolPhotons, brickAddress + ivec3(2, 0, 0), uvec4(nearRightBottom, 0, 0, 0));
    imageStore(brickPoolPhotons, brickAddress + ivec3(0, 2, 0), uvec4(nearLeftTop, 0, 0, 0));
    imageStore(brickPoolPhotons, brickAddress + ivec3(0, 0, 0), uvec4(nearLeftBottom, 0, 0, 0));
    imageStore(brickPoolPhotons, brickAddress + ivec3(2, 2, 2), uvec4(farRightTop, 0, 0, 0));
    imageStore(brickPoolPhotons, brickAddress + ivec3(2, 0, 2), uvec4(farRightBottom, 0, 0, 0));
    imageStore(brickPoolPhotons, brickAddress + ivec3(0, 2, 2), uvec4(farLeftTop, 0, 0, 0));
    imageStore(brickPoolPhotons, brickAddress + ivec3(0, 0, 2), uvec4(farLeftBottom, 0, 0, 0));
}
