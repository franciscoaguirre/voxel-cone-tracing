#version 460 core

layout (location = 0) out vec4 FragColor;

in flat vec4 frag_nodeColor;

void main() {
    if (frag_nodeColor == vec4(0)) {
        discard;
    }
    FragColor = frag_nodeColor;
}
