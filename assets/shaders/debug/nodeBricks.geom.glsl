#version 460 core

layout (points) in;
layout (line_strip, max_vertices = 256) out;

in vec4 geom_nodePosition[];
in float geom_halfNodeSize[];
in ivec3 geom_brickCoordinates[];

out flat vec4 frag_nodeColor;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

uniform uint bricksToShow;

uniform layout(binding = 2, rgba8) image3D brickPoolColors;
uniform layout(binding = 3, r32ui) uimage3D brickPoolPhotons;
uniform layout(binding = 4, rgba8) image3D brickPoolNormals;

#include "assets/shaders/octree/_drawCube.glsl"
#include "assets/shaders/octree/_drawNormal.glsl"

mat4 canonizationMatrix = projection * view * model;

const int BY_PHOTONS = 0;
const int BY_COLOR = 1;

vec4 showProp(ivec3 coordinates, int type) {
    if (type == BY_PHOTONS) {
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
    int type = BY_COLOR;
    vec4 nodePosition = geom_nodePosition[0];
    float voxelBrickSize = (geom_halfNodeSize[0] / 3) - 0.001;
    // So a brick goes fully inside a node, not accurate but works for debugging
    float brickDistance = geom_halfNodeSize[0] * 0.666;
    vec4 cubeCenter, cubeColor;
    vec3 start, normal;
    if (bricksToShow == 1) {
        // Show z = 0
        
        // (0, 0, 0)
        cubeCenter = vec4(nodePosition.xyz - brickDistance, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0], type);
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);
        start = cubeCenter.xyz;
        normal = imageLoad(brickPoolNormals, geom_brickCoordinates[0]).xyz;
        drawNormal(start, normal);

        // (0, 1, 0)
        cubeCenter = vec4(nodePosition.x - brickDistance, nodePosition.y, nodePosition.z - brickDistance, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(0, 1, 0), type);
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);
        start = cubeCenter.xyz;
        normal = imageLoad(brickPoolNormals, geom_brickCoordinates[0] + ivec3(0, 1, 0)).xyz;
        drawNormal(start, normal);

        // (0, 2, 0)
        cubeCenter = vec4(nodePosition.x - brickDistance, nodePosition.y + brickDistance, nodePosition.z - brickDistance, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(0, 2, 0), type);
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);
        start = cubeCenter.xyz;
        normal = imageLoad(brickPoolNormals, geom_brickCoordinates[0] + ivec3(0, 2, 0)).xyz;
        drawNormal(start, normal);

        // (2, 0, 0)
        cubeCenter = vec4(nodePosition.x + brickDistance, nodePosition.yz - brickDistance, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(2, 0, 0), type);
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);
        start = cubeCenter.xyz;
        normal = imageLoad(brickPoolNormals, geom_brickCoordinates[0] + ivec3(2, 0, 0)).xyz;
        drawNormal(start, normal);

        // (2, 2, 0)
        cubeCenter = vec4(nodePosition.xy + brickDistance, nodePosition.z - brickDistance, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(2, 2, 0), type);
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);
        start = cubeCenter.xyz;
        normal = imageLoad(brickPoolNormals, geom_brickCoordinates[0] + ivec3(2, 2, 0)).xyz;
        drawNormal(start, normal);

        // (1, 1, 0)
        cubeCenter = vec4(nodePosition.xy, nodePosition.z - brickDistance, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(1, 1, 0), type);
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);
        start = cubeCenter.xyz;
        normal = imageLoad(brickPoolNormals, geom_brickCoordinates[0] + ivec3(1, 1, 0)).xyz;
        drawNormal(start, normal);

        // (1, 0, 0)
        cubeCenter = vec4(nodePosition.x, nodePosition.yz - brickDistance, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(1, 0, 0), type);
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);
        start = cubeCenter.xyz;
        normal = imageLoad(brickPoolNormals, geom_brickCoordinates[0] + ivec3(1, 0, 0)).xyz;
        drawNormal(start, normal);

        // (1, 2, 0)
        cubeCenter = vec4(nodePosition.x, nodePosition.y + brickDistance, nodePosition.z - brickDistance, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(1, 2, 0), type);
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);
        start = cubeCenter.xyz;
        normal = imageLoad(brickPoolNormals, geom_brickCoordinates[0] + ivec3(1, 2, 0)).xyz;
        drawNormal(start, normal);

        // (2, 1, 0)
        cubeCenter = vec4(nodePosition.x + brickDistance, nodePosition.y, nodePosition.z - brickDistance, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(2, 1, 0), type);
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);
        start = cubeCenter.xyz;
        normal = imageLoad(brickPoolNormals, geom_brickCoordinates[0] + ivec3(2, 1, 0)).xyz;
        drawNormal(start, normal);
    } else if (bricksToShow == 2) {
        // Show z = 1
        
        // (0, 1, 1)
        cubeCenter = vec4(nodePosition.x - brickDistance, nodePosition.yz, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(0, 1, 1), type);
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (1, 0, 1)
        cubeCenter = vec4(nodePosition.x, nodePosition.y - brickDistance, nodePosition.z, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(1, 0, 1), type);
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (0, 0, 1)
        cubeCenter = vec4(nodePosition.xy - brickDistance, nodePosition.z, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(0, 0, 1), type);
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (1, 1, 1)
        cubeCenter = vec4(nodePosition.xyz, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(1, 1, 1), type);
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (0, 2, 1)
        cubeCenter = vec4(nodePosition.x - brickDistance, nodePosition.y + brickDistance, nodePosition.z, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(0, 2, 1), type);
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (1, 2, 1)
        cubeCenter = vec4(nodePosition.x, nodePosition.y + brickDistance, nodePosition.z, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(1, 2, 1), type);
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (2, 1, 1)
        cubeCenter = vec4(nodePosition.x + brickDistance, nodePosition.yz, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(2, 1, 1), type);
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (2, 0, 1)
        cubeCenter = vec4(nodePosition.x + brickDistance, nodePosition.y - brickDistance, nodePosition.z, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(2, 0, 1), type);
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (2, 2, 1)
        cubeCenter = vec4(nodePosition.xy + brickDistance, nodePosition.z, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(2, 2, 1), type);
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

    } else if (bricksToShow == 4) {
        // Show z = 2

        // (0, 0, 2)
        cubeCenter = vec4(nodePosition.xy - brickDistance, nodePosition.z + brickDistance, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(0, 0, 2), type);
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (1, 0, 2)
        cubeCenter = vec4(nodePosition.x, nodePosition.y - brickDistance, nodePosition.z + brickDistance, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(1, 0, 2), type);
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (2, 0, 2)
        cubeCenter = vec4(nodePosition.x + brickDistance, nodePosition.y - brickDistance, nodePosition.z + brickDistance, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(2, 0, 2), type);
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (2, 2, 2)
        cubeCenter = vec4(nodePosition.xyz + brickDistance, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(2, 2, 2), type);
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (0, 2, 2)
        cubeCenter = vec4(nodePosition.x - brickDistance, nodePosition.yz + brickDistance, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(0, 2, 2), type);
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (1, 1, 2)
        cubeCenter = vec4(nodePosition.xy, nodePosition.z + brickDistance, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(1, 1, 2), type);
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (0, 1, 2)
        cubeCenter = vec4(nodePosition.x - brickDistance, nodePosition.y, nodePosition.z + brickDistance, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(0, 1, 2), type);
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (1, 2, 2)
        cubeCenter = vec4(nodePosition.x, nodePosition.yz + brickDistance, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(1, 2, 2), type);
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);

        // (2, 1, 2)
        cubeCenter = vec4(nodePosition.x + brickDistance, nodePosition.y, nodePosition.z + brickDistance, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(2, 1, 2), type);
        drawCube(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor);
    }
}
