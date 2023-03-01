#version 460 core

layout (points) in;
layout (line_strip, max_vertices = 256) out;

in vec4 geom_nodePosition[];
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

    vec4 nodeOrigin = nodeCenter - vec4(vec3(geom_halfNodeSize[0]), 0.0) + vec4(0.001, 0.0, 0.0, 0.0);
    gl_Position = canonizationMatrix * nodeOrigin;
    frag_nodeColor = vec4(0.0, 1.0, 0.0, 1.0);
    EmitVertex();

    gl_Position = canonizationMatrix * (nodeOrigin + vec4(0.0, geom_halfNodeSize[0], 0.0, 0.0));
    frag_nodeColor = vec4(0.0, 1.0, 0.0, 1.0);
    EmitVertex();
    EndPrimitive();
}
