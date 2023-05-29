#version 460 core

layout (points) in;
layout (line_strip, max_vertices = 128) out;

in float geom_halfNodeSize[];
in uint geom_nodeID[];

out vec4 frag_nodeColor;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

uniform uint voxelDimension;

#include "assets/shaders/octree/_constants.glsl"
#include "assets/shaders/octree/_helpers.glsl"

uniform layout(binding = 0, rgb10_a2ui) readonly uimageBuffer nodePositions;
uniform layout(binding = 3, r32ui) readonly uimageBuffer nodePoolNeighbors[3];

#include "assets/shaders/octree/_drawCube.glsl"

mat4 canonizationMatrix = projection * view * model;

void main() {
    vec4 neighborColor = vec4(0.0, 1.0, 0.0, 1.0);

    for (uint i = 0; i < 3; i++) {
        uint neighborID = imageLoad(nodePoolNeighbors[i], int(geom_nodeID[0])).r;

        if (neighborID != 0) {
            uvec3 neighborPosition = imageLoad(nodePositions, int(neighborID)).xyz;
            vec3 normalizedNeighborPosition = vec3(neighborPosition) / float(voxelDimension);
            vec4 finalNeighborPosition = vec4((normalizedNeighborPosition.xyz) * 2.0 - vec3(1.0), 1.0);
            finalNeighborPosition.xyz += geom_halfNodeSize[0];
            drawCube(finalNeighborPosition, geom_halfNodeSize[0], canonizationMatrix, neighborColor);
        }
    }
}
