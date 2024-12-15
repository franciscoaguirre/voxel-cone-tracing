#version 460 core

#include "assets/shaders/octree/_helpers.glsl"

layout (location = 0) out vec4 FragColor;

layout (binding = 0, offset = 0) uniform atomic_uint voxelFragmentCount;

in VoxelData {
    vec3 position;
    vec3 normal;
    vec2 textureCoordinates;
    float z;
} In;

flat in int frag_dominantAxis;
flat in vec4 frag_aabb;

layout(binding = 0, rgb10_a2ui) coherent uniform uimageBuffer voxelPositions;
uniform layout(binding = 1, rgba8) imageBuffer voxelColors;
uniform layout(binding = 2, rgba32f) imageBuffer voxelNormals;

struct Material {
    vec3 color;
    float diffuse;
    float specular;
};
uniform Material material;
uniform bool hasMaterial;

uniform vec3 fallbackColor;
uniform vec3 materialDiffuse;
uniform float shininess;
uniform sampler2D textureDiffuse1;
uniform sampler2D bumpTex;
uniform bool hasTexture;
uniform bool hasDiffuse;
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
        floor(gl_FragCoord.x),
        floor(gl_FragCoord.y),
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
    } else if (hasDiffuse) {
      voxelColor = vec4(materialDiffuse, 1);
    } else if (hasMaterial) {
      voxelColor = vec4(material.color, 1);
    } else {
      voxelColor = vec4(fallbackColor, 1);
    }

    imageStore(voxelPositions, int(fragmentListIndex), voxelCoordinates);
    imageStore(voxelColors, int(fragmentListIndex), voxelColor);
    imageStore(voxelNormals, int(fragmentListIndex), vec4(voxelNormal, 0));
}

void main() {
  // TODO: Which memory barriers do we need
    float voxelZCoordinate = In.z;
    int flooredVoxelZCoordinate = int(floor(voxelZCoordinate));
    float dfdx = dFdx(voxelZCoordinate) / 2.0; 
    float dfdy = dFdy(voxelZCoordinate) / 2.0; 

    discardIfOutsideAabb();
//    memoryBarrier();

    uvec4 voxelCoordinates = calculateVoxelCoordinates(int(flooredVoxelZCoordinate));

    uint fragmentListIndex = atomicCounterIncrement(voxelFragmentCount);
    memoryBarrier();

    if (shouldStore) {
        storeVoxelFragment(voxelCoordinates, fragmentListIndex);
    }
 //   memoryBarrier();

    int zOffset = 0;

    // Triangle is closer to z+1 voxel
    if(fract(voxelZCoordinate) > 0.5) {
      // Check if within this fragment, triangle toples over to z+ voxel
      // TODO: Could it go over both in X and Y?
      if(int(voxelZCoordinate + abs(dfdx)) > flooredVoxelZCoordinate || int(voxelZCoordinate + abs(dfdy)) > flooredVoxelZCoordinate) {
        zOffset = 1;
      } 
    // Triangle is closer to z-1 voxel
    } else if(fract(voxelZCoordinate) < 0.5) {
      // Check if within this fragment, triangle toples over to z- voxel
      if(int(voxelZCoordinate - abs(dfdx)) < flooredVoxelZCoordinate || int(voxelZCoordinate - abs(dfdy)) < flooredVoxelZCoordinate) {
        zOffset = -1;
      } 
    }

    if (zOffset != 0) {
      voxelCoordinates = calculateVoxelCoordinates(int(flooredVoxelZCoordinate) + zOffset);
      fragmentListIndex = atomicCounterIncrement(voxelFragmentCount);
    }

  //  memoryBarrier();

    if (shouldStore && zOffset != 0) {
      storeVoxelFragment(voxelCoordinates, fragmentListIndex);
    }

    FragColor = vec4(1.0, 1.0, 1.0, 1.0);
}
