#version 460 core

layout (location = 0) out vec4 FragColor;

in flat int frag_nonEmptyBranch;
in flat vec4 frag_nodeColor;

void main() {
  if(bool(frag_nonEmptyBranch)) {
    FragColor = frag_nodeColor;
  } else {
    FragColor = vec4(0.0, 1.0, 0.0, 0.0);
  }
}
