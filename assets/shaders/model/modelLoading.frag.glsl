#version 460 core

out vec4 FragColor;

in vec2 fragTexCoords;
in vec3 fragNormal;

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
    FragColor = vec4((normalize(fragNormal) + vec3(1)) / 2.0, 1.0);
}
