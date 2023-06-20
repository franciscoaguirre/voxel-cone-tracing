#shader vertex

#version 460 core

uniform layout(binding = 0, r32ui) uimageBuffer nodesQueried;
uniform layout(binding = 1, r32ui) readonly uimageBuffer nodePool;

uniform mat4 projection;
uniform mat4 view;
uniform uint voxelDimension;
uniform uint maxOctreeLevel;
uniform sampler3D brickPoolColors;

#include "assets/shaders/octree/_constants.glsl"
#include "assets/shaders/octree/_helpers.glsl"
#include "assets/shaders/octree/_traversalHelpers.glsl"
#include "assets/shaders/octree/_octreeTraversal.glsl"
#include "assets/shaders/octree/_brickCoordinates.glsl"
#include "assets/shaders/octree/_coneTrace.glsl"
#include "assets/shaders/octree/_debugConeTrace.glsl"

uniform vec3 position;
uniform vec3 axis;
uniform float coneAngle;

out vec3 geom_position;
out float maxDistance;

void main() {
    vec3 ndc = position * 2.0 - vec3(1);
    geom_position = ndc;
    gl_Position = projection * view * vec4(ndc, 1);

    maxDistance = 1.0;
    debugConeTrace(position, axis, coneAngle, maxDistance, false);
}

#shader geometry

#version 460 core

layout (points) in;
layout (line_strip, max_vertices = 80) out;

in vec3 geom_position[];
in float maxDistance[];

out vec4 frag_color;

uniform mat4 projection;
uniform mat4 view;

uniform vec3 axis;
uniform float coneAngle;

const float PI = 3.14159;

#include "./_drawCone.glsl"

void main() {
    vec3 helper = axis - vec3(0.1, 0, 0); // Random vector
    vec3 tangent = normalize(helper - dot(axis, helper) * axis);
    vec3 bitangent = cross(axis, tangent);

    vec4 startPosition = vec4(geom_position[0], 1);

    frag_color = vec4(1, 1, 0, 1);

    // gl_Position = projection * view * startPosition;
    // EmitVertex();

    // gl_Position = projection * view * (startPosition + vec4(axis, 0) * MAGNITUDE);
    // EmitVertex();

    // EndPrimitive();

    float angleFromAxis = coneAngle;
    // TODO: Corroborar que los angulos dan lo mismo
    float angleFromPlane = (PI / 2) - angleFromAxis;
    drawCone(geom_position[0], axis, angleFromPlane, maxDistance[0]);

    float angle = 1.0472;
    float sinAngle = sin(angle);
    float cosAngle = cos(angle);
    vec3 direction;

    // direction = sinAngle * axis + cosAngle * tangent;
    // drawCone(geom_position[0], direction, angleFromPlane);

    // direction = sinAngle * axis - cosAngle * tangent;
    // drawCone(geom_position[0], direction, angleFromPlane);

    // direction = sinAngle * axis + cosAngle * bitangent;
    // drawCone(geom_position[0], direction, angleFromPlane);

    // direction = sinAngle * axis - cosAngle * bitangent;
    // drawCone(geom_position[0], direction, angleFromPlane);
}

#shader fragment

#version 460 core

layout (location = 0) out vec4 FragColor;

in vec4 frag_color;

void main() {
    FragColor = frag_color;
}
