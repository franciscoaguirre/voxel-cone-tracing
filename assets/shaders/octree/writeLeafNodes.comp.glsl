#version 460 core

#include "./_constants.glsl"
#include "./_helpers.glsl"

layout (local_size_x = WORKING_GROUP_SIZE, local_size_y = 1, local_size_z = 1) in;

uniform layout(binding = 0, rgb10_a2ui) uimageBuffer voxelPositions;
uniform layout(binding = 1, rgba8) imageBuffer voxelColors;
uniform layout(binding = 2, r32ui) uimageBuffer nodePoolBrickPointers;
uniform layout(binding = 3, rgba8) image3D brickPoolColors;
uniform layout(binding = 4, r32ui) uimageBuffer nodePool;

uniform uint voxelDimension;
uniform uint octreeLevel;
uniform uint number_of_voxel_fragments;

#include "./_traversalHelpers.glsl"
#include "./_octreeTraversal.glsl"

void storeInLeaf(vec3 voxelPosition, int nodeID, vec4 voxelColor, float halfNodeSize, vec3 nodeCoordinates) {
    uint brickCoordinatesCompact = imageLoad(nodePoolBrickPointers, nodeID).r;
    memoryBarrier();
    
    // TODO: Why store the brick coordinates in a texture and not calculate them
    // each time? Why is it non-deterministic which brick coordinates a node will
    // get?
    ivec3 brickCoordinates = ivec3(uintXYZ10ToVec3(brickCoordinatesCompact));
    // We find the closest corner in the brick to store the color (we spread it later)
    uint offset = calculateChildLocalID(nodeCoordinates, halfNodeSize, voxelPosition);

    imageStore(
        brickPoolColors,
        brickCoordinates + 2 * ivec3(CHILD_OFFSETS[offset]),
        voxelColor
    );
}

void main() {
    // Get voxel attributes from voxel fragment list
    const uint threadIndex = gl_GlobalInvocationID.x;
    if (threadIndex < number_of_voxel_fragments) {
      // We need to traverse the tree to get the node because we
      // need the voxel attributes (color, normal, etc)
      uvec4 voxelPosition = imageLoad(voxelPositions, int(threadIndex));
      vec4 voxelColor = imageLoad(voxelColors, int(threadIndex));
      // TODO: Load normal from images
      memoryBarrier();

      vec3 normalizedVoxelPosition = vec3(voxelPosition) / float(voxelDimension);

      float halfNodeSize;
      vec3 nodeCoordinates;
      // We send the voxel position to traverse the octree and find the leaf
      int nodeID = traverseOctree(
          normalizedVoxelPosition,
          octreeLevel,
          nodeCoordinates,
          halfNodeSize
      );

      // TODO: We're missing voxel normals here to store in the leaves
      // For some reason we are sending a vec3 instead of a vec4
      storeInLeaf(normalizedVoxelPosition, nodeID, voxelColor, halfNodeSize, nodeCoordinates);
  }
}
