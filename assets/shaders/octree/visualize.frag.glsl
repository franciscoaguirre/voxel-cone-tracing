#version 460 core

layout (location = 0) out vec4 FragColor;

in flat vec4 frag_nodeColor;

void main() {
    FragColor = vec4(0.0, 1.0, 1.0, 1.0);
}
