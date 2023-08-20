#version 460 core

#include "./_constants.glsl"

layout (local_size_x = WORKING_GROUP_SIZE, local_size_y = 1, local_size_z = 1) in;

uniform layout(binding = 0, r32ui) uimageBuffer levelStartIndices;
uniform layout(binding = 1, r32ui) uimage3D brickPoolColorsRaw;
uniform layout(binding = 2, rgba8) image3D brickPoolColors;

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
		uint rawColor = imageLoad(brickPoolColorsRaw, brickCoordinates + offset).r;
		vec4 color = convR32UIToVec4(rawColor) / 255.0f; // We multiplied by `255.0f` in `write_leaf`
		if (color.a != 0) {
			color.a = 1;
		}
		imageStore(brickPoolColors, brickCoordinates + offset, color);
	}
}
