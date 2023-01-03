# Voxel Cone Tracing

Voxel Cone Tracing implementation

## How to debug compute shaders ("print debugging")

### Create and bind a buffer texture

```rust
let (debug_texture, debug_texture_buffer) = helpers::generate_texture_buffer(size, gl::R32F, default_value);
helpers::bind_image_texture(image_index, debug_texture, gl::WRITE_ONLY, gl::R32F);
```

R32F is a good format since everything can be turned into a float.

### Access texture in shader

```glsl
uniform layout(binding = image_index, r32f) imageBuffer debug_buffer;

...

imageStore(debug_buffer, 0, vec4(float(some_value), 0, 0, 0));
```
