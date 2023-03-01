#version 460 core

layout (points) in;
layout (line_strip, max_vertices = 256) out;

in vec4 geom_voxelPosition[];
in float geom_halfNodeSize[];

out vec4 frag_nodeColor;

#include "assets/shaders/octree/_drawCube.glsl"

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

mat4 canonizationMatrix = projection * view * model;

void main() {
    vec4 voxelCenter = geom_voxelPosition[0];
    vec4 voxelColor = vec4(1.0, 0.0, 0.0, 1.0);
    drawCube(voxelCenter, geom_halfNodeSize[0], canonizationMatrix, voxelColor);

    vec4 voxelOrigin = voxelCenter - vec4(vec3(geom_halfNodeSize[0]), 0.0) + vec4(0.001, 0.0, 0.0, 0.0);
    gl_Position = canonizationMatrix * voxelOrigin;
    frag_nodeColor = vec4(0.0, 1.0, 0.0, 1.0);
    EmitVertex();

    float modifier = 2.1;
    float step = 0.0078125 * modifier;
    gl_Position = canonizationMatrix * (voxelOrigin + vec4(0.0, step, 0.0, 0.0));
    frag_nodeColor = vec4(0.0, 1.0, 0.0, 1.0);
    EmitVertex();
    EndPrimitive();
}
