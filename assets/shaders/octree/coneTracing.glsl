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

struct PointLight {
    vec3 position;
    vec3 color;
};

struct DirectionalLight {
    vec3 direction;
    vec3 color;
};

// Scalar attributes
uniform uint voxelDimension;
uniform uint maxOctreeLevel;
uniform DirectionalLight directionalLight;
uniform PointLight pointLight;
uniform bool isDirectional;
uniform float shininess;
uniform mat4 lightViewMatrix;
uniform mat4 lightProjectionMatrix;
uniform float halfConeAngle;
uniform float photonPower;
uniform bool showIndirectLight;
uniform vec3 eyePosition;

// Boolean toggles
uniform bool shouldShowColor;
uniform bool shouldShowDirect;
uniform bool shouldShowIndirect;
uniform bool shouldShowIndirectSpecular;
uniform bool shouldShowAmbientOcclusion;

// Brick attributes
uniform sampler3D brickPoolColorsX;
uniform sampler3D brickPoolColorsXNeg;
uniform sampler3D brickPoolColorsY;
uniform sampler3D brickPoolColorsYNeg;
uniform sampler3D brickPoolColorsZ;
uniform sampler3D brickPoolColorsZNeg;

uniform sampler3D brickPoolNormals;

// Irradiance
uniform sampler3D brickPoolIrradianceX;
uniform sampler3D brickPoolIrradianceXNeg;
uniform sampler3D brickPoolIrradianceY;
uniform sampler3D brickPoolIrradianceYNeg;
uniform sampler3D brickPoolIrradianceZ;
uniform sampler3D brickPoolIrradianceZNeg;

// G-buffers
uniform sampler2D gBufferColors;
uniform sampler2D gBufferPositions;
uniform sampler2D gBufferNormals;
uniform sampler2D gBufferSpeculars;

#include "./_constants.glsl"
#include "./_helpers.glsl"
#include "./_traversalHelpers.glsl"
#include "./_octreeTraversal.glsl"
#include "./_brickCoordinates.glsl"
#include "./_anisotropicColor.glsl"
#include "./_anisotropicIrradiance.glsl"
#include "./_coneTrace.glsl"

const float PI = 3.14159;

vec4 gatherIndirectLight(vec3 position, vec3 normal, vec3 tangent, bool useLighting);
vec4 gatherSpecularIndirectLight(vec3 position, vec3 eyeDirection, vec3 normal);
float traceShadowCone(vec3 origin, vec3 direction, float targetDistance);

void main() {
    // We should use `positionWorldSpace` when relating to other objects in the scene
    vec3 positionWorldSpace = texture(gBufferPositions, In.textureCoordinates).xyz;
    // We should use `positionVoxelSpace` when cone tracing
    vec3 positionVoxelSpace = (positionWorldSpace + vec3(1)) / 2.0;

    vec3 eyeDirection = normalize(positionWorldSpace - eyePosition);

    // TODO: Pretty sure this can't be negative right now
    vec3 normal = texture(gBufferNormals, In.textureCoordinates).xyz;
    vec3 helper = normal - vec3(0.1, 0.1, 0); // Random vector
    vec3 tangent = normalize(helper - dot(normal, helper) * normal);

    vec4 color = texture(gBufferColors, In.textureCoordinates);

    if (color == vec4(0.0)) {
        discard;
    }

    bool useLighting = false;
    float ambientOcclusion;
    if (shouldShowAmbientOcclusion) {
        ambientOcclusion = gatherIndirectLight(positionVoxelSpace, normal, tangent, useLighting).a;
    }

    useLighting = true;
    vec3 indirectLight = vec3(0);
    if (shouldShowIndirect) {
        // We should pre-multiply by alpha probably? Instead of just ignoring it
        indirectLight = gatherIndirectLight(positionVoxelSpace, normal, tangent, useLighting).rgb;
    }

    float specularFactor = texture(gBufferSpeculars, In.textureCoordinates).r;
    vec3 specularIndirectLight = vec3(0);
    if (shouldShowIndirectSpecular && specularFactor > 0.0) {
      // We should pre-multiply by alpha probably? Instead of just ignoring it
        specularIndirectLight = specularFactor * gatherSpecularIndirectLight(positionVoxelSpace, eyeDirection, normal).rgb;
    }

    // float h = normalize((lightDirection - view);
    // float specular = pow(max(0.0, dot(normal, h)), shininess);
    float visibility = 1.0;
    vec3 lightDirection = normalize(pointLight.position - positionWorldSpace);
    visibility = traceShadowCone(positionVoxelSpace, lightDirection, 1.0);
    float lightAngle = dot(normal, lightDirection);
    float diffuse = max(lightAngle, 0.0);
    vec3 directLight = vec3(diffuse);

    vec4 finalImage = vec4(0);

    bool shouldShowOnlyColor = (
        !shouldShowDirect &&
            !shouldShowIndirect &&
            !shouldShowAmbientOcclusion &&
            !shouldShowIndirectSpecular
    );

    if (shouldShowOnlyColor) {
        finalImage = color;
    }

    if (shouldShowDirect) {
        finalImage += visibility * vec4(directLight, 1.0);
    }
    if (shouldShowIndirect) {
        finalImage += vec4(indirectLight, 1.0);
    }
    if (shouldShowAmbientOcclusion) {
        finalImage += vec4(vec3(1.0 - ambientOcclusion), 1.0);
    }
    if (!shouldShowOnlyColor && shouldShowColor) {
        finalImage *= color;
    }
    if (shouldShowIndirectSpecular) {
        finalImage += vec4(specularIndirectLight, 1.0);
    }
    
    outColor = vec4(finalImage.xyz, 1.0);
}

float traceShadowCone(vec3 origin, vec3 direction, float targetDistance) {
    // TODO: Possibly add a little bit in the direction of the normal
    float occlusion = coneTrace(origin, direction, halfConeAngle, targetDistance).a;
    return 1 - occlusion;
}

vec4 gatherSpecularIndirectLight(vec3 position, vec3 eyeDirection, vec3 normal) {
    vec3 reflectDirection = normalize(reflect(eyeDirection, normalize(normal)));
    //vec3 reflectDirection = normalize(reflect(eyeDirection, vec3(0, 0, -1)));
    float halfConeAngle = 0.005;
    float maxDistance = 5;
    bool useLighting = true;

    return coneTrace(position, reflectDirection, halfConeAngle, maxDistance);
}

vec4 gatherIndirectLight(vec3 position, vec3 normal, vec3 tangent, bool useLighting) {
    float maxDistance = useLighting ? 1.0 : 0.01;
    vec3 bitangent = cross(normal, tangent);
    vec3 direction;
    vec4 indirectLight = vec4(0);

    float angleFromAxis = 1.0472;
    float sinAngle = sin(angleFromAxis);
    float cosAngle = cos(angleFromAxis);

    float coneWeight = (PI / 2) - angleFromAxis; // TODO: Shouldn't it be a cosine?

    direction = sinAngle * normal + cosAngle * tangent;
    
    indirectLight += coneWeight * coneTrace(position, direction, halfConeAngle, maxDistance);

    direction = sinAngle * normal - cosAngle * tangent;
    indirectLight += coneWeight * coneTrace(position, direction, halfConeAngle, maxDistance);

    direction = sinAngle * normal + cosAngle * bitangent;
    indirectLight += coneWeight * coneTrace(position, direction, halfConeAngle, maxDistance);

    direction = sinAngle * normal - cosAngle * bitangent;
    indirectLight += coneWeight * coneTrace(position, direction, halfConeAngle, maxDistance);

    indirectLight /= coneWeight * 4;

    return indirectLight;
}
