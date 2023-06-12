#version 460 core

#include "./_constants.glsl"

layout (local_size_x = WORKING_GROUP_SIZE, local_size_y = 1, local_size_z = 1) in;

uniform layout(binding = 0, r32ui) uimage3D brickPoolPhotons;

uniform uint voxelDimension;

#include "./_helpers.glsl"
#include "assets/shaders/octree/_brickCoordinates.glsl"

void main() {
    int nodeID = int(gl_GlobalInvocationID.x);
    ivec3 brickCoordinates = calculateBrickCoordinates(nodeID);
	uvec4 clearPhotons = uvec4(0);
    for (uint x = 0; x < 3; x++) {
        for (uint y = 0; y < 3; y++) {
            for (uint z = 0; z < 3; z++) {
                imageStore(brickPoolPhotons, brickCoordinates + ivec3(x, y, z), clearPhotons);
            }
        }
    }
}
