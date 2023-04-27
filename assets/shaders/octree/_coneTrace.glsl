// Requires:
// - uniform uint voxelDimension

const float PI = 3.14159;

vec4 coneTrace(vec3 rayOrigin, vec3 rayDirection, float coneDiameter, int steps) {
    vec4 result = vec4(0);
    float coneAngle = PI / 3;
    float voxelSize = float(1) / voxelDimension;
    float sampleStep = voxelSize / 2; // ???
    float t = 0;

    for (int i = 0; i < steps; i++) {
        // sampleStep = sampleStep * sampleFactor;
    }
}
