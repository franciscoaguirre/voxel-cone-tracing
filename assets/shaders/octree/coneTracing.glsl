#shader vertex

#version 460 core

layout (location = 0) in vec3 position;

out VertexData {
    vec2 textureCoordinates;
} Out;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main() {
    gl_Position = vec4(position, 1.0);
    Out.textureCoordinates = position.xy * 0.5 + 0.5;
}

#shader fragment

#version 460 core

layout (location = 0) out vec4 outColor;

in VertexData {
    vec2 textureCoordinates;
} In;

uniform layout(binding = 0, r32ui) readonly uimageBuffer nodePool;

// Scalar attributes
uniform uint voxelDimension;
uniform uint maxOctreeLevel;
uniform bool useLighting;
uniform vec3 lightDirection;
uniform float shininess;
uniform mat4 lightViewMatrix;
uniform mat4 lightProjectionMatrix;

// Brick attributes
uniform sampler3D brickPoolColors;
// uniform sampler3D brickPoolNormals; // TODO: Use later
uniform sampler3D brickPoolPhotons;

// G-buffers
uniform sampler2D gBufferColors;
uniform sampler2D gBufferPositions;
uniform sampler2D gBufferNormals;
uniform sampler2D shadowMap;

#include "./_constants.glsl"
#include "./_helpers.glsl"
#include "./_traversalHelpers.glsl"
#include "./_octreeTraversal.glsl"
#include "./_coneTrace.glsl"

vec4 gatherIndirectLight(vec3 position, vec3 normal, vec3 tangent);
float visibilityCalculation(vec4 positionInLightSpace, vec3 normal);

void main() {
    // vec4 indirectDiffuse = vec4(0, 0, 0, 1);
    // uint numberDiffuseCones = 5;
    // for (uint i = 0; i < numberDiffuseCones; i++)
    //     indirectDiffuse += coneTrace(...);
    // indirectDiffuse /= numberDiffuseCones;

    // vec4 specular = coneTrace(..., specular);

    // vec4 phongDiffuse = min(dot(...), 0);
    // vec4 directDiffuse = phongDiffuse * texture(texture_diffuse1, frag_textureCoordinates);

    // vec4 totalColor = (directDiffuse + indirectDiffuse + specular);
    // FragColor = totalColor * ambientOcclusion;

    // float coneAngle = 0.000001;

    vec3 positionRaw = texture(gBufferPositions, In.textureCoordinates).xyz;
    vec3 position = positionRaw * 0.5 + 0.5;

    vec3 normal = texture(gBufferNormals, In.textureCoordinates).xyz;
    // vec3 normal = vec3(0, 1, 0);
    vec3 helper = normal - vec3(0.1, 0, 0); // Random vector
    vec3 tangent = normalize(helper - dot(normal, helper) * normal);

    vec4 color = texture(gBufferColors, In.textureCoordinates);

    if (color == vec4(0.0)) {
        discard;
    }

    vec4 indirectLight = gatherIndirectLight(position, normal, tangent);

    vec4 positionInLightSpace = lightProjectionMatrix * lightViewMatrix * vec4(positionRaw, 1.0);
    float visibility = visibilityCalculation(positionInLightSpace, normal);

    float diffuse = max(0.0, dot(lightDirection, normal));
    // float h = normalize((lightDirection - view);
    // float specular = pow(max(0.0, dot(normal, h)), shininess);
    vec3 directLight = vec3(1) * diffuse;
    vec3 ambient = vec3(1) * 0.15;

    vec3 lightIntensity = ambient + visibility * (directLight); // TODO: Add indirectLight.rgb

    // FragColor = vec4(texture(texture_diffuse1, frag_textureCoordinates).xyz - vec3(AO), 1);
    outColor = vec4(1.0 - indirectLight.aaa, 1.0);
    // outColor = color * vec4(lightIntensity, 1.0);
    // outColor = texture(gBufferColors, In.textureCoordinates);
    // outColor = vec4(position, 1.0);
    // outColor = vec4(normal, 1.0);
    // outColor = vec4(color.aaa, 1.0);
    //vec4 color = texture(texture_diffuse1, frag_textureCoordinates);
    //FragColor = vec4(color.rgb * AO, color.a);
}

float visibilityCalculation(vec4 positionInLightSpace, vec3 normal) {
    vec3 projectedPosition = positionInLightSpace.xyz / positionInLightSpace.w;
    projectedPosition = projectedPosition * 0.5 + 0.5;
    float closestDepth = texture(shadowMap, projectedPosition.xy).r;
    float currentDepth = projectedPosition.z;
    float bias = max(0.01 * (1.0 - dot(normal, lightDirection)), 0.003);
    float shadow = 0.0;
    vec2 texelSize = 1.0 / textureSize(shadowMap, 0);
    for (int x = -2; x <= 2; x++) {
        for (int y = -2; y <= 2; y++) {
            float pcfDepth = texture(shadowMap, projectedPosition.xy + vec2(x, y) * texelSize).r;
            shadow += (currentDepth - bias) > pcfDepth ? 1.0 : 0.0;
        }
    }
    shadow /= 25.0;
    if (projectedPosition.z > 1.0) {
        shadow = 0.0;
    }
    return 1.0 - shadow;
}

vec4 gatherIndirectLight(vec3 position, vec3 normal, vec3 tangent) {
    float maxDistance = 0.01;
    float coneAngle = 0.261799;
    vec3 bitangent = cross(normal, tangent);
    vec3 direction = normal;
    vec4 indirectLight = vec4(0);
    indirectLight += coneTrace(position, direction, coneAngle, maxDistance); // 15deg as rad

    float angle = 1.0472;
    float sinAngle = sin(angle);
    float cosAngle = cos(angle);

    direction = sinAngle * normal + cosAngle * tangent;
    indirectLight += 0.707 * coneTrace(position, direction, coneAngle, maxDistance);

    direction = sinAngle * normal - cosAngle * tangent;
    indirectLight += 0.707 * coneTrace(position, direction, coneAngle, maxDistance);

    direction = sinAngle * normal + cosAngle * bitangent;
    indirectLight += 0.707 * coneTrace(position, direction, coneAngle, maxDistance);

    direction = sinAngle * normal - cosAngle * bitangent;
    indirectLight += 0.707 * coneTrace(position, direction, coneAngle, maxDistance);

    indirectLight /= 3.828;

    return indirectLight;
}
