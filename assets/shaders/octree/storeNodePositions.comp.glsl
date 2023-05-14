#version 460 core

#include "./_constants.glsl"
#include "./_helpers.glsl"

layout (local_size_x = WORKING_GROUP_SIZE, local_size_y = 1, local_size_z = 1) in;

uniform layout(binding = 0, rgb10_a2ui) uimageBuffer voxelPositions;
uniform layout(binding = 1, rgb10_a2ui) uimageBuffer nodePositions;
uniform layout(binding = 2, r32ui) uimageBuffer nodePool;

uniform uint octreeLevel;
uniform uint voxelDimension;

#include "./_traversalHelpers.glsl"
#include "./_octreeTraversal.glsl"

void main() {
    uint threadIndex = gl_GlobalInvocationID.x;
    uvec3 voxelFragmentPosition = imageLoad(voxelPositions, int(threadIndex)).xyz;

    float halfNodeSize;
    vec3 nodeCoordinates;
    int nodeID = traverseOctree(
        normalizedFromIntCoordinates(voxelFragmentPosition, float(voxelDimension)),
        octreeLevel,
        nodeCoordinates,
        halfNodeSize
    );

    // TODO: Check if `floor` or `ceil` are better. We don't have a standard.
    // I think floor is okay -Felipe
    uvec3 nodeCoordinatesInteger = uvec3(floor(nodeCoordinates * float(voxelDimension)));

    // TODO: It's overkill to use so many voxel fragments to store this
    // imageStore(nodePositions, nodeID, uvec4(nodeCoordinatesInteger, 1));
    imageStore(nodePositions, nodeID, uvec4(nodeCoordinatesInteger, 0));
}
