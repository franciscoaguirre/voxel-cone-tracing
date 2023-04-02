#version 460 core

layout (points) in;
layout (line_strip, max_vertices = 256) out;

in vec4 geom_nodePosition[];
in float geom_halfNodeSize[];
in uint geom_nodeID[];
in ivec3 geom_brickCoordinates[];

out flat vec4 frag_nodeColor;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

uniform uint voxelDimension;
uniform uint bricksToShow;

uniform layout(binding = 3, rgba8) image3D brickPoolColors;

#include "assets/shaders/octree/_drawCube.glsl"

mat4 canonizationMatrix = projection * view * model;

void main() {
    vec4 nodePosition = geom_nodePosition[0];
    float voxelBrickSize = (geom_halfNodeSize[0] / 3) - 0.001;
    // So a brick goes fully inside a node, not accurate but works for debugging
    float brickDistance = geom_halfNodeSize[0] * 0.666;
    vec4 cubeCenter;
    vec4 cubeColor;
    if ((bricksToShow & 1) > 0) {
        // Show z = 0
        
        // (0, 0, 0)
        cubeCenter = vec4(nodePosition.xyz - brickDistance, nodePosition.w);
        cubeColor = vec4(imageLoad(brickPoolColors, geom_brickCoordinates[0]).xyz, 1.0);
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (0, 1, 0)
        cubeCenter = vec4(nodePosition.x - brickDistance, nodePosition.y, nodePosition.z - brickDistance, nodePosition.w);
        cubeColor = vec4(imageLoad(brickPoolColors, geom_brickCoordinates[0] + ivec3(0, 1, 0)).xyz, 1.0);
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (0, 2, 0)
        cubeCenter = vec4(nodePosition.x - brickDistance, nodePosition.y + brickDistance, nodePosition.z - brickDistance, nodePosition.w);
        cubeColor = vec4(imageLoad(brickPoolColors, geom_brickCoordinates[0] + ivec3(0, 2, 0)).xyz, 1.0);
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (2, 0, 0)
        cubeCenter = vec4(nodePosition.x + brickDistance, nodePosition.yz - brickDistance, nodePosition.w);
        cubeColor = vec4(imageLoad(brickPoolColors, geom_brickCoordinates[0] + ivec3(2, 0, 0)).xyz, 1.0);
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (2, 2, 0)
        cubeCenter = vec4(nodePosition.xy + brickDistance, nodePosition.z - brickDistance, nodePosition.w);
        cubeColor = vec4(imageLoad(brickPoolColors, geom_brickCoordinates[0] + ivec3(2, 2, 0)).xyz, 1.0);
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (1, 1, 0)
        cubeCenter = vec4(nodePosition.xy, nodePosition.z - brickDistance, nodePosition.w);
        cubeColor = vec4(imageLoad(brickPoolColors, geom_brickCoordinates[0] + ivec3(1, 1, 0)).xyz, 1.0);
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (1, 0, 0)
        cubeCenter = vec4(nodePosition.x, nodePosition.yz - brickDistance, nodePosition.w);
        cubeColor = vec4(imageLoad(brickPoolColors, geom_brickCoordinates[0] + ivec3(1, 0, 0)).xyz, 1.0);
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (1, 2, 0)
        cubeCenter = vec4(nodePosition.x, nodePosition.y + brickDistance, nodePosition.z - brickDistance, nodePosition.w);
        cubeColor = vec4(imageLoad(brickPoolColors, geom_brickCoordinates[0] + ivec3(1, 2, 0)).xyz, 1.0);
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (2, 1, 0)
        cubeCenter = vec4(nodePosition.x + brickDistance, nodePosition.y, nodePosition.z - brickDistance, nodePosition.w);
        cubeColor = vec4(imageLoad(brickPoolColors, geom_brickCoordinates[0] + ivec3(2, 1, 0)).xyz, 1.0);
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);
    }
    if ((bricksToShow & (1 << 1)) > 0) {
        // Show z = 1
        
        // (0, 1, 1)
        cubeCenter = vec4(nodePosition.x - brickDistance, nodePosition.yz, nodePosition.w);
        cubeColor = vec4(imageLoad(brickPoolColors, geom_brickCoordinates[0] + ivec3(0, 1, 1)).xyz, 1.0);
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (1, 0, 1)
        cubeCenter = vec4(nodePosition.x, nodePosition.y - brickDistance, nodePosition.z, nodePosition.w);
        cubeColor = vec4(imageLoad(brickPoolColors, geom_brickCoordinates[0] + ivec3(1, 0, 1)).xyz, 1.0);
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (0, 0, 1)
        cubeCenter = vec4(nodePosition.xy - brickDistance, nodePosition.z, nodePosition.w);
        cubeColor = vec4(imageLoad(brickPoolColors, geom_brickCoordinates[0] + ivec3(0, 0, 1)).xyz, 1.0);
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (1, 1, 1)
        cubeCenter = vec4(nodePosition.xyz, nodePosition.w);
        cubeColor = vec4(imageLoad(brickPoolColors, geom_brickCoordinates[0] + ivec3(1, 1, 1)).xyz, 1.0);
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (0, 2, 1)
        cubeCenter = vec4(nodePosition.x - brickDistance, nodePosition.y + brickDistance, nodePosition.z, nodePosition.w);
        cubeColor = vec4(imageLoad(brickPoolColors, geom_brickCoordinates[0] + ivec3(0, 2, 1)).xyz, 1.0);
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (1, 2, 1)
        cubeCenter = vec4(nodePosition.x, nodePosition.y + brickDistance, nodePosition.z, nodePosition.w);
        cubeColor = vec4(imageLoad(brickPoolColors, geom_brickCoordinates[0] + ivec3(1, 2, 1)).xyz, 1.0);
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (2, 1, 1)
        cubeCenter = vec4(nodePosition.x + brickDistance, nodePosition.yz, nodePosition.w);
        cubeColor = vec4(imageLoad(brickPoolColors, geom_brickCoordinates[0] + ivec3(2, 1, 1)).xyz, 1.0);
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (2, 0, 1)
        cubeCenter = vec4(nodePosition.x + brickDistance, nodePosition.y - brickDistance, nodePosition.z, nodePosition.w);
        cubeColor = vec4(imageLoad(brickPoolColors, geom_brickCoordinates[0] + ivec3(2, 0, 1)).xyz, 1.0);
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (2, 2, 1)
        cubeCenter = vec4(nodePosition.xy + brickDistance, nodePosition.z, nodePosition.w);
        cubeColor = vec4(imageLoad(brickPoolColors, geom_brickCoordinates[0] + ivec3(2, 2, 1)).xyz, 1.0);
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

    }
    if ((bricksToShow & (1 << 2)) > 0) {
        // Show z = 2

        // (0, 0, 2)
        cubeCenter = vec4(nodePosition.xy - brickDistance, nodePosition.z + brickDistance, nodePosition.w);
        cubeColor = vec4(imageLoad(brickPoolColors, geom_brickCoordinates[0] + ivec3(0, 0, 2)).xyz, 1.0);
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (1, 0, 2)
        cubeCenter = vec4(nodePosition.x, nodePosition.y - brickDistance, nodePosition.z + brickDistance, nodePosition.w);
        cubeColor = vec4(imageLoad(brickPoolColors, geom_brickCoordinates[0] + ivec3(1, 0, 2)).xyz, 1.0);
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (2, 0, 2)
        cubeCenter = vec4(nodePosition.x + brickDistance, nodePosition.y - brickDistance, nodePosition.z + brickDistance, nodePosition.w);
        cubeColor = vec4(imageLoad(brickPoolColors, geom_brickCoordinates[0] + ivec3(2, 0, 2)).xyz, 1.0);
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (2, 2, 2)
        cubeCenter = vec4(nodePosition.xyz + brickDistance, nodePosition.w);
        cubeColor = vec4(imageLoad(brickPoolColors, geom_brickCoordinates[0] + ivec3(2, 2, 2)).xyz, 1.0);
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (0, 2, 2)
        cubeCenter = vec4(nodePosition.x - brickDistance, nodePosition.yz + brickDistance, nodePosition.w);
        cubeColor = vec4(imageLoad(brickPoolColors, geom_brickCoordinates[0] + ivec3(0, 2, 2)).xyz, 1.0);
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (1, 1, 2)
        cubeCenter = vec4(nodePosition.xy, nodePosition.z + brickDistance, nodePosition.w);
        cubeColor = vec4(imageLoad(brickPoolColors, geom_brickCoordinates[0] + ivec3(1, 1, 2)).xyz, 1.0);
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (0, 1, 2)
        cubeCenter = vec4(nodePosition.x - brickDistance, nodePosition.y, nodePosition.z + brickDistance, nodePosition.w);
        cubeColor = vec4(imageLoad(brickPoolColors, geom_brickCoordinates[0] + ivec3(0, 1, 2)).xyz, 1.0);
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (1, 2, 2)
        cubeCenter = vec4(nodePosition.x, nodePosition.yz + brickDistance, nodePosition.w);
        cubeColor = vec4(imageLoad(brickPoolColors, geom_brickCoordinates[0] + ivec3(1, 2, 2)).xyz, 1.0);
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (2, 1, 2)
        cubeCenter = vec4(nodePosition.x + brickDistance, nodePosition.y, nodePosition.z + brickDistance, nodePosition.w);
        cubeColor = vec4(imageLoad(brickPoolColors, geom_brickCoordinates[0] + ivec3(2, 1, 2)).xyz, 1.0);
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);
    }
}
