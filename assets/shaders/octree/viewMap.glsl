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

uniform mat3 normalMatrix;
uniform mat4 modelNormalizationMatrix;
uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main() {
    gl_Position = projection * view * modelNormalizationMatrix * model * vec4(position, 1.0);
    // Position in world space
    Out.position = modelNormalizationMatrix * model * vec4(position, 1.0);
    Out.normal = normalize(normalMatrix * normal);
    Out.textureCoordinates = textureCoordinates;
}

#shader fragment

#version 460 core

layout (location = 0) out vec4 viewMapPositions;
layout (location = 1) out vec4 viewMapViewOutput;
layout (location = 2) out vec4 viewMapNormals;
layout (location = 3) out vec4 viewMapColors;
layout (location = 4) out vec4 viewMapSpecular;

in VertexData {
    vec4 position;
    vec3 normal;
    vec2 textureCoordinates;
} In;

struct Material {
    vec3 color;
    float diffuse;
    float specular;
};
uniform Material material;

// TODO: Bring back?
// uniform uint voxelDimension;
uniform bool hasTexture;
uniform bool hasSpecular;
uniform sampler2D texture_diffuse1;
uniform sampler2D texture_specular1;
uniform vec3 materialSpecular;

void main() {
    // We take world space position (-1, 1) and move it to voxel space (0, 1)
    vec4 normalizedGlobalPosition = vec4(
        ((In.position.xyz / In.position.w) + vec3(1.0)) / 2.0,
        1.0
    );
    //uvec3 unnormalizedGlobalPosition = uvec3(floor(normalizedGlobalPosition.xyz * float(voxelDimension) * 1.5));
    
    viewMapPositions = vec4(In.position.xyz / In.position.w, 1);
    // For some reason normals are not normalized.
    // Normalizing them here makes everything better.
    // Might need to look into the models we load.
    viewMapNormals = vec4(normalize(In.normal), 1);
    if (hasTexture) {
        viewMapColors = texture(texture_diffuse1, In.textureCoordinates);
    } else {
        viewMapColors = vec4(material.color, 1);
    }
    if (hasSpecular) {
        viewMapSpecular = texture(texture_specular1, In.textureCoordinates) * vec4(materialSpecular, 1.0);
    } else {
        viewMapSpecular = vec4(vec3(0.3), 1);
    }

    viewMapViewOutput = normalizedGlobalPosition;
}
