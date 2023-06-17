#version 460 core

#include "./_constants.glsl"

layout (local_size_x = WORKING_GROUP_SIZE, local_size_y = 1, local_size_z = 1) in;

uniform layout(binding = 0, r32ui) uimageBuffer nodePool;
uniform layout(binding = 1, rgb10_a2ui) uimageBuffer voxelPositions;

uniform layout(binding = 2, r32ui) uimageBuffer nodePoolNeighborsPositive;
uniform layout(binding = 3, r32ui) uimageBuffer nodePoolNeighborsNegative;
uniform layout(binding = 4, r32ui) readonly uimageBuffer levelStartIndices;

uniform int axis;
uniform uint octreeLevel;
uniform uint voxelDimension;

#include "./_traversalHelpers.glsl"
#include "./_helpers.glsl"
#include "./_octreeTraversal.glsl"

void main() {
    const uint threadIndex = gl_GlobalInvocationID.x;
    uvec3 voxelPosition = imageLoad(voxelPositions, int(threadIndex)).xyz;
    vec3 normalizedVoxelPosition = normalizedFromIntCoordinates(voxelPosition, float(voxelDimension));

    // In voxel position coordinates, the octree level
    // defines a different node size, which we need as a step to reach
    // possible neighbors.
    // The step is halfNodeSize.
    float halfNodeSize;
    vec3 nodeCoordinates;
    int nodeID = traverseOctree(
        normalizedVoxelPosition,
        octreeLevel,
        nodeCoordinates, // Already normalized
        halfNodeSize
    );
    //// Normalized to NDC
    //float normalizedHalfNodeSize = halfNodeSize * 2.0;
    //vec3 nodeCoordinatesToRender = nodeCoordinates * 2.0 - vec3(1.0);
    //nodeCoordinatesToRender += normalizedHalfNodeSize;

    // Get center of node
    nodeCoordinates += halfNodeSize;
    
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
    float coordinatePosibleNeighborPosition;
    if (axis == 0) {
      neighborOffset.x = nodeSize;
    } else if (axis == 1) {
      neighborOffset.y = nodeSize;
    } else if (axis == 2) {
      neighborOffset.z = nodeSize;
    }

    possibleNeighborPosition = nodeCoordinates + neighborOffset;
    if (axis == 0) {
      coordinatePosibleNeighborPosition = possibleNeighborPosition.x;
    } else if (axis == 1) {
      coordinatePosibleNeighborPosition = possibleNeighborPosition.y;
    } else if (axis == 2) {
      coordinatePosibleNeighborPosition = possibleNeighborPosition.z;
    }

    if (coordinatePosibleNeighborPosition < 1) {
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

    possibleNeighborPosition = nodeCoordinates - neighborOffset;
    if (axis == 0) {
      coordinatePosibleNeighborPosition = possibleNeighborPosition.x;
    } else if (axis == 1) {
      coordinatePosibleNeighborPosition = possibleNeighborPosition.y;
    } else if (axis == 2) {
      coordinatePosibleNeighborPosition = possibleNeighborPosition.z;
    }

    if (coordinatePosibleNeighborPosition > 0) {
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
