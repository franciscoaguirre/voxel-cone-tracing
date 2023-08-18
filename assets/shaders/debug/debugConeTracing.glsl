#shader vertex

#version 460 core

uniform layout(binding = 0, r32ui) uimageBuffer nodesQueried;
uniform layout(binding = 1, r32ui) readonly uimageBuffer nodePool;

uniform layout(binding = 0, offset = 0) atomic_uint queriedNodesCounter;

uniform float coneAngle;
uniform mat4 projection;
uniform mat4 view;
uniform uint voxelDimension;
uniform uint maxOctreeLevel;
uniform float photonPower;

uniform sampler3D brickPoolColorsX;
uniform sampler3D brickPoolColorsXNeg;
uniform sampler3D brickPoolColorsY;
uniform sampler3D brickPoolColorsYNeg;
uniform sampler3D brickPoolColorsZ;
uniform sampler3D brickPoolColorsZNeg;

uniform usampler3D brickPoolPhotons;

// Irradiance
uniform sampler3D brickPoolIrradianceX;
uniform sampler3D brickPoolIrradianceXNeg;
uniform sampler3D brickPoolIrradianceY;
uniform sampler3D brickPoolIrradianceYNeg;
uniform sampler3D brickPoolIrradianceZ;
uniform sampler3D brickPoolIrradianceZNeg;

#include "assets/shaders/octree/_constants.glsl"
#include "assets/shaders/octree/_helpers.glsl"
#include "assets/shaders/octree/_traversalHelpers.glsl"
#include "assets/shaders/octree/_octreeTraversal.glsl"
#include "assets/shaders/octree/_brickCoordinates.glsl"
#include "assets/shaders/octree/_anisotropicColor.glsl"
#include "assets/shaders/octree/_anisotropicIrradiance.glsl"
#include "assets/shaders/octree/_coneTrace.glsl"

uniform vec3 position;
uniform vec3 axis;

out vec3 geom_position;
out float maxDistance;
out int threadIndex;
out vec3 direction;

void main() {
    threadIndex = gl_VertexID;
    vec3 ndc = position * 2.0 - vec3(1);
    geom_position = ndc;
    gl_Position = projection * view * vec4(ndc, 1);

    maxDistance = 1.0;

    float angle = 1.0472;
    float sinAngle = sin(angle);
    float cosAngle = cos(angle);

    vec3 helper = axis - vec3(0.1, 0, 0); // Random vector
    vec3 tangent = normalize(helper - dot(axis, helper) * axis);
    vec3 bitangent = cross(axis, tangent);

    if (threadIndex == 0) {
        direction = sinAngle * axis + cosAngle * tangent;
        debugConeTrace(position, direction, coneAngle, maxDistance, false);
    }

    if (threadIndex == 1) {
        direction = sinAngle * axis - cosAngle * tangent;
        debugConeTrace(position, direction, coneAngle, maxDistance, false);
    }

    if (threadIndex == 2) {
        direction = sinAngle * axis + cosAngle * bitangent;
        debugConeTrace(position, direction, coneAngle, maxDistance, false);
    }

    if (threadIndex == 3) {
        direction = sinAngle * axis - cosAngle * bitangent;
        debugConeTrace(position, direction, coneAngle, maxDistance, false);
    }
}

#shader geometry

#version 460 core

layout (points) in;
layout (line_strip, max_vertices = 80) out;

in vec3 geom_position[];
in float maxDistance[];
in vec3 direction[];

out vec4 frag_color;

uniform mat4 projection;
uniform mat4 view;

uniform float coneAngle;

const float PI = 3.14159;

#include "./_drawCone.glsl"

void main() {
    vec4 startPosition = vec4(geom_position[0], 1);

    frag_color = vec4(1, 1, 0, 1);

    float angleFromPlane = (PI / 2) - coneAngle;
    drawCone(geom_position[0], direction[0], angleFromPlane, maxDistance[0]);
}

#shader fragment

#version 460 core

layout (location = 0) out vec4 FragColor;

in vec4 frag_color;

void main() {
    FragColor = frag_color;
}
