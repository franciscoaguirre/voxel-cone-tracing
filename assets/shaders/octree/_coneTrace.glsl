// Requires:
// - uniform uint voxelDimension
// - uniform uint maxOctreeLevel
// - uniform sampler3D brickPoolColors
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

float findVoxelOcclusion(vec3 queryCoordinates, Node node) {
    vec3 brickCoordinates = calculateBrickCoordinates(node.id) / (brickPoolResolution - 1.0);
    // offset between 0 and 1
    vec3 brickOffset = calculateNormalizedBrickVoxel(node.coordinates, node.halfNodeSize, queryCoordinates);
    // offset between 0 and brickPoolColors normalized brick size
    vec3 brickOffsetColors = brickOffset * brickPoolBrickSize;
    vec4 color = texture(brickPoolColors, brickCoordinates + brickOffsetColors);
    return color.a;
}

// Linearly interpolates a and b with weight applying to a and (1 - weight) to b
float interpolate(float a, float b, float weight) {
    return a * (1 - weight) + b * weight;
}

bool fallsOutsideNode(vec3 queryCoordinates, Node node) {
    bool fallsOutsideX = (
        queryCoordinates.x > node.coordinates.x ||
            queryCoordinates.x < node.coordinates.x
    );
    return fallsOutsideX;
}

// rayOrigin should be between 0 and 1
// maxDistance should be max 1
float ambientOcclusion(vec3 coneOrigin, vec3 coneDirection, float coneHalfAngle, float maxDistance) {
    float totalOcclusion = 0.0;
    uint previousOctreeLevel = maxOctreeLevel;
    float voxelSize = 1.0 / float(voxelDimension);
    float stepMultiplier = 1.0 / 3.0;
    float sampleStep = voxelSize * stepMultiplier;
    float coneDiameterCoefficient = 2 * tan(coneHalfAngle);
    float distanceAlongCone = 0.0;
    Node previousNode = Node(0, vec3(0), 0.0);
    Node previousParentNode;
    int steps = 0;

    // distanceAlongCone += voxelSize;
    while (distanceAlongCone < maxDistance && totalOcclusion < 1.0) {
        float coneDiameter = coneDiameterCoefficient * distanceAlongCone;
        // float lod = calculateLod(coneDiameter);
        float lod = 7;
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

        float childOcclusion = findVoxelOcclusion(queryCoordinates, node);
        float parentOcclusion = findVoxelOcclusion(queryCoordinates, parentNode);

        float occlusion = interpolate(childOcclusion, parentOcclusion, parentWeight); // Quadrilinear interpolation
        // occlusion = 1.0 - pow((1.0 - occlusion), stepMultiplier); // Step correction
        totalOcclusion += (1 - totalOcclusion) * occlusion;

        distanceAlongCone += sampleStep;

        steps++;

        // Prepare for next iteration
        previousOctreeLevel = octreeLevel;
        previousNode = node;
        previousParentNode = parentNode;

        break;
    }

    return max(1.0 - totalOcclusion, 0.0);
}

// vec4 coneTrace(vec3 rayOrigin, vec3 rayDirection, float maxDistance) {
//     vec4 result = vec4(0);
//     float coneHalfAngle = PI / 3;
//     float voxelSize = float(1) / voxelDimension;
//     float sampleStep = voxelSize / 2; // ???
//     float sampleFactor = 1; // ??? De a cuanto avanzamos
//     float t = 0;

//     vec4 totalColor = 0;
//     float totalAlpha = 0;
//     for (int i = 0; i < steps && totalAlpha < 1.0; i++) {
//         // sampleStep = sampleStep * sampleFactor;
//         float coneDiameter = h(sampleStep);
//         uint octreeLevel = f(coneDiameter);
//         int nodeID = traverseOctree(coordinates, octreeLevel);
//         vec4 color = imageLoad(brickPoolColors, nodeID);
//         uint photonCount = imageLoad(brickPoolPhotons, nodeID).r;
//         float occlusion = color.a;

//         // Quick math
//         totalColor = color * radiance; //something
//         totalAlpha = (1 - occlusion) + totalAlpha; //something
//     }

//     return totalColor;
// }
