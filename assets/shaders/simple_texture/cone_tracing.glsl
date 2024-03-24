#shader vertex

#version 460 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 normal;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

out VertexData {
    vec3 worldPosition;
    vec3 normal;
} Out;

void main() {
    Out.worldPosition = vec3(model * vec4(position, 1));
    Out.normal = normalize(mat3(transpose(inverse(model))) * normal);
    gl_Position = projection * view * vec4(Out.worldPosition, 1);
}

#shader fragment

#version 460 core

// Light
#define DIRECT_LIGHT_INTENSITY 1

// Attenuation
#define CONSTANT 1
#define LINEAR 0
#define QUADRATIC 1

// Voxels
#define VOXEL_SIZE (1/64)

struct PointLight {
    vec3 position;
    vec3 color;
};
uniform PointLight pointLight;

struct Material {
    vec3 color;
};
uniform Material material;

uniform sampler3D voxelsTexture;

in VertexData {
    vec3 worldPosition;
    vec3 normal;
} In;

out vec4 color;

vec3 normal = normalize(In.normal);

float attenuate(float distance) {
    return 1.f / (CONSTANT + LINEAR * distance + QUADRATIC * distance * distance);
}

// Returns true if the point p is inside the unity cube. 
bool isInsideCube(const vec3 p, float e) { return abs(p.x) < 1 + e && abs(p.y) < 1 + e && abs(p.z) < 1 + e; }

// Scales and bias a given vector (i.e. from [-1, 1] to [0, 1]).
vec3 scaleAndBias(const vec3 p) {
    return 0.5f * p + vec3(0.5f);
}

float traceShadowCone(vec3 from, vec3 direction, float targetDistance) {
    from += normal * 0.05f; // Removes artifacts.
    float accumulator = 0;
    float distance = 3 * VOXEL_SIZE;
    const float STOP = targetDistance - 1 * VOXEL_SIZE;
    int steps = 0;
    while (distance < STOP && accumulator < 1 && steps < 500) {
        vec3 current = from + distance * direction;
        if (!isInsideCube(current, 0)) {
            break;
        }
        current = scaleAndBias(current);
        // float lod = pow(distance, 2); // Inverse square falloff for shadows.
        // float sample1 = textureLod(voxelsTexture, current, 1 + 0.75 * lod).a;
        // float sample2 = textureLod(voxelsTexture, current, 4.5 * lod).a;
        // float interpolatedSample = 0.062 * sample1 + 0.135 * sample2;
        float interpolatedSample = texture(voxelsTexture, current).a;
        accumulator += (1 - accumulator) * interpolatedSample;
        distance += 0.9 * VOXEL_SIZE;
        steps += 1;
    }
    return 1 - accumulator;
}

vec3 calculateDirectLight(const PointLight light) {
    vec3 lightDirection = light.position - In.worldPosition;
    const float distanceToLight = length(lightDirection);
    lightDirection = lightDirection / distanceToLight;
    const float lightAngle = dot(normal, lightDirection);
    float diffuseAngle = max(lightAngle, 0.0f);
    float shadowBlend = 1;
    if (diffuseAngle > 0) {
        shadowBlend = traceShadowCone(In.worldPosition, lightDirection, distanceToLight);
    }
    diffuseAngle = min(shadowBlend, diffuseAngle);
    const vec3 total = light.color * diffuseAngle;
    return attenuate(distanceToLight) * total;
}

vec3 directLight() {
    vec3 direct = vec3(0.0f);
    direct += calculateDirectLight(pointLight); // TODO: Handle more lights
    direct *= DIRECT_LIGHT_INTENSITY;
    return direct;
}

void main() {
    color = vec4(0, 0, 0, 1);

    // Direct light.
    color.rgb += directLight();

    color.rgb = pow(color.rgb, vec3(1.0 / 2.2));
}
