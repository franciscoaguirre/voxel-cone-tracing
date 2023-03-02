#version 460 core

layout (points) in;
layout (line_strip, max_vertices = 256) out;

in vec4 geom_nodePosition[];
in vec4 geom_neighbourPosition[];
in float geom_hasNeighborX[];
in float geom_halfNodeSize[];

out vec4 frag_nodeColor;

#include "assets/shaders/octree/_drawCube.glsl"

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

mat4 canonizationMatrix = projection * view * model;

void main() {
    vec4 nodeCenter = geom_nodePosition[0];
    vec4 nodeColor = vec4(1.0, 0.0, 0.0, 1.0);
    drawCube(nodeCenter, geom_halfNodeSize[0], canonizationMatrix, nodeColor);

    if (geom_hasNeighborX[0] == 1.0) {
        drawCube(geom_neighbourPosition[0], geom_halfNodeSize[0], canonizationMatrix, vec4(0.0, 1.0, 0.0, 1.0));
    }
}
