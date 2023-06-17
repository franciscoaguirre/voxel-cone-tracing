#version 460 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 normal;
layout (location = 2) in vec2 textureCoordinates;

out VertexData {
    vec3 position;
    vec3 normal;
    vec2 textureCoordinates;
} Out;

uniform mat4 modelNormalizationMatrix;

void main()
{
    Out.position = (modelNormalizationMatrix * vec4(position, 1.0)).xyz;
    Out.textureCoordinates = textureCoordinates;
    Out.normal = normal;
}
