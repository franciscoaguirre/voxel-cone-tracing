#shader vertex

#version 460 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 normal;

uniform mat4 projection;
uniform mat4 view;
uniform mat4 model;

out VertexData {
    vec3 worldPosition;
    vec3 normal;
} Out;

void main() {
    Out.worldPosition = vec3(model * vec4(position, 1));
    Out.normal = normalize(mat3(transpose(inverse(model))) * normal);
    gl_Position = projection * view * vec4(Out.worldPosition, 1);
}

#shader geometry

#version 460 core

layout (triangles) in;
layout (triangle_strip, max_vertices = 3) out;

in VertexData {
    vec3 worldPosition;
    vec3 normal;
} In[];

out GeometryData {
    vec3 worldPosition;
    vec3 normal;
} Out;

void main() {
    const vec3 p1 = In[1].worldPosition - In[0].worldPosition;
    const vec3 p2 = In[2].worldPosition - In[0].worldPosition;
    const vec3 p = abs(cross(p1, p2));
    for (uint i = 0; i < 3; i++) {
        Out.worldPosition = In[i].worldPosition;
        Out.normal = In[i].normal;
        if (p.z > p.x && p.z > p.y) {
            gl_Position = vec4(Out.worldPosition.x, Out.worldPosition.y, 0, 1);
        } else if (p.x > p.y && p.x > p.z) {
            gl_Position = vec4(Out.worldPosition.y, Out.worldPosition.z, 0, 1);
        } else {
            gl_Position = vec4(Out.worldPosition.x, Out.worldPosition.z, 0, 1);
        }
        EmitVertex();
    }
    EndPrimitive();
}

#shader fragment

#version 460 core

// Light
#define LIGHT_INTENSITY 1

// Attenuation
#define CONSTANT 1
#define LINEAR 0
#define QUADRATIC 1

struct PointLight {
    vec3 position;
    vec3 color;
};

uniform PointLight pointLight;

layout (binding = 0, rgba8) uniform image3D voxelsTexture;

in GeometryData {
    vec3 worldPosition;
    vec3 normal;
} In;

float attenuate(float distance) {
    return 1.f / (CONSTANT + LINEAR * distance + QUADRATIC * distance * distance);
}

// TODO: Also take into account directional lights or spot lights.
// TODO: Allow multiple lights.
vec3 calculatePointLightContribution(const PointLight light) {
    const vec3 direction = normalize(light.position - In.worldPosition);
    const float distanceToLight = distance(light.position, In.worldPosition);
    const float attenuation = attenuate(distanceToLight);
    const float diffuse = max(dot(normalize(In.normal), direction), 0.f);
    return diffuse * LIGHT_INTENSITY * attenuation * light.color;
}

vec3 scaleAndBias(vec3 p) {
    return 0.5f * p + vec3(0.5f);
}

void main() {
    vec3 color = vec3(0);
    color += calculatePointLightContribution(pointLight);
    // Output lighting to 3D texture.
    vec3 voxel = scaleAndBias(In.worldPosition);
    ivec3 dimension = imageSize(voxelsTexture);
    vec4 result = vec4(vec3(color), 1);
    imageStore(voxelsTexture, ivec3(dimension * voxel), result);
}
