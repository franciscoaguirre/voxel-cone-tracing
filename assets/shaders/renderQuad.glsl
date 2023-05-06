#shader vertex

#version 460 core

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aTexCoords;

out vec2 TexCoords;

void main() {
    gl_Position = vec4(aPos, 1.0);
    TexCoords = aTexCoords;
}

#shader fragment

#version 460 core

layout (location = 0) out vec4 FragColor;

in vec2 TexCoords;

uniform sampler2D ourTexture;

void main() {
    FragColor = texture(ourTexture, TexCoords);
}
