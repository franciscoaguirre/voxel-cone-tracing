#version 460 core

out vec4 FragColor;

in vec2 fragTexCoords;
in vec3 fragNormal;

uniform bool hasTexture;
uniform sampler2D texture_diffuse1;

uniform bool hasDiffuse;
uniform vec3 materialDiffuse;

struct Material {
    vec3 color;
    float diffuse;
    float specular;
};
uniform Material material;

void main()
{
    if (hasTexture) {
        FragColor = texture(texture_diffuse1, fragTexCoords);
    } else if (hasDiffuse) {
        FragColor = vec4(materialDiffuse, 1);
    } else {
        FragColor = vec4(material.color, 1);
    }
}
