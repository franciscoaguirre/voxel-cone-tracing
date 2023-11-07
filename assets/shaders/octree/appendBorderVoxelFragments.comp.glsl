#version 460 core

#include "./_constants.glsl"

layout (local_size_x = WORKING_GROUP_SIZE, local_size_y = 1, local_size_z = 1) in;

uniform layout(binding = 0, rgb10_a2ui) writeonly uimageBuffer borderVoxelFragments;
uniform layout(binding = 1, rgb10_a2ui) readonly uimageBuffer nodePositions;
uniform layout(binding = 2, r32ui) readonly uimageBuffer nodePoolNeighbors[3];
uniform layout(binding = 7, r32f) imageBuffer debug;

uniform layout(binding = 0, offset = 0) atomic_uint nextVoxelFragmentCounter;

uniform uint octreeLevel;
uniform uint maxOctreeLevel;
uniform uint callOffset;
uniform uint voxelDimension;
uniform uint levelStart;
uniform uint nextLevelStart;

const ivec4 NEIGHBOR_OFFSETS[6] = {
    ivec4(2, 0, 0, 0),
    ivec4(-2, 0, 0, 0),
    ivec4(0, 2, 0, 0),
    ivec4(0, -2, 0, 0),
    ivec4(0, 0, 2, 0),
    ivec4(0, 0, -2, 0)
};

// Maps to positions in NEIGHBOR_OFFSETS
// 0 maps to X+ in NEIGHBOR_OFFSETS, 
// Here 0 maps to 2, that represents the start of the Y directions (Y+ and Y-)
// So X+ is responsible to add the voxel fragments for X+Y+, X+Y-
const uint DIAGONAL_DIRECTION_MAPPING[6] = {
    2,
    2,
    3,
    3,
    0,
    0
};

#include "./_helpers.glsl"
#include "./_threadNodeUtilNoTexture.glsl"

void save(uvec4 borderVoxelFragmentPosition) {
    if (
        !isOutsideRange(borderVoxelFragmentPosition.xyz, ivec3(0), ivec3(voxelDimension - 1))
    ) {
      uint nextVoxelFragment = atomicCounterIncrement(nextVoxelFragmentCounter);
      imageStore(borderVoxelFragments, int(nextVoxelFragment), borderVoxelFragmentPosition);

      //imageStore(debug, int(nextVoxelFragment * 3 + 3), vec4(float(borderVoxelFragmentPosition.x), 0, 0, 0));
      //imageStore(debug, int(nextVoxelFragment * 3 + 4), vec4(float(borderVoxelFragmentPosition.y), 0, 0, 0));
      //imageStore(debug, int(nextVoxelFragment * 3 + 5), vec4(float(borderVoxelFragmentPosition.z), 0, 0, 0));
    }
}

ivec4 getNeighborOffset(uint neighbor) {
  // Should probably be this to do it for all levels
  //return NEIGHBOR_OFFSETS[neighbor] * pow(2, maxOctreeLevel - octreeLevel);
  return NEIGHBOR_OFFSETS[neighbor];
}

void main() {
    int nodeID = getThreadNode();

    if (nodeID != 1) {
        return;
    }

    ivec4 nodePosition = ivec4(imageLoad(nodePositions, nodeID));

    for (uint i = 0; i < 3; i++) {
        uint neighborID = imageLoad(nodePoolNeighbors[i], nodeID).r;
        imageStore(debug, int(callOffset + i), vec4(float(neighborID), 0, 0, 0));

        if (neighborID == 0) {
            uint offest = i + callOffset;
            uvec4 borderVoxelBaseFragmentPosition = ivec4(nodePosition) + getNeighborOffset(offest);
            save(borderVoxelBaseFragmentPosition);
            memoryBarrier();

            //for (uint directionSign = 0; directionSign <= 1; directionSign++) {
              //ivec4 diagonalCoordinates = getNeighborOffset(DIAGONAL_DIRECTION_MAPPING[offest] + directionSign); 
              //save(borderVoxelBaseFragmentPosition + diagonalCoordinates);
              //memoryBarrier();
            //}

            //for (uint directionSignX = 0; directionSignX <= 1; directionSignX++) {
              //ivec4 diagonalCoordinatesX = NEIGHBOR_OFFSETS[directionSignX];
              //for (uint directionSignY = 0; directionSignY <= 1; directionSignY++) {
                //ivec4 diagonalCoordinatesY = NEIGHBOR_OFFSETS[directionSignY + 2]; 

                //if (offest >= 4) {
                  //save(borderVoxelBaseFragmentPosition + diagonalCoordinatesX + diagonalCoordinatesY);
                //}
                //memoryBarrier();
              //}
            //}
        }
    }
}
