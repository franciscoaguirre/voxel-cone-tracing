// requires
// - uniform layout(binding = x, r32ui) readonly uimageBuffer levelStartIndices;
// - uniform layout(binding = y, r32ui) readonly uimageBuffer borderLevelStartIndices;

const int NO_NODES_ON_LEVEL = -1;

uint findOctreeLevel(
    uint nodeID,
    uint maxOctreeLevel
) {
    uint octreeLevel = maxOctreeLevel - 1;
    bool foundLevel = false;

    // First try to find it in levelStartIndices
    for (uint level = 0; level < maxOctreeLevel; level++) {
        uint levelStartIndex = imageLoad(levelStartIndices, int(level)).r;

        if (levelStartIndex == NO_NODES_ON_LEVEL) {
            continue;
        }

        if (levelStartIndex > nodeID) {
            octreeLevel = level - 1;
            foundLevel = true;
            break;
        }
    }

    if (!foundLevel) {
        // Try on borderLevelStartIndices
        foundLevel = false;
        for (uint level = 0; level < maxOctreeLevel; level++) {
            uint levelStartIndex = imageLoad(borderLevelStartIndices, int(level)).r;

            if (levelStartIndex == NO_NODES_ON_LEVEL) {
                continue;
            }

            if (levelStartIndex > nodeID) {
                octreeLevel = level - 1;
                foundLevel = true;
                break;
            }
        }
        
        // If not found in either, return last level
        if (!foundLevel) {
            octreeLevel = maxOctreeLevel - 1;
        }
    }

    return octreeLevel;
}
