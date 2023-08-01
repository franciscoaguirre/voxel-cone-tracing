/// `visualizeBricks.vert.glsl`
/// Visualize all bricks in the octree, one Z layer at a time.

#version 460 core

#include "./_constants.glsl"
#include "./_helpers.glsl"

uniform uint octreeLevel;
uniform uint voxelDimension;
uniform uint maxOctreeLevel;

uniform layout(binding = 0, rgb10_a2ui) readonly uimageBuffer nodePositions;
uniform layout(binding = 1, r32ui) readonly uimageBuffer levelStartIndices;

out VertexData {
    vec4 nodePosition;
    float halfNodeSize;
    ivec3 brickCoordinates;
} Out;

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
  Out.brickCoordinates = brickCoordinates;

  float normalizedHalfNodeSize = halfNodeSize * 2.0;
  Out.nodePosition = vec4(nodeCoordinates.xyz * 2.0 - vec3(1.0), 1.0);

  Out.nodePosition.xyz += vec3(normalizedHalfNodeSize);
  gl_Position = Out.nodePosition;

  Out.halfNodeSize = normalizedHalfNodeSize;
}
