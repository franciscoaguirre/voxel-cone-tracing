// Requires:
// - uniform uint voxelDimension
// - uniform uint maxOctreeLevel
// - uniform sampler3D brickPoolNormals
// - _anisotropicIrradiance
// - _traversalHelpers
// - _octreeTraversal
// - _brickCoordinates
// if debug
// - uniform (r32ui) nodesQueried
// - uniform atomic_uint queriedNodesCounter
// - uniform (r32f) sampledColor

float brickPoolResolutionf = float(textureSize(brickPoolIrradianceX, 0).x);
float brickPoolBrickSize = 3.0 / brickPoolResolutionf;

// Returns values in [0, maxOctreeLevel]
float calculateLod(float coneDiameter) {
    // Could approximate log2 by lines between y = a and y = a + 1
    // Shouldn't this be log2(1 / coneDiameter) + 1 or something similar?
    return clamp(log2(1 / coneDiameter) - 1, 0, maxOctreeLevel);
}

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

vec4 getLeafIrradiance(vec3 voxelCoordinates) {
    return texture(brickPoolIrradianceX, voxelCoordinates);
}

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
    // NOTE: I removed this from here in favor of moving the origin for the different type of cones outside of this function.
    vec3 offsetedConeOrigin = coneOrigin;// + coneDirection * voxelSize; // * 2.0;
    while (distanceAlongCone < maxDistance && returnColor.a < 0.99) {
        float coneDiameter = clamp(coneDiameterCoefficient * distanceAlongCone, 0.0009765625, 100.0);
        float lod = calculateLod(coneDiameter);
        uint octreeLevel = uint(ceil(lod));
        float parentWeight = octreeLevel - lod; // Non-linear, we should approximate the log with many lines

        bool changedOctreeLevel = octreeLevel != previousOctreeLevel;

        vec3 queryCoordinates = offsetedConeOrigin + distanceAlongCone * coneDirection;
        if(isOutsideRange(queryCoordinates, vec3(0), vec3(1))) {
          break;
        }
        bool changedNode = steps == 0 || fallsOutsideNode(queryCoordinates, previousNode); // Should be true on first iteration

        Node node, parentNode;
        if (changedNode || changedOctreeLevel) {
            traverseOctree(
                queryCoordinates,
                octreeLevel,
                node,
                parentNode
            ); // TODO: We are visiting the same node twice for some reason

            if (changedOctreeLevel) {
                // To account for the larger voxelSize in the new level
                //sampleStep *= 2; // Increase sampleStep, same as increasing voxelSize by 2
                sampleStep = (1 / pow(2, octreeLevel + 1));
            }
            if (node.id == NODE_NOT_FOUND) {
                distanceAlongCone += sampleStep;
                continue;
            }
            #if debug
                int nodesCount = int(atomicCounterIncrement(queriedNodesCounter));
                imageStore(nodesQueried, nodesCount, uvec4(uint(node.id), 0, 0, 0));
            #endif
        } else {
            node = previousNode;
            parentNode = previousParentNode;
        }

        float c1 = 1.0;
        float c2 = 0.09;
        float c3 = 0.032;
        float distance = distanceAlongCone;
        float distanceFactor = c1 + c2 * distance + c3 * distance * distance;

        vec3 childVoxelCoordinates = findVoxel(queryCoordinates, node);
        vec3 parentVoxelCoordinates = findVoxel(queryCoordinates, parentNode);
        vec4 childColor;
        vec4 parentColor;
        if (octreeLevel == maxOctreeLevel) {
            childColor = getLeafIrradiance(childVoxelCoordinates);
        } else {
            childColor = getAnisotropicIrradiance(childVoxelCoordinates, coneDirection);
        }
        #if debug
            int aux = 5;
            imageStore(sampledColor, steps * aux + 0 + 5, vec4(childColor.r, 0, 0, 0));
            imageStore(sampledColor, steps * aux + 1 + 5, vec4(childColor.g, 0, 0, 0));
            imageStore(sampledColor, steps * aux + 2 + 5, vec4(childColor.b, 0, 0, 0));
            imageStore(sampledColor, steps * aux + 3 + 5, vec4(childColor.a, 0, 0, 0));
            imageStore(sampledColor, steps * aux + 4 + 5, vec4(octreeLevel, 0, 0, 0));
        #endif
        parentColor = getAnisotropicIrradiance(parentVoxelCoordinates, coneDirection);

        vec4 newColor = mix(childColor, parentColor, parentWeight); // Quadrilinear interpolation
        newColor.rgb /= distanceFactor;

        returnColor += (1 - returnColor.a) * newColor;

        // Prepare for next iteration
        distanceAlongCone += sampleStep;
        steps++;
        previousOctreeLevel = octreeLevel;
        previousNode = node;
        previousParentNode = parentNode;
    }

    #if debug
        imageStore(sampledColor, 0, vec4(returnColor.r, 0, 0, 0));
        imageStore(sampledColor, 1, vec4(returnColor.g, 0, 0, 0));
        imageStore(sampledColor, 2, vec4(returnColor.b, 0, 0, 0));
        imageStore(sampledColor, 3, vec4(returnColor.a, 0, 0, 0));
        // imageStore(sampledColor, 4, vec4(float(octreeLevel), 0, 0, 0));
    #endif

    if(returnColor.a >= 0.97) {
      returnColor.a = 1;
    }
    returnColor.a = min(returnColor.a, 1.0);

    #if debug
        imageStore(sampledColor, 4, vec4(returnColor.a, 0, 0, 0));
    #endif

    return returnColor;
}
