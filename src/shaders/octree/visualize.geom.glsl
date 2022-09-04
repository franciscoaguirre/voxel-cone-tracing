#version 430 core

layout (points) in;
layout (triangle_strip, max_vertices = 22) out;

in vec4 node_position[];
in float half_node_size[];
in int non_empty_branch[];

out flat int branch_not_empty;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

mat4 canonization_matrix = projection * view * model;

void create_z_positive_face() {
    vec4 position;

    position = vec4(
        node_position[0].x - half_node_size[0],
        node_position[0].y - half_node_size[0],
        node_position[0].z + half_node_size[0],
        node_position[0].w
    );
    gl_Position = canonization_matrix * position;
    EmitVertex();

    position = vec4(
        node_position[0].x - half_node_size[0],
        node_position[0].y + half_node_size[0],
        node_position[0].z + half_node_size[0],
        node_position[0].w
    );
    gl_Position = canonization_matrix * position;
    EmitVertex();

    position = vec4(
        node_position[0].x + half_node_size[0],
        node_position[0].y - half_node_size[0],
        node_position[0].z + half_node_size[0],
        node_position[0].w
    );
    gl_Position = canonization_matrix * position;
    EmitVertex();

    position = vec4(
        node_position[0].x + half_node_size[0],
        node_position[0].y + half_node_size[0],
        node_position[0].z + half_node_size[0],
        node_position[0].w
    );
    gl_Position = canonization_matrix * position;
    EmitVertex();
}

void create_x_positive_face() {
    vec4 position;

    position = vec4(
        node_position[0].x + half_node_size[0],
        node_position[0].y - half_node_size[0],
        node_position[0].z - half_node_size[0],
        node_position[0].w
    );
    gl_Position = canonization_matrix * position;
    EmitVertex();

    position = vec4(
        node_position[0].x + half_node_size[0],
        node_position[0].y + half_node_size[0],
        node_position[0].z - half_node_size[0],
        node_position[0].w
    );
    gl_Position = canonization_matrix * position;
    EmitVertex();
}

void create_z_negative_face() {
    vec4 position;

    position = vec4(
        node_position[0].x - half_node_size[0],
        node_position[0].y - half_node_size[0],
        node_position[0].z - half_node_size[0],
        node_position[0].w
    );
    gl_Position = canonization_matrix * position;
    EmitVertex();

    position = vec4(
        node_position[0].x - half_node_size[0],
        node_position[0].y + half_node_size[0],
        node_position[0].z - half_node_size[0],
        node_position[0].w
    );
    gl_Position = canonization_matrix * position;
    EmitVertex();
}

void create_x_negative_face() {
    vec4 position;

    position = vec4(
        node_position[0].x - half_node_size[0],
        node_position[0].y - half_node_size[0],
        node_position[0].z + half_node_size[0],
        node_position[0].w
    );
    gl_Position = canonization_matrix * position;
    EmitVertex();

    position = vec4(
        node_position[0].x - half_node_size[0],
        node_position[0].y + half_node_size[0],
        node_position[0].z + half_node_size[0],
        node_position[0].w
    );
    gl_Position = canonization_matrix * position;
    EmitVertex();
}

void create_y_positive_face() {
    vec4 position;

    position = vec4(
        node_position[0].x - half_node_size[0],
        node_position[0].y + half_node_size[0],
        node_position[0].z - half_node_size[0],
        node_position[0].w
    );
    gl_Position = canonization_matrix * position;
    EmitVertex();

    position = vec4(
        node_position[0].x + half_node_size[0],
        node_position[0].y + half_node_size[0],
        node_position[0].z - half_node_size[0],
        node_position[0].w
    );
    gl_Position = canonization_matrix * position;
    EmitVertex();

    EmitVertex(); // To start from scratch

    position = vec4(
        node_position[0].x + half_node_size[0],
        node_position[0].y + half_node_size[0],
        node_position[0].z + half_node_size[0],
        node_position[0].w
    );
    gl_Position = canonization_matrix * position;
    EmitVertex();

    position = vec4(
        node_position[0].x - half_node_size[0],
        node_position[0].y + half_node_size[0],
        node_position[0].z + half_node_size[0],
        node_position[0].w
    );
    gl_Position = canonization_matrix * position;
    EmitVertex();
}

void create_y_negative_face() {
    vec4 position;

    position = vec4(
        node_position[0].x - half_node_size[0],
        node_position[0].y - half_node_size[0],
        node_position[0].z + half_node_size[0],
        node_position[0].w
    );
    gl_Position = canonization_matrix * position;
    EmitVertex();
    EmitVertex(); // To start from here

    position = vec4(
        node_position[0].x - half_node_size[0],
        node_position[0].y - half_node_size[0],
        node_position[0].z - half_node_size[0],
        node_position[0].w
    );
    gl_Position = canonization_matrix * position;
    EmitVertex();

    position = vec4(
        node_position[0].x + half_node_size[0],
        node_position[0].y - half_node_size[0],
        node_position[0].z + half_node_size[0],
        node_position[0].w
    );
    gl_Position = canonization_matrix * position;
    EmitVertex();

    position = vec4(
        node_position[0].x + half_node_size[0],
        node_position[0].y - half_node_size[0],
        node_position[0].z - half_node_size[0],
        node_position[0].w
    );
    gl_Position = canonization_matrix * position;
    EmitVertex();
}

void main() {
    branch_not_empty = non_empty_branch[0];
    create_z_positive_face();
    create_x_positive_face();
    create_z_negative_face();
    create_x_negative_face();

    create_y_positive_face();

    EmitVertex(); // To start from scratch

    create_y_negative_face();

    EndPrimitive();
}
