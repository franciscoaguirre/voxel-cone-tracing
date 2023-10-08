#shader vertex

#version 460 core

layout (location = 0) in vec3 position;

out VertexData {
    vec4 position;
} Out;

uniform mat4 model;
uniform mat4 modelNormalizationMatrix;
uniform mat4 view;
uniform mat4 projection;

void main() {
    gl_Position = projection * view * modelNormalizationMatrix * model * vec4(position, 1.0);
    Out.position = model * vec4(position, 1.0);
}

#shader fragment

#version 460 core

layout (location = 0) out uvec4 viewMapPositions;
layout (location = 1) out vec4 viewMapViewOutput;

in VertexData {
    vec4 position;
} In;

uniform uint voxelDimension;

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
}
