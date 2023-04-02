#version 460 core

layout (location = 0) in vec3 position;

out vec3 geom_position;

void main() {
    geom_position = position;
}
