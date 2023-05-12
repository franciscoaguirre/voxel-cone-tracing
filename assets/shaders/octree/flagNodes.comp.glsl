#version 460 core

#include "./_constants.glsl"

layout (local_size_x = WORKING_GROUP_SIZE, local_size_y = 1, local_size_z = 1) in;

uniform uint octreeLevel;
uniform uint numberOfVoxelFragments;
uniform uint voxelDimension;

uniform layout(binding = 0, rgb10_a2ui) uimageBuffer voxelPositions;
uniform layout(binding = 1, r32ui) uimageBuffer nodePool;

#include "./_traversalHelpers.glsl"
#include "./_octreeTraversal.glsl"

void main()
{
    const uint threadIndex = gl_GlobalInvocationID.x;

    if (threadIndex >= numberOfVoxelFragments) {
        return;
    }

    uvec4 voxelFragmentPosition = imageLoad(voxelPositions, int(threadIndex));
    
    vec3 normalizedFragmentPosition = vec3(voxelFragmentPosition) / float(voxelDimension);
    
    vec3 nodeCoordinates;
    float halfNodeSize;
    int nodeID = traverseOctree(
        normalizedFragmentPosition,
        octreeLevel,
        nodeCoordinates,
        halfNodeSize
    );
    uint childLocalID = calculateChildLocalID(nodeCoordinates, halfNodeSize, normalizedFragmentPosition);
    uint childGlobalID = nodeID * CHILDREN_PER_NODE + childLocalID;

    uint nodePoolValue = imageLoad(nodePool, int(childGlobalID)).r;

    if (nodePoolValue == 0) {
        imageStore(nodePool, int(childGlobalID), uvec4(NODE_FLAG_VALUE, 0, 0, 0));
    }
}
