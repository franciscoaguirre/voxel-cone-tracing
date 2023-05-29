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
    uint left = mipmapIsotropic(ivec3(0, 2, 2));
    uint right = mipmapIsotropic(ivec3(4, 2, 2));
    uint bottom = mipmapIsotropic(ivec3(2, 0, 2));
    uint top = mipmapIsotropic(ivec3(2, 4, 2));
    uint near = mipmapIsotropic(ivec3(2, 2, 0));
    uint far = mipmapIsotropic(ivec3(2, 2, 4));

    memoryBarrier();

    ivec3 brickAddress = calculateBrickCoordinates(nodeID);
    imageStore(brickPoolPhotons, brickAddress + ivec3(0,1,1), uvec4(left, 0, 0, 0));
    imageStore(brickPoolPhotons, brickAddress + ivec3(2,1,1), uvec4(right, 0, 0, 0));
    imageStore(brickPoolPhotons, brickAddress + ivec3(1,0,1), uvec4(bottom, 0, 0, 0));
    imageStore(brickPoolPhotons, brickAddress + ivec3(1,2,1), uvec4(top, 0, 0, 0));
    imageStore(brickPoolPhotons, brickAddress + ivec3(1,1,0), uvec4(near, 0, 0, 0));
    imageStore(brickPoolPhotons, brickAddress + ivec3(1,1,2), uvec4(far, 0, 0, 0));
}
