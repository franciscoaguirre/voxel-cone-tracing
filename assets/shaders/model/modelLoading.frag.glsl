#version 460 core

out vec4 FragColor;

in vec2 fragTexCoords;

uniform bool hasTexture;
uniform bool hasDiffuse;
uniform vec3 materialDiffuse;
uniform sampler2D texture_diffuse1;

void main()
{
    if (hasTexture) {
        FragColor = texture(texture_diffuse1, fragTexCoords);
    } else if (hasDiffuse) {
        FragColor = vec4(materialDiffuse, 1);
    }
}
