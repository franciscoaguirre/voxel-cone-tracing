const int NODES_PER_TILE = 8;
const int NODE_FLAG_VALUE = 0x80000000;
const int WORKING_GROUP_SIZE = 64; // Working group size in x
const uvec3 CHILD_OFFSETS[8] = {
    uvec3(0, 0, 0),
    uvec3(1, 0, 0),
    uvec3(0, 1, 0),
    uvec3(1, 1, 0),
    uvec3(0, 0, 1),
    uvec3(1, 0, 1),
    uvec3(0, 1, 1),
    uvec3(1, 1, 1)
};