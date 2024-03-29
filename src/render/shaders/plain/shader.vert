#version 450

// vertex
layout(location = 0) in vec3 pos;
layout(location = 1) in vec3 color;

// instance
layout(location = 2) in mat4 matrix;

layout(location = 0) out vec3 frag_color;

void main() {
  gl_Position = matrix * vec4(pos, 1.0);
  frag_color = color;
}
