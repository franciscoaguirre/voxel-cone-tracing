#shader vertex

#version 460 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 normal;
layout (location = 2) in vec2 textureCoordinates;

out vec3 frag_position;
out vec3 frag_normal;
out vec2 frag_textureCoordinates;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main() {
    gl_Position = projection * view * model * vec4(position, 1);
    frag_position = (model * vec4(position, 1)).xyz;
    frag_normal = normal;
    frag_textureCoordinates = textureCoordinates;
}

#shader fragment

#version 460 core

layout (location = 0) out vec4 FragColor;

in vec3 frag_position;
in vec3 frag_normal;
in vec2 frag_textureCoordinates;

uniform layout(binding = 0, r32ui) readonly uimageBuffer nodePool;
uniform layout(binding = 1, rgba8) readonly image3D brickPoolColors;
uniform layout(binding = 2, r32f) writeonly imageBuffer debug;

uniform uint voxelDimension;
uniform uint maxOctreeLevel;
uniform sampler2D texture_diffuse1;

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

    vec3 position = (frag_position + vec3(1)) / float(2);
    // mat3 normalMatrix = mat3(transpose(inverse(view * model)));
    // vec3 normal = normalize(vec3(vec4(normalMatrix * frag_normal, 0)));
    vec3 direction = frag_normal;
    vec3 helper = direction - vec3(0.1, 0, 0); // Random vector
    vec3 tangent = normalize(helper - dot(direction, helper) * direction);
    vec3 bitangent = cross(direction, tangent);
    float AO;
    AO += ambientOcclusion(position, direction, 0.261799, 1.0); // 15deg as rad

    float angle = 1.0472;
    float sinAngle = sin(angle);
    float cosAngle = cos(angle);

    direction = sinAngle * frag_normal + cosAngle * tangent;
    AO += ambientOcclusion(position, direction, 0.261799, 1.0);

    direction = sinAngle * frag_normal - cosAngle * tangent;
    AO += ambientOcclusion(position, direction, 0.261799, 1.0);

    direction = sinAngle * frag_normal + cosAngle * bitangent;
    AO += ambientOcclusion(position, direction, 0.261799, 1.0);

    direction = sinAngle * frag_normal - cosAngle * bitangent;
    AO += ambientOcclusion(position, direction, 0.261799, 1.0);
    // float AO = ambientOcclusion(vec3(0.5, 0.5, 0.46), vec3(0, 0, 1), 0.261799, 1.0); // 15deg as rad

    AO /= 5;

    // FragColor = vec4(texture(texture_diffuse1, frag_textureCoordinates).xyz - vec3(AO), 1);
    // FragColor = vec4(vec3(1) - vec3(AO), 1);
    FragColor = texture(texture_diffuse1, frag_textureCoordinates);
}
