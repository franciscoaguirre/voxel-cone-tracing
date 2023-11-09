#version 460 core

#include "./_constants.glsl"

layout (local_size_x = WORKING_GROUP_SIZE, local_size_y = 1, local_size_z = 1) in;

uniform layout(binding = 0, rgb10_a2ui) writeonly uimageBuffer borderVoxelFragments;
uniform layout(binding = 1, rgb10_a2ui) readonly uimageBuffer nodePositions;
uniform layout(binding = 2, r32ui) readonly uimageBuffer nodePoolNeighbors[HALF_NEIGHBORS];
uniform layout(binding = 5, r32f) imageBuffer debug;

uniform layout(binding = 0, offset = 0) atomic_uint nextVoxelFragmentCounter;

uniform uint offsetTexture;
uniform uint sideOffsetTexture;
uniform uint octreeLevel;
uniform uint maxOctreeLevel;
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
  int curretOctreeLevelMultiplier = int(round(pow(2, maxOctreeLevel - octreeLevel)));
  return NEIGHBOR_OFFSETS[neighbor] * curretOctreeLevelMultiplier;
}

void main() {
    int nodeID = getThreadNode();

    if (nodeID == NODE_NOT_FOUND) {
        return;
    }

    ivec4 nodePosition = ivec4(imageLoad(nodePositions, nodeID));
    //imageStore(debug, int(0), vec4(float(nodePosition.x), 0, 0, 0));
    //imageStore(debug, int(1), vec4(float(nodePosition.y), 0, 0, 0));
    //imageStore(debug, int(2), vec4(float(nodePosition.z), 0, 0, 0));

    uint baseNeighborID = imageLoad(nodePoolNeighbors[0], nodeID).r;
    //imageStore(debug, int(0), vec4(float(baseNeighborID), 0, 0, 0));

    if (baseNeighborID == 0) {
        uvec4 borderVoxelBaseFragmentPosition = ivec4(nodePosition) + getNeighborOffset(offsetTexture);
        save(borderVoxelBaseFragmentPosition);
        memoryBarrier();

        for (uint directionSign = 0; directionSign <= 1; directionSign++) {
          uint sideNeighborID = imageLoad(nodePoolNeighbors[1 + directionSign], nodeID).r;

          // If offsetTexture is 0 it means we are running X+ case. Then we want to posibly get the diagonals for X+Y+ and X+Y-
          // So on directionSign 0 with sideNeighborID we check if Y+ exists as sideOffsetTexture should be 2, if it exists 
          // that node at Y+ will take care of adding the neighbor X+Y+ when the appendBorderVoxel call runs for it
          // if it doesn't exist then this node is responsible of adding that diagonal
          if (sideNeighborID == 0) {
            ivec4 diagonalCoordinates = getNeighborOffset(sideOffsetTexture + directionSign); 
            save(borderVoxelBaseFragmentPosition + diagonalCoordinates);
          }
          memoryBarrier();
        }

    }
}
//
            //for (uint directionSignX = 0; directionSignX <= 1; directionSignX++) {
              //ivec4 diagonalCoordinatesX = NEIGHBOR_OFFSETS[directionSignX];
              //for (uint directionSignY = 0; directionSignY <= 1; directionSignY++) {
                //ivec4 diagonalCoordinatesY = NEIGHBOR_OFFSETS[directionSignY + 2]; 

                //if (offsetTexture >= 4) {
                  //save(borderVoxelBaseFragmentPosition + diagonalCoordinatesX + diagonalCoordinatesY);
                //}
                //memoryBarrier();
              //}
            //}
