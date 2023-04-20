#version 460 core

layout (triangles) in;
layout (triangle_strip, max_vertices = 3) out;

in vec2 TexCoords[];
out vec2 fragTexCoords;

void main() {
    for (int i = 0; i < 3; i++) {
      gl_Position = gl_in[i].gl_Position;
      fragTexCoords = TexCoords[i];
      EmitVertex();
    }

    EndPrimitive();
}
