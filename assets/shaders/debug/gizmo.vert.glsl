#version 460 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 color;

out vec3 geom_position;
out vec3 geom_color;

void main() {
    geom_position = position;
    geom_color = color;
}
