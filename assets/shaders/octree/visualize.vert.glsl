#version 460 core

#include "./_constants.glsl"
#include "./_helpers.glsl"
#include "./_traversal_helpers.glsl"
#include "./_octree_traversal.glsl"

uniform uint octreeLevels;
uniform bool showEmptyNodes;
uniform uint voxelDimension;

uniform layout(binding = 0, r32ui) uimageBuffer nodePool;
uniform layout(binding = 1, r32ui) uimageBuffer nodePoolBrickPointers;
uniform layout(binding = 2, rgba8) image3D brickPoolColors;
uniform layout(binding = 3, rgb10_a2ui) uimageBuffer voxelPositions;

out vec4 nodePosition;
out float geom_halfNodeSize;
out int nonEmptyBranch;
out vec4 nodeColor;
out ivec3 geom_brickCoordinates;

void main() {
  int threadIndex = gl_VertexID;

  // TODO: Find an efficient way to render both occupied and empty nodes.
  // This approach uses voxel fragment positions and therefore doesn't show
  // empty nodes.
  vec4 voxelFragmentPosition = imageLoad(voxelPositions, threadIndex);

  uint tileIndex;
  float halfNodeSize;
  vec3 nodeCoordinates;
  int nodeIndex = traverse_octree_returning_node_coordinates(
    vec3(voxelFragmentPosition) / float(voxelDimension),
    octreeLevels,
    nodePool,
    halfNodeSize,
    nodeCoordinates,
    tileIndex
  );

  uint brickCoordinatesCompact = imageLoad(nodePoolBrickPointers, nodeIndex).r;
  ivec3 brickCoordinates = ivec3(uintXYZ10ToVec3(brickCoordinatesCompact));
  geom_brickCoordinates = brickCoordinates;

  // NOTE: Bricks start at (0, 0, 0) and go to (2, 2, 2)
  ivec3 offsetToCenter = ivec3(1, 1, 1);
  vec4 centerVoxelColor = imageLoad(brickPoolColors, brickCoordinates + offsetToCenter);
  nodeColor = centerVoxelColor;

  // Normalized device coordinates go from -1.0 to 1.0, our coordinates go from 0.0 to 1.0
  nodePosition = vec4((nodeCoordinates.xyz) * 2.0 - vec3(1.0), 1.0);
  float normalizedHalfNodeSize = halfNodeSize * 2.0;

  nodePosition.xyz += normalizedHalfNodeSize;
  gl_Position = nodePosition;

  if (tileIndex != 0 || octreeLevels == 0) {
    geom_halfNodeSize = normalizedHalfNodeSize;
    nonEmptyBranch = 1;
  } else {
    geom_halfNodeSize = normalizedHalfNodeSize * int(showEmptyNodes);
    nonEmptyBranch = 0;
  }
}
