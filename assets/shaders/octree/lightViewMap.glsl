#shader vertex

#version 460 core

layout (location = 0) in vec3 position;

out VertexData {
    vec4 position;
} Out;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main() {
    gl_Position = projection * view * model * vec4(position, 1.0);
    Out.position = model * vec4(position, 1.0);
}

#shader fragment

#version 460 core

layout (location = 0) out uvec3 viewMapPositions;
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
    uvec3 unnormalizedGlobalPosition = uvec3(round(normalizedGlobalPosition.xyz * float(voxelDimension) * 1.5));
    
    viewMapPositions = uvec3(unnormalizedGlobalPosition);
    viewMapViewOutput = normalizedGlobalPosition;
}
