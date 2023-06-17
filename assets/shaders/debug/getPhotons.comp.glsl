#version 460 core

layout (local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

uniform layout(binding = 0, r32ui) writeonly uimageBuffer photonBuffer;
uniform layout(binding = 1, r32ui) readonly uimage3D brickPoolPhotons;

uniform uint nodeID;
uniform uint voxelDimension;

#include "assets/shaders/octree/_helpers.glsl"
#include "assets/shaders/octree/_brickCoordinates.glsl"

void main() {
    ivec3 brickCoordinates = calculateBrickCoordinates(int(nodeID));
    
    for (int z = 0; z < 3; z++) {
        for (int y = 0; y < 3; y++) {
            for (int x = 0; x < 3; x++) {
                uint photonCount = imageLoad(brickPoolPhotons, brickCoordinates + ivec3(x, y, z)).r;
                imageStore(photonBuffer, z * 3 * 3 + y * 3 + x, uvec4(photonCount, 0, 0, 0));
            }
        }
    }
}
