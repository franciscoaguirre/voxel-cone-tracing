#shader vertex

#version 460 core

layout (location = 0) in vec3 position;

uniform mat4 model;
uniform mat4 modelNormalizationMatrix;

void main() {
    gl_Position = modelNormalizationMatrix * model * vec4(position, 1.0);
}

#shader geometry

#version 460 core

layout (triangles) in;
layout (triangle_strip, max_vertices=18) out;

uniform mat4 shadowMatrices[6];

out GeometryData {
    vec4 position;
} Out;

void main() {
    for (int face = 0; face < 6; face++) {
        gl_Layer = face;
        for (int i = 0; i < 3; i++) {
            Out.position = gl_in[i].gl_Position;
            gl_Position = shadowMatrices[face] * Out.position;
            EmitVertex();
        }
        EndPrimitive();
    }
}

#shader fragment

#version 460 core

layout (location = 0) out uvec4 viewMapPositions;
layout (location = 1) out vec4 viewMapViewOutput;

in GeometryData {
    vec4 position;
} In;

uniform uint voxelDimension;
uniform vec3 lightPosition;
uniform float farPlane;

void main() {
    vec4 normalizedGlobalPosition = vec4(
        ((In.position.xyz / In.position.w) + vec3(1.0)) / 2.0,
        1.0
    );
    /// We need to multiply by 2 since the nodes are already divided into 2x2x2, but we need to address
    /// each quarter to get the correct voxel.
    uvec3 unnormalizedGlobalPosition = uvec3(floor(normalizedGlobalPosition.xyz * float(voxelDimension)));
    
    viewMapPositions = uvec4(unnormalizedGlobalPosition, 1.0);
    viewMapViewOutput = normalizedGlobalPosition;

    float lightDistance = length(In.position.xyz - lightPosition);
    lightDistance = lightDistance / farPlane;
    gl_FragDepth = lightDistance;
}
