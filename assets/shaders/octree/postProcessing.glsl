//! Shader that does postprocessing on the final image
//! Effects:
//! - Normalization
//! - Gamma correction

#shader vertex

#version 460 core

layout (location = 0) in vec3 position;

out VertexData {
    vec2 textureCoordinates;
} Out;

uniform sampler2D inputTexture;

void main() {
    gl_Position = vec4(position, 1.0);
    Out.textureCoordinates = position.xy * 0.5 + 0.5;
}

#shader fragment

#version 460

layout (location = 0) out vec4 outColor;

uniform sampler2D inputTexture;
uniform float maxNorm;

in VertexData {
    vec2 textureCoordinates;
} In;

void main() {
    vec4 color = texture(inputTexture, In.textureCoordinates);

    if (maxNorm > 0.0) {
        color.rgb /= maxNorm;
    }

    // TODO: Makes it look worse for some reason
    // float gamma = 2.2;
    // color.rgb = pow(color.rgb, vec3(1.0 / gamma));

    outColor = color;
}
