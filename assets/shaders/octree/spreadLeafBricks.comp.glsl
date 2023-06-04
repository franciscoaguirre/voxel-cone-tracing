#version 460 core

#include "./_constants.glsl"

layout (local_size_x = WORKING_GROUP_SIZE, local_size_y = 1, local_size_z = 1) in;

uniform layout(binding = 0, rgba8) image3D brickPoolValues;
uniform layout(binding = 1, r32ui) readonly uimageBuffer levelStartIndices;

uniform uint octreeLevel;

#include "./_helpers.glsl"
#include "./_threadNodeUtil.glsl"

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

    vec4 accumulator = vec4(0);

    // Load center voxel
    for (int i = 0; i < 8; i++) {
       accumulator += voxelValues[i];
    }
    imageStore(brickPoolValues, brickAddress + ivec3(1, 1, 1), accumulator);

    // Neg and Pos X
    for(int i = 0; i <= 1; i++) {
      accumulator = vec4(0);
      accumulator += voxelValues[0 + i];
      accumulator += voxelValues[2 + i];
      accumulator += voxelValues[4 + i];
      accumulator += voxelValues[6 + i];
      imageStore(brickPoolValues, brickAddress + ivec3(i * 2,1,1), accumulator);
    }

    // Neg and Pos Y
    for(int i = 0; i <= 1; i++) {
      accumulator = vec4(0);
      accumulator += voxelValues[0 + i];
      accumulator += voxelValues[1 + i];
      accumulator += voxelValues[4 + i * 2];
      accumulator += voxelValues[5 + i * 2];
      imageStore(brickPoolValues, brickAddress + ivec3(1,i * 2,1), accumulator);
    }

    // Neg and Pos Z
    for(int i = 0; i <= 1; i++) {
      accumulator = vec4(0);
      accumulator += voxelValues[0 + i * 4];
      accumulator += voxelValues[1 + i * 4];
      accumulator += voxelValues[2 + i * 4];
      accumulator += voxelValues[3 + i * 4];
      imageStore(brickPoolValues, brickAddress + ivec3(1,1,i * 2), accumulator);
    }

    // Central edges parallel to z-y plane
    for(int z = 0; z <= 1; z++) {
      for(int y = 0; y <= 1; y++) {
        accumulator = vec4(0);
        accumulator += voxelValues[0 + y * 2 + z * 4];
        accumulator += voxelValues[1 + y * 2 + z * 4];
        imageStore(brickPoolValues, brickAddress + ivec3(1,y * 2,z * 2), accumulator);
      }
    }

    for(int z = 0; z <= 1; z++) {
      for(int x = 0; x <= 1; x++) {
        accumulator = vec4(0);
        accumulator += voxelValues[0 + x + 4 * z];
        accumulator += voxelValues[2 + x + 4 * z];
        imageStore(brickPoolValues, brickAddress + ivec3(x * 2,1, z * 2), accumulator);
      }
    }

    for(int y = 0; y <= 1; y++) {
      for(int x = 0; x <= 1; x++) {
        accumulator = vec4(0);
        accumulator += voxelValues[0 + x + 2 * y];
        accumulator += voxelValues[4 + x + 2 * y];
        imageStore(brickPoolValues, brickAddress + ivec3(x * 2,y * 2, 1), accumulator);
      }
    }
}
