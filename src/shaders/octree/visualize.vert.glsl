#version 430 core

uniform int octree_levels;
uniform int voxel_dimension;

uniform layout(binding = 0, r32ui) uimageBuffer u_nodePoolBuff;

out vec4 node_position;
out float half_node_size;
out int non_empty_branch;

const int NODES_PER_TILE = 8;

bvec3 number_to_subsection(int number) {
    bvec3 subsection;
    subsection.x = bool(number & 1);
    subsection.y = bool(number & 2);
    subsection.z = bool(number & 4);
    return subsection;
}

uvec3 update_node_coordinates(
  uvec3 node_coordinates,
  bvec3 subsection,
  uint half_node_size
) {
  uvec3 ret = node_coordinates;
  if (subsection.x) {
    ret.x += half_node_size;
  }
  if (subsection.y) {
    ret.y += half_node_size;
  }
  if (subsection.z) {
    ret.z += half_node_size;
  }
  return ret;
}

void main() {
  int tile = 0;
  int thread_index = gl_VertexID;
  int subnode = thread_index % 8;
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

  float normalized_half_node_size = current_half_node_size * 2.0 / float(voxel_dimension);
  node_position = vec4((current_node_coordinates.xyz / float(voxel_dimension)) * 2.0 - vec3(1.0), 1.0);
  node_position += normalized_half_node_size;
  gl_Position = node_position;
  if(tile != 0 || octree_levels == 0) {
    half_node_size = normalized_half_node_size;
    non_empty_branch = 1;
  } else {
    half_node_size = normalized_half_node_size - 0.15;
    non_empty_branch = 0;
  }
}
