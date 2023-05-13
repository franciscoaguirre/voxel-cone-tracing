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

// Brick attributes
uniform sampler3D brickPoolColors;
// uniform sampler3D brickPoolNormals; // TODO: Use later
uniform sampler3D brickPoolPhotons;

// G-buffers
uniform sampler2D gBufferColors;
uniform sampler2D gBufferPositions;
uniform sampler2D gBufferNormals;
// uniform sampler2D shadowMap; // TODO: Later

#include "./_constants.glsl"
#include "./_helpers.glsl"
#include "./_traversalHelpers.glsl"
#include "./_octreeTraversal.glsl"
#include "./_coneTrace.glsl"

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

    float maxDistance = 0.01;
    float coneAngle = 0.261799;
    // float coneAngle = 0.000001;
    vec3 position = texture(gBufferPositions, In.textureCoordinates).xyz * 0.5 + 0.5;
    vec3 normal = texture(gBufferNormals, In.textureCoordinates).xyz;
    // vec3 normal = vec3(0, 1, 0);
    vec3 helper = normal - vec3(0.1, 0, 0); // Random vector
    vec3 tangent = normalize(helper - dot(normal, helper) * normal);
    vec3 bitangent = cross(normal, tangent);

    vec4 color = texture(gBufferColors, In.textureCoordinates);

    if (color == vec4(0.0)) {
        discard;
    }

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

    // FragColor = vec4(texture(texture_diffuse1, frag_textureCoordinates).xyz - vec3(AO), 1);
    outColor = vec4(1.0 - indirectLight.aaa, 1.0);
    // outColor = texture(gBufferColors, In.textureCoordinates);
    // outColor = vec4(position, 1.0);
    // outColor = vec4(normal, 1.0);
    // outColor = vec4(color.aaa, 1.0);
    //vec4 color = texture(texture_diffuse1, frag_textureCoordinates);
    //FragColor = vec4(color.rgb * AO, color.a);
}
