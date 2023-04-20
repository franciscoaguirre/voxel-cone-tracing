#version 460 core

layout (location = 0) in vec3 inVertexPosition;
layout (location = 1) in vec3 inNormal;
layout (location = 2) in vec2 inTexCoordinates;

out vec3 geom_vertexPosition;
out vec3 geom_normal;
out vec2 geom_texCoordinates;

uniform mat4 modelNormalizationMatrix;

void main()
{
    geom_normal = inNormal;
    geom_texCoordinates = inTexCoordinates;
    gl_Position = modelNormalizationMatrix * vec4(inVertexPosition, 1.0);
    geom_vertexPosition = gl_Position.xyz / gl_Position.w;
}
