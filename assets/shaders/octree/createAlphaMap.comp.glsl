/// `createAlphaMap.comp.glsl`
/// Creates a new texture from `brickPoolColors`, that has only the alpha values.

#version 460 core

#include "./_constants.glsl"

layout (local_size_x = WORKING_GROUP_SIZE, local_size_y = 1, local_size_z = 1) in;

uniform layout(binding = 0, r32ui) uimageBuffer levelStartIndices;
uniform layout(binding = 1, rgba8) image3D brickPoolColors;
uniform layout(binding = 2, rgba8) image3D brickPoolAlpha;

uniform uint octreeLevel;

#include "./_helpers.glsl"
#include "./_threadNodeUtil.glsl"
#include "./_brickCoordinates.glsl"

void main() {
	int nodeID = getThreadNode();

	if (nodeID == NODE_NOT_FOUND) {
		return;
	}

	ivec3 brickCoordinates = calculateBrickCoordinates(nodeID);
	for (uint corner = 0; corner < CHILD_OFFSETS.length(); corner++) {
		ivec3 offset = 2 * ivec3(CHILD_OFFSETS[corner]);
		vec4 color = imageLoad(brickPoolColors, brickCoordinates + offset);
		// imageStore(brickPoolAlpha, brickCoordinates + offset, vec4(0, 0, 0, color.a));
    // TODO: revert before merging
		imageStore(brickPoolAlpha, brickCoordinates + offset, color);
  }
}
