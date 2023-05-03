#shader vertex

#version 460 core

uniform mat4 projection;
uniform mat4 view;

out vec3 geom_position;

vec3 position = vec3(0.5, 0.5, 0.46);

void main() {
    vec3 ndc = position * 2.0 - vec3(1);
    geom_position = ndc;
    gl_Position = projection * view * vec4(ndc, 1);
}

#shader geometry

#version 460 core

layout (points) in;
layout (line_strip, max_vertices = 80) out;

in vec3 geom_position[];

out vec4 frag_color;

uniform mat4 projection;
uniform mat4 view;

const float MAGNITUDE = 0.5;
const float PI = 3.14159;

#include "./_drawCone.glsl"

void main() {
    vec3 axis = vec3(0, 0, 1);
    vec3 helper = axis - vec3(0.1, 0, 0); // Random vector
    vec3 tangent = normalize(helper - dot(axis, helper) * axis);
    vec3 bitangent = cross(axis, tangent);

    vec4 startPosition = vec4(geom_position[0], 1);

    frag_color = vec4(1, 1, 0, 1);

    // gl_Position = projection * view * startPosition;
    // EmitVertex();

    // gl_Position = projection * view * (startPosition + vec4(axis, 0) * MAGNITUDE);
    // EmitVertex();

    // EndPrimitive();
    
    float angleFromAxis = 0.261799;
    float angleFromPlane = (PI / 2) - angleFromAxis;
    drawCone(geom_position[0], axis, angleFromPlane);

    float angle = 1.0472;
    float sinAngle = sin(angle);
    float cosAngle = cos(angle);
    vec3 direction;

    // direction = sinAngle * axis + cosAngle * tangent;
    // drawCone(geom_position[0], direction, angleFromPlane);

    // direction = sinAngle * axis - cosAngle * tangent;
    // drawCone(geom_position[0], direction, angleFromPlane);

    // direction = sinAngle * axis + cosAngle * bitangent;
    // drawCone(geom_position[0], direction, angleFromPlane);

    // direction = sinAngle * axis - cosAngle * bitangent;
    // drawCone(geom_position[0], direction, angleFromPlane);
}

#shader fragment

#version 460 core

layout (location = 0) out vec4 FragColor;

in vec4 frag_color;

void main() {
    FragColor = frag_color;
}
