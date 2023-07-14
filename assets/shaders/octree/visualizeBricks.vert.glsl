#version 460 core

#include "./_constants.glsl"
#include "./_helpers.glsl"

uniform uint octreeLevel;
uniform uint voxelDimension;
uniform uint maxOctreeLevel;

uniform layout(binding = 0, rgba8) image3D brickPoolColors;
uniform layout(binding = 1, rgb10_a2ui) uimageBuffer nodePositions;
uniform layout(binding = 2, r32ui) uimage3D brickPoolPhotons;
uniform layout(binding = 3, r32ui) readonly uimageBuffer levelStartIndices;

out vec4 geom_nodePosition;
out float geom_halfNodeSize;
out ivec3 geom_brickCoordinates;

#include "assets/shaders/octree/_brickCoordinates.glsl"

void main() {
  int nodeID = gl_VertexID;
  int levelStart = int(imageLoad(levelStartIndices, int(octreeLevel)).r);
  int nextLevelStart = int(imageLoad(levelStartIndices, int(octreeLevel + 1)).r);
  memoryBarrier();

  nodeID += levelStart;
  if (nodeID >= nextLevelStart) {
      nodeID = levelStart;
  }

  uvec3 nodeCoordinatesRaw = imageLoad(nodePositions, nodeID).xyz;
  vec3 nodeCoordinates = nodeCoordinatesRaw / float(voxelDimension);
  float halfNodeSize = calculateHalfNodeSize(octreeLevel);

  ivec3 brickCoordinates = calculateBrickCoordinates(nodeID);
  geom_brickCoordinates = brickCoordinates;

  float normalizedHalfNodeSize = halfNodeSize * 2.0;
  geom_nodePosition = vec4(nodeCoordinates.xyz * 2.0 - vec3(1.0), 1.0);

  geom_nodePosition.xyz += vec3(normalizedHalfNodeSize);
  gl_Position = geom_nodePosition;

  geom_halfNodeSize = normalizedHalfNodeSize;
}
