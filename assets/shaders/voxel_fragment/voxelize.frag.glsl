#version 460 core

#include "assets/shaders/octree/_helpers.glsl"

layout (location = 0) out vec4 FragColor;

layout (binding = 0, offset = 0) uniform atomic_uint voxelFragmentCount;
uniform layout(binding = 4, r32f) imageBuffer debug;

in VoxelData {
    vec3 position;
    vec3 normal;
    vec2 textureCoordinates;
    float z;
} In;

flat in int frag_dominantAxis;
flat in vec4 frag_aabb;
flat in vec4 trianglePlane;

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
uvec4 calculateVoxelCoordinates(int z) {
    uvec4 temp = uvec4(
        uint(gl_FragCoord.x),
        uint(gl_FragCoord.y),
        z,
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

float findZ(vec2 xyScreenCoordinates);

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
    float voxelZCoordinate = float(voxelDimension) * In.z;
    int flooredVoxelZCoordinate = int(voxelZCoordinate);
    int dfdx = int(dFdx(flooredVoxelZCoordinate)); 
    int dfdy = int(dFdy(flooredVoxelZCoordinate)); 

    discardIfOutsideAabb();
    memoryBarrier();

    uvec4 voxelCoordinates = calculateVoxelCoordinates(int(flooredVoxelZCoordinate));

    uint fragmentListIndex = atomicCounterIncrement(voxelFragmentCount);
    memoryBarrier();

    if (shouldStore) {
        storeVoxelFragment(voxelCoordinates, fragmentListIndex);
    }
    memoryBarrier();

    int side = 0;

    if (abs(dfdx) > 0.0 || abs(dfdy) > 0.0) {
      side = fract(voxelZCoordinate) > 0.5 ? 1 : side;
      side = fract(voxelZCoordinate) < 0.5 ? -1 : side;
      voxelCoordinates = calculateVoxelCoordinates(int(flooredVoxelZCoordinate) + side);

      if (side != 0) {
        fragmentListIndex = atomicCounterIncrement(voxelFragmentCount);
      }
    }

    memoryBarrier();

    if (shouldStore && side != 0) {
        storeVoxelFragment(voxelCoordinates, fragmentListIndex);
    }

    FragColor = vec4(1.0, 1.0, 1.0, 1.0);

    //// Test certain fragment coordinates
    //// gl_FragCoord is by default on center of voxels
    //ivec3 fragCoord = ivec3(floor(gl_FragCoord.xyz));
    ////if (fragCoord.x < 10 && fragCoord.x == fragCoord.y) {
    //if (fragCoord.x == 1 && (fragCoord.y == 1 || fragCoord.y == 0)) {
      //imageStore(debug, fragCoord.y * 6 + 0, vec4(float(side), 0, 0, 0));
      //imageStore(debug, fragCoord.y * 6 + 1, vec4(float(dfdx), 0, 0, 0));
      //imageStore(debug, fragCoord.y * 6 + 2, vec4(float(dfdy), 0, 0, 0));
      //imageStore(debug, fragCoord.y * 6 + 3, vec4(float(voxelCoordinates.x), 0, 0, 0));
      //imageStore(debug, fragCoord.y * 6 + 4, vec4(float(voxelCoordinates.y), 0, 0, 0));
      //imageStore(debug, fragCoord.y * 6 + 5, vec4(float(voxelCoordinates.z), 0, 0, 0));
    //}
}

float findZ(vec2 xyScreenCoordinates) {
  float zClipSpace = zFromPlaneAndPoint(xyScreenCoordinates, trianglePlane, 69.0);

  return (zClipSpace * -1.0 + 1.0) / 2.0;
}
