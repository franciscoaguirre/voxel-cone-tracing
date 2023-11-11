#version 460 core

#include "./_constants.glsl"

layout (local_size_x = WORKING_GROUP_SIZE, local_size_y = 1, local_size_z = 1) in;

uniform layout(binding = 0, r32ui) uimageBuffer nodePool;
uniform layout(binding = 1, rgb10_a2ui) uimageBuffer voxelPositions;

uniform layout(binding = 2, r32ui) uimageBuffer nodePoolNeighborsPositive;
uniform layout(binding = 3, r32ui) uimageBuffer nodePoolNeighborsNegative;
uniform layout(binding = 4, rgb10_a2ui) readonly uimageBuffer nodePositions;

uniform int axis;
uniform uint octreeLevel;
uniform uint voxelDimension;
uniform uint levelStart;
uniform uint nextLevelStart;

#include "./_traversalHelpers.glsl"
#include "./_helpers.glsl"
#include "./_octreeTraversal.glsl"
#include "./_threadNodeUtilNoTexture.glsl"

void main() {
    int nodeID = getThreadNode();
    if (nodeID == NODE_NOT_FOUND) {
        return;
    }
    ivec4 nodePosition = ivec4(imageLoad(nodePositions, nodeID));
    float halfNodeSize = calculateHalfNodeSize(octreeLevel);
    // Normalized node position
    vec3 nodeCoordinates = nodePosition.xyz / float(voxelDimension);

    // Get center of node
    vec3 centerNodeCoordinates = nodeCoordinates + halfNodeSize;
    
    int neighborPositive = 0;
    int neighborNegative = 0;
    
    uint neighborLevel = 0;

    vec3 _nodeCoordinates;
    float _halfNodeSize;
    float nodeSize = halfNodeSize * 2;
    vec3 possibleNeighborPosition;
    
    // TODO: Check if this shouldn't be `<=`
    // If this is 1, it means that the voxel is at the very edge
    // of the grid. Is this possible? If it is, do we still represent
    // the voxel on a brick?
    vec3 neighborOffset = vec3(0);
    neighborOffset[axis] = nodeSize;

    possibleNeighborPosition = centerNodeCoordinates + neighborOffset;

    if (possibleNeighborPosition[axis] < 1) {
      neighborPositive = traverseOctree(
        possibleNeighborPosition,
        octreeLevel,
        _nodeCoordinates,
        _halfNodeSize
      );

      if (neighborPositive == NODE_NOT_FOUND) {
        neighborPositive = 0;
      }
    }

    possibleNeighborPosition = centerNodeCoordinates - neighborOffset;

    if (possibleNeighborPosition[axis] > 0) {
      neighborNegative = traverseOctree(
        possibleNeighborPosition,
        octreeLevel,
        _nodeCoordinates,
        _halfNodeSize
      );

      if (neighborNegative == NODE_NOT_FOUND) {
        neighborNegative = 0;
      }
    }

    imageStore(nodePoolNeighborsPositive, nodeID, uvec4(neighborPositive));
    imageStore(nodePoolNeighborsNegative, nodeID, uvec4(neighborNegative));
}
