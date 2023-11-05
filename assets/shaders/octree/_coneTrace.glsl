// Requires:
// - uniform uint voxelDimension
// - uniform uint maxOctreeLevel
// - _anisotropicColor
// - _anisotropicIrradiance
// - _traversalHelpers
// - _octreeTraversal
// - _brickCoordinates
// if debug
// - uniform (r32ui) nodesQueried
// - uniform atomic_uint queriedNodesCounter
// - uniform (r32f) sampledColor
// - uniform (rgb10_a2ui) queriedCoordinates

float brickPoolResolutionf = float(textureSize(brickPoolColorsX, 0).x);
float brickPoolBrickSize = 3.0 / brickPoolResolutionf;

// Returns values in [0, maxOctreeLevel]
float calculateLod(float coneDiameter) {
    // Could approximate log2 by lines between y = a and y = a + 1
    // Shouldn't this be log2(1 / coneDiameter) + 1 or something similar?
    //return max(maxOctreeLevel - log2(1 + coneDiameter * voxelDimension), 0);
    return clamp(log2(1 / coneDiameter) - 1, 0, maxOctreeLevel);
}

// Brick marching
//float findVoxelOcclusion(vec3 queryCoordinates, Node node) {
   //ivec3 brickCoordinates = calculateBrickCoordinates(node.id);
   ////ivec3 brickOffset = ivec3(calculateBrickVoxel(node.coordinates, node.halfNodeSize, queryCoordinates));
   //vec3 brickOffset = (queryCoordinates - node.coordinates) / (2.0 * node.halfNodeSize);
   //vec4 color = texture(brickPoolColors, (brickCoordinates / brickPoolResolutionf + brickOffset * brickPoolBrickSize));
   //return color.a;
//}

vec3 findVoxel(vec3 queryCoordinates, Node node) {
    // offset between 0 and 1
    vec3 normalizedBrickOffset = calculateNormalizedBrickVoxel(node.coordinates, node.halfNodeSize, queryCoordinates);
    // offset between 0.5 and 2.5
    vec3 offset = (normalizedBrickOffset * (2.0 / 3.0) + (1.0 / 6.0)) * 3.0;

    vec3 voxelCoordinates = vec3(calculateBrickCoordinates(node.id)) + offset;
    vec3 normalizedVoxelCoordinates = voxelCoordinates / brickPoolResolutionf;
    return normalizedVoxelCoordinates;
}

bool fallsOutsideNode(vec3 queryCoordinates, Node node) {
    vec3 range_start = node.coordinates; 
    vec3 range_end = node.coordinates + vec3(node.halfNodeSize * 2);
    return isOutsideRange(queryCoordinates, range_start, range_end);
}

vec4 getLeafColor(vec3 voxelCoordinates) {
    return texture(brickPoolColorsX, voxelCoordinates);
}

// vec4 getLeafIrradiance(vec3 voxelCoordinates) {
//     return texture(brickPoolIrradianceX, voxelCoordinates);
// }

// rayOrigin should be between 0 and 1
// maxDistance should be max 1
vec4 coneTrace(
    vec3 coneOrigin,
    vec3 coneDirection, // Normalized
    float coneHalfAngle,
    float maxDistance
) {
    vec4 returnColor = vec4(0);
    uint previousOctreeLevel = maxOctreeLevel;
    float voxelSize = 1.0 / float(voxelDimension);
    float sampleStep = voxelSize;
    float coneDiameterCoefficient = 2 * tan(coneHalfAngle);
    float distanceAlongCone = 0.0;
    Node previousNode = Node(0, vec3(0), 0.0);
    Node previousParentNode;
    int steps = 0;
    
    // Move the cone origin so it doesn't intersect with own voxels
    vec3 offsetedConeOrigin = coneOrigin + coneDirection * voxelSize * 2;
    while (distanceAlongCone < maxDistance && returnColor.a < 1.0) {
        float coneDiameter = clamp(coneDiameterCoefficient * distanceAlongCone, 0.0009765625, 100.0);
        float lod = calculateLod(coneDiameter);
        uint octreeLevel = uint(ceil(lod));
        float parentWeight = octreeLevel - lod; // Non-linear, we should approximate the log with many lines

        bool changedOctreeLevel = octreeLevel != previousOctreeLevel;
        if (changedOctreeLevel) {
            // To account for the larger voxelSize in the new level
            sampleStep *= 2; // Increase sampleStep, same as increasing voxelSize by 2
        }

        vec3 queryCoordinates = offsetedConeOrigin + distanceAlongCone * coneDirection;
        bool changedNode = steps == 0 || fallsOutsideNode(queryCoordinates, previousNode); // Should be true on first iteration

        Node node, parentNode;
        if (changedNode || changedOctreeLevel) {
            traverseOctree(
                queryCoordinates,
                octreeLevel,
                node,
                parentNode
            ); // TODO: We are visiting the same node twice for some reason
            if (node.id == NODE_NOT_FOUND) {
                distanceAlongCone += sampleStep;
                //break;
                continue;
            }
        } else {
            node = previousNode;
            parentNode = previousParentNode;
        }

        vec3 childVoxelCoordinates = findVoxel(queryCoordinates, node);
        vec3 parentVoxelCoordinates = findVoxel(queryCoordinates, parentNode);
        vec4 childColor;
        vec4 parentColor;
        childColor = getLeafColor(childVoxelCoordinates);
        parentColor = getLeafColor(parentVoxelCoordinates);
        vec4 newColor = mix(childColor, parentColor, parentWeight); // Quadrilinear interpolation

        returnColor += (1 - returnColor.a) * newColor;

        // Prepare for next iteration
        distanceAlongCone += sampleStep;
        steps++;
        previousOctreeLevel = octreeLevel;
        previousNode = node;
        previousParentNode = parentNode;
    }

    // returnColor.a = min(returnColor.a, 1.0);

    return returnColor;
}
