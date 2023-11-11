#version 460 core

layout (local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

uniform layout(binding = 0, r32ui) uimageBuffer maxColorBuffer;

uniform sampler2D inputTexture;

void main() {
    vec3 color = texelFetch(
        inputTexture,
        ivec2(gl_GlobalInvocationID.xy),
        0
    ).rgb;
    // We multiply by a big number just because atomic operations don't work
    // on floats
    uint colorNorm = uint(length(color) * 1000.0);

    imageAtomicMax(maxColorBuffer, 0, colorNorm);
}
