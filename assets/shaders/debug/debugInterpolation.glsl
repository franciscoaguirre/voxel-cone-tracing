#shader vertex

#version 460 core

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aTexCoords;

out vec2 TexCoords;

void main() {
    gl_Position = vec4((aPos.xy - vec2(1.0, -1.0)) * 2.0 + vec2(1.0, -1.0), aPos.z, 1.0);
    gl_Position.x *= -1;
    TexCoords = aTexCoords;
}

#shader fragment

#version 460 core

layout (location = 0) out vec4 nodeColor;
layout (location = 1) out vec4 neighborColor;

uniform layout (binding = 0, r32f) writeonly imageBuffer debug;

in vec2 TexCoords;

uniform sampler3D brickPoolColors;

uniform uint nodeID;
uniform bool isNeighbor;
uniform float brickPoolResolutionf;
uniform uint voxelDimension;

#include "assets/shaders/octree/_helpers.glsl"
#include "assets/shaders/octree/_brickCoordinates.glsl"

void main() {
    // FragColor = texture(brickPoolColors, TexCoords);

    vec3 brickCoordinates = calculateBrickCoordinates(int(nodeID)) / (brickPoolResolutionf - 1.0);
    if (isNeighbor) {
        neighborColor = vec4(1);
    } else {
        // O está desfasado, o el offset es muy grande
        // El valor del texel está arriba a la izquierda en vez de en el centro
        nodeColor = vec4(
            vec3(texture(
                brickPoolColors,
                brickCoordinates + (vec3(TexCoords, 0) * (2.0 / 3.0) + (1.0 / 6.0)) / (brickPoolResolutionf - 1.0)
            ).a),
            1
        );
        nodeColor.rgb = vec3(1) - nodeColor.rgb;
    }
}
