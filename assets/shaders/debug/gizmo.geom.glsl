#version 460 core

layout (points) in;
layout (line_strip, max_vertices = 20) out;

in vec3 geom_position[];
in vec3 geom_color[];

out vec4 frag_nodeColor;

uniform mat4 projection;
uniform mat4 view;
uniform mat4 model;

mat4 canonizationMatrix = projection * view * model;

#include "assets/shaders/octree/_drawCube.glsl"

void main() {
    vec4 center = vec4(geom_position[0], 1.0);
    float halfNodeSize = 0.1;
    vec4 color = vec4(geom_color[0], 1.0);
    drawCube(center, halfNodeSize, canonizationMatrix, color);
}
