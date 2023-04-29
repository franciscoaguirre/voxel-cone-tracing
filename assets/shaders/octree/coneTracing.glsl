#shader vertex

#version 460 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 normal;
layout (location = 2) in vec2 textureCoordinates;

out vec3 frag_normal;
out vec2 frag_textureCoordinates;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main() {
    gl_Position = projection * view * model * vec4(position, 1.0);
    frag_normal = normal;
    frag_textureCoordinates = textureCoordinates;
}

#shader fragment

#version 460 core

layout (location = 0) out vec4 FragColor;

in vec3 frag_normal;
in vec2 frag_textureCoordinates;

uniform uint voxelDimension;
uniform sampler2D texture_diffuse1;

void main() {
    vec4 color = vec4(0, 0, 0, 1);
    
    FragColor = texture(texture_diffuse1, frag_textureCoordinates);
}
