#version 460 core

#include "./_constants.glsl"
#include "./_helpers.glsl"
#include "./_traversal_helpers.glsl"

uniform int octree_levels;
uniform int offset;
uniform int show_empty_nodes;
uniform int draw_by_parts;
uniform int voxel_dimension;

uniform layout(binding = 0, r32ui) uimageBuffer u_nodePoolBuff;
uniform layout(binding = 1, r32ui) uimageBuffer node_pool_brick_pointers;
uniform layout(binding = 2, rgba8) image3D brick_pool_colors;

out vec4 node_position;
out float half_node_size;
out int non_empty_branch;
out int keep_on_going;
out vec4 node_color;

bvec3 number_to_subsection(int number) {
    bvec3 subsection;
    subsection.x = bool(number & 1);
    subsection.y = bool(number & 2);
    subsection.z = bool(number & 4);
    return subsection;
}

void main() {
  int tile = 0;
  int thread_index = gl_VertexID;
  int subnode;
  if (draw_by_parts == 1) {
    subnode = offset;
  } else {
    subnode = thread_index % 8; // TODO: Throw less threads
  }
  uvec3 current_node_coordinates = uvec3(0, 0, 0);
  uint current_half_node_size = voxel_dimension / 2;
  bool continue_for = true;
  int i = 0;

  for (i = 0; (i < octree_levels) && continue_for; i++) {
    tile = int(imageLoad(u_nodePoolBuff, tile * NODES_PER_TILE + subnode).r);

    if (tile == 0) {
      continue_for = false;
    }

    bvec3 subsection = number_to_subsection(subnode);
    current_node_coordinates = update_node_coordinates(current_node_coordinates, subsection, current_half_node_size);

    thread_index /= 8;
    subnode = thread_index % 8;
    current_half_node_size /= 2;
  }
  
  int node_address = tile * NODES_PER_TILE + subnode;
  uint brick_coordinates_compact = imageLoad(node_pool_brick_pointers, node_address).r;
  ivec3 brick_coordinates = ivec3(uintXYZ10ToVec3(brick_coordinates_compact));
  vec4 center_voxel_color = imageLoad(brick_pool_colors, brick_coordinates);
  node_color = center_voxel_color;

  float normalized_half_node_size = current_half_node_size * 2.0 / float(voxel_dimension);
  node_position = vec4((current_node_coordinates.xyz / float(voxel_dimension)) * 2.0 - vec3(1.0), 1.0);
  node_position.xyz += normalized_half_node_size;
  gl_Position = node_position;
  if(tile != 0 || octree_levels == 0) {
    half_node_size = normalized_half_node_size;
    non_empty_branch = 1;
  } else {
    half_node_size = normalized_half_node_size * int(show_empty_nodes);
    non_empty_branch = 0;
  }
}