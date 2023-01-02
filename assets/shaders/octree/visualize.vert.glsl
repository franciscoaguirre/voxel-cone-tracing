#version 460 core

#include "./_constants.glsl"
#include "./_helpers.glsl"
#include "./_traversal_helpers.glsl"
#include "./_octree_traversal.glsl"

uniform int octree_levels;
uniform int show_empty_nodes;
uniform int voxel_dimension;

uniform layout(binding = 0, r32ui) uimageBuffer node_pool;
uniform layout(binding = 1, r32ui) uimageBuffer node_pool_brick_pointers;
uniform layout(binding = 2, rgba8) image3D brick_pool_colors;
uniform layout(binding = 3, rgb10_a2ui) uimageBuffer voxel_positions;
uniform layout(binding = 4, r32f) imageBuffer debug_buffer;

out vec4 node_position;
out float geom_half_node_size;
out int non_empty_branch;
out int keep_on_going;
out vec4 node_color;

void main() {
  int thread_index = gl_VertexID;

  // TODO: Find an efficient way to render both occupied and empty nodes.
  // This approach uses voxel fragment positions and therefore doesn't show
  // empty nodes.
  vec4 voxel_fragment_position = imageLoad(voxel_positions, thread_index);

  uint tile_index;
  float half_node_size;
  vec3 node_coordinates;
  int node_index = traverse_octree_returning_node_coordinates(
    vec3(voxel_fragment_position) / float(voxel_dimension),
    octree_levels,
    node_pool,
    half_node_size,
    node_coordinates,
    tile_index
  );

  // Debug statements
  // imageStore(debug_buffer, 0, vec4(float(node_index), 0, 0, 0));
  // imageStore(debug_buffer, 1, vec4(float(half_node_size), 0, 0, 0));
  // imageStore(debug_buffer, 2, vec4(float(node_coordinates.x), 0, 0, 0));
  // imageStore(debug_buffer, 3, vec4(float(node_coordinates.y), 0, 0, 0));
  // imageStore(debug_buffer, 4, vec4(float(node_coordinates.z), 0, 0, 0));
  // imageStore(debug_buffer, 5, vec4(float(octree_levels), 0, 0, 0));
  // imageStore(debug_buffer, 6, vec4(float(tile_index), 0, 0, 0));
  // imageStore(debug_buffer, 7, vec4(float(voxel_fragment_position.x), 0, 0, 0));
  // imageStore(debug_buffer, 8, vec4(float(voxel_fragment_position.y), 0, 0, 0));
  // imageStore(debug_buffer, 9, vec4(float(voxel_fragment_position.z), 0, 0, 0));

  uint brick_coordinates_compact = imageLoad(node_pool_brick_pointers, node_index).r;
  ivec3 brick_coordinates = ivec3(uintXYZ10ToVec3(brick_coordinates_compact));

  // NOTE: Bricks start at (0, 0, 0) and go to (2, 2, 2)
  ivec3 offset_to_center = ivec3(1, 1, 1);
  vec4 center_voxel_color = imageLoad(brick_pool_colors, brick_coordinates + offset_to_center);
  node_color = center_voxel_color;

  // Normalized device coordinates go from -1.0 to 1.0, our coordinates go from 0.0 to 1.0
  node_position = vec4((node_coordinates.xyz) * 2.0 - vec3(1.0), 1.0);
  float normalized_half_node_size = half_node_size * 2.0;

  node_position.xyz += normalized_half_node_size;
  gl_Position = node_position;

  if (tile_index != 0 || octree_levels == 0) {
    geom_half_node_size = normalized_half_node_size;
    non_empty_branch = 1;
  } else {
    geom_half_node_size = normalized_half_node_size * int(show_empty_nodes);
    non_empty_branch = 0;
  }
}
