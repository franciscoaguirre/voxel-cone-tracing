#version 460 core

#include "./_constants.glsl"

layout (local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

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
#include "./_octreeTraversal.glsl"

void main() {
    const uint threadIndex = gl_GlobalInvocationID.x;
    uvec4 voxelPosition = imageLoad(voxelPositions, int(threadIndex));
    vec3 normalizedVoxelPosition = vec3(voxelPosition) / float(voxelDimension);

	// In voxel position coordinates, the octree level
	// defines a different node size, which we need as a step to reach
	// possible neighbours.
	// The step is halfNodeSize.
    uint tileIndex;
    float halfNodeSize;
    vec3 nodeCoordinates;
    int nodeAddress = traverseOctreeReturningNodeCoordinates(
        normalizedVoxelPosition,
        octreeLevel,
        halfNodeSize,
        nodeCoordinates, // Already normalized
        tileIndex
    );
	float normalizedHalfNodeSize = halfNodeSize * 2.0;
	vec3 nodeCoordinatesToRender = nodeCoordinates * 2.0 - vec3(1.0);
	nodeCoordinatesToRender += normalizedHalfNodeSize;
	nodeCoordinates += halfNodeSize;
	
	int neighborX = 0;
	int neighborXNegative = 0;
	int neighborY = 0;
	int neighborYNegative = 0;
	int neighborZ = 0;
	int neighborZNegative = 0;
	
	uint neighborLevel = 0;
	
	// TODO: Check if this shouldn't be `<=`
	// If this is 1, it means that the voxel is at the very edge
	// of the grid. Is this possible? If it is, do we still represent
	// the voxel on a brick?
	if (normalizedVoxelPosition.x + halfNodeSize < 1) {
		neighborX = traverseOctreeReturningLevel(
			normalizedVoxelPosition + vec3(halfNodeSize, 0, 0),
			octreeLevel,
			neighborLevel
		);

		// It is possible that the current voxel fragment's neighbour
		// is on another level, one that ended before the max level.
		if (neighborLevel != octreeLevel) {
			neighborX = 0;
		}
	}
	
	if (normalizedVoxelPosition.y + halfNodeSize < 1) {
		neighborY = traverseOctreeReturningLevel(
			nodeCoordinates + uvec3(0, halfNodeSize, 0),
			octreeLevel,
			neighborLevel
		);

		imageStore(debugBuffer, 0, vec4(float(neighborY), 0, 0, 0));
		imageStore(debugBuffer, 1, vec4(float(octreeLevel), 0, 0, 0));
		imageStore(debugBuffer, 2, vec4(float(neighborLevel), 0, 0, 0));
		imageStore(debugBuffer, 3, vec4(float(nodeAddress), 0, 0, 0));
		imageStore(debugBuffer, 4, vec4(float(nodeCoordinatesToRender.x), 0, 0, 0));
		imageStore(debugBuffer, 5, vec4(float(nodeCoordinatesToRender.y), 0, 0, 0));
		imageStore(debugBuffer, 6, vec4(float(nodeCoordinatesToRender.z), 0, 0, 0));
		
		if (neighborLevel != octreeLevel) {
			neighborY = 0;
		}
	}

	if (normalizedVoxelPosition.z + halfNodeSize < 1) {
		neighborZ = traverseOctreeReturningLevel(
			normalizedVoxelPosition + uvec3(0, 0, halfNodeSize),
			octreeLevel,
			neighborLevel
		);

		if (neighborLevel != octreeLevel) {
			neighborZ = 0;
		}
	}

	if (normalizedVoxelPosition.x - halfNodeSize > 0) {
		neighborXNegative = traverseOctreeReturningLevel(
			normalizedVoxelPosition - uvec3(halfNodeSize, 0, 0),
			octreeLevel,
			neighborLevel
		);
		
		if (neighborLevel != octreeLevel) {
			neighborXNegative = 0;
		}
	}

	if (normalizedVoxelPosition.y - halfNodeSize > 0) {
		neighborYNegative = traverseOctreeReturningLevel(
			normalizedVoxelPosition - uvec3(0, halfNodeSize, 0),
			octreeLevel,
			neighborLevel
		);
		
		if (neighborLevel != octreeLevel) {
			neighborYNegative = 0;
		}
	}

	if (normalizedVoxelPosition.z - halfNodeSize > 0) {
		neighborZNegative = traverseOctreeReturningLevel(
			normalizedVoxelPosition - uvec3(0, 0, halfNodeSize),
			octreeLevel,
			neighborLevel
		);
		
		if (neighborLevel != octreeLevel) {
			neighborZNegative = 0;
		}
	}

	imageStore(nodePoolNeighborsX, nodeAddress, uvec4(neighborX));
	imageStore(nodePoolNeighborsY, nodeAddress, uvec4(neighborY));
	imageStore(nodePoolNeighborsZ, nodeAddress, uvec4(neighborZ));
	imageStore(nodePoolNeighborsXNegative, nodeAddress, uvec4(neighborXNegative));
	imageStore(nodePoolNeighborsYNegative, nodeAddress, uvec4(neighborYNegative));
	imageStore(nodePoolNeighborsZNegative, nodeAddress, uvec4(neighborZNegative));
}
