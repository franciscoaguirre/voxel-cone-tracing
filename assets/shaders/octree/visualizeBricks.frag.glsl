#version 460 core

layout (location = 0) out vec4 FragColor;

in flat vec4 frag_nodeColor;

void main() {
    if (frag_nodeColor.xyz == vec3(0)) {
        discard;
    }
    FragColor = vec4(frag_nodeColor.xyz, 1.0);
}
