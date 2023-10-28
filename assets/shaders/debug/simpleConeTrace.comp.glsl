//! Simple shader to test `coneTrace`
//! Meant to be compiled with debug = true

#version 460 core

layout (local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

uniform layout (binding = 0, r32ui) readonly uimageBuffer nodePool;
uniform layout (binding = 1, rgba32f) writeonly imageBuffer queriedCoordinates;

uniform layout(binding = 2, r32ui) uimageBuffer nodesQueried;
uniform layout(binding = 3, r32f) imageBuffer sampledColor;

uniform layout(binding = 0, offset = 0) atomic_uint queriedNodesCounter;

// Uniforms used inside `coneTrace`
uniform uint voxelDimension;
uniform uint maxOctreeLevel;
uniform sampler3D brickPoolColorsX;

// Parameters to `coneTrace`
uniform vec3 coneOrigin;
uniform vec3 coneDirection;
uniform float coneHalfAngle;
uniform float maxDistance;

#include "assets/shaders/octree/_constants.glsl"
#include "assets/shaders/octree/_helpers.glsl"
#include "assets/shaders/octree/_traversalHelpers.glsl"
#include "assets/shaders/octree/_octreeTraversal.glsl"
#include "assets/shaders/octree/_brickCoordinates.glsl"
#include "assets/shaders/octree/_coneTrace.glsl"

void main() {
    coneTrace(coneOrigin, coneDirection, coneHalfAngle, maxDistance);
}
