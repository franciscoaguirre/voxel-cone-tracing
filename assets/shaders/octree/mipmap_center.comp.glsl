#version 460 core

#include "./_constants.glsl"

layout (local_size_x = WORKING_GROUP_SIZE, local_size_y = 1, local_size_z = 1) in;

uniform layout(binding = 0, r32ui) uimageBuffer nodePool;
uniform layout(binding = 1, r32ui) uimageBuffer nodePoolBrickPointers;
uniform layout(binding = 2, rgba8) image3D brickPoolValues;
uniform layout(binding = 3, rgb10_a2ui) uimageBuffer voxelPositions;
uniform layout(binding = 4, r32f) imageBuffer debugBuffer;

uniform uint voxelDimension;
uniform uint maxOctreeLevel;

#include "./_helpers.glsl"
#include "./_traversal_helpers.glsl"
#include "./_octree_traversal.glsl"
#include "./_mipmapUtil.glsl"

void main() {
    const uint threadIndex = gl_GlobalInvocationID.x;
    uvec4 voxelPosition = imageLoad(voxelPositions, int(threadIndex));
    vec3 normalizedVoxelPosition = vec3(voxelPosition) / float(voxelDimension);

    int nodeAddress = traverse_octree(
        normalizedVoxelPosition,
        maxOctreeLevel,
        nodePool
    );

    ivec3 brickAddress = ivec3(uintXYZ10ToVec3(imageLoad(nodePoolBrickPointers, int(nodeAddress)).r));

    uint childAddress = imageLoad(nodePool, int(nodeAddress)).r * NODES_PER_TILE;
    loadChildTile(int(childAddress));

    vec4 color = mipmapIsotropic(ivec3(2, 2, 2));
    // vec4 color = vec4(1);
    memoryBarrier();

    imageStore(brickPoolValues, brickAddress + ivec3(1, 1, 1), color);
}
