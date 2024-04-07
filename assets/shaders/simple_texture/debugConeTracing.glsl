#shader vertex

#version 460 core

uniform vec2 gBufferQueryCoordinates;

struct PointLight {
    vec3 position;
    vec3 color;
};

uniform PointLight pointLight;

uniform sampler2D gBufferPositions;

uniform mat4 projection;
uniform mat4 view;

out VertexData {
    vec3 position;
    vec3 direction;
} Out;

void main() {
    vec3 positionWorldSpace = texture(gBufferPositions, gBufferQueryCoordinates).xyz;
    Out.position = positionWorldSpace;
    Out.direction = normalize(pointLight.position - positionWorldSpace);
    gl_Position = projection * view * vec4(positionWorldSpace, 1);
}

#shader geometry

#version 460 core

layout (points) in;
layout (line_strip, max_vertices = 256) out;

uniform mat4 projection;
uniform mat4 view;

in VertexData {
    vec3 position;
    vec3 direction;
} In[];

void main() {
    vec3 from = In[0].position;
    vec3 direction = In[0].direction;
    float distance = 0;

    float voxelSize = 1.f / 256.f;
    int steps = 256;

    for (int i = 0; i < steps; i++) {
        vec3 current = from + distance * direction;
        gl_Position = projection * view * vec4(current, 1);
        EmitVertex();
        distance += voxelSize;
    }

    EndPrimitive();
}

#shader fragment

#version 460 core

out vec4 color;

void main() {
    color = vec4(1, 0, 1, 1);
}
