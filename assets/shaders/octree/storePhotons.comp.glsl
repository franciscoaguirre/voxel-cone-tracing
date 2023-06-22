#version 460 core

#include "./_constants.glsl"

layout (local_size_x = 32, local_size_y = 32, local_size_z = 1) in;

uniform layout(binding = 0, r32ui) readonly uimageBuffer nodePool;
uniform layout(binding = 1, r32ui) uimage3D brickPoolPhotons;
uniform layout(binding = 2, rgba8) image2D lightViewMapView;
uniform layout(binding = 3, r32ui) uimageBuffer totalPhotonHits;

uniform usampler2D lightViewMap;
uniform uint octreeLevel;
uniform uint voxelDimension;

#include "./_helpers.glsl"
#include "./_traversalHelpers.glsl"
#include "./_octreeTraversal.glsl"
#include "assets/shaders/octree/_brickCoordinates.glsl"

void main() {
    uvec3 queryCoordinates = texelFetch(
        lightViewMap,
        ivec2(gl_GlobalInvocationID.xy),
        0
    ).xyz;

    if (queryCoordinates == uvec3(0)) {
        return;
    }
    vec3 normalizedQueryCoordinates = normalizedFromIntCoordinates(queryCoordinates, float(voxelDimension) * 2.0);

    float halfNodeSize;
    vec3 nodeCoordinates;
    int nodeID = traverseOctree(
        normalizedQueryCoordinates,
        octreeLevel,
        nodeCoordinates,
        halfNodeSize
    );

    if (nodeID == NODE_NOT_FOUND) {
        //imageStore(lightViewMapView, ivec2(gl_GlobalInvocationID.xy), ivec4(0, 0, 0, 1));
        return;
    }
    //if (uvec2(floor(nodeCoordinates * float(voxelDimension)).xy) != uvec2(32, 92)) {
      //return;
    //}

    ivec3 brickCoordinates = calculateBrickCoordinates(nodeID);
    ivec3 brickOffset = ivec3(calculateBrickVoxel(nodeCoordinates, halfNodeSize, normalizedQueryCoordinates));

    imageAtomicAdd(brickPoolPhotons, brickCoordinates + brickOffset, uint(1));
    //imageStore(totalPhotonHits, 0, uvec4(nodeID));
}
