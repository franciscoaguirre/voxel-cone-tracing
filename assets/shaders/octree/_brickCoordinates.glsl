int brickPoolResolution = 384; // TODO: Get this from a uniform. No hardcoding.

ivec3 calculateBrickCoordinates(int nodeID) {
    ivec3 coordinates = ivec3(0);
    int brickPoolResolutionBricks = brickPoolResolution / 3;
    coordinates.x = nodeID % brickPoolResolutionBricks;
    coordinates.y = (nodeID / brickPoolResolutionBricks) % brickPoolResolutionBricks;
    coordinates.z = nodeID / (brickPoolResolutionBricks * brickPoolResolutionBricks);
    coordinates *= 3;
    return coordinates;
}
