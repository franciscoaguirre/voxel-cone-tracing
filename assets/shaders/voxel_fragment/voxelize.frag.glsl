#version 460 core

layout (pixel_center_integer) in vec4 gl_FragCoord;
layout (location = 0) out vec4 FragColor;

layout (binding = 0, offset = 0) uniform atomic_uint voxelFragmentCount;

in VoxelData {
    vec3 position;
    vec3 normal;
    vec2 textureCoordinates;
} In;

flat in int frag_dominantAxis;
flat in vec4 frag_aabb;

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
    if (In.position.x < frag_aabb.x || In.position.y < frag_aabb.y || In.position.x > frag_aabb.z || In.position.y > frag_aabb.w) {
        discard;
    }
}

// NOTE: We store voxel coordinates without normalizing them, i.e. they go from 0 to voxelDimension - 1
// We do this to not have to store floating point numbers.
// The octree traversal functions use normalized voxel coordinates, we just normalize them at that point
// by dividing them by voxelDimension.
uvec4 calculateVoxelCoordinates() {
    uvec4 temp = uvec4(
        floor(gl_FragCoord.x),
        floor(gl_FragCoord.y),
        floor(float(voxelDimension) * gl_FragCoord.z),
        0
    );
    uvec4 voxelCoordinates;

    if (frag_dominantAxis == 0) {
        voxelCoordinates.x = temp.z;
        voxelCoordinates.y = temp.y;
        voxelCoordinates.z = temp.x;
    } else if (frag_dominantAxis == 1) {
        voxelCoordinates.x = temp.x;
        voxelCoordinates.y = temp.z;
        voxelCoordinates.z = temp.y;
    } else {
        voxelCoordinates.x = temp.x;
        voxelCoordinates.y = temp.y;
        voxelCoordinates.z = temp.z;
    }

    return voxelCoordinates;
}

void storeVoxelFragment(uvec4 voxelCoordinates, uint fragmentListIndex) {
    vec3 voxelNormal;
    vec4 voxelColor;

    if (hasBump) {
       voxelNormal = texture(bumpTex, In.textureCoordinates).rgb;
    } else {
       voxelNormal = In.normal;
    }

    if (hasTexture) {
      voxelColor = texture(textureDiffuse1, In.textureCoordinates);
    } else {
      voxelColor = vec4(fallbackColor, 1);
    }

    imageStore(voxelPositions, int(fragmentListIndex), voxelCoordinates);
    imageStore(voxelColors, int(fragmentListIndex), voxelColor);
    imageStore(voxelNormals, int(fragmentListIndex), vec4(voxelNormal, 0));
}

void main() {
    discardIfOutsideAabb();

    uvec4 voxelCoordinates = calculateVoxelCoordinates();

    uint fragmentListIndex = atomicCounterIncrement(voxelFragmentCount);
    memoryBarrier();

    if (shouldStore) {
        storeVoxelFragment(voxelCoordinates, fragmentListIndex);
    }

    FragColor = vec4(1.0, 1.0, 1.0, 1.0);
}
