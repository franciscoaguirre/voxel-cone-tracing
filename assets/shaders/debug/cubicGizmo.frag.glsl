#version 460 core

layout (location = 0) out vec4 FragColor;

in vec4 frag_nodeColor;

void main() {
    FragColor = frag_nodeColor;
}
