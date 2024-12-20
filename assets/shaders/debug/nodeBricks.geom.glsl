#version 460 core

layout (points) in;
layout (triangle_strip, max_vertices = 128) out;

uniform layout(binding = 2, rgba8) image3D brickPoolColors;
uniform layout(binding = 3, r32ui) uimage3D brickPoolPhotons;
uniform layout(binding = 4, rgba8) image3D brickPoolNormals;

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

uniform float brickPadding = 0.0;
uniform vec3 colorDirection = vec3(1.0, 0.0, 0.0);

uniform sampler3D brickPoolColorsX;
uniform sampler3D brickPoolColorsXNeg;
uniform sampler3D brickPoolColorsY;
uniform sampler3D brickPoolColorsYNeg;
uniform sampler3D brickPoolColorsZ;
uniform sampler3D brickPoolColorsZNeg;

uniform sampler3D brickPoolIrradianceX;
uniform sampler3D brickPoolIrradianceXNeg;
uniform sampler3D brickPoolIrradianceY;
uniform sampler3D brickPoolIrradianceYNeg;
uniform sampler3D brickPoolIrradianceZ;
uniform sampler3D brickPoolIrradianceZNeg;

mat4 canonizationMatrix = projection * view * model;

#include "assets/shaders/octree/_drawCubeFilled.glsl"
#include "assets/shaders/octree/_anisotropicColor.glsl"
#include "assets/shaders/octree/_anisotropicIrradiance.glsl"
#include "assets/shaders/octree/_helpers.glsl"
#include "assets/shaders/octree/_drawNormal.glsl"

const uint BY_NONE = 0;
const uint BY_COLOR = 1;
const uint BY_PHOTONS = 2;

// TODO: Use the **exact** same as in `visualizeBricks.glsl`
vec4 showProp(ivec3 coordinates, uint type) {
    if (type == BY_NONE) {
      return vec4(0); // Will be discarded
    } else if (type == BY_PHOTONS) {
      vec3 normalizedCoordinates = normalizedFromIntCoordinates(coordinates, 384.0);
      return getAnisotropicIrradiance(normalizedCoordinates, colorDirection);
    } else if (type == BY_COLOR) {
      vec3 normalizedCoordinates = normalizedFromIntCoordinates(coordinates, 384.0);
      return getAnisotropicColor(normalizedCoordinates, colorDirection);
    }
}

void main() {
    vec4 nodePosition = geom_nodePosition[0];
    float voxelBrickSize = (geom_halfNodeSize[0] / 2);
    // So a brick goes fully inside a node, not accurate but works for debugging
    float brickDistance = geom_halfNodeSize[0];
    vec3 minCorner = nodePosition.xyz - vec3(geom_halfNodeSize[0]);
    vec3 maxCorner = nodePosition.xyz + vec3(geom_halfNodeSize[0]);
    vec4 cubeCenter, cubeColor;
    vec3 start, normal;
    if (bricksToShow == 0) {
        // Show z = 0
        
        // (0, 0, 0)
        cubeCenter = vec4(nodePosition.xyz - brickDistance, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0], mode);
        drawCubeFilled(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor, minCorner, maxCorner);
        start = cubeCenter.xyz;
        normal = imageLoad(brickPoolNormals, geom_brickCoordinates[0]).xyz;
        //drawNormal(start, normal);

        // (0, 1, 0)
        cubeCenter = vec4(nodePosition.x - brickDistance, nodePosition.y, nodePosition.z - brickDistance, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(0, 1, 0), mode);
        drawCubeFilled(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor, minCorner, maxCorner);
        start = cubeCenter.xyz;
        normal = imageLoad(brickPoolNormals, geom_brickCoordinates[0] + ivec3(0, 1, 0)).xyz;
        //drawNormal(start, normal);

        // (0, 2, 0)
        cubeCenter = vec4(nodePosition.x - brickDistance, nodePosition.y + brickDistance, nodePosition.z - brickDistance, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(0, 2, 0), mode);
        drawCubeFilled(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor, minCorner, maxCorner);
        start = cubeCenter.xyz;
        normal = imageLoad(brickPoolNormals, geom_brickCoordinates[0] + ivec3(0, 2, 0)).xyz;
        //drawNormal(start, normal);
    } else if (bricksToShow == 1) {

        // (2, 0, 0)
        cubeCenter = vec4(nodePosition.x + brickDistance, nodePosition.yz - brickDistance, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(2, 0, 0), mode);
        drawCubeFilled(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor, minCorner, maxCorner);
        start = cubeCenter.xyz;
        normal = imageLoad(brickPoolNormals, geom_brickCoordinates[0] + ivec3(2, 0, 0)).xyz;
        //drawNormal(start, normal);

        // (2, 2, 0)
        cubeCenter = vec4(nodePosition.xy + brickDistance, nodePosition.z - brickDistance, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(2, 2, 0), mode);
        drawCubeFilled(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor, minCorner, maxCorner);
        start = cubeCenter.xyz;
        normal = imageLoad(brickPoolNormals, geom_brickCoordinates[0] + ivec3(2, 2, 0)).xyz;
        //drawNormal(start, normal);

        // (1, 1, 0)
        cubeCenter = vec4(nodePosition.xy, nodePosition.z - brickDistance, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(1, 1, 0), mode);
        drawCubeFilled(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor, minCorner, maxCorner);
        start = cubeCenter.xyz;
        normal = imageLoad(brickPoolNormals, geom_brickCoordinates[0] + ivec3(1, 1, 0)).xyz;
        //drawNormal(start, normal);
    } else if (bricksToShow == 2) {

        // (1, 0, 0)
        cubeCenter = vec4(nodePosition.x, nodePosition.yz - brickDistance, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(1, 0, 0), mode);
        drawCubeFilled(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor, minCorner, maxCorner);
        start = cubeCenter.xyz;
        normal = imageLoad(brickPoolNormals, geom_brickCoordinates[0] + ivec3(1, 0, 0)).xyz;
        //drawNormal(start, normal);

        // (1, 2, 0)
        cubeCenter = vec4(nodePosition.x, nodePosition.y + brickDistance, nodePosition.z - brickDistance, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(1, 2, 0), mode);
        drawCubeFilled(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor, minCorner, maxCorner);
        start = cubeCenter.xyz;
        normal = imageLoad(brickPoolNormals, geom_brickCoordinates[0] + ivec3(1, 2, 0)).xyz;
        //drawNormal(start, normal);

        // (2, 1, 0)
        cubeCenter = vec4(nodePosition.x + brickDistance, nodePosition.y, nodePosition.z - brickDistance, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(2, 1, 0), mode);
        drawCubeFilled(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor, minCorner, maxCorner);
        start = cubeCenter.xyz;
        normal = imageLoad(brickPoolNormals, geom_brickCoordinates[0] + ivec3(2, 1, 0)).xyz;
        //drawNormal(start, normal);
    } else if (bricksToShow == 3) {
        // Show z = 1
        
        // (0, 1, 1)
        cubeCenter = vec4(nodePosition.x - brickDistance, nodePosition.yz, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(0, 1, 1), mode);
        drawCubeFilled(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor, minCorner, maxCorner);

        // (1, 0, 1)
        cubeCenter = vec4(nodePosition.x, nodePosition.y - brickDistance, nodePosition.z, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(1, 0, 1), mode);
        drawCubeFilled(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor, minCorner, maxCorner);

        // (0, 0, 1)
        cubeCenter = vec4(nodePosition.xy - brickDistance, nodePosition.z, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(0, 0, 1), mode);
        drawCubeFilled(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor, minCorner, maxCorner);

    } else if (bricksToShow == 4) {
        // (1, 1, 1)
        cubeCenter = vec4(nodePosition.xyz, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(1, 1, 1), mode);
        drawCubeFilled(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor, minCorner, maxCorner);

        // (0, 2, 1)
        cubeCenter = vec4(nodePosition.x - brickDistance, nodePosition.y + brickDistance, nodePosition.z, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(0, 2, 1), mode);
        drawCubeFilled(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor, minCorner, maxCorner);

        // (1, 2, 1)
        cubeCenter = vec4(nodePosition.x, nodePosition.y + brickDistance, nodePosition.z, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(1, 2, 1), mode);
        drawCubeFilled(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor, minCorner, maxCorner);

    } else if (bricksToShow == 5) {
        // (2, 1, 1)
        cubeCenter = vec4(nodePosition.x + brickDistance, nodePosition.yz, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(2, 1, 1), mode);
        drawCubeFilled(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor, minCorner, maxCorner);

        // (2, 0, 1)
        cubeCenter = vec4(nodePosition.x + brickDistance, nodePosition.y - brickDistance, nodePosition.z, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(2, 0, 1), mode);
        drawCubeFilled(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor, minCorner, maxCorner);

        // (2, 2, 1)
        cubeCenter = vec4(nodePosition.xy + brickDistance, nodePosition.z, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(2, 2, 1), mode);
        drawCubeFilled(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor, minCorner, maxCorner);

    } else if (bricksToShow == 6) {
        // Show z = 2

        // (0, 0, 2)
        cubeCenter = vec4(nodePosition.xy - brickDistance, nodePosition.z + brickDistance, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(0, 0, 2), mode);
        drawCubeFilled(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor, minCorner, maxCorner);

        // (1, 0, 2)
        cubeCenter = vec4(nodePosition.x, nodePosition.y - brickDistance, nodePosition.z + brickDistance, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(1, 0, 2), mode);
        drawCubeFilled(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor, minCorner, maxCorner);

        // (2, 0, 2)
        cubeCenter = vec4(nodePosition.x + brickDistance, nodePosition.y - brickDistance, nodePosition.z + brickDistance, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(2, 0, 2), mode);
        drawCubeFilled(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor, minCorner, maxCorner);

    } else if (bricksToShow == 7) {
        // (2, 2, 2)
        cubeCenter = vec4(nodePosition.xyz + brickDistance, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(2, 2, 2), mode);
        drawCubeFilled(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor, minCorner, maxCorner);

        // (0, 2, 2)
        cubeCenter = vec4(nodePosition.x - brickDistance, nodePosition.yz + brickDistance, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(0, 2, 2), mode);
        drawCubeFilled(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor, minCorner, maxCorner);

        // (1, 1, 2)
        cubeCenter = vec4(nodePosition.xy, nodePosition.z + brickDistance, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(1, 1, 2), mode);
        drawCubeFilled(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor, minCorner, maxCorner);

    } else if (bricksToShow == 8) {
        // (0, 1, 2)
        cubeCenter = vec4(nodePosition.x - brickDistance, nodePosition.y, nodePosition.z + brickDistance, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(0, 1, 2), mode);
        drawCubeFilled(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor, minCorner, maxCorner);

        // (1, 2, 2)
        cubeCenter = vec4(nodePosition.x, nodePosition.yz + brickDistance, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(1, 2, 2), mode);
        drawCubeFilled(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor, minCorner, maxCorner);

        // (2, 1, 2)
        cubeCenter = vec4(nodePosition.x + brickDistance, nodePosition.y, nodePosition.z + brickDistance, nodePosition.w);
        cubeColor = showProp(geom_brickCoordinates[0] + ivec3(2, 1, 2), mode);
        drawCubeFilled(cubeCenter, voxelBrickSize, canonizationMatrix, cubeColor, minCorner, maxCorner);
    }
}
