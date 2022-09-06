#version 430 core

layout (points) in;
layout (line_strip, max_vertices = 22) out;

in vec4 node_position[];
in float half_node_size[];
in int non_empty_branch[];
in uint geometry_color[];

out flat int branch_not_empty;
out vec4 fragment_color;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

vec4 convRGBA8ToVec4(in uint val)
{
    return vec4(
        float((int(val) & 0x000000FF)),
        float((int(val) & 0x0000FF00) >> 8U),
	    float((int(val) & 0x00FF0000) >> 16U),
        float((int(val) & 0xFF000000) >> 24U)
    );
}

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
        node_position[0].x - half_node_size[0],
        node_position[0].y - half_node_size[0],
        node_position[0].z + half_node_size[0],
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
        node_position[0].x + half_node_size[0],
        node_position[0].y + half_node_size[0],
        node_position[0].z + half_node_size[0],
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

    position = vec4(
        node_position[0].x - half_node_size[0],
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

void create_y_negative_face() {
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
        node_position[0].y - half_node_size[0],
        node_position[0].z + half_node_size[0],
        node_position[0].w
    );
    gl_Position = canonization_matrix * position;
    EmitVertex();
}

void main() {
    vec4 color = convRGBA8ToVec4(geometry_color[0]).rgba / 255;
    fragment_color = color;

    branch_not_empty = non_empty_branch[0];
    create_z_positive_face();
    create_x_negative_face();
    create_y_positive_face();
    create_z_negative_face();
    create_y_negative_face();

    EndPrimitive();
}
