#version 460 core

#include "./_constants.glsl"

layout (local_size_x = WORKING_GROUP_SIZE, local_size_y = 1, local_size_z = 1) in;

uniform layout(binding = 0, r32ui) uimageBuffer nodePool;
uniform layout(binding = 1, rgb10_a2ui) uimageBuffer voxelPositions;

uniform layout(binding = 2, r32ui) uimageBuffer nodePoolNeighborsX;
uniform layout(binding = 3, r32ui) uimageBuffer nodePoolNeighborsXNegative;
uniform layout(binding = 4, r32ui) uimageBuffer nodePoolNeighborsY;
uniform layout(binding = 5, r32ui) uimageBuffer nodePoolNeighborsYNegative;
uniform layout(binding = 6, r32ui) uimageBuffer nodePoolNeighborsZ;
uniform layout(binding = 7, r32ui) uimageBuffer nodePoolNeighborsZNegative;
uniform layout(binding = 8, r32f) imageBuffer debugBuffer;
uniform layout(binding = 9, r32ui) readonly uimageBuffer levelStartIndices;

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
    
    int neighborX = 0;
    int neighborXNegative = 0;
    int neighborY = 0;
    int neighborYNegative = 0;
    int neighborZ = 0;
    int neighborZNegative = 0;
    
    uint neighborLevel = 0;

    vec3 _nodeCoordinates;
    float _halfNodeSize;
    float nodeSize = halfNodeSize * 2;
    vec3 possibleNeighborPosition;
    
    // TODO: Check if this shouldn't be `<=`
    // If this is 1, it means that the voxel is at the very edge
    // of the grid. Is this possible? If it is, do we still represent
    // the voxel on a brick?
    possibleNeighborPosition = nodeCoordinates + vec3(nodeSize,0,0);
    if (possibleNeighborPosition.x < 1) {
      neighborX = traverseOctree(
        possibleNeighborPosition,
        octreeLevel,
        _nodeCoordinates,
        _halfNodeSize
      );

      // It is possible that the current voxel fragment's neighbor
      // is on another level, one that ended before the max level.
      if (neighborX == NODE_NOT_FOUND) {
        neighborX = 0;
      }
    }
    
    possibleNeighborPosition = nodeCoordinates + vec3(0,nodeSize,0);
    if (possibleNeighborPosition.y < 1) {
      neighborY = traverseOctree(
        possibleNeighborPosition,
        octreeLevel,
        _nodeCoordinates,
        _halfNodeSize
      );

      //imageStore(debugBuffer, 4, vec4(float(nodeCoordinatesToRender.x), 0, 0, 0));
      //imageStore(debugBuffer, 5, vec4(float(nodeCoordinatesToRender.y), 0, 0, 0));
      //imageStore(debugBuffer, 6, vec4(float(nodeCoordinatesToRender.z), 0, 0, 0));
      
      if (neighborY == NODE_NOT_FOUND) {
        neighborY = 0;
      }
    }

    possibleNeighborPosition = nodeCoordinates + vec3(0,0,nodeSize);
    if (possibleNeighborPosition.z < 1) {
      neighborZ = traverseOctree(
        possibleNeighborPosition,
        octreeLevel,
        _nodeCoordinates,
        _halfNodeSize
      );

      if (neighborZ == NODE_NOT_FOUND) {
        neighborZ = 0;
      }
    }

    possibleNeighborPosition = nodeCoordinates - vec3(nodeSize,0,0);
    if (possibleNeighborPosition.x > 0) {
      neighborXNegative = traverseOctree(
        possibleNeighborPosition,
        octreeLevel,
        _nodeCoordinates,
        _halfNodeSize
      );

      imageStore(debugBuffer, 0, vec4(float(neighborXNegative), 0, 0, 0));
      imageStore(debugBuffer, 1, vec4(float(octreeLevel), 0, 0, 0));
      imageStore(debugBuffer, 3, vec4(float(nodeID), 0, 0, 0));
      
      if (neighborXNegative == NODE_NOT_FOUND) {
        neighborXNegative = 0;
      }
    }

    possibleNeighborPosition = nodeCoordinates - vec3(0,nodeSize,0);
    if (possibleNeighborPosition.y > 0) {
      neighborYNegative = traverseOctree(
        possibleNeighborPosition,
        octreeLevel,
        _nodeCoordinates,
        _halfNodeSize
      );
      
      if (neighborYNegative == NODE_NOT_FOUND) {
        neighborYNegative = 0;
      }
    }

    possibleNeighborPosition = nodeCoordinates - vec3(0,0,nodeSize);
    if (possibleNeighborPosition.z > 0) {
      neighborZNegative = traverseOctree(
        possibleNeighborPosition,
        octreeLevel,
        _nodeCoordinates,
        _halfNodeSize
      );
      
      if (neighborZNegative == NODE_NOT_FOUND) {
        neighborZNegative = 0;
      }
    }

    imageStore(nodePoolNeighborsX, nodeID, uvec4(neighborX));
    imageStore(nodePoolNeighborsY, nodeID, uvec4(neighborY));
    imageStore(nodePoolNeighborsZ, nodeID, uvec4(neighborZ));
    imageStore(nodePoolNeighborsXNegative, nodeID, uvec4(neighborXNegative));
    imageStore(nodePoolNeighborsYNegative, nodeID, uvec4(neighborYNegative));
    imageStore(nodePoolNeighborsZNegative, nodeID, uvec4(neighborZNegative));
}
