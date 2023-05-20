// Requires:
// - uniform uint voxelDimension
// - uniform uint maxOctreeLevel
// - uniform sampler3D brickPoolColors
// - uniform bool useLighting
// - _traversalHelpers
// - _octreeTraversal

const float brickPoolResolution = 384.0;
const float brickPoolBrickSize = 3.0 / brickPoolResolution;

// Returns values in [0, maxOctreeLevel]
float calculateLod(float coneDiameter) {
    // Could approximate log2 by lines between y = a and y = a + 1
    return maxOctreeLevel - log2(1 + coneDiameter * voxelDimension);
}

// Brick marching
//float findVoxelOcclusion(vec3 queryCoordinates, Node node) {
   //ivec3 brickCoordinates = calculateBrickCoordinates(node.id);
   ////ivec3 brickOffset = ivec3(calculateBrickVoxel(node.coordinates, node.halfNodeSize, queryCoordinates));
   //vec3 brickOffset = (queryCoordinates - node.coordinates) / (2.0 * node.halfNodeSize);
   //vec4 color = texture(brickPoolColors, (brickCoordinates / brickPoolResolution + brickOffset * brickPoolBrickSize));
   //return color.a;
//}

vec3 findVoxel(vec3 queryCoordinates, Node node) {
    // offset between 0 and 1
    vec3 normalizedBrickOffset = calculateNormalizedBrickVoxel(node.coordinates, node.halfNodeSize, queryCoordinates);
    // offset between 0.5 and 2.5
    vec3 offset = (normalizedBrickOffset * (2.0 / 3.0) + (1.0 / 6.0)) * 3.0;

    vec3 voxelCoordinates = vec3(calculateBrickCoordinates(node.id)) + offset;
    vec3 normalizedVoxelCoordinates = voxelCoordinates / brickPoolResolution;
    return normalizedVoxelCoordinates;
}

// bool fallsOutsideNode(vec3 queryCoordinates, Node node) {
//     bool fallsOutsideX = (
//         queryCoordinates.x > node.coordinates.x ||
//             queryCoordinates.x < node.coordinates.x
//     );
//     return fallsOutsideX;
// }

// rayOrigin should be between 0 and 1
// maxDistance should be max 1
vec4 coneTrace(vec3 coneOrigin, vec3 coneDirection, float coneHalfAngle, float maxDistance) {
    vec4 returnColor = vec4(0);
    uint previousOctreeLevel = maxOctreeLevel;
    float voxelSize = 1.0 / float(voxelDimension);
    float stepMultiplier = 1.0 / 3.0;
    float sampleStep = voxelSize * stepMultiplier;
    float coneDiameterCoefficient = 2 * tan(coneHalfAngle);
    float distanceAlongCone = 0.0;
    Node previousNode = Node(0, vec3(0), 0.0);
    Node previousParentNode;
    int steps = 0;

    distanceAlongCone += voxelSize;
    while (distanceAlongCone < maxDistance && returnColor.a < 1.0) {
        float coneDiameter = coneDiameterCoefficient * distanceAlongCone;
        float lod = calculateLod(coneDiameter);
        uint octreeLevel = uint(ceil(lod));
        float parentWeight = octreeLevel - lod; // Non-linear, we should approximate the log with many lines

        bool changedOctreeLevel = octreeLevel != previousOctreeLevel;
        if (changedOctreeLevel) {
            // To account for the larger voxelSize in the new level
            sampleStep *= 2; // Increase sampleStep, same as increasing voxelSize by 2
        }

        vec3 queryCoordinates = coneOrigin + distanceAlongCone * coneDirection;
        // bool changedNode = fallsOutsideNode(queryCoordinates, previousNode); // Should be true on first iteration
        bool changedNode = true;

        Node node, parentNode;
        if (changedNode || changedOctreeLevel) {
            traverseOctree(
                queryCoordinates,
                octreeLevel,
                node,
                parentNode
            );
            if (node.id == NODE_NOT_FOUND) {
                distanceAlongCone += sampleStep;
                continue;
            }
        } else {
            node = previousNode;
            parentNode = previousParentNode;
        }

        vec3 childVoxelCoordinates = findVoxel(queryCoordinates, node);
        vec4 childColor = texture(brickPoolColors, childVoxelCoordinates);
        if (useLighting) {
            childColor *= texture(brickPoolPhotons, childVoxelCoordinates) / 1000000;
        }
        childColor.a = 1.0 - pow((1.0 - childColor.a), stepMultiplier); // Step correction
        vec3 parentVoxelCoordinates = findVoxel(queryCoordinates, parentNode);
        vec4 parentColor = texture(brickPoolColors, parentVoxelCoordinates);
        if (useLighting) {
            childColor *= texture(brickPoolPhotons, parentVoxelCoordinates) / 1000000;
        }
        parentColor.a = 1.0 - pow((1.0 - parentColor.a), stepMultiplier); // Step correction

        vec4 newColor = mix(childColor, parentColor, parentWeight); // Quadrilinear interpolation
        returnColor += (1 - returnColor.a) * newColor;

        distanceAlongCone += sampleStep;

        steps++;

        // Prepare for next iteration
        previousOctreeLevel = octreeLevel;
        previousNode = node;
        previousParentNode = parentNode;

        // break;
    }

    returnColor.a = min(returnColor.a, 1.0);

    return returnColor;
}

// TODO: For use later with indirect light
// void correctAlpha(inout vec4 color, in float alphaCorrection) {
//   const float oldColA = color.a;
//   color.a = 1.0 - pow((1.0 - color.a), alphaCorrection);
//   color.xyz *= color.a / clamp(oldColA, 0.0001, 10000.0);
// }
