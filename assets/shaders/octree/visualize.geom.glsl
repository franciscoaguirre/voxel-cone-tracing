#version 460 core

layout (points) in;
layout (line_strip, max_vertices = 128) out;

in VertexData {
    vec4 nodePosition;
    float halfNodeSize;
} In[];

out vec4 frag_nodeColor;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

#include "./_drawCube.glsl"

mat4 canonizationMatrix = projection * view * model;

void main() {
    vec4 nodePosition = In[0].nodePosition;
    vec4 cubeCenter = nodePosition;
    vec4 cubeColor = vec4(0, 0, 1, 1);
    drawCube(cubeCenter, In[0].halfNodeSize, canonizationMatrix, cubeColor);
}
