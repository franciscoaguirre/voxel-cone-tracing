#version 460 core

layout (points) in;
layout (line_strip, max_vertices = 256) out;

in vec4 geom_nodePosition[];
in float geom_halfNodeSize[];
in uint geom_nodeID[];

out vec4 frag_nodeColor;

#include "assets/shaders/octree/_drawCube.glsl"

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

uniform uint voxelDimension;

const uint MAX_NEIGHBORS = 6;

uniform layout(binding = 0, rgb10_a2ui) readonly uimageBuffer nodePositions;
uniform layout(binding = 3, r32ui) readonly uimageBuffer nodePoolNeighbors[MAX_NEIGHBORS];

mat4 canonizationMatrix = projection * view * model;

void main() {
    vec4 nodeCenter = geom_nodePosition[0];
    vec4 nodeColor = vec4(1.0, 0.0, 0.0, 1.0);
    vec4 neighborColor = vec4(0.0, 1.0, 0.0, 1.0);
    drawCube(nodeCenter, geom_halfNodeSize[0], canonizationMatrix, nodeColor);

    for (uint i = 0; i < MAX_NEIGHBORS; i++) {
        uint neighborID = imageLoad(nodePoolNeighbors[i], int(geom_nodeID[0])).r;

        if (neighborID != 0) {
            vec4 neighborPosition = imageLoad(nodePositions, int(neighborID));
            vec3 normalizedNeighborPosition = neighborPosition.xyz / float(voxelDimension);
            vec4 finalNeighborPosition = vec4((normalizedNeighborPosition.xyz) * 2.0 - vec3(1.0), 1.0);
            finalNeighborPosition.xyz += geom_halfNodeSize[0];
            drawCube(finalNeighborPosition, geom_halfNodeSize[0], canonizationMatrix, neighborColor);
        }
    }
}
