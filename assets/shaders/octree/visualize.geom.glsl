#version 460 core

layout (points) in;
layout (line_strip, max_vertices = 256) out;

in vec4 nodePosition[];
in float geom_halfNodeSize[];
in vec4 nodeColor[];
in ivec3 geom_brickCoordinates[];

out flat vec4 frag_nodeColor;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

uniform uint showBricks;

uniform layout(binding = 2, rgba8) image3D brickPoolColors;

#include "./_drawCube.glsl"

mat4 canonizationMatrix = projection * view * model;

void main() {
    frag_nodeColor = nodeColor[0];

    vec4 cubeCenter = nodePosition[0];

    float voxelBrickSize = (geom_halfNodeSize[0] / 3) - 0.001;

    // So a brick goes fully inside a node, not accurate but works for debugging
    float brick_distance = geom_halfNodeSize[0] * 0.666;
    vec4 cubeColor;

    if (showBricks == 1) {
        // Show z = 0
        
        // (0, 0, 0)
        cubeCenter = vec4(nodePosition[0].xyz - brick_distance, nodePosition[0].w);
        cubeColor = imageLoad(brickPoolColors, geom_brickCoordinates[0]);
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (0, 1, 0)
        cubeCenter = vec4(nodePosition[0].x - brick_distance, nodePosition[0].y, nodePosition[0].z - brick_distance, nodePosition[0].w);
        cubeColor = imageLoad(brickPoolColors, geom_brickCoordinates[0] + ivec3(0, 1, 0));
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (0, 2, 0)
        cubeCenter = vec4(nodePosition[0].x - brick_distance, nodePosition[0].y + brick_distance, nodePosition[0].z - brick_distance, nodePosition[0].w);
        cubeColor = imageLoad(brickPoolColors, geom_brickCoordinates[0] + ivec3(0, 2, 0));
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (2, 0, 0)
        cubeCenter = vec4(nodePosition[0].x + brick_distance, nodePosition[0].yz - brick_distance, nodePosition[0].w);
        cubeColor = imageLoad(brickPoolColors, geom_brickCoordinates[0] + ivec3(2, 0, 0));
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (2, 2, 0)
        cubeCenter = vec4(nodePosition[0].xy + brick_distance, nodePosition[0].z - brick_distance, nodePosition[0].w);
        cubeColor = imageLoad(brickPoolColors, geom_brickCoordinates[0] + ivec3(2, 2, 0));
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (1, 1, 0)
        cubeCenter = vec4(nodePosition[0].xy, nodePosition[0].z - brick_distance, nodePosition[0].w);
        cubeColor = imageLoad(brickPoolColors, geom_brickCoordinates[0] + ivec3(1, 1, 0));
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (1, 0, 0)
        cubeCenter = vec4(nodePosition[0].x, nodePosition[0].yz - brick_distance, nodePosition[0].w);
        cubeColor = imageLoad(brickPoolColors, geom_brickCoordinates[0] + ivec3(1, 0, 0));
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (1, 2, 0)
        cubeCenter = vec4(nodePosition[0].x, nodePosition[0].y + brick_distance, nodePosition[0].z - brick_distance, nodePosition[0].w);
        cubeColor = imageLoad(brickPoolColors, geom_brickCoordinates[0] + ivec3(1, 2, 0));
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (2, 1, 0)
        cubeCenter = vec4(nodePosition[0].x + brick_distance, nodePosition[0].y, nodePosition[0].z - brick_distance, nodePosition[0].w);
        cubeColor = imageLoad(brickPoolColors, geom_brickCoordinates[0] + ivec3(2, 1, 0));
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);
    } else if (showBricks == 2) {
        // Show z = 1
        
        // (0, 1, 1)
        cubeCenter = vec4(nodePosition[0].x - brick_distance, nodePosition[0].yz, nodePosition[0].w);
        cubeColor = imageLoad(brickPoolColors, geom_brickCoordinates[0] + ivec3(0, 1, 1));
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (1, 0, 1)
        cubeCenter = vec4(nodePosition[0].x, nodePosition[0].y - brick_distance, nodePosition[0].z, nodePosition[0].w);
        cubeColor = imageLoad(brickPoolColors, geom_brickCoordinates[0] + ivec3(1, 0, 1));
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (0, 0, 1)
        cubeCenter = vec4(nodePosition[0].xy - brick_distance, nodePosition[0].z, nodePosition[0].w);
        cubeColor = imageLoad(brickPoolColors, geom_brickCoordinates[0] + ivec3(0, 0, 1));
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (1, 1, 1)
        cubeCenter = vec4(nodePosition[0].xyz, nodePosition[0].w);
        cubeColor = imageLoad(brickPoolColors, geom_brickCoordinates[0] + ivec3(1, 1, 1));
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (0, 2, 1)
        cubeCenter = vec4(nodePosition[0].x - brick_distance, nodePosition[0].y + brick_distance, nodePosition[0].z, nodePosition[0].w);
        cubeColor = imageLoad(brickPoolColors, geom_brickCoordinates[0] + ivec3(0, 2, 1));
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (1, 2, 1)
        cubeCenter = vec4(nodePosition[0].x, nodePosition[0].y + brick_distance, nodePosition[0].z, nodePosition[0].w);
        cubeColor = imageLoad(brickPoolColors, geom_brickCoordinates[0] + ivec3(1, 2, 1));
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (2, 1, 1)
        cubeCenter = vec4(nodePosition[0].x + brick_distance, nodePosition[0].yz, nodePosition[0].w);
        cubeColor = imageLoad(brickPoolColors, geom_brickCoordinates[0] + ivec3(2, 1, 1));
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (2, 0, 1)
        cubeCenter = vec4(nodePosition[0].x + brick_distance, nodePosition[0].y - brick_distance, nodePosition[0].z, nodePosition[0].w);
        cubeColor = imageLoad(brickPoolColors, geom_brickCoordinates[0] + ivec3(2, 0, 1));
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (2, 2, 1)
        cubeCenter = vec4(nodePosition[0].xy + brick_distance, nodePosition[0].z, nodePosition[0].w);
        cubeColor = imageLoad(brickPoolColors, geom_brickCoordinates[0] + ivec3(2, 2, 1));
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

    } else if (showBricks == 3) {
        // Show z = 2

        // (0, 0, 2)
        cubeCenter = vec4(nodePosition[0].xy - brick_distance, nodePosition[0].z + brick_distance, nodePosition[0].w);
        cubeColor = imageLoad(brickPoolColors, geom_brickCoordinates[0] + ivec3(0, 0, 2));
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (1, 0, 2)
        cubeCenter = vec4(nodePosition[0].x, nodePosition[0].y - brick_distance, nodePosition[0].z + brick_distance, nodePosition[0].w);
        cubeColor = imageLoad(brickPoolColors, geom_brickCoordinates[0] + ivec3(1, 0, 2));
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (2, 0, 2)
        cubeCenter = vec4(nodePosition[0].x + brick_distance, nodePosition[0].y - brick_distance, nodePosition[0].z + brick_distance, nodePosition[0].w);
        cubeColor = imageLoad(brickPoolColors, geom_brickCoordinates[0] + ivec3(2, 0, 2));
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (2, 2, 2)
        cubeCenter = vec4(nodePosition[0].xyz + brick_distance, nodePosition[0].w);
        cubeColor = imageLoad(brickPoolColors, geom_brickCoordinates[0] + ivec3(2, 2, 2));
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (0, 2, 2)
        cubeCenter = vec4(nodePosition[0].x - brick_distance, nodePosition[0].yz + brick_distance, nodePosition[0].w);
        cubeColor = imageLoad(brickPoolColors, geom_brickCoordinates[0] + ivec3(0, 2, 2));
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (1, 1, 2)
        cubeCenter = vec4(nodePosition[0].xy, nodePosition[0].z + brick_distance, nodePosition[0].w);
        cubeColor = imageLoad(brickPoolColors, geom_brickCoordinates[0] + ivec3(1, 1, 2));
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (0, 1, 2)
        cubeCenter = vec4(nodePosition[0].x - brick_distance, nodePosition[0].y, nodePosition[0].z + brick_distance, nodePosition[0].w);
        cubeColor = imageLoad(brickPoolColors, geom_brickCoordinates[0] + ivec3(0, 1, 2));
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (1, 2, 2)
        cubeCenter = vec4(nodePosition[0].x, nodePosition[0].yz + brick_distance, nodePosition[0].w);
        cubeColor = imageLoad(brickPoolColors, geom_brickCoordinates[0] + ivec3(1, 2, 2));
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (2, 1, 2)
        cubeCenter = vec4(nodePosition[0].x + brick_distance, nodePosition[0].y, nodePosition[0].z + brick_distance, nodePosition[0].w);
        cubeColor = imageLoad(brickPoolColors, geom_brickCoordinates[0] + ivec3(2, 1, 2));
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);
    }
}
