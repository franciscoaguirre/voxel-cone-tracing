#version 460 core

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

uniform uint numberOfPoints;
uniform vec3 points[5];

mat4 canonizationMatrix = projection * view * model;

void main() {
    for (uint i = 0; i < numberOfPoints; i++) {
        gl_Position = canonizationMatrix * vec4(points[i], 1.0);
    }
}
