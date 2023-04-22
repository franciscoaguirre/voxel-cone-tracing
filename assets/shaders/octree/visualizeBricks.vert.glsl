#version 460 core

#include "./_constants.glsl"
#include "./_helpers.glsl"

uniform uint octreeLevels;
uniform uint voxelDimension;

uniform layout(binding = 0, r32ui) uimageBuffer nodePool;
uniform layout(binding = 1, rgba8) image3D brickPoolColors;
uniform layout(binding = 2, rgb10_a2ui) uimageBuffer voxelPositions;
uniform layout(binding = 3, r32ui) uimage3D brickPoolPhotons;

out vec4 geom_nodePosition;
out float geom_halfNodeSize;
out ivec3 geom_brickCoordinates;

#include "./_traversalHelpers.glsl"
#include "./_octreeTraversal.glsl"

void main() {
  int threadIndex = gl_VertexID;

  vec4 voxelFragmentPosition = imageLoad(voxelPositions, threadIndex);

  float halfNodeSize;
  vec3 nodeCoordinates;
  int nodeID = traverseOctree(
    vec3(voxelFragmentPosition) / float(voxelDimension),
    octreeLevels,
    nodeCoordinates,
    halfNodeSize
  );

  ivec3 brickCoordinates = calculateBrickCoordinates(nodeID);
  geom_brickCoordinates = brickCoordinates;

  // Normalized device coordinates go from -1.0 to 1.0, our coordinates go from 0.0 to 1.0
  geom_nodePosition = vec4((nodeCoordinates.xyz) * 2.0 - vec3(1.0), 1.0);
  float normalizedHalfNodeSize = halfNodeSize * 2.0;

  geom_nodePosition.xyz += normalizedHalfNodeSize;
  gl_Position = geom_nodePosition;

  geom_halfNodeSize = normalizedHalfNodeSize;
}
