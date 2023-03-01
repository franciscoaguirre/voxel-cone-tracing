#version 460 core

#include "./_constants.glsl"

layout (local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

uniform layout(binding = 0, rgb10_a2ui) uimageBuffer voxelPositions;
uniform layout(binding = 1, rgb10_a2ui) uimageBuffer nodePositions;
uniform layout(binding = 2, r32ui) uimageBuffer nodePool;
uniform layout(binding = 3, r32f) imageBuffer debugBuffer;

uniform uint octreeLevel;
uniform uint voxelDimension;

#include "./_traversalHelpers.glsl"
#include "./_octreeTraversal.glsl"

void main() {
    uint threadIndex = gl_GlobalInvocationID.x;
    vec4 voxelFragmentPosition = imageLoad(voxelPositions, int(threadIndex));

    uint tileIndex;
    float halfNodeSize;
    vec3 nodeCoordinates;
    int nodeIndex = traverseOctreeReturningNodeCoordinates(
        vec3(voxelFragmentPosition) / float(voxelDimension),
        octreeLevel,
        halfNodeSize,
        nodeCoordinates,
        tileIndex
    );

    imageStore(debugBuffer, 0, vec4(float(nodeIndex), 0, 0, 0));
    imageStore(debugBuffer, 1, vec4(float(nodeCoordinates.x), 0, 0, 0));
    imageStore(debugBuffer, 2, vec4(float(nodeCoordinates.y), 0, 0, 0));
    imageStore(debugBuffer, 3, vec4(float(nodeCoordinates.z), 0, 0, 0));

    uvec3 nodeCoordinatesInteger = uvec3(round(nodeCoordinates * float(voxelDimension)));

    imageStore(debugBuffer, 4, vec4(float(nodeCoordinatesInteger.x), 0, 0, 0));
    imageStore(debugBuffer, 5, vec4(float(nodeCoordinatesInteger.y), 0, 0, 0));
    imageStore(debugBuffer, 6, vec4(float(nodeCoordinatesInteger.z), 0, 0, 0));
    // uvec3 nodeCoordinatesInteger = uvec3(44, 44, 44);

    // TODO: It's overkill to use so many voxel fragments to store this
    // imageStore(nodePositions, nodeIndex, uvec4(nodeCoordinatesInteger, 1));
    imageStore(nodePositions, nodeIndex, uvec4(nodeCoordinatesInteger, 0));
}
