#version 430 core

layout (local_size_x = 64, local_size_y = 1, local_size_z = 1) in;

uniform layout(binding = 0, r32ui) uimageBuffer octree_node_pool;
uniform layout(binding = 1, r32ui) uimageBuffer octree_diffuse_texture;

uniform layout(binding = 2, rgb10_a2ui) uimageBuffer voxel_position_texture;
uniform layout(binding = 3, rgba8) imageBuffer voxel_diffuse_texture;

uniform int voxel_dimension;
uniform int octree_level;
uniform int number_of_voxel_fragments;

const uint NODES_PER_TILE = 8;

void imageAtomicRGBA8Average(vec4 val, int coord);
uint convVec4ToRGBA8(vec4 val);
vec4 convRGBA8ToVec4(uint val);

bool within_second_half(uint min, uint half_node_size, uint coordinate_position) {
 return coordinate_position > min + half_node_size;
}

// Each node is divided into 8 subsections/children, for each coordinate the node is divided in two.
// Given the borders of the node, for each coordinate we calculate if the fragment is within the first or second
// half of the node.
bvec3 calculate_node_subsection(uvec3 node_coordinates, uint half_node_size, uvec3 fragment_position) {
  bvec3 subsection;
  subsection.x = within_second_half(node_coordinates.x, half_node_size, fragment_position.x);
  subsection.y = within_second_half(node_coordinates.y, half_node_size, fragment_position.y);
  subsection.z = within_second_half(node_coordinates.z, half_node_size, fragment_position.z);
  return subsection;
}

// As we have one pointer for all 2x2x2 children, we calculate the index of the child this voxel fragment falls into
uint calculate_node_index(uint tile_index, bvec3 subsection) {
  return (tile_index * NODES_PER_TILE) + 
         uint(subsection[0]) +
         uint(subsection[1]) * 2 +
         uint(subsection[2]) * 4; // binary -> base10, this gives a unique index per subsection. Then add it to the tile_index
}

uvec3 update_node_coordinates(
  uvec3 current_node_coordinates,
  bvec3 subsection,
  uint current_half_node_size
) {
  uvec3 ret = current_node_coordinates;
  if (subsection.x) {
    ret.x += current_half_node_size;
  }
  if (subsection.y) {
    ret.y += current_half_node_size;
  }
  if (subsection.z) {
    ret.z += current_half_node_size;
  }

  return ret;
}

void main() {
    uint thread_index = gl_GlobalInvocationID.x;

    if (thread_index >= number_of_voxel_fragments) {
        return;
    }

    uvec4 voxel_fragment_position = imageLoad(voxel_position_texture, int(thread_index));
    uint current_half_node_size = voxel_dimension / 2;
    uint current_tile_index = 0;
    uvec3 current_node_coordinates = uvec3(0, 0, 0);

    bvec3 subsection = calculate_node_subsection(
        current_node_coordinates,
        current_half_node_size,
        voxel_fragment_position.xyz
    );

    uint current_node_index = calculate_node_index(current_tile_index, subsection);

    current_node_coordinates = update_node_coordinates(
      current_node_coordinates,
      subsection,
      current_half_node_size
    );

    current_half_node_size /= 2;

    for (uint i = 0; i < octree_level; i++)
    {
        current_tile_index = imageLoad(octree_node_pool, int(current_node_index)).r;

        bvec3 subsection = calculate_node_subsection(current_node_coordinates,
                                                     current_half_node_size,
                                                     voxel_fragment_position.xyz);

        current_node_index = calculate_node_index(current_tile_index, subsection);

        current_node_coordinates = update_node_coordinates(
          current_node_coordinates,
          subsection,
          current_half_node_size
        );

        current_half_node_size /= 2;
    }

    vec4 color = imageLoad(voxel_diffuse_texture, int(thread_index));

    imageAtomicRGBA8Average(color, int(current_node_index));
}

vec4 convRGBA8ToVec4(in uint val)
{
    return vec4(
        float((int(val) & 0x000000FF)),
        float((int(val) & 0x0000FF00) >> 8U),
	    float((int(val) & 0x00FF0000) >> 16U),
        float((int(val) & 0xFF000000) >> 24U)
    );
}

uint convVec4ToRGBA8(in vec4 val)
{
    return (int(val.w) & 0x000000FF) << 24U
        | (int(val.z) & 0x000000FF) << 16U
        | (int(val.y) & 0x000000FF) << 8U
        | (int(val.x) & 0x000000FF);
}

void imageAtomicRGBA8Average(vec4 val, int coord)
{
    val.rgb *= 255.0;
	val.a = 1;

	uint newVal = convVec4ToRGBA8(val);
	uint prev = 0;
	uint cur;

    // Loop as long as destination value gets changed by other threads
    while((cur = imageAtomicCompSwap(octree_diffuse_texture, coord, prev, newVal)) != prev)
    {
       prev = cur;
	   vec4 rval = convRGBA8ToVec4( cur );
	   rval.xyz = rval.xyz*rval.w;
	   vec4 curVal = rval +  val;
	   curVal.xyz /= curVal.w;
	   newVal = convVec4ToRGBA8( curVal );
    }
}
