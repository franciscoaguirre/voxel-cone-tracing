#shader vertex

#version 460 core

layout (location = 0) in vec3 position;

uniform mat4 projection;
uniform mat4 view;
uniform mat4 model;

mat4 canonizationMatrix = projection * view * model;

void main() {
    gl_Position = canonizationMatrix * vec4(position, 1.0);
}

#shader geometry

#version 460 core

layout (points) in;

// +1, Eye ray
// +8, Cone
// +2, Tangent and bitangent
// --
// 11 lines -> 22 vertices
layout (line_strip, max_vertices = 22) out;

out vec4 frag_color;

uniform usampler2D eyeViewMap;
uniform sampler2D eyeViewMapNormals;
uniform uint voxelDimension;

uniform mat4 projection;
uniform mat4 view;

#include "./_drawCone.glsl"

void main() {
    frag_color = vec4(1);
    gl_Position = gl_in[0].gl_Position;
    EmitVertex();

    ivec2 pixelCoordinates = ivec2(384, 384);

    uvec3 queryCoordinates = texelFetch(
        eyeViewMap,
        pixelCoordinates,
        0
    ).xyz - uvec3(0, 0, 5);
    vec3 normal = texelFetch(
        eyeViewMapNormals,
        pixelCoordinates,
        0
    ).xyz;

    if (queryCoordinates == uvec3(0)) {
        return;
    }

    vec3 normalizedQueryCoordinates = vec3(queryCoordinates / (float(voxelDimension) * 1.5));
    vec3 ndc = normalizedQueryCoordinates.xyz * 2.0 - 1.0;

    gl_Position = projection * view * vec4(ndc, 1.0);
    EmitVertex();
    EndPrimitive();

    frag_color = vec4(0, 0, 1, 1); // Tangent is blue
    gl_Position = projection * view * vec4(ndc, 1.0);
    EmitVertex();
    vec3 helper = vec3(0.12, 0.32, 0.82); // Random values
    // vec3 tangent = cross(vec3(0, 1, 0), normal);
    vec3 tangent = normalize(helper - dot(normal, helper) * normal);
    gl_Position = projection * view * vec4(ndc.xyz + tangent * 0.5, 1.0);
    EmitVertex();
    EndPrimitive();

    frag_color = vec4(0, 1, 0, 1); // Bitangent is green
    gl_Position = projection * view * vec4(ndc, 1.0);
    EmitVertex();
    vec3 bitangent = cross(normal, tangent);
    gl_Position = projection * view * vec4(ndc + bitangent * 0.5, 1.0);
    EmitVertex();
    EndPrimitive();

    drawCone(ndc, normal, 0.523599);
}

#shader fragment

#version 460 core

layout (location = 0) out vec4 FragColor;

in vec4 frag_color;

void main() {
    FragColor = frag_color;
}
