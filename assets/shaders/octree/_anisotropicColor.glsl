// Requires:
// - uniform sampler3D brickPoolColorsX
// - uniform sampler3D brickPoolColorsXNeg
// - uniform sampler3D brickPoolColorsY
// - uniform sampler3D brickPoolColorsYNeg
// - uniform sampler3D brickPoolColorsZ
// - uniform sampler3D brickPoolColorsZNeg

vec4 getAnisotropicColor(vec3 coordinates, vec3 direction) {
    float weightX = dot(direction, vec3(1, 0, 0));
    float weightY = dot(direction, vec3(0, 1, 0));
    float weightZ = dot(direction, vec3(0, 0, 1));

    // Normalize weights
    float weightSum = abs(weightX) + abs(weightY) + abs(weightZ);
    weightX /= weightSum;
    weightY /= weightSum;
    weightZ /= weightSum;

    vec4 colorX = weightX > 0 ? texture(brickPoolColorsX, coordinates) : texture(brickPoolColorsXNeg, coordinates);
    vec4 colorY = weightY > 0 ? texture(brickPoolColorsY, coordinates) : texture(brickPoolColorsYNeg, coordinates);
    vec4 colorZ = weightZ > 0 ? texture(brickPoolColorsZ, coordinates) : texture(brickPoolColorsZNeg, coordinates);
    return colorX * abs(weightX) + colorY * abs(weightY) + colorZ * abs(weightZ);
}
