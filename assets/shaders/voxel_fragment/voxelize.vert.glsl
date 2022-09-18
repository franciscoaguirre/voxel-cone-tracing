#version 460 core

layout (location = 0) in vec3 in_vertex_position;
layout (location = 1) in vec3 in_normal;
layout (location = 2) in vec2 in_tex_coordinates;

out vec3 vertex_position;
out vec3 normal;
out vec2 tex_coordinates;

uniform mat4 model_normalization_matrix;

void main()
{
    normal = in_normal;
    tex_coordinates = in_tex_coordinates;
    gl_Position = model_normalization_matrix * vec4(in_vertex_position, 1.0);
    vertex_position = vec3(gl_Position);
}
