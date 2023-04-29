#version 460 core

layout (points) in;
layout (triangle_strip, max_vertices = 22) out;

out flat vec3 frag_normal;
out flat vec4 frag_color;

in vec4 geom_position[];
in vec4 geom_color[];
in int geom_vertexID[];

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

uniform float halfDimension; // TODO: Why is this half dimension?

mat4 canonizationMatrix = projection * view * model;

void createZPositiveFace() {
    vec4 position;
    frag_normal = vec3(0,0,1);

    position = vec4(
        geom_position[0].x - halfDimension,
        geom_position[0].y - halfDimension,
        geom_position[0].z + halfDimension,
        geom_position[0].w
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();

    position = vec4(
        geom_position[0].x - halfDimension,
        geom_position[0].y + halfDimension,
        geom_position[0].z + halfDimension,
        geom_position[0].w
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();

    position = vec4(
        geom_position[0].x + halfDimension,
        geom_position[0].y - halfDimension,
        geom_position[0].z + halfDimension,
        geom_position[0].w
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();

    position = vec4(
        geom_position[0].x + halfDimension,
        geom_position[0].y + halfDimension,
        geom_position[0].z + halfDimension,
        geom_position[0].w
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();
}

void createXPositiveFace() {
    vec4 position;
    frag_normal = vec3(1,0,0);

    position = vec4(
        geom_position[0].x + halfDimension,
        geom_position[0].y - halfDimension,
        geom_position[0].z - halfDimension,
        geom_position[0].w
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();

    position = vec4(
        geom_position[0].x + halfDimension,
        geom_position[0].y + halfDimension,
        geom_position[0].z - halfDimension,
        geom_position[0].w
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();
}

void createZNegativeFace() {
    vec4 position;
    frag_normal = vec3(0,0,-1);

    position = vec4(
        geom_position[0].x - halfDimension,
        geom_position[0].y - halfDimension,
        geom_position[0].z - halfDimension,
        geom_position[0].w
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();

    position = vec4(
        geom_position[0].x - halfDimension,
        geom_position[0].y + halfDimension,
        geom_position[0].z - halfDimension,
        geom_position[0].w
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();
}

void createXNegativeFace() {
    vec4 position;
    frag_normal = vec3(-1,0,0);

    position = vec4(
        geom_position[0].x - halfDimension,
        geom_position[0].y - halfDimension,
        geom_position[0].z + halfDimension,
        geom_position[0].w
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();

    position = vec4(
        geom_position[0].x - halfDimension,
        geom_position[0].y + halfDimension,
        geom_position[0].z + halfDimension,
        geom_position[0].w
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();
}

void createYPositiveFace() {
    vec4 position;
    frag_normal = vec3(0,1,0);

    position = vec4(
        geom_position[0].x - halfDimension,
        geom_position[0].y + halfDimension,
        geom_position[0].z - halfDimension,
        geom_position[0].w
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();

    position = vec4(
        geom_position[0].x + halfDimension,
        geom_position[0].y + halfDimension,
        geom_position[0].z - halfDimension,
        geom_position[0].w
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();

    EmitVertex(); // To start from scratch

    position = vec4(
        geom_position[0].x + halfDimension,
        geom_position[0].y + halfDimension,
        geom_position[0].z + halfDimension,
        geom_position[0].w
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();

    position = vec4(
        geom_position[0].x - halfDimension,
        geom_position[0].y + halfDimension,
        geom_position[0].z + halfDimension,
        geom_position[0].w
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();
}

void createYNegativeFace() {
    vec4 position;
    frag_normal = vec3(0,-1,0);

    position = vec4(
        geom_position[0].x - halfDimension,
        geom_position[0].y - halfDimension,
        geom_position[0].z + halfDimension,
        geom_position[0].w
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();
    EmitVertex(); // To start from here

    position = vec4(
        geom_position[0].x - halfDimension,
        geom_position[0].y - halfDimension,
        geom_position[0].z - halfDimension,
        geom_position[0].w
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();

    position = vec4(
        geom_position[0].x + halfDimension,
        geom_position[0].y - halfDimension,
        geom_position[0].z + halfDimension,
        geom_position[0].w
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();

    position = vec4(
        geom_position[0].x + halfDimension,
        geom_position[0].y - halfDimension,
        geom_position[0].z - halfDimension,
        geom_position[0].w
    );
    gl_Position = canonizationMatrix * position;
    EmitVertex();
}

void main() {
    frag_color = geom_color[0];
    createZPositiveFace();
    createXPositiveFace();
    createZNegativeFace();
    createXNegativeFace();

    createYPositiveFace();

    EmitVertex(); // To start from scratch

    createYNegativeFace();

    EndPrimitive();
}
