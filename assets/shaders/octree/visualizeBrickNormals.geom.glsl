#version 460 core

layout (points) in;
layout (line_strip, max_vertices = 256) out;

in VertexData {
	vec4 nodePosition;
	float halfNodeSize;
	ivec3 brickCoordinates;
} In[];

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

uniform uint bricksToShow;

uniform layout(binding = 2, rgba32f) readonly image3D brickPoolNormals;

#include "assets/shaders/octree/_drawNormal.glsl"

void main() {
    vec4 nodePosition = In[0].nodePosition;
    float voxelBrickSize = (In[0].halfNodeSize / 3) * 0.97;
    // So a brick goes fully inside a node, not accurate but works for debugging
    float brickDistance = In[0].halfNodeSize * 0.666;
	ivec3 brickCoordinates = In[0].brickCoordinates;
    vec4 center;
    vec3 start, normal;
    if (bricksToShow == 1) {
        // Show z = 0
        
        // (0, 0, 0)
        center = vec4(nodePosition.xyz - brickDistance, nodePosition.w);
        start = center.xyz;
        normal = imageLoad(brickPoolNormals, brickCoordinates).xyz;
        drawNormal(start, normal);

        // (0, 1, 0)
        center = vec4(nodePosition.x - brickDistance, nodePosition.y, nodePosition.z - brickDistance, nodePosition.w);
        start = center.xyz;
        normal = imageLoad(brickPoolNormals, brickCoordinates + ivec3(0, 1, 0)).xyz;
        drawNormal(start, normal);

        // (0, 2, 0)
        center = vec4(nodePosition.x - brickDistance, nodePosition.y + brickDistance, nodePosition.z - brickDistance, nodePosition.w);
        start = center.xyz;
        normal = imageLoad(brickPoolNormals, brickCoordinates + ivec3(0, 2, 0)).xyz;
        drawNormal(start, normal);

        // (2, 0, 0)
        center = vec4(nodePosition.x + brickDistance, nodePosition.yz - brickDistance, nodePosition.w);
        start = center.xyz;
        normal = imageLoad(brickPoolNormals, brickCoordinates + ivec3(2, 0, 0)).xyz;
        drawNormal(start, normal);

        // (2, 2, 0)
        center = vec4(nodePosition.xy + brickDistance, nodePosition.z - brickDistance, nodePosition.w);
        start = center.xyz;
        normal = imageLoad(brickPoolNormals, brickCoordinates + ivec3(2, 2, 0)).xyz;
        drawNormal(start, normal);

        // (1, 1, 0)
        center = vec4(nodePosition.xy, nodePosition.z - brickDistance, nodePosition.w);
        start = center.xyz;
        normal = imageLoad(brickPoolNormals, brickCoordinates + ivec3(1, 1, 0)).xyz;
        drawNormal(start, normal);

        // (1, 0, 0)
        center = vec4(nodePosition.x, nodePosition.yz - brickDistance, nodePosition.w);
        start = center.xyz;
        normal = imageLoad(brickPoolNormals, brickCoordinates + ivec3(1, 0, 0)).xyz;
        drawNormal(start, normal);

        // (1, 2, 0)
        center = vec4(nodePosition.x, nodePosition.y + brickDistance, nodePosition.z - brickDistance, nodePosition.w);
        start = center.xyz;
        normal = imageLoad(brickPoolNormals, brickCoordinates + ivec3(1, 2, 0)).xyz;
        drawNormal(start, normal);

        // (2, 1, 0)
        center = vec4(nodePosition.x + brickDistance, nodePosition.y, nodePosition.z - brickDistance, nodePosition.w);
        start = center.xyz;
        normal = imageLoad(brickPoolNormals, brickCoordinates + ivec3(2, 1, 0)).xyz;
        drawNormal(start, normal);
    } else if (bricksToShow == 2) {
        // Show z = 1
        
        // (0, 1, 1)
        center = vec4(nodePosition.x - brickDistance, nodePosition.yz, nodePosition.w);
        start = center.xyz;
        normal = imageLoad(brickPoolNormals, brickCoordinates + ivec3(0, 1, 1)).xyz;
        drawNormal(start, normal);

        // (1, 0, 1)
        center = vec4(nodePosition.x, nodePosition.y - brickDistance, nodePosition.z, nodePosition.w);
        start = center.xyz;
        normal = imageLoad(brickPoolNormals, brickCoordinates + ivec3(1, 0, 1)).xyz;
        drawNormal(start, normal);

        // (0, 0, 1)
        center = vec4(nodePosition.xy - brickDistance, nodePosition.z, nodePosition.w);
        start = center.xyz;
        normal = imageLoad(brickPoolNormals, brickCoordinates + ivec3(0, 0, 1)).xyz;
        drawNormal(start, normal);

        // (1, 1, 1)
        center = vec4(nodePosition.xyz, nodePosition.w);
        start = center.xyz;
        normal = imageLoad(brickPoolNormals, brickCoordinates + ivec3(1, 1, 1)).xyz;
        drawNormal(start, normal);

        // (0, 2, 1)
        center = vec4(nodePosition.x - brickDistance, nodePosition.y + brickDistance, nodePosition.z, nodePosition.w);
        start = center.xyz;
        normal = imageLoad(brickPoolNormals, brickCoordinates + ivec3(0, 2, 1)).xyz;
        drawNormal(start, normal);

        // (1, 2, 1)
        center = vec4(nodePosition.x, nodePosition.y + brickDistance, nodePosition.z, nodePosition.w);
        start = center.xyz;
        normal = imageLoad(brickPoolNormals, brickCoordinates + ivec3(1, 2, 1)).xyz;
        drawNormal(start, normal);

        // (2, 1, 1)
        center = vec4(nodePosition.x + brickDistance, nodePosition.yz, nodePosition.w);
        start = center.xyz;
        normal = imageLoad(brickPoolNormals, brickCoordinates + ivec3(2, 1, 1)).xyz;
        drawNormal(start, normal);

        // (2, 0, 1)
        center = vec4(nodePosition.x + brickDistance, nodePosition.y - brickDistance, nodePosition.z, nodePosition.w);
        start = center.xyz;
        normal = imageLoad(brickPoolNormals, brickCoordinates + ivec3(2, 0, 1)).xyz;
        drawNormal(start, normal);

        // (2, 2, 1)
        center = vec4(nodePosition.xy + brickDistance, nodePosition.z, nodePosition.w);
        start = center.xyz;
        normal = imageLoad(brickPoolNormals, brickCoordinates + ivec3(2, 2, 1)).xyz;
        drawNormal(start, normal);

    } else if (bricksToShow == 4) {
        // Show z = 2

        // (0, 0, 2)
        center = vec4(nodePosition.xy - brickDistance, nodePosition.z + brickDistance, nodePosition.w);
        start = center.xyz;
        normal = imageLoad(brickPoolNormals, brickCoordinates + ivec3(0, 0, 2)).xyz;
        drawNormal(start, normal);

        // (1, 0, 2)
        center = vec4(nodePosition.x, nodePosition.y - brickDistance, nodePosition.z + brickDistance, nodePosition.w);
        start = center.xyz;
        normal = imageLoad(brickPoolNormals, brickCoordinates + ivec3(1, 0, 2)).xyz;
        drawNormal(start, normal);

        // (2, 0, 2)
        center = vec4(nodePosition.x + brickDistance, nodePosition.y - brickDistance, nodePosition.z + brickDistance, nodePosition.w);
        start = center.xyz;
        normal = imageLoad(brickPoolNormals, brickCoordinates + ivec3(2, 0, 2)).xyz;
        drawNormal(start, normal);

        // (2, 2, 2)
        center = vec4(nodePosition.xyz + brickDistance, nodePosition.w);
        start = center.xyz;
        normal = imageLoad(brickPoolNormals, brickCoordinates + ivec3(2, 2, 2)).xyz;
        drawNormal(start, normal);

        // (0, 2, 2)
        center = vec4(nodePosition.x - brickDistance, nodePosition.yz + brickDistance, nodePosition.w);
        start = center.xyz;
        normal = imageLoad(brickPoolNormals, brickCoordinates + ivec3(0, 2, 2)).xyz;
        drawNormal(start, normal);

        // (1, 1, 2)
        center = vec4(nodePosition.xy, nodePosition.z + brickDistance, nodePosition.w);
        start = center.xyz;
        normal = imageLoad(brickPoolNormals, brickCoordinates + ivec3(1, 1, 2)).xyz;
        drawNormal(start, normal);

        // (0, 1, 2)
        center = vec4(nodePosition.x - brickDistance, nodePosition.y, nodePosition.z + brickDistance, nodePosition.w);
        start = center.xyz;
        normal = imageLoad(brickPoolNormals, brickCoordinates + ivec3(0, 1, 2)).xyz;
        drawNormal(start, normal);

        // (1, 2, 2)
        center = vec4(nodePosition.x, nodePosition.yz + brickDistance, nodePosition.w);
        start = center.xyz;
        normal = imageLoad(brickPoolNormals, brickCoordinates + ivec3(1, 2, 2)).xyz;
        drawNormal(start, normal);

        // (2, 1, 2)
        center = vec4(nodePosition.x + brickDistance, nodePosition.y, nodePosition.z + brickDistance, nodePosition.w);
        start = center.xyz;
        normal = imageLoad(brickPoolNormals, brickCoordinates + ivec3(2, 1, 2)).xyz;
        drawNormal(start, normal);
    }
}
