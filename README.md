# Voxel Cone Tracing

Voxel Cone Tracing implementation

## Tests

To run the tests, do `cargo test -- --test-threads 1`.
Because all tests initialize an OpenGL context, that gets cleaned up at the end of the test,
they can't run concurrently.

## Process

We voxelize the entire scene. This generates a voxel fragment list.
We then use this list to create the sparse voxel octree (SVO).
Every voxel fragment ends up in an octree leaf.

Every node has at least one child node unless it's the last level.
This means there are no leaves on higher levels.

## Standards

### Octree terminology and utils

We'll use as an example the triangle.obj model.

This is node 0, it represents the whole scene -> [0, 0, 0, 0, 1, 2, 3, 4].
We find these nodes in the OCTREE_NODE_POOL buffer texture
(could be called OCTREE_NODE_CHILD_POINTERS for consistency).
Every one of the 8 elements in the node is a pointer to a child node.

If a texture holds pointers, its layout is r32ui, if it holds coordinates, it's rgb10_a2ui.

Level 0 is composed of node 0.
Level 1 is composed of nodes 1, 2, 3, and 4. This is because the octree is sparse.
The indices of the first nodes in each level are stored in OCTREE_LEVEL_START_INDICES.
In our example, its [0, 1, 5, 9, 22, 65, 216, 775] for 8 levels.

We find the position of each of these nodes in OCTREE_NODE_POSITIONS.
This positions are used 

Each node has a brick that's accessed by its index in OCTREE_NODE_POOL_BRICK_POINTERS.

Each node's neighbor is stored in the buffer textures called OCTREE_NODE_POOL_NEIGHBOR_N,
with N being one of (X, X_NEGATIVE, Y, Y_NEGATIVE, Z, Z_NEGATIVE).

`_octreeTraversal.glsl` holds functions to query the SVO with coordinates and get different results back.
All of them return the node where they found the query coordinates, or `NODE_NOT_FOUND` if they did not.

## How to debug compute shaders ("print debugging")

### Create and bind a buffer texture

```rust
let (debug_texture, debug_texture_buffer) = helpers::generate_texture_buffer(size, gl::R32F, default_value);
helpers::bind_image_texture(image_index, debug_texture, gl::WRITE_ONLY, gl::R32F);
```

R32F is a good format since everything can be turned into a float.

### Access texture in shader

```glsl
uniform layout(binding = image_index, r32f) imageBuffer debugBuffer;

...

imageStore(debugBuffer, 0, vec4(float(someValue), 0, 0, 0));
```

### Get values from buffer

```rust
let values = helpers::get_values_from_texture_buffer(debug_texture_buffer, size, default_value);
dbg!(&values);
```
