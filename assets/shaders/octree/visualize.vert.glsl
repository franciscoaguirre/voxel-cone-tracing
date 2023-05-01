#version 460 core

#include "./_constants.glsl"
#include "./_helpers.glsl"

uniform uint octreeLevels;
uniform uint voxelDimension;

uniform layout(binding = 0, r32ui) uimageBuffer nodePool;
uniform layout(binding = 2, rgba8) image3D brickPoolColors;
uniform layout(binding = 3, rgb10_a2ui) uimageBuffer voxelPositions;
uniform layout(binding = 4, r32ui) uimage3D brickPoolPhotons;

out vec4 geom_nodePosition;
out float geom_halfNodeSize;
out vec4 nodeColor;
out ivec3 geom_brickCoordinates;

#include "./_traversalHelpers.glsl"
#include "./_octreeTraversal.glsl"

void main() {
  int threadIndex = gl_VertexID;

  // TODO: Find an efficient way to render both occupied and empty nodes.
  // This approach uses voxel fragment positions and therefore doesn't show
  // empty nodes.
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

  // NOTE: Bricks start at (0, 0, 0) and go to (2, 2, 2)
  ivec3 offsetToCenter = ivec3(1, 1, 1);
  vec4 centerVoxelColor = imageLoad(brickPoolColors, brickCoordinates + offsetToCenter);
  nodeColor = centerVoxelColor;
  // nodeColor = vec4(1);

  // uint photonCount = imageLoad(brickPoolPhotons, brickCoordinates + offsetToCenter).r;
  // if (photonCount > 0) {
  //   nodeColor = vec4(1.0, 1.0, 1.0, 1.0);
  // } else {
  //   nodeColor = vec4(0.0, 0.0, 0.0, 1.0);
  // }

  // Normalized device coordinates go from -1.0 to 1.0, our coordinates go from 0.0 to 1.0
  geom_nodePosition = vec4((nodeCoordinates.xyz) * 2.0 - vec3(1.0), 1.0);
  float normalizedHalfNodeSize = halfNodeSize * 2.0;

  geom_nodePosition.xyz += normalizedHalfNodeSize;
  gl_Position = geom_nodePosition;

  geom_halfNodeSize = normalizedHalfNodeSize;
}
