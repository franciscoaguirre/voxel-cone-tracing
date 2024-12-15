#version 460 core

layout (location = 0) out vec4 FragColor;

in flat vec3 frag_normal;
in flat vec4 frag_nodeColor;

void main() {
    FragColor = vec4(frag_nodeColor.xyz, 1.0);
}
