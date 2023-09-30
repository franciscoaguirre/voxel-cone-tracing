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
uniform vec3 lightDirection;
uniform float shininess;
uniform mat4 lightViewMatrix;
uniform mat4 lightProjectionMatrix;
uniform float coneAngle;
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
uniform sampler2D shadowMap;

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
float visibilityCalculation(vec4 positionInLightSpace, vec3 normal);

void main() {
    vec3 positionRaw = texture(gBufferPositions, In.textureCoordinates).xyz;
    vec3 position = positionRaw * 0.5 + 0.5;

    vec3 eyeDirection = normalize(positionRaw - eyePosition);

    vec3 normal = texture(gBufferNormals, In.textureCoordinates).xyz;
    // If normal is paralel to the random vector we got trouble in our hands
    vec3 helper = normal - vec3(0.1, 0.1, 0); // Random vector
    vec3 tangent = normalize(helper - dot(normal, helper) * normal);

    vec4 color = texture(gBufferColors, In.textureCoordinates);

    if (color == vec4(0.0)) {
        discard;
    }

    bool useLighting = false;
    float ambientOcclusion;
    if (shouldShowAmbientOcclusion) {
        ambientOcclusion = gatherIndirectLight(position, normal, tangent, useLighting).a;
    }

    useLighting = true;

    vec3 indirectLight = vec3(0);
    if (shouldShowIndirect) {
      // We should pre-multiply by alpha probably? Instead of just ignoring it
        indirectLight = gatherIndirectLight(position, normal, tangent, useLighting).rgb;
    }

    vec3 specularIndirectLight = vec3(0);
    if (shouldShowIndirectSpecular) {
      // We should pre-multiply by alpha probably? Instead of just ignoring it
        specularIndirectLight = gatherSpecularIndirectLight(position, eyeDirection, normal).rgb;
    }

    vec4 positionInLightSpace = lightProjectionMatrix * lightViewMatrix * vec4(positionRaw, 1.0);
    float visibility = visibilityCalculation(positionInLightSpace, normal);

    float diffuse = max(0.0, dot(lightDirection, normal));
    // float h = normalize((lightDirection - view);
    // float specular = pow(max(0.0, dot(normal, h)), shininess);
    vec3 directLight = vec3(diffuse);

    vec4 finalImage = vec4(0);

    if (shouldShowDirect) {
        finalImage += vec4(visibility * directLight, 1.0);
    }
    if (shouldShowIndirect) {
        finalImage += vec4(indirectLight, 1.0);
    }
    if (shouldShowAmbientOcclusion) {
        finalImage += vec4(vec3(1.0 - ambientOcclusion), 1.0);
    }
    if (shouldShowColor) {
        finalImage *= color;
    }
    // if (shouldShowIndirectSpecular) {
    //     finalImage += vec4(specularIndirectLight, 1.0);
    // }
    
    outColor = vec4(finalImage.xyz, 1.0);
}

float SampleShadowMap(sampler2D shadowMap, vec2 coords, float currentDepth, float bias)
{
    float pcfDepth = texture(shadowMap, coords).r;
    float shadow = (currentDepth - bias) > pcfDepth ? 1.0 : 0.0;
    return shadow;
}

float SampleShadowMapLinear(sampler2D shadowMap, vec2 coords, float currentDepth, vec2 texelSize, float bias)
{
	vec2 pixelPos = coords/texelSize + vec2(0.5);
	vec2 fracPart = fract(pixelPos);
	vec2 startTexel = (pixelPos - fracPart) * texelSize;

	float blTexel = SampleShadowMap(shadowMap, startTexel, currentDepth, bias);
	float brTexel = SampleShadowMap(shadowMap, startTexel + vec2(texelSize.x, 0.0), currentDepth, bias);
	float tlTexel = SampleShadowMap(shadowMap, startTexel + vec2(0.0, texelSize.y), currentDepth, bias);
	float trTexel = SampleShadowMap(shadowMap, startTexel + texelSize, currentDepth, bias);

	float mixA = mix(blTexel, tlTexel, fracPart.y);
	float mixB = mix(brTexel, trTexel, fracPart.y);

	return mix(mixA, mixB, fracPart.x);
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
            float partialShadow = SampleShadowMapLinear(shadowMap, projectedPosition.xy + vec2(x, y) * texelSize, currentDepth, texelSize, bias);
            shadow += partialShadow;
        }
    }
    shadow /= 25.0;
    if (projectedPosition.z > 1.0) {
        shadow = 0.0;
    }
    return 1.0 - shadow;
}

vec4 gatherSpecularIndirectLight(vec3 position, vec3 eyeDirection, vec3 normal) {
    vec3 reflectDirection = normalize(reflect(eyeDirection, normalize(normal)));
    //vec3 reflectDirection = normalize(reflect(eyeDirection, vec3(0, 0, -1)));
    float coneAngle = 0.005;
    float maxDistance = 5;
    bool useLighting = true;

    return coneTrace(position, reflectDirection, coneAngle, maxDistance, useLighting);
}

vec4 gatherIndirectLight(vec3 position, vec3 normal, vec3 tangent, bool useLighting) {
    float maxDistance = useLighting ? 1.0 : 0.01;
    // float coneAngle = 0.261799;
    //float coneAngle = 0.0001;
    vec3 bitangent = cross(normal, tangent);
    vec3 direction;
    vec4 indirectLight = vec4(0);

    float angleFromAxis = 1.0472;
    float sinAngle = sin(angleFromAxis);
    float cosAngle = cos(angleFromAxis);

    float coneWeight = (PI / 2) - angleFromAxis; // TODO: Shouldn't it be a cosine?

    direction = normal;
    
    indirectLight += coneTrace(position, normalize(direction), coneAngle, maxDistance, useLighting);

    //direction = sinAngle * normal + cosAngle * tangent;
    
    //indirectLight += coneWeight * coneTrace(position, direction, coneAngle, maxDistance, useLighting);

    //direction = sinAngle * normal - cosAngle * tangent;
    //indirectLight += coneWeight * coneTrace(position, direction, coneAngle, maxDistance, useLighting);

    //direction = sinAngle * normal + cosAngle * bitangent;
    //indirectLight += coneWeight * coneTrace(position, direction, coneAngle, maxDistance, useLighting);

    //direction = sinAngle * normal - cosAngle * bitangent;
    //indirectLight += coneWeight * coneTrace(position, direction, coneAngle, maxDistance, useLighting);

    //indirectLight /= coneWeight * 4 + 1;

    return indirectLight;
}
