#version 460 core

layout (points) in;
layout (line_strip, max_vertices = 30) out;

in vec3 geom_position[];

out vec4 frag_nodeColor;

uniform mat4 projection;
uniform mat4 view;
uniform mat4 model;

uniform vec3 color;
uniform vec3 dimensions;

mat4 canonizationMatrix = projection * view * model;

#include "assets/shaders/octree/_drawCube.glsl"

void drawOrtho(vec4 center);

void main() {
    vec4 center = vec4(geom_position[0], 1.0);
    drawCube(center, dimensions, canonizationMatrix, vec4(color, 1.0));
    drawOrtho(center);
}

void drawOrtho(vec4 center) {
    vec4 position;

    position = vec4(
        center.x - dimensions.x,
        center.y - dimensions.y,
        center.z,
        center.w
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();

    position = vec4(
        center.x - dimensions.x,
        center.y - dimensions.y,
        center.z + 10.0,
        center.w
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();
    EndPrimitive();

    position = vec4(
        center.x + dimensions.x,
        center.y - dimensions.y,
        center.z,
        center.w
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();

    position = vec4(
        center.x + dimensions.x,
        center.y - dimensions.y,
        center.z + 10.0,
        center.w
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();
    EndPrimitive();

    position = vec4(
        center.x + dimensions.x,
        center.y + dimensions.y,
        center.z,
        center.w
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();

    position = vec4(
        center.x + dimensions.x,
        center.y + dimensions.y,
        center.z + 10.0,
        center.w
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();
    EndPrimitive();

    position = vec4(
        center.x - dimensions.x,
        center.y + dimensions.y,
        center.z,
        center.w
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();

    position = vec4(
        center.x - dimensions.x,
        center.y + dimensions.y,
        center.z + 10.0,
        center.w
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();
    EndPrimitive();
}
