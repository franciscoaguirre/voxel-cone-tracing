// Requires:
// - uniform uint voxelDimension
// - uniform uint maxOctreeLevel
// - uniform sampler3D brickPoolColors
// - _traversalHelpers
// - _octreeTraversal
// - _brickCoordinates

void correctAlpha(inout vec4 color, in float alphaCorrection);

float brickPoolResolutionf = float(textureSize(brickPoolColors, 0).x);
float brickPoolBrickSize = 3.0 / brickPoolResolutionf;

// Returns values in [0, maxOctreeLevel]
float calculateLod(float coneDiameter) {
    // Could approximate log2 by lines between y = a and y = a + 1
    // Shouldn't this be log2(1 / coneDiameter) + 1 or something similar?
    return max(maxOctreeLevel - log2(1 + coneDiameter * voxelDimension), 0);
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

// rayOrigin should be between 0 and 1
// maxDistance should be max 1
vec4 coneTrace(
    vec3 coneOrigin,
    vec3 coneDirection,
    float coneHalfAngle,
    float maxDistance,
    bool useLighting
) {
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
    //float firstStep = 0;
    float firstStep = voxelSize;

    distanceAlongCone += firstStep;
    while (distanceAlongCone < maxDistance && returnColor.a < 1.0) {
        float coneDiameter = clamp(coneDiameterCoefficient * distanceAlongCone, 0.0009765625, 100.0);
        float lod = calculateLod(coneDiameter);
        // float lod = 6;
        uint octreeLevel = uint(ceil(lod));
        float parentWeight = octreeLevel - lod; // Non-linear, we should approximate the log with many lines

        bool changedOctreeLevel = octreeLevel != previousOctreeLevel;
        if (changedOctreeLevel) {
            // To account for the larger voxelSize in the new level
            sampleStep *= 2; // Increase sampleStep, same as increasing voxelSize by 2
        }

        vec3 queryCoordinates = coneOrigin + distanceAlongCone * coneDirection;
        bool changedNode = steps == 0 || fallsOutsideNode(queryCoordinates, previousNode); // Should be true on first iteration

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
                //break;
                continue;
            }
        } else {
            node = previousNode;
            parentNode = previousParentNode;
        }

        vec3 childVoxelCoordinates = findVoxel(queryCoordinates, node);
        vec4 childColor = texture(brickPoolColors, childVoxelCoordinates);
        float c1 = 1.0;
        float c2 = 0.09;
        float c3 = 0.032;
        float distance = (distanceAlongCone - firstStep) * 29;
        float distanceFactor = c1 + c2 * distance + c3 * distance * distance;
        if (useLighting) {
            childColor.rgb *= clamp(texture(brickPoolPhotons, childVoxelCoordinates).r * photonPower, 0, 1) / distanceFactor;
        }

        correctAlpha(childColor, stepMultiplier);
        vec3 parentVoxelCoordinates = findVoxel(queryCoordinates, parentNode);
        vec4 parentColor = texture(brickPoolColors, parentVoxelCoordinates);
        if (useLighting) {
            parentColor.rgb *= clamp(texture(brickPoolPhotons, parentVoxelCoordinates).r * photonPower, 0, 1) / distanceFactor;
        }
        correctAlpha(parentColor, stepMultiplier * 2); // Step correction

        vec4 newColor = mix(childColor, parentColor, parentWeight); // Quadrilinear interpolation

        // We probably should multiply by newColor.a
        returnColor.rgb = returnColor.rgb * returnColor.a + (1 - returnColor.a) * newColor.rgb;
        returnColor.a += (1 - returnColor.a) * newColor.a;
        // returnColor += (1.0 - returnColor.a) * newColor;

        distanceAlongCone += sampleStep;

        steps++;

        // Prepare for next iteration
        previousOctreeLevel = octreeLevel;
        previousNode = node;
        previousParentNode = parentNode;
        // returnColor.a = clamp(float(texture(brickPoolPhotons, childVoxelCoordinates).r) / 100, 0, 1);
        //float photonIntensity = texture(brickPoolPhotons, childVoxelCoordinates).r * photonPower;
        //returnColor.rgba = vec4(clamp(photonIntensity, 0.0, 1.0));
        // break;
    }

    returnColor.a = min(returnColor.a, 1.0);

    return returnColor;
}

void correctAlpha(inout vec4 color, in float alphaCorrection) {
  const float oldColA = color.a;
  color.a = 1.0 - pow((1.0 - color.a), alphaCorrection);
  // color.rgb *= color.a / clamp(oldColA, 0.0001, 10000.0);
}
