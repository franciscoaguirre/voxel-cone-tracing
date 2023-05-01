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

    float AO = ambientOcclusion(vec3(0.5, 0.5, 0.4), vec3(0, 0, 1), 0.261799, 1.0); // 15deg as rad
    
    FragColor = vec4(vec3(AO), 1);
}
