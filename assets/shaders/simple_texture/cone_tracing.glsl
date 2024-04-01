#shader vertex

#version 460 core

layout (location = 0) in vec3 position;

out VertexData {
    vec2 textureCoordinates;
} Out;

void main() {
    Out.textureCoordinates = position.xy * 0.5 + 0.5;
    gl_Position = vec4(position, 1);
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
#define VOXEL_SIZE (1/256)

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

uniform sampler2D gBufferColors;
uniform sampler2D gBufferPositions;
uniform sampler2D gBufferNormals;
uniform sampler2D gBufferSpeculars;

in VertexData {
    vec2 textureCoordinates;
} In;

out vec4 color;

// Scales and bias a given vector (i.e. from [-1, 1] to [0, 1]).
vec3 scaleAndBias(const vec3 p) {
    return 0.5f * p + 0.5f;
}

float traceShadowCone(vec3 from, vec3 direction, float targetDistance) {
    float accumulator = 0;
    float distance = 0 * VOXEL_SIZE;
    const float STOP = targetDistance - 1 * VOXEL_SIZE;
    int steps = 0;
    while (distance < STOP && accumulator < 1 && steps < 500) {
        vec3 current = from + distance * direction;
        // if (!isInsideCube(current, 0)) {
        //     break;
        // }
        current = scaleAndBias(current);
        // float lod = pow(distance, 2); // Inverse square falloff for shadows.
        // float sample1 = textureLod(voxelsTexture, current, 1 + 0.75 * lod).a;
        // float sample2 = textureLod(voxelsTexture, current, 4.5 * lod).a;
        // float interpolatedSample = 0.062 * sample1 + 0.135 * sample2;
        float interpolatedSample = textureLod(voxelsTexture, current, 0).a;
        accumulator += (1 - accumulator) * interpolatedSample;
        distance += 0.9 * VOXEL_SIZE;
        steps += 1;
    }
    return 1 - accumulator;
}

vec3 calculateDirectLight(const PointLight light) {
    vec3 worldPosition = texture(gBufferPositions, In.textureCoordinates).xyz;
    vec3 normal = texture(gBufferNormals, In.textureCoordinates).xyz;
    vec3 lightDirection = light.position - worldPosition;
    const float distanceToLight = length(lightDirection);
    lightDirection = lightDirection / distanceToLight;
    const float lightAngle = dot(normal, lightDirection);
    float diffuseAngle = max(lightAngle, 0.0f);
    float shadowBlend = 1;
    if (diffuseAngle > 0) {
        shadowBlend = traceShadowCone(worldPosition + normal * 0.05f, lightDirection, distanceToLight);
    }
    diffuseAngle = min(shadowBlend, diffuseAngle);
    const vec3 total = light.color * diffuseAngle;
    return total;
}

vec3 directLight() {
    vec3 direct = vec3(0.0f);
    direct += calculateDirectLight(pointLight); // TODO: Handle more lights
    // direct *= DIRECT_LIGHT_INTENSITY;
    return direct;
}

void main() {
    color = vec4(0, 0, 0, 1);

    // Direct light.
    color.rgb += directLight();

    color.rgb = pow(color.rgb, vec3(1.0 / 2.2));
}
