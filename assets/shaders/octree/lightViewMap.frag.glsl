#version 460 core

layout (location = 0) out vec4 FragColor;

in vec4 frag_position;

void main() {
    FragColor = vec4(((frag_position.xyz / frag_position.w) + vec3(1.0)) / 2.0, 1.0);
}
