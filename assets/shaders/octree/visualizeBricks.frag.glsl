#version 460 core

layout (location = 0) out vec4 FragColor;

in flat vec4 frag_nodeColor;

void main() {
    if (frag_nodeColor.x < 0.2 || frag_nodeColor.y < 0.2 || frag_nodeColor.z < 0.2) {
        discard;
    }
    FragColor = frag_nodeColor;
}
