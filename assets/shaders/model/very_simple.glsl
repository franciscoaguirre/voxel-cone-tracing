#shader vertex

#version 460 core

layout (location = 0) in vec3 position;

uniform mat4 projection;
uniform mat4 view;

void main() {
    gl_Position = projection * view * vec4(position, 1);
}

#shader fragment

#version 460 core

out vec4 color;

void main() {
    color = vec4(1);
}
