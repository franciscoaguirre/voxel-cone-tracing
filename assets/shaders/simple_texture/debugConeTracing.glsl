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
    Out.direction = pointLight.position - positionWorldSpace;
    gl_Position = projection * view * vec4(positionWorldSpace, 1);
}

#shader geometry

#version 460 core

#define STEP_LENGTH 0.005f
#define INV_STEP_LENGTH (1.0f / STEP_LENGTH)

layout (points) in;
layout (points, max_vertices = 256) out;

uniform mat4 projection;
uniform mat4 view;

uniform sampler3D voxelsTexture;

in VertexData {
    vec3 position;
    vec3 direction;
} In[];

out GeometryData {
    vec4 color;
} Out;

vec3 scaleAndBias(const vec3 p) {
    return 0.5f * p + 0.5f;
}

// For debugging, they show up in renderDoc.
out uint numberOfSteps;
out vec3 voxelSpaceCoordinates;
out float targetDistance;

void main() {
    vec3 from = In[0].position;
    vec3 direction = In[0].direction;

    targetDistance = length(direction);
    numberOfSteps = uint(INV_STEP_LENGTH * targetDistance);
    direction = normalize(direction);

    for (int step = 0; step < numberOfSteps; ++step) {
        vec3 current = from + STEP_LENGTH * step * direction;
        gl_Position = projection * view * vec4(current, 1);
        voxelSpaceCoordinates = scaleAndBias(current);
        Out.color = textureLod(voxelsTexture, voxelSpaceCoordinates, 0);
        EmitVertex();
    }

    EndPrimitive();
}

#shader fragment

#version 460 core

in GeometryData {
    vec4 color;
} In;

out vec4 color;

void main() {
    color = In.color;
    if (color.a == 0) {
        discard;
    }
}
