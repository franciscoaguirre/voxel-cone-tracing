#version 460 core

layout(location = 0) in vec3 point;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

mat4 canonizationMatrix = projection * view * model;

void main() {
    gl_Position = canonizationMatrix * vec4(point, 1.0);
}
