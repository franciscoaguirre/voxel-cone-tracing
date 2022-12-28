#version 460 core

in vec3 frag_position;
in vec3 frag_normal;
in vec2 frag_texCoordinates;
flat in int frag_dominantAxis;
flat in vec4 frag_aabb;

layout (location = 0) out vec4 FragColor;
layout (pixel_center_integer) in vec4 gl_FragCoord;

layout (binding = 0, offset = 0) uniform atomic_uint voxelFragmentCount;

uniform layout(binding = 0, rgb10_a2ui) uimageBuffer voxelPositions;
uniform layout(binding = 1, rgba8) imageBuffer voxelColors;
uniform layout(binding = 2, rgba8) imageBuffer voxelNormals;

uniform vec3 fallbackColor;
uniform float shininess;
uniform sampler2D textureDiffuse1;
uniform sampler2D bumpTex;
uniform bool hasTexture;
uniform bool hasBump;
uniform int voxelDimension;
uniform bool shouldStore;

void discardIfOutsideAabb() {
    if (frag_position.x < frag_aabb.x || frag_position.y < frag_aabb.y || frag_position.x > frag_aabb.z || frag_position.y > frag_aabb.w) {
        discard;
    }
}

// NOTE: We store voxel coordinates without normalizing them, i.e. they go from 0 to voxelDimension - 1
// We do this to not have to store floating point numbers.
// The octree traversal functions use normalized voxel coordinates, we just normalize them at that point
// by dividing them by voxelDimension.
uvec4 calculateVoxelCoordinates() {
    uvec4 temp = uvec4(
        gl_FragCoord.x,
        gl_FragCoord.y,
        voxelDimension * gl_FragCoord.z,
        0
    );
    uvec4 voxelCoordinates;

    if (frag_dominantAxis == 0) {
        voxelCoordinates.x = voxelDimension - temp.z;
        voxelCoordinates.y = temp.y;
        voxelCoordinates.z = temp.x;
    } else if (frag_dominantAxis == 1) {
        voxelCoordinates.x = temp.x;
        voxelCoordinates.y = voxelDimension - temp.z;
        voxelCoordinates.z = temp.y;
    } else {
        voxelCoordinates.x = temp.x;
        voxelCoordinates.y = temp.y;
        voxelCoordinates.z = voxelDimension - temp.z;
    }

    return voxelCoordinates;
}

void storeVoxelFragment(uvec4 voxelCoordinates, uint fragmentListIndex) {
    vec3 voxelNormal, voxelColor;

    if (hasBump) {
       voxelNormal = texture(bumpTex, frag_texCoordinates).rgb;
    } else {
       voxelNormal = frag_normal;
    }

    if (hasTexture) {
      voxelColor = texture(textureDiffuse1, frag_texCoordinates).rgb;
    } else {
      voxelColor = fallbackColor;
    }

    imageStore(voxelPositions, int(fragmentListIndex), voxelCoordinates);
    imageStore(voxelColors, int(fragmentListIndex), vec4(voxelColor, 0));
    imageStore(voxelNormals, int(fragmentListIndex), vec4(voxelNormal, 0));
}

void main() {
    discardIfOutsideAabb();

    uvec4 voxelCoordinates = calculateVoxelCoordinates();

    uint fragmentListIndex = atomicCounterIncrement(voxelFragmentCount);

    if (shouldStore) {
        storeVoxelFragment(voxelCoordinates, fragmentListIndex);
    }

    FragColor = vec4(1.0, 1.0, 1.0, 1.0);
}
