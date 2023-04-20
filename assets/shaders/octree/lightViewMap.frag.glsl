#version 460 core

layout (location = 0) out uvec3 lightViewMapOutput;
layout (location = 1) out vec4 lightViewMapViewOutput;

in vec4 frag_position;

uniform uint voxelDimension;

void main() {
    vec4 normalizedGlobalPosition = vec4(
        ((frag_position.xyz / frag_position.w) + vec3(1.0)) / 2.0,
        1.0
    );
    uvec3 unnormalizedGlobalPosition = uvec3(floor(normalizedGlobalPosition.xyz * float(1023)));
    
    lightViewMapViewOutput = normalizedGlobalPosition;
    lightViewMapOutput = unnormalizedGlobalPosition;
}
