#shader vertex

#version 460 core

layout (location = 0) in vec3 position;

out VertexData {
    vec2 textureCoordinates;
} Out;

void main() {
    gl_Position = vec4(position, 1.0);
    Out.textureCoordinates = position.xy * 0.5 + 0.5;
}

#shader fragment

#version 460 core

out vec4 FragColor;

in VertexData {
    vec2 textureCoordinates;
} In;

uniform sampler2D depthMap;

void main() {
    float depthValue = texture(depthMap, In.textureCoordinates).r;
    FragColor = vec4(vec3(depthValue), 1.0);
}
