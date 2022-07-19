#version 430 core

in vec3 fragment_position;
in vec3 fragment_normal;
in vec2 fragment_texture_coordinates;
flat in int fragment_dominant_axis;
flat in vec4 fragment_aabb;

layout (location = 0) out vec4 FragColor;
layout (pixel_center_integer) in vec4 gl_FragCoord;

layout (binding = 0, offset = 0) uniform atomic_uint voxel_fragment_count;

uniform layout(binding = 0, rgb10_a2ui) uimageBuffer u_voxelPos;
uniform layout(binding = 1, rgba8 ) imageBuffer u_voxelKd;
uniform layout(binding = 2, rgba16f) imageBuffer u_voxelNrml;

uniform vec3 fallback_color;
uniform float u_shininess;
uniform sampler2D texture_diffuse1;
uniform sampler2D u_bumpTex;
uniform bool has_texture;
uniform int u_bBump;
uniform int voxel_dimension;
uniform bool should_store;

void discard_if_outside_aabb() {
    if(fragment_position.x < fragment_aabb.x || fragment_position.y < fragment_aabb.y || fragment_position.x > fragment_aabb.z || fragment_position.y > fragment_aabb.w)
        discard;
}

uvec4 calculate_texture_coordinates() {
    uvec4 temp = uvec4(
        gl_FragCoord.x,
        gl_FragCoord.y,
        voxel_dimension * gl_FragCoord.z,
        0
    );
    uvec4 texture_coordinates;
    if (fragment_dominant_axis == 0) {
        texture_coordinates.x = voxel_dimension - temp.z;
        texture_coordinates.z = temp.x;
        texture_coordinates.y = temp.y;
    } else if(fragment_dominant_axis == 1) {
        texture_coordinates.z = temp.y;
        texture_coordinates.y = voxel_dimension - temp.z;
        texture_coordinates.x = temp.x;
    } else {
        texture_coordinates = temp;
    }

    texture_coordinates.z = voxel_dimension - texture_coordinates.z;
    return texture_coordinates;
}

void store_voxel_fragment(uvec4 texture_coordinates, uint fragment_list_index) {
    vec3 voxel_normal, voxel_color;
    if(u_bBump == 1)
       voxel_normal = texture(u_bumpTex, fragment_texture_coordinates).rgb;
    else
       voxel_normal = fragment_normal;

    if (has_texture)
      voxel_color = texture(texture_diffuse1, fragment_texture_coordinates).rgb;
    else
      voxel_color = fallback_color;

    imageStore(u_voxelPos, int(fragment_list_index), texture_coordinates);
    imageStore(u_voxelNrml, int(fragment_list_index), vec4(voxel_normal, 0));
    imageStore(u_voxelKd, int(fragment_list_index), vec4(voxel_color, 0));
}

void main() {
    discard_if_outside_aabb();

    uvec4 texture_coordinates = calculate_texture_coordinates();

    uint fragment_list_index = atomicCounterIncrement(voxel_fragment_count);

    if (should_store) {
        store_voxel_fragment(texture_coordinates, fragment_list_index);
    }

    FragColor = vec4(1.0, 1.0, 1.0, 1.0);
}
