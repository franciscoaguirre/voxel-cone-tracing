#version 460 core

#include "./_constants.glsl"

layout (local_size_x = 32, local_size_y = 32, local_size_z = 1) in;

uniform layout(binding = 0, r32ui) uimageBuffer nodePool;
uniform layout(binding = 1, r32ui) uimage3D brickPoolPhotons;

uniform usampler2D lightViewMap;
uniform uint octreeLevel;
uniform uint voxelDimension;

#include "./_helpers.glsl"
#include "./_mipmapUtil.glsl"
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

    loadChildNodeIDs(nodeID, nodePool);
    uint nearBottom = mipmapIsotropic(ivec3(2, 0, 0), brickPoolPhotons);
    uint nearRight = mipmapIsotropic(ivec3(4, 2, 0), brickPoolPhotons);
    uint nearTop = mipmapIsotropic(ivec3(2, 4, 0), brickPoolPhotons);
    uint nearLeft = mipmapIsotropic(ivec3(0, 2, 0), brickPoolPhotons);
    uint farBottom = mipmapIsotropic(ivec3(2, 0, 4), brickPoolPhotons);
    uint farRight = mipmapIsotropic(ivec3(4, 2, 4), brickPoolPhotons);
    uint farTop = mipmapIsotropic(ivec3(2, 4, 4), brickPoolPhotons);
    uint farLeft = mipmapIsotropic(ivec3(0, 2, 4), brickPoolPhotons);
    uint leftBottom = mipmapIsotropic(ivec3(0, 0, 2), brickPoolPhotons);
    uint leftTop = mipmapIsotropic(ivec3(0, 4, 2), brickPoolPhotons);
    uint rightBottom = mipmapIsotropic(ivec3(4, 0, 2), brickPoolPhotons);
    uint rightTop = mipmapIsotropic(ivec3(4, 4, 2), brickPoolPhotons);

    memoryBarrier();

    ivec3 brickAddress = calculateBrickCoordinates(nodeID);
    imageStore(brickPoolPhotons, brickAddress + ivec3(1,0,0), uvec4(nearBottom, 0, 0, 0));
    imageStore(brickPoolPhotons, brickAddress + ivec3(2,1,0), uvec4(nearRight, 0, 0, 0));
    imageStore(brickPoolPhotons, brickAddress + ivec3(1,2,0), uvec4(nearTop, 0, 0, 0));
    imageStore(brickPoolPhotons, brickAddress + ivec3(0,1,0), uvec4(nearLeft, 0, 0, 0));
    imageStore(brickPoolPhotons, brickAddress + ivec3(1,0,2), uvec4(farBottom, 0, 0, 0));
    imageStore(brickPoolPhotons, brickAddress + ivec3(2,1,2), uvec4(farRight, 0, 0, 0));
    imageStore(brickPoolPhotons, brickAddress + ivec3(1,2,2), uvec4(farTop, 0, 0, 0));
    imageStore(brickPoolPhotons, brickAddress + ivec3(0,1,2), uvec4(farLeft, 0, 0, 0));
    imageStore(brickPoolPhotons, brickAddress + ivec3(0, 0, 1), uvec4(leftBottom, 0, 0, 0));
    imageStore(brickPoolPhotons, brickAddress + ivec3(2, 0, 1), uvec4(rightBottom, 0, 0, 0));
    imageStore(brickPoolPhotons, brickAddress + ivec3(0, 2, 1), uvec4(leftTop, 0, 0, 0));
    imageStore(brickPoolPhotons, brickAddress + ivec3(2, 2, 1), uvec4(rightTop, 0, 0, 0));
}
