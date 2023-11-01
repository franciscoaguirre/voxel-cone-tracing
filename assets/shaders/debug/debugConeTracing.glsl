#shader vertex

#version 460 core

uniform layout(binding = 0, r32ui) uimageBuffer nodesQueried;
uniform layout(binding = 1, r32ui) readonly uimageBuffer nodePool;
uniform layout(binding = 2, r32f) imageBuffer sampledColor;

uniform layout(binding = 0, offset = 0) atomic_uint queriedNodesCounter;

uniform float halfConeAngle;
uniform mat4 projection;
uniform mat4 view;
uniform uint voxelDimension;
uniform uint maxOctreeLevel;

uniform sampler3D brickPoolColorsX;
uniform sampler3D brickPoolColorsXNeg;
uniform sampler3D brickPoolColorsY;
uniform sampler3D brickPoolColorsYNeg;
uniform sampler3D brickPoolColorsZ;
uniform sampler3D brickPoolColorsZNeg;

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
uniform float maxDistance;

out VertexData {
    vec3 position;
    vec3 direction;
} Out;

void main() {
    int threadIndex = gl_VertexID;
    vec3 ndc = position * 2.0 - vec3(1);
    Out.position = ndc;
    gl_Position = projection * view * vec4(ndc, 1);

    float angle = 1.0472;
    float sinAngle = sin(angle);
    float cosAngle = cos(angle);

    vec3 helper = axis - vec3(0.1, 0, 0); // Random vector
    vec3 tangent = normalize(helper - dot(axis, helper) * axis);
    vec3 bitangent = cross(axis, tangent);

    if (threadIndex == 0) {
        Out.direction = axis;
        coneTrace(position, Out.direction, halfConeAngle, maxDistance);
    }

    if (threadIndex == 1) {
        Out.direction = sinAngle * axis - cosAngle * tangent;
        coneTrace(position, Out.direction, halfConeAngle, maxDistance);
    }

    if (threadIndex == 2) {
        Out.direction = sinAngle * axis + cosAngle * bitangent;
        coneTrace(position, Out.direction, halfConeAngle, maxDistance);
    }

    if (threadIndex == 3) {
        Out.direction = sinAngle * axis - cosAngle * bitangent;
        coneTrace(position, Out.direction, halfConeAngle, maxDistance);
    }
}

#shader geometry

#version 460 core

layout (points) in;
layout (line_strip, max_vertices = 80) out;

in VertexData {
    vec3 position;
    vec3 direction;
} In[];

out vec4 frag_color;

uniform mat4 projection;
uniform mat4 view;

uniform float maxDistance;
uniform float halfConeAngle;

const float PI = 3.14159;

#include "./_drawCone.glsl"

void main() {
    vec4 startPosition = vec4(In[0].position, 1);

    frag_color = vec4(1, 1, 0, 1);

    float angleFromPlane = (PI / 2) - halfConeAngle;
    drawCone(In[0].position, In[0].direction, angleFromPlane, maxDistance);
}

#shader fragment

#version 460 core

layout (location = 0) out vec4 FragColor;

in vec4 frag_color;

void main() {
    FragColor = frag_color;
}
