#version 460 core

layout (points) in;
layout (line_strip, max_vertices = 256) out;

in vec4 nodePosition[];
in float geom_halfNodeSize[];
in vec4 nodeColor[];
in ivec3 geom_brickCoordinates[];

out flat vec4 frag_nodeColor;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

uniform layout(binding = 2, rgba8) image3D brickPoolColors;

#include "./_drawCube.glsl"

mat4 canonizationMatrix = projection * view * model;

void main() {
    vec4 cubeCenter = nodePosition[0];
    drawCube(cubeCenter, geom_halfNodeSize[0], canonizationMatrix, nodeColor[0]);
}
