#version 460 core

layout (points) in;
layout (line_strip, max_vertices = 30) out;

out vec4 frag_nodeColor;

uniform mat4 projection;
uniform mat4 view;
uniform mat4 model;

uniform vec3 color = vec3(1);

mat4 canonizationMatrix = projection * view * model;

#include "assets/shaders/octree/_drawCube.glsl"

void main() {
    vec4 center = gl_in[0].gl_Position;
    float halfNodeSize = 0.01;
    drawCube(center, halfNodeSize, canonizationMatrix, vec4(color, 1));
}
