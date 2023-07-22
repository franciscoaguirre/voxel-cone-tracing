#version 460 core

layout (points) in;
layout (triangle_strip, max_vertices = 256) out;

in vec4 geom_nodePosition[];
in float geom_halfNodeSize[];
in ivec3 geom_brickCoordinates[];

out flat vec4 frag_nodeColor;
out vec3 frag_normal;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

uniform uint bricksToShow;
uniform uint mode;

uniform layout(binding = 0, rgba8) image3D brickPoolColors;
uniform layout(binding = 2, r32ui) uimage3D brickPoolPhotons;

mat4 canonizationMatrix = projection * view * model;

#include "assets/shaders/octree/_drawCubeFilled.glsl"

const uint BY_NONE = 0;
const uint BY_COLOR = 1;
const uint BY_PHOTONS = 2;

vec4 showProp(ivec3 coordinates, uint type) {
    if (type == BY_NONE) {
        return vec4(0); // Will be discarded
    } else if (type == BY_PHOTONS) {
      uint photonCount = imageLoad(brickPoolPhotons, coordinates).r;
      if (photonCount > 0) {
          return vec4(1.0, 1.0, 1.0, 1.0);
      }
      return vec4(0.0, 0.0, 0.0, 1.0);
    } else if (type == BY_COLOR) {
      return imageLoad(brickPoolColors, coordinates);
    }
}

void main() {
    vec4 nodePosition = geom_nodePosition[0];
    float voxelBrickSize = (geom_halfNodeSize[0] / 3) * 0.97;
    // So a brick goes fully inside a node, not accurate but works for debugging
    float brickDistance = geom_halfNodeSize[0] * 0.666;
    vec4 cubeCenter, cubeColor;
    vec3 start, normal;
    if (bricksToShow == 1) {
        // Show z = 0
        
        // (0, 0, 0)
        cubeCenter = vec4(nodePosition.xyz - brickDistance, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0], mode);
        drawCubeFilled(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (0, 1, 0)
        cubeCenter = vec4(nodePosition.x - brickDistance, nodePosition.y, nodePosition.z - brickDistance, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(0, 1, 0), mode);
        drawCubeFilled(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (0, 2, 0)
        cubeCenter = vec4(nodePosition.x - brickDistance, nodePosition.y + brickDistance, nodePosition.z - brickDistance, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(0, 2, 0), mode);
        drawCubeFilled(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (2, 0, 0)
        cubeCenter = vec4(nodePosition.x + brickDistance, nodePosition.yz - brickDistance, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(2, 0, 0), mode);
        drawCubeFilled(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (2, 2, 0)
        cubeCenter = vec4(nodePosition.xy + brickDistance, nodePosition.z - brickDistance, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(2, 2, 0), mode);
        drawCubeFilled(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (1, 1, 0)
        cubeCenter = vec4(nodePosition.xy, nodePosition.z - brickDistance, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(1, 1, 0), mode);
        drawCubeFilled(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (1, 0, 0)
        cubeCenter = vec4(nodePosition.x, nodePosition.yz - brickDistance, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(1, 0, 0), mode);
        drawCubeFilled(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (1, 2, 0)
        cubeCenter = vec4(nodePosition.x, nodePosition.y + brickDistance, nodePosition.z - brickDistance, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(1, 2, 0), mode);
        drawCubeFilled(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (2, 1, 0)
        cubeCenter = vec4(nodePosition.x + brickDistance, nodePosition.y, nodePosition.z - brickDistance, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(2, 1, 0), mode);
        drawCubeFilled(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);
    } else if (bricksToShow == 2) {
        // Show z = 1
        
        // (0, 1, 1)
        cubeCenter = vec4(nodePosition.x - brickDistance, nodePosition.yz, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(0, 1, 1), mode);
        drawCubeFilled(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (1, 0, 1)
        cubeCenter = vec4(nodePosition.x, nodePosition.y - brickDistance, nodePosition.z, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(1, 0, 1), mode);
        drawCubeFilled(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (0, 0, 1)
        cubeCenter = vec4(nodePosition.xy - brickDistance, nodePosition.z, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(0, 0, 1), mode);
        drawCubeFilled(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (1, 1, 1)
        cubeCenter = vec4(nodePosition.xyz, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(1, 1, 1), mode);
        drawCubeFilled(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (0, 2, 1)
        cubeCenter = vec4(nodePosition.x - brickDistance, nodePosition.y + brickDistance, nodePosition.z, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(0, 2, 1), mode);
        drawCubeFilled(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (1, 2, 1)
        cubeCenter = vec4(nodePosition.x, nodePosition.y + brickDistance, nodePosition.z, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(1, 2, 1), mode);
        drawCubeFilled(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (2, 1, 1)
        cubeCenter = vec4(nodePosition.x + brickDistance, nodePosition.yz, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(2, 1, 1), mode);
        drawCubeFilled(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (2, 0, 1)
        cubeCenter = vec4(nodePosition.x + brickDistance, nodePosition.y - brickDistance, nodePosition.z, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(2, 0, 1), mode);
        drawCubeFilled(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (2, 2, 1)
        cubeCenter = vec4(nodePosition.xy + brickDistance, nodePosition.z, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(2, 2, 1), mode);
        drawCubeFilled(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

    } else if (bricksToShow == 4) {
        // Show z = 2

        // (0, 0, 2)
        cubeCenter = vec4(nodePosition.xy - brickDistance, nodePosition.z + brickDistance, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(0, 0, 2), mode);
        drawCubeFilled(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (1, 0, 2)
        cubeCenter = vec4(nodePosition.x, nodePosition.y - brickDistance, nodePosition.z + brickDistance, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(1, 0, 2), mode);
        drawCubeFilled(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (2, 0, 2)
        cubeCenter = vec4(nodePosition.x + brickDistance, nodePosition.y - brickDistance, nodePosition.z + brickDistance, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(2, 0, 2), mode);
        drawCubeFilled(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (2, 2, 2)
        cubeCenter = vec4(nodePosition.xyz + brickDistance, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(2, 2, 2), mode);
        drawCubeFilled(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (0, 2, 2)
        cubeCenter = vec4(nodePosition.x - brickDistance, nodePosition.yz + brickDistance, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(0, 2, 2), mode);
        drawCubeFilled(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (1, 1, 2)
        cubeCenter = vec4(nodePosition.xy, nodePosition.z + brickDistance, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(1, 1, 2), mode);
        drawCubeFilled(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (0, 1, 2)
        cubeCenter = vec4(nodePosition.x - brickDistance, nodePosition.y, nodePosition.z + brickDistance, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(0, 1, 2), mode);
        drawCubeFilled(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (1, 2, 2)
        cubeCenter = vec4(nodePosition.x, nodePosition.yz + brickDistance, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(1, 2, 2), mode);
        drawCubeFilled(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (2, 1, 2)
        cubeCenter = vec4(nodePosition.x + brickDistance, nodePosition.y, nodePosition.z + brickDistance, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(2, 1, 2), mode);
        drawCubeFilled(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);
    }
}
