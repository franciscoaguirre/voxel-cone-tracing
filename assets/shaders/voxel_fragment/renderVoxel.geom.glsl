#version 460 core

layout (points) in;
layout (triangle_strip, max_vertices = 22) out;

out flat vec3 frag_normal;
out flat vec4 frag_nodeColor;

in vec4 geom_position[];
in vec4 geom_color[];
in int geom_vertexID[];

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

uniform float halfDimension; // TODO: Why is this half dimension?

mat4 canonizationMatrix = projection * view * model;

#include "assets/shaders/octree/_drawCubeFilled.glsl"

void main() {
    drawCubeFilled(geom_position[0], halfDimension, canonizationMatrix, geom_color[0]);
}
