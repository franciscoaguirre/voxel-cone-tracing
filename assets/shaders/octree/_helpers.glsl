uint vec3ToUintXYZ10(uvec3 val) {
    return (uint(val.z) & 0x000003FF)   << 20U
            |(uint(val.y) & 0x000003FF) << 10U 
            |(uint(val.x) & 0x000003FF);
}

uvec3 uintXYZ10ToVec3(uint val) {
    return uvec3(uint((val & 0x000003FF)),
                 uint((val & 0x000FFC00) >> 10U), 
                 uint((val & 0x3FF00000) >> 20U));
}

uint findOctreeLevel(uint nodeID, readonly uimageBuffer levelStartIndices, uint maxOctreeLevel) {
    uint octreeLevel = 0;
    bool foundLevel = false;

    for (uint level = 0; level < maxOctreeLevel; level++) {
        uint levelStartIndex = imageLoad(levelStartIndices, int(level)).r;

        if (levelStartIndex > nodeID) {
            octreeLevel = level - 1;
            foundLevel = true;
            break;
        }

        if (!foundLevel) {
            octreeLevel = maxOctreeLevel - 1;
        }
    }

    return octreeLevel;
}
