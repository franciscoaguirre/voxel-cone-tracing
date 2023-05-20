#shader vertex

#version 460 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 normal;
layout (location = 2) in vec2 textureCoordinates;

out VertexData {
    vec4 position;
    vec3 normal;
    vec2 textureCoordinates;
} Out;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main() {
    gl_Position = projection * view * model * vec4(position, 1.0);
    Out.position = model * vec4(position, 1.0);
    Out.normal = normal;
    Out.textureCoordinates = textureCoordinates;
}

#shader fragment

#version 460 core

layout (location = 0) out vec4 viewMapPositions;
layout (location = 1) out vec4 viewMapViewOutput;
layout (location = 2) out vec4 viewMapNormals;
layout (location = 3) out vec4 viewMapColors;

in VertexData {
    vec4 position;
    vec3 normal;
    vec2 textureCoordinates;
} In;

uniform uint voxelDimension;
uniform sampler2D texture_diffuse1;

void main() {
    vec4 normalizedGlobalPosition = vec4(
        ((In.position.xyz / In.position.w) + vec3(1.0)) / 2.0,
        1.0
    );
    uvec3 unnormalizedGlobalPosition = uvec3(floor(normalizedGlobalPosition.xyz * float(voxelDimension)));
    
    viewMapPositions = vec4(In.position.xyz / In.position.w, 1);
    viewMapNormals = vec4(In.normal, 1);
    viewMapColors = texture(texture_diffuse1, In.textureCoordinates);

    viewMapViewOutput = normalizedGlobalPosition;
}
