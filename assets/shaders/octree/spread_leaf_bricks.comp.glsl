#version 460 core

#include "./_constants.glsl"
#include "./_helpers.glsl"
#include "./_traversal_helpers.glsl"
#include "./_octree_traversal.glsl"

layout (local_size_x = WORKING_GROUP_SIZE, local_size_y = 1, local_size_z = 1) in;

uniform layout(binding = 0, r32ui) readonly uimageBuffer node_pool_brick_pointers;
uniform layout(binding = 1, rgba8) volatile image3D brick_pool_colors;
uniform layout(binding = 2, r32ui) uimageBuffer node_pool;
uniform layout(binding = 3, rgb10_a2ui) readonly uimageBuffer voxel_positions;

uniform uint voxel_dimension;
uniform uint octree_levels;

vec4[8] load_corner_voxels_values(ivec3 brick_address) {
    vec4 voxel_values[8];

    // Get voxels from the corners of the brick
    for (int i = 0; i < 8; i++) {
        voxel_values[i] = imageLoad(
            brick_pool_colors,
            brick_address + 2 * ivec3(CHILD_OFFSETS[i])
        );
    }
    
    return voxel_values;
}

void main() {
    // TODO: Don't traverse the tree, we can just use the node and brick
    const uint thread_index = gl_GlobalInvocationID.x;
    uvec4 voxel_position = imageLoad(voxel_positions, int(thread_index));

    int node_address = traverse_octree(
        vec3(voxel_position) / float(voxel_dimension),
        int(octree_levels),
        node_pool
    );
    
    ivec3 brick_address = ivec3(
        uintXYZ10ToVec3(
            imageLoad(node_pool_brick_pointers, int(node_address)).r
        )
    ); 
    
    vec4[] voxel_values = load_corner_voxels_values(brick_address);
    
    vec4 accumulator = vec4(0);
    
    // Load center voxel
    for (int i = 0; i < 8; i++) {
        accumulator += 0.125 * voxel_values[i];
    }
    imageStore(brick_pool_colors, brick_address + ivec3(1, 1, 1), accumulator);
    
    // Face X
    accumulator = vec4(0);
    accumulator += 0.25 * voxel_values[1];
    accumulator += 0.25 * voxel_values[3];
    accumulator += 0.25 * voxel_values[5];
    accumulator += 0.25 * voxel_values[7];
    imageStore(brick_pool_colors, brick_address + ivec3(2,1,1), accumulator);

    // Face X Neg
    accumulator = vec4(0);
    accumulator += 0.25 * voxel_values[0];
    accumulator += 0.25 * voxel_values[2];
    accumulator += 0.25 * voxel_values[4];
    accumulator += 0.25 * voxel_values[6];
    imageStore(brick_pool_colors, brick_address + ivec3(0,1,1), accumulator);


    // Face Y
    accumulator = vec4(0);
    accumulator += 0.25 * voxel_values[2];
    accumulator += 0.25 * voxel_values[3];
    accumulator += 0.25 * voxel_values[6];
    accumulator += 0.25 * voxel_values[7];
    imageStore(brick_pool_colors, brick_address + ivec3(1,2,1), accumulator);

    // Face Y Neg
    accumulator = vec4(0);
    accumulator += 0.25 * voxel_values[0];
    accumulator += 0.25 * voxel_values[1];
    accumulator += 0.25 * voxel_values[4];
    accumulator += 0.25 * voxel_values[5];
    imageStore(brick_pool_colors, brick_address + ivec3(1,0,1), accumulator);


    // Face Z
    accumulator = vec4(0);
    accumulator += 0.25 * voxel_values[4];
    accumulator += 0.25 * voxel_values[5];
    accumulator += 0.25 * voxel_values[6];
    accumulator += 0.25 * voxel_values[7];
    imageStore(brick_pool_colors, brick_address + ivec3(1,1,2), accumulator);

    // Face Z Neg
    accumulator = vec4(0);
    accumulator += 0.25 * voxel_values[0];
    accumulator += 0.25 * voxel_values[1];
    accumulator += 0.25 * voxel_values[2];
    accumulator += 0.25 * voxel_values[3];
    imageStore(brick_pool_colors, brick_address + ivec3(1,1,0), accumulator);


    // Edges
    accumulator = vec4(0);
    accumulator += 0.5 * voxel_values[0];
    accumulator += 0.5 * voxel_values[1];
    imageStore(brick_pool_colors, brick_address + ivec3(1,0,0), accumulator);

    accumulator = vec4(0);
    accumulator += 0.5 * voxel_values[0];
    accumulator += 0.5 * voxel_values[2];
    imageStore(brick_pool_colors, brick_address + ivec3(0,1,0), accumulator);

    accumulator = vec4(0);
    accumulator += 0.5 * voxel_values[2];
    accumulator += 0.5 * voxel_values[3];
    imageStore(brick_pool_colors, brick_address + ivec3(1,2,0), accumulator);

    accumulator = vec4(0);
    accumulator += 0.5 * voxel_values[3];
    accumulator += 0.5 * voxel_values[1];
    imageStore(brick_pool_colors, brick_address + ivec3(2,1,0), accumulator);

    accumulator = vec4(0);
    accumulator += 0.5 * voxel_values[0];
    accumulator += 0.5 * voxel_values[4];
    imageStore(brick_pool_colors, brick_address + ivec3(0,0,1), accumulator);

    accumulator = vec4(0);
    accumulator += 0.5 * voxel_values[2];
    accumulator += 0.5 * voxel_values[6];
    imageStore(brick_pool_colors, brick_address + ivec3(0,2,1), accumulator);

    accumulator = vec4(0);
    accumulator += 0.5 * voxel_values[3];
    accumulator += 0.5 * voxel_values[7];
    imageStore(brick_pool_colors, brick_address + ivec3(2,2,1), accumulator);

    accumulator = vec4(0);
    accumulator += 0.5 * voxel_values[1];
    accumulator += 0.5 * voxel_values[5];
    imageStore(brick_pool_colors, brick_address + ivec3(2,0,1), accumulator);

    accumulator = vec4(0);
    accumulator += 0.5 * voxel_values[4];
    accumulator += 0.5 * voxel_values[6];
    imageStore(brick_pool_colors, brick_address + ivec3(0,1,2), accumulator);

    accumulator = vec4(0);
    accumulator += 0.5 * voxel_values[6];
    accumulator += 0.5 * voxel_values[7];
    imageStore(brick_pool_colors, brick_address + ivec3(1,2,2), accumulator);

    accumulator = vec4(0);
    accumulator += 0.5 * voxel_values[5];
    accumulator += 0.5 * voxel_values[7];
    imageStore(brick_pool_colors, brick_address + ivec3(2,1,2), accumulator);

    accumulator = vec4(0);
    accumulator += 0.5 * voxel_values[4];
    accumulator += 0.5 * voxel_values[5];
    imageStore(brick_pool_colors, brick_address + ivec3(1,0,2), accumulator);
}
