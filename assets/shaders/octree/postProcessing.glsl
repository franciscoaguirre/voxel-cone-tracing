//! Shader that does postprocessing on the final image
//! Effects:
//! - HDR
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
uniform float exposure;

in VertexData {
    vec2 textureCoordinates;
} In;

void main() {
    vec3 hdrColor = texture(inputTexture, In.textureCoordinates).rgb;

    // Exposure tone mapping
    vec3 mapped = vec3(1.0) - exp(-hdrColor * exposure);

    outColor = vec4(mapped, 1.0);
}
