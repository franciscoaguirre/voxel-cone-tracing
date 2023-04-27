#shader vertex

#version 460 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 normal;

out vec4 frag_position;
out vec3 frag_normal;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main() {
    gl_Position = projection * view * model * vec4(position, 1.0);
    frag_position = model * vec4(position, 1.0);
    frag_normal = normal;
}

#shader fragment

#version 460 core

layout (location = 0) out uvec3 viewMapPositions;
layout (location = 1) out vec4 viewMapViewOutput;
layout (location = 2) out vec3 viewMapNormals;

in vec4 frag_position;
in vec3 frag_normal;

uniform uint voxelDimension;

void main() {
    vec4 normalizedGlobalPosition = vec4(
        ((frag_position.xyz / frag_position.w) + vec3(1.0)) / 2.0,
        1.0
    );
    uvec3 unnormalizedGlobalPosition = uvec3(round(normalizedGlobalPosition.xyz * float(voxelDimension) * 1.5));
    
    viewMapViewOutput = vec4(frag_normal, 1.0);
    viewMapPositions = unnormalizedGlobalPosition;
    viewMapNormals = frag_normal;
}
