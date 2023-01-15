#version 460 core

#include "./_constants.glsl"
#include "./_helpers.glsl"

layout (local_size_x = WORKING_GROUP_SIZE, local_size_y = 1, local_size_z = 1) in;

uniform layout(binding = 0, r32ui) readonly uimageBuffer nodePoolBrickPointers;
uniform layout(binding = 1, rgba8) image3D brickPoolValues;
uniform layout(binding = 2, r32ui) readonly uimageBuffer levelStartIndices;

uniform uint octreeLevel;

#include "./_threadNodeUtil.glsl"

vec4[8] loadCornerVoxelValues(ivec3 brickAddress) {
    vec4 voxelValues[8];

    // Get voxels from the corners of the brick
    for (int i = 0; i < 8; i++) {
        voxelValues[i] = imageLoad(
            brickPoolValues,
            brickAddress + 2 * ivec3(CHILD_OFFSETS[i])
        );
    }
    
    return voxelValues;
}

void main() {
    int nodeAddress = getThreadNode();

    if (nodeAddress == NODE_NOT_FOUND) {
        return;
    }

    ivec3 brickAddress = ivec3(
        uintXYZ10ToVec3(
            imageLoad(nodePoolBrickPointers, int(nodeAddress)).r
        )
    ); 

    vec4[] voxelValues = loadCornerVoxelValues(brickAddress);

    vec4 accumulator = vec4(0);

    // Load center voxel
    for (int i = 0; i < 8; i++) {
        accumulator += 0.125 * voxelValues[i];
    }
    imageStore(brickPoolValues, brickAddress + ivec3(1, 1, 1), accumulator);

    // Face X
    accumulator = vec4(0);
    accumulator += 0.25 * voxelValues[1];
    accumulator += 0.25 * voxelValues[3];
    accumulator += 0.25 * voxelValues[5];
    accumulator += 0.25 * voxelValues[7];
    imageStore(brickPoolValues, brickAddress + ivec3(2,1,1), accumulator);

    // Face X Neg
    accumulator = vec4(0);
    accumulator += 0.25 * voxelValues[0];
    accumulator += 0.25 * voxelValues[2];
    accumulator += 0.25 * voxelValues[4];
    accumulator += 0.25 * voxelValues[6];
    imageStore(brickPoolValues, brickAddress + ivec3(0,1,1), accumulator);

    // Face Y
    accumulator = vec4(0);
    accumulator += 0.25 * voxelValues[2];
    accumulator += 0.25 * voxelValues[3];
    accumulator += 0.25 * voxelValues[6];
    accumulator += 0.25 * voxelValues[7];
    imageStore(brickPoolValues, brickAddress + ivec3(1,2,1), accumulator);

    // Face Y Neg
    accumulator = vec4(0);
    accumulator += 0.25 * voxelValues[0];
    accumulator += 0.25 * voxelValues[1];
    accumulator += 0.25 * voxelValues[4];
    accumulator += 0.25 * voxelValues[5];
    imageStore(brickPoolValues, brickAddress + ivec3(1,0,1), accumulator);


    // Face Z
    accumulator = vec4(0);
    accumulator += 0.25 * voxelValues[4];
    accumulator += 0.25 * voxelValues[5];
    accumulator += 0.25 * voxelValues[6];
    accumulator += 0.25 * voxelValues[7];
    imageStore(brickPoolValues, brickAddress + ivec3(1,1,2), accumulator);

    // Face Z Neg
    accumulator = vec4(0);
    accumulator += 0.25 * voxelValues[0];
    accumulator += 0.25 * voxelValues[1];
    accumulator += 0.25 * voxelValues[2];
    accumulator += 0.25 * voxelValues[3];
    imageStore(brickPoolValues, brickAddress + ivec3(1,1,0), accumulator);

    // Edges
    accumulator = vec4(0);
    accumulator += 0.5 * voxelValues[0];
    accumulator += 0.5 * voxelValues[1];
    imageStore(brickPoolValues, brickAddress + ivec3(1,0,0), accumulator);

    accumulator = vec4(0);
    accumulator += 0.5 * voxelValues[0];
    accumulator += 0.5 * voxelValues[2];
    imageStore(brickPoolValues, brickAddress + ivec3(0,1,0), accumulator);

    accumulator = vec4(0);
    accumulator += 0.5 * voxelValues[2];
    accumulator += 0.5 * voxelValues[3];
    imageStore(brickPoolValues, brickAddress + ivec3(1,2,0), accumulator);

    accumulator = vec4(0);
    accumulator += 0.5 * voxelValues[3];
    accumulator += 0.5 * voxelValues[1];
    imageStore(brickPoolValues, brickAddress + ivec3(2,1,0), accumulator);

    accumulator = vec4(0);
    accumulator += 0.5 * voxelValues[0];
    accumulator += 0.5 * voxelValues[4];
    imageStore(brickPoolValues, brickAddress + ivec3(0,0,1), accumulator);

    accumulator = vec4(0);
    accumulator += 0.5 * voxelValues[2];
    accumulator += 0.5 * voxelValues[6];
    imageStore(brickPoolValues, brickAddress + ivec3(0,2,1), accumulator);

    accumulator = vec4(0);
    accumulator += 0.5 * voxelValues[3];
    accumulator += 0.5 * voxelValues[7];
    imageStore(brickPoolValues, brickAddress + ivec3(2,2,1), accumulator);

    accumulator = vec4(0);
    accumulator += 0.5 * voxelValues[1];
    accumulator += 0.5 * voxelValues[5];
    imageStore(brickPoolValues, brickAddress + ivec3(2,0,1), accumulator);

    accumulator = vec4(0);
    accumulator += 0.5 * voxelValues[4];
    accumulator += 0.5 * voxelValues[6];
    imageStore(brickPoolValues, brickAddress + ivec3(0,1,2), accumulator);

    accumulator = vec4(0);
    accumulator += 0.5 * voxelValues[6];
    accumulator += 0.5 * voxelValues[7];
    imageStore(brickPoolValues, brickAddress + ivec3(1,2,2), accumulator);

    accumulator = vec4(0);
    accumulator += 0.5 * voxelValues[5];
    accumulator += 0.5 * voxelValues[7];
    imageStore(brickPoolValues, brickAddress + ivec3(2,1,2), accumulator);

    accumulator = vec4(0);
    accumulator += 0.5 * voxelValues[4];
    accumulator += 0.5 * voxelValues[5];
    imageStore(brickPoolValues, brickAddress + ivec3(1,0,2), accumulator);
}
