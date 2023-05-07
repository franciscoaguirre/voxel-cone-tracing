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
    gl_Position = projection * view * model * vec4(position, 1.0);
    vec4 lol = model * vec4(position, 1.0);
    frag_position = lol.xyz / lol.w;
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
//uniform layout(binding = 4, r32f) writeonly imageBuffer debug;

uniform uint voxelDimension;
uniform uint maxOctreeLevel;
uniform sampler2D texture_diffuse1;
uniform sampler3D brickPoolColors;

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
    // float coneAngle = 0.261799;
    float coneAngle = 0.000001;
    vec3 position = (frag_position + vec3(1.0)) / 2.0;
    // mat3 normalMatrix = mat3(transpose(inverse(view * model)));
    // vec3 normal = normalize(vec3(vec4(normalMatrix * frag_normal, 0)));
    // vec3 direction = normalize(frag_normal);
    vec3 direction = vec3(0, 0, 1);
    vec3 helper = direction - vec3(0.1, 0, 0); // Random vector
    vec3 tangent = normalize(helper - dot(direction, helper) * direction);
    vec3 bitangent = cross(direction, tangent);
    float AO = 0.0;
    AO += ambientOcclusion(position, direction, coneAngle, maxDistance); // 15deg as rad

    float angle = 1.0472;
    float sinAngle = sin(angle);
    float cosAngle = cos(angle);

    // direction = sinAngle * frag_normal + cosAngle * tangent;
    // AO += ambientOcclusion(position, direction, coneAngle, maxDistance);

    // direction = sinAngle * frag_normal - cosAngle * tangent;
    // AO += ambientOcclusion(position, direction, coneAngle, maxDistance);

    // direction = sinAngle * frag_normal + cosAngle * bitangent;
    // AO += ambientOcclusion(position, direction, coneAngle, maxDistance);

    // direction = sinAngle * frag_normal - cosAngle * bitangent;
    // AO += ambientOcclusion(position, direction, coneAngle, maxDistance);
    // //float AO = ambientOcclusion(vec3(0.5, 0.5, 0.46), vec3(0, 0, 1), 0.261799, maxDistance); // 15deg as rad

    // AO /= 5;

    // FragColor = vec4(texture(texture_diffuse1, frag_textureCoordinates).xyz - vec3(AO), 1);
     FragColor = vec4(vec3(AO), 1.0);
    //vec4 color = texture(texture_diffuse1, frag_textureCoordinates);
    //FragColor = vec4(color.rgb * AO, color.a);
}
