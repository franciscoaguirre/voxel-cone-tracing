#shader vertex

#version 460 core

uniform vec2 gBufferQueryCoordinates;

struct PointLight {
    vec3 position;
    vec3 color;
};

uniform PointLight pointLight;

out VertexData {
    vec3 position;
    vec3 direction;
} Out;

void main() {
    int threadIndex = gl_VertexID;
    vec3 positionWorldSpace = texture(gBufferPositions, gBufferQueryCoordinates).xyz;
    Out.position = positionWorldSpace;
    Out.direction = normalize(pointLight.position - positionWorldSpace);
}

#shader geometry

#version 460 core

layout (points) in;
layout (line_strip, max_vertices = 10) out;

uniform mat4 projection;
uniform mat4 view;

in VertexData {
    vec3 position;
    vec3 direction;
} In;

vec3 toVoxelSpace(vec3 positionInWorldSpace) {
    return 0.5f * positionInWorldSpace + 0.5f;
}

void main() {
    vec3 from = In.position;
    vec3 direction = In.direction;
    float distance = 0;

    float voxelSize = 1.f / 256.f;
    int steps = 100;

    for (int i = 0; i < steps; i++) {
        vec3 current = from + distance * direction;
        gl_Position = projection * view * current;
        EmitVertex();
        distance += voxelSize;
    }
}

#shader fragment

#version 460 core

out vec4 color;

void main() {
    color = vec4(1, 0, 0, 1);
}
