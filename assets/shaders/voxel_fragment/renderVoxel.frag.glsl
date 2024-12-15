#version 460 core

layout (location = 0) out vec4 FragColor;

in flat vec3 frag_normal;
in flat vec4 frag_nodeColor;

void main() {
    vec3 pointOfView = vec3(0.25,0.5,-1.0);
    float diffuse = abs(dot(normalize(frag_normal), pointOfView)); 
    FragColor = vec4(frag_nodeColor.xyz, 1.0);
}
