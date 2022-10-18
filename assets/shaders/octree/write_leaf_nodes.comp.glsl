#version 460 core

#include "./_constants.glsl"
#include "./_helpers.glsl"
#include "./_traversal_helpers.glsl"
#include "./_octree_traversal.glsl"

layout (local_size_x = WORKING_GROUP_SIZE, local_size_y = 1, local_size_z = 1) in;

uniform layout(binding = 0, rgb10_a2ui) uimageBuffer voxel_positions;
uniform layout(binding = 1, rgba8) imageBuffer voxel_colors;
uniform layout(binding = 2, r32ui) uimageBuffer node_pool_brick_pointers;
uniform layout(binding = 3, rgba8) image3D brick_pool_colors;
uniform layout(binding = 4, r32ui) uimageBuffer node_pool;

uniform uint voxel_dimension;
uniform int max_octree_level;

void store_in_leaf(vec3 voxel_position, int node_address, uvec4 voxel_color) {
    uint brick_coordinates_compact = imageLoad(node_pool_brick_pointers, node_address).r;
    memoryBarrier();
    
    // TODO: Why store the brick coordinates in a texture and not calculate them
    // each time. Why is it non-deterministic which brick coordinates a node will
    // get.
    ivec3 brick_coordinates = ivec3(uintXYZ10ToVec3(brick_coordinates_compact));
    uvec3 offset_vector = uvec3(voxel_position);
    uint offset = offset_vector.x + offset_vector.y * 2U + offset_vector.z * 4U;
    
    imageStore(
        brick_pool_colors,
        brick_coordinates + 2 * ivec3(CHILD_OFFSETS[offset]),
        voxel_color / 255.0
    );
}

void main() {
    // Get voxel attributes from voxel fragment list
    const uint thread_index = gl_GlobalInvocationID.x;
    uvec4 voxel_position = imageLoad(voxel_positions, int(thread_index));
    uvec4 voxel_color = imageLoad(voxel_colors, int(thread_index));
    // TODO: Load normal from images
    memoryBarrier();
    
    // We send the voxel position to traverse the octree and find the leaf
    int node_address = traverse_octree(
        uvec3(voxel_position),
        voxel_dimension,
        max_octree_level,
        node_pool
    );
    
    vec3 normalized_voxel_position = vec3(voxel_position) / vec3(voxel_dimension);
    
    // TODO: We're missing voxel normals here to store in the leaves
    // For some reason we are sending a vec3 instead of a vec4
    store_in_leaf(normalized_voxel_position, node_address, voxel_color);
}