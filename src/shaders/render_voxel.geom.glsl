#version 430 core

layout (points) in;
layout (triangle_strip, max_vertices = 22) out;

out flat int geom_vertex_id;
out flat vec3 fragment_normal;

in vec4 voxel_position[];
in int vertex_id[];

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;
uniform int voxel_dimension;
uniform float half_dimension; // TODO: Why is this half dimension?

mat4 canonization_matrix = projection * view * model;

void create_z_positive_face() {
    vec4 position;
    fragment_normal = vec3(0,0,1);

    position = vec4(
        voxel_position[0].x - half_dimension,
        voxel_position[0].y - half_dimension,
        voxel_position[0].z + half_dimension,
        voxel_position[0].w
    );
    gl_Position = canonization_matrix * position;
    EmitVertex();

    position = vec4(
        voxel_position[0].x - half_dimension,
        voxel_position[0].y + half_dimension,
        voxel_position[0].z + half_dimension,
        voxel_position[0].w
    );
    gl_Position = canonization_matrix * position;
    EmitVertex();

    position = vec4(
        voxel_position[0].x + half_dimension,
        voxel_position[0].y - half_dimension,
        voxel_position[0].z + half_dimension,
        voxel_position[0].w
    );
    gl_Position = canonization_matrix * position;
    EmitVertex();

    position = vec4(
        voxel_position[0].x + half_dimension,
        voxel_position[0].y + half_dimension,
        voxel_position[0].z + half_dimension,
        voxel_position[0].w
    );
    gl_Position = canonization_matrix * position;
    EmitVertex();
}

void create_x_positive_face() {
    vec4 position;
    fragment_normal = vec3(1,0,0);

    position = vec4(
        voxel_position[0].x + half_dimension,
        voxel_position[0].y - half_dimension,
        voxel_position[0].z - half_dimension,
        voxel_position[0].w
    );
    gl_Position = canonization_matrix * position;
    EmitVertex();

    position = vec4(
        voxel_position[0].x + half_dimension,
        voxel_position[0].y + half_dimension,
        voxel_position[0].z - half_dimension,
        voxel_position[0].w
    );
    gl_Position = canonization_matrix * position;
    EmitVertex();
}

void create_z_negative_face() {
    vec4 position;
    fragment_normal = vec3(0,0,-1);

    position = vec4(
        voxel_position[0].x - half_dimension,
        voxel_position[0].y - half_dimension,
        voxel_position[0].z - half_dimension,
        voxel_position[0].w
    );
    gl_Position = canonization_matrix * position;
    EmitVertex();

    position = vec4(
        voxel_position[0].x - half_dimension,
        voxel_position[0].y + half_dimension,
        voxel_position[0].z - half_dimension,
        voxel_position[0].w
    );
    gl_Position = canonization_matrix * position;
    EmitVertex();
}

void create_x_negative_face() {
    vec4 position;
    fragment_normal = vec3(-1,0,0);

    position = vec4(
        voxel_position[0].x - half_dimension,
        voxel_position[0].y - half_dimension,
        voxel_position[0].z + half_dimension,
        voxel_position[0].w
    );
    gl_Position = canonization_matrix * position;
    EmitVertex();

    position = vec4(
        voxel_position[0].x - half_dimension,
        voxel_position[0].y + half_dimension,
        voxel_position[0].z + half_dimension,
        voxel_position[0].w
    );
    gl_Position = canonization_matrix * position;
    EmitVertex();
}

void create_y_positive_face() {
    vec4 position;
    fragment_normal = vec3(0,1,0);

    position = vec4(
        voxel_position[0].x - half_dimension,
        voxel_position[0].y + half_dimension,
        voxel_position[0].z - half_dimension,
        voxel_position[0].w
    );
    gl_Position = canonization_matrix * position;
    EmitVertex();

    position = vec4(
        voxel_position[0].x + half_dimension,
        voxel_position[0].y + half_dimension,
        voxel_position[0].z - half_dimension,
        voxel_position[0].w
    );
    gl_Position = canonization_matrix * position;
    EmitVertex();

    EmitVertex(); // To start from scratch

    position = vec4(
        voxel_position[0].x + half_dimension,
        voxel_position[0].y + half_dimension,
        voxel_position[0].z + half_dimension,
        voxel_position[0].w
    );
    gl_Position = canonization_matrix * position;
    EmitVertex();

    position = vec4(
        voxel_position[0].x - half_dimension,
        voxel_position[0].y + half_dimension,
        voxel_position[0].z + half_dimension,
        voxel_position[0].w
    );
    gl_Position = canonization_matrix * position;
    EmitVertex();
}

void create_y_negative_face() {
    vec4 position;
    fragment_normal = vec3(0,-1,0);

    position = vec4(
        voxel_position[0].x - half_dimension,
        voxel_position[0].y - half_dimension,
        voxel_position[0].z + half_dimension,
        voxel_position[0].w
    );
    gl_Position = canonization_matrix * position;
    EmitVertex();
    EmitVertex(); // To start from here

    position = vec4(
        voxel_position[0].x - half_dimension,
        voxel_position[0].y - half_dimension,
        voxel_position[0].z - half_dimension,
        voxel_position[0].w
    );
    gl_Position = canonization_matrix * position;
    EmitVertex();

    position = vec4(
        voxel_position[0].x + half_dimension,
        voxel_position[0].y - half_dimension,
        voxel_position[0].z + half_dimension,
        voxel_position[0].w
    );
    gl_Position = canonization_matrix * position;
    EmitVertex();

    position = vec4(
        voxel_position[0].x + half_dimension,
        voxel_position[0].y - half_dimension,
        voxel_position[0].z - half_dimension,
        voxel_position[0].w
    );
    gl_Position = canonization_matrix * position;
    EmitVertex();
}

void main() {
    create_z_positive_face();
    create_x_positive_face();
    create_z_negative_face();
    create_x_negative_face();

    create_y_positive_face();

    EmitVertex(); // To start from scratch

    create_y_negative_face();

    EndPrimitive();

    geom_vertex_id = vertex_id[0];
}
