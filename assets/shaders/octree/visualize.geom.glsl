#version 460 core

layout (points) in;
layout (line_strip, max_vertices = 256) out;

in vec4 geom_nodePosition[];
in float geom_halfNodeSize[];
in vec4 nodeColor[];

out flat vec4 frag_nodeColor;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

uniform layout(binding = 4, r32ui) uimage3D brickPoolPhotons;

#include "./_drawCube.glsl"

mat4 canonizationMatrix = projection * view * model;

void main() {
    vec4 nodePosition = geom_nodePosition[0];
    vec4 cubeCenter = nodePosition;
    // drawCube(cubeCenter, geom_halfNodeSize[0], canonizationMatrix, nodeColor[0]);
}
