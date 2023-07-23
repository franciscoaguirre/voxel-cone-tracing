#version 460 core

#include "./_constants.glsl"

layout (local_size_x = WORKING_GROUP_SIZE, local_size_y = 1, local_size_z = 1) in;

uniform layout(binding = 0, rgba8) image3D brickPoolValues;
uniform layout(binding = 1, r32ui) readonly uimageBuffer levelStartIndices;

uniform uint octreeLevel;
uniform uint voxelDimension;

#include "./_helpers.glsl"
#include "./_threadNodeUtil.glsl"
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

    vec4 average = vec4(0);
    for (int i = 0; i < 8; i++) {
      average += 0.125 * voxelValues[i];
    }
    imageStore(brickPoolValues, brickAddress + ivec3(1, 1, 1), average);

    // Faces X: Left and right
    for (int i = 0; i <= 1; i++) {
      vec4 farBottom = voxelValues[0 + i];
      vec4 farTop = voxelValues[2 + i];
      vec4 nearBottom = voxelValues[4 + i];
      vec4 nearTop = voxelValues[6 + i];
      vec4 average = 0.25 * (farBottom + farTop + nearBottom + nearTop);
      imageStore(brickPoolValues, brickAddress + ivec3(i * 2, 1, 1), average);
    }

    // Faces Y: Bottom and top
    for (int i = 0; i <= 1; i++) {
      vec4 farLeft = voxelValues[0 + i * 2];
      vec4 farRight = voxelValues[1 + i * 2];
      vec4 nearLeft = voxelValues[4 + i * 2];
      vec4 nearRight = voxelValues[5 + i * 2];
      vec4 average = 0.25 * (farLeft + farRight + nearLeft + nearRight);
      imageStore(brickPoolValues, brickAddress + ivec3(1, i * 2, 1), average);
    }

    // Faces Z: Far and near
    for (int i = 0; i <= 1; i++) {
      vec4 bottomLeft = voxelValues[0 + i * 4];
      vec4 bottomRight = voxelValues[1 + i * 4];
      vec4 topLeft = voxelValues[2 + i * 4];
      vec4 topRight = voxelValues[3 + i * 4];
      vec4 average = 0.25 * (bottomLeft + bottomRight + topLeft + topRight);
      imageStore(brickPoolValues, brickAddress + ivec3(1, 1, i * 2), average);
    }

    // Edges perpendicular to z-y plane
    for (int z = 0; z <= 1; z++) {
      for (int y = 0; y <= 1; y++) {
        vec4 leftVoxel = voxelValues[0 + y * 2 + z * 4];
        vec4 rightVoxel = voxelValues[1 + y * 2 + z * 4];
        vec4 average = 0.5 * (leftVoxel * rightVoxel);
        imageStore(brickPoolValues, brickAddress + ivec3(1, y * 2, z * 2), average);
      }
    }

    // Edges perpendicular to z-x plane
    for (int z = 0; z <= 1; z++) {
      for (int x = 0; x <= 1; x++) {
        vec4 bottomVoxel = voxelValues[x + 0 + z * 4];
        vec4 topVoxel = voxelValues[x + 2 + z * 4];
        vec4 average = 0.5 * (bottomVoxel + topVoxel);
        imageStore(brickPoolValues, brickAddress + ivec3(x * 2, 1, z * 2), average);
      }
    }

    // Edges perpendicular to y-x plane
    for (int y = 0; y <= 1; y++) {
      for (int x = 0; x <= 1; x++) {
        vec4 farVoxel = voxelValues[x + y * 2 + 0];
        vec4 nearVoxel = voxelValues[x + y * 2 + 4];
        vec4 average = 0.5 * (farVoxel + nearVoxel);
        imageStore(brickPoolValues, brickAddress + ivec3(x * 2, y * 2, 1), average);
      }
    }
}
