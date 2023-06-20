// Requires:
// - uniform uint voxelDimension
// - uniform uint maxOctreeLevel
// - uniform sampler3D brickPoolColors
// - uniform (r32ui) nodesQueried
// - _traversalHelpers
// - _octreeTraversal
// - _brickCoordinates
// - _coneTrace


// rayOrigin should be between 0 and 1
// maxDistance should be max 1
vec4 debugConeTrace(
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
    int nodesCount = 0; // For debugging

    //distanceAlongCone += voxelSize * 2;
    while (distanceAlongCone < maxDistance && returnColor.a < 1.0) {
        float coneDiameter = clamp(coneDiameterCoefficient * distanceAlongCone, 0.0009765625, 1.0);
        float lod = calculateLod(coneDiameter);
        // float lod = 7;
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
                // break;
                continue;
            }
            imageStore(nodesQueried, nodesCount + 1, uvec4(uint(node.id), 0, 0, 0)); // For debugging
            nodesCount += 1; // For debugging
        } else {
            node = previousNode;
            parentNode = previousParentNode;
        }

        vec3 childVoxelCoordinates = findVoxel(queryCoordinates, node);
        vec4 childColor = texture(brickPoolColors, childVoxelCoordinates);
        if (useLighting) {
           // childColor.rgb *= float(texture(brickPoolPhotons, childVoxelCoordinates).r) / 10;
        }
        correctAlpha(childColor, stepMultiplier);
        vec3 parentVoxelCoordinates = findVoxel(queryCoordinates, parentNode);
        vec4 parentColor = texture(brickPoolColors, parentVoxelCoordinates);
        if (useLighting) {
            // parentColor.rgb *= float(texture(brickPoolPhotons, parentVoxelCoordinates).r) / 10;
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
        // break;
    }

    imageStore(nodesQueried, 0, uvec4(nodesCount, 0, 0, 0)); // For debugging
    returnColor.a = min(returnColor.a, 1.0);

    return returnColor;
}
