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

void store_in_leaf(vec3 voxel_position, int node_address, vec4 voxel_color, float half_node_size, vec3 node_coordinates) {
    uint brick_coordinates_compact = imageLoad(node_pool_brick_pointers, node_address).r;
    memoryBarrier();
    
    // TODO: Why store the brick coordinates in a texture and not calculate them
    // each time? Why is it non-deterministic which brick coordinates a node will
    // get?
    ivec3 brick_coordinates = ivec3(uintXYZ10ToVec3(brick_coordinates_compact));

    // NOTE: We find out which subsection the current voxel occupies inside the node
    // Remember leaves don't have nodes, so leaf bricks effectively have 2x2x2 voxels.
    bvec3 subsection = calculate_node_subsection(node_coordinates, half_node_size, voxel_position);
    uint offset = uint(subsection[0]) + uint(subsection[1]) * 2 + uint(subsection[2]) * 4;

    imageStore(
        brick_pool_colors,
        brick_coordinates + 2 * ivec3(CHILD_OFFSETS[offset]),
        voxel_color
    );
}

void main() {
    // Get voxel attributes from voxel fragment list
    const uint thread_index = gl_GlobalInvocationID.x;
    // We need to traverse the tree to get the node because we
    // need the voxel attributes (color, normal, etc)
    uvec4 voxel_position = imageLoad(voxel_positions, int(thread_index));
    vec4 voxel_color = imageLoad(voxel_colors, int(thread_index));
    // TODO: Load normal from images
    memoryBarrier();

    vec3 normalized_voxel_position = vec3(voxel_position) / float(voxel_dimension);

    uint _tile_index; // Unused
    float half_node_size;
    vec3 node_coordinates;
    // We send the voxel position to traverse the octree and find the leaf
    int node_address = traverse_octree_returning_node_coordinates(
        normalized_voxel_position,
        max_octree_level,
        node_pool,
        half_node_size,
        node_coordinates,
        _tile_index
    );

    // TODO: We're missing voxel normals here to store in the leaves
    // For some reason we are sending a vec3 instead of a vec4
    store_in_leaf(normalized_voxel_position, node_address, voxel_color, half_node_size, node_coordinates);
}