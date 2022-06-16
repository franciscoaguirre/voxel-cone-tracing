#version 430 core

layout (location = 0) out vec4 FragColor;

in flat int geom_vertex_id;

void main() {
    FragColor = vec4(256 - (geom_vertex_id % 256), (geom_vertex_id % 256), 256 - (geom_vertex_id % 256), 1.0);
}
