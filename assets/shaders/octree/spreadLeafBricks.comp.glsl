#version 460 core

#include "./_constants.glsl"

layout (local_size_x = WORKING_GROUP_SIZE, local_size_y = 1, local_size_z = 1) in;

uniform layout(binding = 0, rgba8) image3D brickPoolValues;
uniform layout(binding = 1, r32ui) readonly uimageBuffer levelStartIndices;

uniform uint octreeLevel;
uniform uint voxelDimension;

#include "./_helpers.glsl"
#include "./_threadNodeUtil.glsl"
#include "./_averageHelpers.glsl"
#include "assets/shaders/octree/_brickCoordinates.glsl"

vec4[8] loadVoxelValues(in ivec3 brickAddress) {
  vec4 voxelValues[8];
  // accumulatorlect the original voxel accumulatorors (from voxelfragmentlist-voxels)
  // which were stored at the corners of the brick texture.
  for(int i = 0; i < 8; ++i) {
    voxelValues[i] = imageLoad(brickPoolValues, 
                               brickAddress + 2 * ivec3(CHILD_OFFSETS[i]));
  }

  return voxelValues;
}

void main() {
    int nodeAddress = getThreadNode();

    if (nodeAddress == NODE_NOT_FOUND) {
        return;
    }

    ivec3 brickAddress = calculateBrickCoordinates(nodeAddress);

    vec4[] voxelValues = loadVoxelValues(brickAddress);

    vec4 average = averageHandlingEmpty(voxelValues);

    imageStore(brickPoolValues, brickAddress + ivec3(1, 1, 1), average);

    // Neg and Pos X
    for (int i = 0; i <= 1; i++) {
      vec4[] valuesToAverage = {
        voxelValues[0 + i],
        voxelValues[2 + i],
        voxelValues[4 + i],
        voxelValues[6 + i]
      };
      vec4 average = averageHandlingEmpty(valuesToAverage);
      imageStore(brickPoolValues, brickAddress + ivec3(i * 2,1,1), average);
    }

    // Neg and Pos Y
    for (int i = 0; i <= 1; i++) {
      vec4[] valuesToAverage = {
        voxelValues[0 + i],
        voxelValues[1 + i],
        voxelValues[4 + i * 2],
        voxelValues[5 + i * 2]
      };
      vec4 average = averageHandlingEmpty(valuesToAverage);
      imageStore(brickPoolValues, brickAddress + ivec3(1,i * 2,1), average);
    }

    // Neg and Pos Z
    for (int i = 0; i <= 1; i++) {
      vec4[] valuesToAverage = {
        voxelValues[0 + i * 4],
        voxelValues[1 + i * 4],
        voxelValues[2 + i * 4],
        voxelValues[3 + i * 4]
      };
      vec4 average = averageHandlingEmpty(valuesToAverage);
      imageStore(brickPoolValues, brickAddress + ivec3(1,1,i * 2), average);
    }

    // Central edges parallel to z-y plane
    for (int z = 0; z <= 1; z++) {
      for (int y = 0; y <= 1; y++) {
        vec4[] valuesToAverage = { voxelValues[0 + y * 2 + z * 4], voxelValues[1 + y * 2 + z * 4] };
        vec4 average = averageHandlingEmpty(valuesToAverage);
        imageStore(brickPoolValues, brickAddress + ivec3(1,y * 2,z * 2), average);
      }
    }

    for (int z = 0; z <= 1; z++) {
      for (int x = 0; x <= 1; x++) {    
        vec4[] valuesToAverage = { voxelValues[0 + x + 4 * z], voxelValues[2 + x + 4 * z] };
        vec4 average = averageHandlingEmpty(valuesToAverage);
        imageStore(brickPoolValues, brickAddress + ivec3(x * 2,1, z * 2), average);
      }
    }

    for (int y = 0; y <= 1; y++) {
      for (int x = 0; x <= 1; x++) {
        vec4[] valuesToAverage = { voxelValues[0 + x + 2 * y], voxelValues[4 + x + 2 * y] };
        vec4 average = averageHandlingEmpty(valuesToAverage);
        imageStore(brickPoolValues, brickAddress + ivec3(x * 2,y * 2, 1), average);
      }
    }
}
