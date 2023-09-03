#version 460 core

layout (triangles) in;
layout (triangle_strip, max_vertices = 3) out;

in vec2 TexCoords[];
in vec3 Normal[];
out vec2 fragTexCoords;
out vec3 fragNormal;

void main() {
    for (int i = 0; i < 3; i++) {
      gl_Position = gl_in[i].gl_Position;
      fragTexCoords = TexCoords[i];
      fragNormal = Normal[i];
      EmitVertex();
    }

    EndPrimitive();
}
