#shader vertex

#version 460 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 normal;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main() {
    gl_Position = projection * view * model * vec4(position, 1.0);
}

#shader fragment

#version 460 core

layout (location = 0) out vec4 FragColor;

uniform uint voxelDimension;

void main() {
    vec4 color = vec4(0, 0, 0, 1);
    
    FragColor = vec4(1.0, 0.0, 0.0, 1.0);
}
