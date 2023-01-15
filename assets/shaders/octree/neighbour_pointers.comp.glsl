#version 460 core

#include "./_constants.glsl"
#include "./_traversal_helpers.glsl"
#include "./_octree_traversal.glsl"

layout (local_size_x = WORKING_GROUP_SIZE, local_size_y = 1, local_size_z = 1) in;

uniform layout(binding = 0, r32ui) uimageBuffer node_pool;
uniform layout(binding = 1, rgb10_a2ui) uimageBuffer voxel_positions;

uniform layout(binding = 2, r32ui) uimageBuffer node_pool_neighbours_x;
uniform layout(binding = 3, r32ui) uimageBuffer node_pool_neighbours_x_negative;
uniform layout(binding = 4, r32ui) uimageBuffer node_pool_neighbours_y;
uniform layout(binding = 5, r32ui) uimageBuffer node_pool_neighbours_y_negative;
uniform layout(binding = 6, r32ui) uimageBuffer node_pool_neighbours_z;
uniform layout(binding = 7, r32ui) uimageBuffer node_pool_neighbours_z_negative;

uniform uint octreeLevel;
uniform uint voxelDimension;

void main() {
    const uint thread_index = gl_GlobalInvocationID.x;
    uvec4 voxel_position = imageLoad(voxel_positions, int(thread_index));
    vec3 normalized_voxel_position = vec3(voxel_position) / float(voxelDimension);

	// In voxel position coordinates, the octree level
	// defines a different node size, which we need as a step to reach
	// possible neighbours.
    float step = 1.0 / float(pow(2.0, float(octreeLevel)));
	
	int node_address = traverse_octree(
		normalized_voxel_position,
		octreeLevel,
		node_pool
	);
	
	int neighbour_x = 0;
	int neighbour_x_negative = 0;
	int neighbour_y = 0;
	int neighbour_y_negative = 0;
	int neighbour_z = 0;
	int neighbour_z_negative = 0;
	
	uint neighborLevel = 0;
	
	// TODO: Check if this shouldn't be `<=`
	// If this is 1, it means that the voxel is at the very edge
	// of the grid. Is this possible? If it is, do we still represent
	// the voxel on a brick?
	if (normalized_voxel_position.x + step < 1) {
		neighbour_x = traverse_octree_returning_level(
			normalized_voxel_position + vec3(step, 0, 0),
			octreeLevel,
			node_pool,
			neighborLevel
		);
		
		// It is possible that the current voxel fragment's neighbour
		// is on another level, one that ended before the max level.
		if (neighborLevel != octreeLevel) {
			neighbour_x = 0;
		}
	}
	
	if (voxel_position.y + step < voxelDimension) {
		neighbour_y = traverse_octree_returning_level(
			normalized_voxel_position + uvec3(0, step, 0),
			octreeLevel,
			node_pool,
			neighborLevel
		);
		
		if (neighborLevel != octreeLevel) {
			neighbour_y = 0;
		}
	}

	if (voxel_position.z + step < voxelDimension) {
		neighbour_z = traverse_octree_returning_level(
			normalized_voxel_position + uvec3(0, 0, step),
			octreeLevel,
			node_pool,
			neighborLevel
		);
		
		if (neighborLevel != octreeLevel) {
			neighbour_z = 0;
		}
	}

	if (voxel_position.x - step > 0) {
		neighbour_x_negative = traverse_octree_returning_level(
			normalized_voxel_position - uvec3(step, 0, 0),
			octreeLevel,
			node_pool,
			neighborLevel
		);
		
		if (neighborLevel != octreeLevel) {
			neighbour_x_negative = 0;
		}
	}

	if (voxel_position.y - step > 0) {
		neighbour_y_negative = traverse_octree_returning_level(
			normalized_voxel_position - uvec3(0, step, 0),
			octreeLevel,
			node_pool,
			neighborLevel
		);
		
		if (neighborLevel != octreeLevel) {
			neighbour_y_negative = 0;
		}
	}

	if (voxel_position.z - step > 0) {
		neighbour_z_negative = traverse_octree_returning_level(
			normalized_voxel_position - uvec3(0, 0, step),
			octreeLevel,
			node_pool,
			neighborLevel
		);
		
		if (neighborLevel != octreeLevel) {
			neighbour_z_negative = 0;
		}
	}

	imageStore(node_pool_neighbours_x, node_address, uvec4(neighbour_x));
	imageStore(node_pool_neighbours_y, node_address, uvec4(neighbour_y));
	imageStore(node_pool_neighbours_z, node_address, uvec4(neighbour_z));
	imageStore(node_pool_neighbours_x_negative, node_address, uvec4(neighbour_x_negative));
	imageStore(node_pool_neighbours_y_negative, node_address, uvec4(neighbour_y_negative));
	imageStore(node_pool_neighbours_z_negative, node_address, uvec4(neighbour_z_negative));
}
