#shader vertex

#version 460 core

layout (location = 0) in vec3 position;

uniform mat4 projection;
uniform mat4 view;

out VertexData {
    vec3 worldPosition;
} Out;

void main() {
    Out.worldPosition = position;
    gl_Position = projection * view * vec4(Out.worldPosition, 1);
}

#shader fragment

#version 460 core

in VertexData {
    vec3 worldPosition;
} In;

out vec4 color;

void main() {
    color.rgb = In.worldPosition;
}
