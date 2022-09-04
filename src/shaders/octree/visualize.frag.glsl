#version 430 core

layout (location = 0) out vec4 FragColor;

in flat int geom_vertex_id;
in flat vec3 fragment_normal;
in flat vec4 fragment_color;
in flat int branch_not_empty;

void main() {
  if(bool(branch_not_empty)) {
    FragColor = vec4(0.0, 1.0, 0.0, 1.0);
  } else {
    FragColor = vec4(0.0, 0.0, 0.0, 0.0);
  }
}
