#version 450

// vertex
layout(location = 0) in vec2 vertexPos;
layout(location = 1) in vec3 color;

// instance
layout(location = 2) in vec2 pos;
layout(location = 3) in float size;

layout(location = 0) out vec3 fragColor;

void main() {
  gl_Position = vec4(pos + (vertexPos * size), 0.0, 1.0);
  fragColor = color;
}
