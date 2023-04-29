#version 460 core

layout (location = 0) out vec4 FragColor;

in flat vec3 frag_normal;
in flat vec4 frag_color;

void main() {
    // Hecho a ojo, la verdad que funciona de milagro
    vec3 pointOfView = vec3(0.25,0.5,-1.0);
    float lol = abs(dot(normalize(frag_normal), pointOfView)); 
    FragColor = vec4(frag_color.xyz * lol, 1.0);
}
