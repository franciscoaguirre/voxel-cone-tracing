#version 460 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 normal;
layout (location = 2) in vec2 textureCoordinates;

out VertexData {
    vec3 position;
    vec3 normal;
    vec2 textureCoordinates;
} Out;

// This is the individual model matrix
uniform mat4 model;
// This is the scene aabb normalization matrix to fit in our voxelization box
uniform mat4 modelNormalizationMatrix;
// This is the normal matrix to fix normals
uniform mat3 normalMatrix;

void main()
{
    Out.position = (modelNormalizationMatrix * model * vec4(position, 1.0)).xyz;
    Out.textureCoordinates = textureCoordinates;
    Out.normal = normalize(normalMatrix * normal);
}
