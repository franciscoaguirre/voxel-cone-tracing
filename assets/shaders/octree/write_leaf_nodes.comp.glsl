#version 460 core

#include "./_constants.glsl"
#include "./_helpers.glsl"
#include "./_traversal_helpers.glsl"
#include "./_octree_traversal.glsl"

layout (local_size_x = WORKING_GROUP_SIZE, local_size_y = 1, local_size_z = 1) in;

uniform layout(binding = 0, rgb10_a2ui) uimageBuffer voxelPositions;
uniform layout(binding = 1, rgba8) imageBuffer voxelColors;
uniform layout(binding = 2, r32ui) uimageBuffer nodePoolBrickPointers;
uniform layout(binding = 3, rgba8) image3D brickPoolColors;
uniform layout(binding = 4, r32ui) uimageBuffer nodePool;

uniform uint voxelDimension;
uniform uint octreeLevel;

void storeInLeaf(vec3 voxelPosition, int nodeAddress, vec4 voxelColor, float halfNodeSize, vec3 nodeCoordinates) {
    uint brickCoordinatesCompact = imageLoad(nodePoolBrickPointers, nodeAddress).r;
    memoryBarrier();
    
    // TODO: Why store the brick coordinates in a texture and not calculate them
    // each time? Why is it non-deterministic which brick coordinates a node will
    // get?
    ivec3 brickCoordinates = ivec3(uintXYZ10ToVec3(brickCoordinatesCompact));

    // NOTE: We find out which subsection the current voxel occupies inside the node
    // Remember leaves don't have nodes, so leaf bricks effectively have 2x2x2 voxels.
    bvec3 subsection = calculate_node_subsection(nodeCoordinates, halfNodeSize, voxelPosition);
    uint offset = uint(subsection[0]) + uint(subsection[1]) * 2 + uint(subsection[2]) * 4;

    imageStore(
        brickPoolColors,
        brickCoordinates + 2 * ivec3(CHILD_OFFSETS[offset]),
        voxelColor
    );
}

void main() {
    // Get voxel attributes from voxel fragment list
    const uint threadIndex = gl_GlobalInvocationID.x;
    // We need to traverse the tree to get the node because we
    // need the voxel attributes (color, normal, etc)
    uvec4 voxelPosition = imageLoad(voxelPositions, int(threadIndex));
    vec4 voxelColor = imageLoad(voxelColors, int(threadIndex));
    // TODO: Load normal from images
    memoryBarrier();

    vec3 normalizedVoxelPosition = vec3(voxelPosition) / float(voxelDimension);

    uint _tileIndex; // Unused
    float halfNodeSize;
    vec3 nodeCoordinates;
    // We send the voxel position to traverse the octree and find the leaf
    int nodeAddress = traverse_octree_returning_node_coordinates(
        normalizedVoxelPosition,
        octreeLevel,
        nodePool,
        halfNodeSize,
        nodeCoordinates,
        _tileIndex
    );

    // TODO: We're missing voxel normals here to store in the leaves
    // For some reason we are sending a vec3 instead of a vec4
    storeInLeaf(normalizedVoxelPosition, nodeAddress, voxelColor, halfNodeSize, nodeCoordinates);
}