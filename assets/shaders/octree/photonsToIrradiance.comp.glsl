#version 460 core

#include "./_constants.glsl"

layout (local_size_x = 32, local_size_y = 32, local_size_z = 1) in;

uniform layout(binding = 0, rgba8) writeonly image3D brickPoolIrradiance;
uniform layout(binding = 1, r32ui) readonly uimageBuffer nodePool;

uniform sampler3D brickPoolColors;
uniform usampler3D brickPoolPhotons;
uniform usampler2D lightViewMap;

uniform uint voxelDimension;
uniform uint octreeLevel;

#include "./_helpers.glsl"
#include "./_traversalHelpers.glsl"
#include "./_octreeTraversal.glsl"
#include "./_brickCoordinates.glsl"

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
        return;
    }

    ivec3 brickCoordinates = calculateBrickCoordinates(nodeID);
    ivec3 brickOffset = ivec3(calculateBrickVoxel(nodeCoordinates, halfNodeSize, normalizedQueryCoordinates));

    vec4 voxelColor = texelFetch(brickPoolColors, brickCoordinates + brickOffset, 0);
    uint numberOfPhotons = texelFetch(brickPoolPhotons, brickCoordinates + brickOffset, 0).r;
    vec4 irradiance = numberOfPhotons * vec4(1.0 / 20.0);
    imageStore(brickPoolIrradiance, brickCoordinates + brickOffset, vec4(vec3(numberOfPhotons / 1150.0), 1.0));
}
