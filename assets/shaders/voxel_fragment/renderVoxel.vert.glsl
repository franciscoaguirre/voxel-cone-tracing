#version 460 core

out vec4 geom_position;
out vec4 geom_color;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

uniform uint voxelDimension;

uniform layout(binding = 0, rgb10_a2ui) uimageBuffer voxelPositions;
uniform layout(binding = 1, rgba8) imageBuffer voxelColors;

void main() {
    float floatVoxelDimension = float(voxelDimension);

    geom_position = vec4(imageLoad(voxelPositions, gl_VertexID.x).xyz, 1.0);
    // Voxel structure maps points from 0 to 1, transform them from -1 to 1
    geom_position.xyz = (geom_position.xyz / floatVoxelDimension) * 2.0 - vec3(1.0);

    // Move point to middle of voxel (instead of bottom right of voxel)
    float halfPixel = 1.0 / floatVoxelDimension;
    geom_position.xyz = geom_position.xyz + vec3(halfPixel);

    gl_Position = geom_position;
    gl_PointSize = 25.0;
    geom_color = imageLoad(voxelColors, gl_VertexID);
}
