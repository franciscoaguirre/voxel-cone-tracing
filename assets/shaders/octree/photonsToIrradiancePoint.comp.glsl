#version 460 core

#include "./_constants.glsl"

layout (local_size_x = 12, local_size_y = 12, local_size_z = 6) in;

uniform layout(binding = 0, rgba8) writeonly image3D brickPoolIrradiance;
uniform layout(binding = 1, r32ui) readonly uimageBuffer nodePool;

uniform sampler3D brickPoolColors;
uniform usampler3D brickPoolPhotons;
uniform usampler2DArray lightViewMap;

uniform uint voxelDimension;
uniform uint octreeLevel;

uniform float lightIntensity;
uniform vec3 lightPosition;

#include "./_helpers.glsl"
#include "./_traversalHelpers.glsl"
#include "./_octreeTraversal.glsl"
#include "./_brickCoordinates.glsl"

vec3 toVoxelSpace(vec3 positionWorldSpace) {
  return (positionWorldSpace + vec3(1)) / 2.0;
}

void main() {
    uvec3 queryCoordinates = texelFetch(
        lightViewMap,
        ivec3(gl_GlobalInvocationID.xy, gl_LocalInvocationID.z),
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

    ivec3 brickCoordinates = calculateBrickCoordinates(nodeID);
    uint offset = calculateChildLocalID(nodeCoordinates, halfNodeSize, normalizedQueryCoordinates);
    ivec3 brickOffset = 2 * ivec3(CHILD_OFFSETS[offset]);

    vec4 voxelColor = texelFetch(brickPoolColors, brickCoordinates + brickOffset, 0);
    uint numberOfPhotons = texelFetch(brickPoolPhotons, brickCoordinates + brickOffset, 0).r;

    float lightDistance = length(toVoxelSpace(lightPosition) - normalizedQueryCoordinates);
    float c1 = 1.0;
    float c2 = 0.09;
    float c3 = 0.032;
    float attenuation = c1 + c2 * lightDistance + c3 * lightDistance * lightDistance;

    // TODO: Use also total photon hits here for the multiplier.
    // Every octree level added separates the current surface touched by photons in
    // 4 (2D section of a voxel is separated in 4 new voxels, each with a fourth of the amount of photons)
    // float multiplier = numberOfPhotons * pow(4, octreeLevel) / float(262144); 
    // float multiplier = clamp(float(numberOfPhotons), 0.0, 1.0);
    vec4 irradiance = vec4(voxelColor.xyz * numberOfPhotons / attenuation, 1.0);

    imageStore(brickPoolIrradiance, brickCoordinates + brickOffset, irradiance);
}
