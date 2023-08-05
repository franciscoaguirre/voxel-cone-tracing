// Requires:
// - uniform sampler3D brickPoolIrradianceX
// - uniform sampler3D brickPoolIrradianceXNeg
// - uniform sampler3D brickPoolIrradianceY
// - uniform sampler3D brickPoolIrradianceYNeg
// - uniform sampler3D brickPoolIrradianceZ
// - uniform sampler3D brickPoolIrradianceZNeg

vec4 getAnisotropicColor(vec3 coordinates, vec3 direction) {
    float weightX = dot(direction, vec3(1, 0, 0));
    float weightY = dot(direction, vec3(0, 1, 0));
    float weightZ = dot(direction, vec3(0, 0, 1));

    // Normalize weights
    float weightSum = abs(weightX) + abs(weightY) + abs(weightZ);
    weightX /= weightSum;
    weightY /= weightSum;
    weightZ /= weightSum;

    vec4 colorX = weightX > 0 ? texture(brickPoolIrradianceX, coordinates) : texture(brickPoolIrradianceXNeg, coordinates);
    vec4 colorY = weightY > 0 ? texture(brickPoolIrradianceY, coordinates) : texture(brickPoolIrradianceYNeg, coordinates);
    vec4 colorZ = weightZ > 0 ? texture(brickPoolIrradianceZ, coordinates) : texture(brickPoolIrradianceZNeg, coordinates);
    return colorX * abs(weightX) + colorY * abs(weightY) + colorZ * abs(weightZ);
}
