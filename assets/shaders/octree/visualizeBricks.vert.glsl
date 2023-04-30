#version 460 core

#include "./_constants.glsl"
#include "./_helpers.glsl"

uniform uint octreeLevel;
uniform uint voxelDimension;
uniform uint maxOctreeLevel;

uniform layout(binding = 0, r32ui) uimageBuffer nodePool;
uniform layout(binding = 1, rgba8) image3D brickPoolColors;
uniform layout(binding = 2, rgb10_a2ui) uimageBuffer nodePositions;
uniform layout(binding = 3, r32ui) uimage3D brickPoolPhotons;
uniform layout(binding = 4, r32ui) readonly uimageBuffer levelStartIndices;

out vec4 geom_nodePosition;
out float geom_halfNodeSize;
out ivec3 geom_brickCoordinates;

void main() {
  int nodeID = gl_VertexID;
  int levelStart = int(imageLoad(levelStartIndices, int(octreeLevel)).r);
  int nextLevelStart = int(imageLoad(levelStartIndices, int(octreeLevel + 1)).r);
  memoryBarrier();

  nodeID += levelStart;
  if (nodeID >= nextLevelStart) {
      nodeID = levelStart;
  }

  vec3 nodeCoordinates = imageLoad(nodePositions, nodeID).xyz;
  nodeCoordinates /= float(voxelDimension);
  float halfNodeSize = calculateHalfNodeSize(octreeLevel);

  ivec3 brickCoordinates = calculateBrickCoordinates(nodeID);
  geom_brickCoordinates = brickCoordinates;

  // Normalized device coordinates go from -1.0 to 1.0, our coordinates go from 0.0 to 1.0
  geom_nodePosition = vec4((nodeCoordinates.xyz) * 2.0 - vec3(1.0), 1.0);
  float normalizedHalfNodeSize = halfNodeSize * 2.0;

  geom_nodePosition.xyz += normalizedHalfNodeSize;
  gl_Position = geom_nodePosition;

  geom_halfNodeSize = normalizedHalfNodeSize;
}
