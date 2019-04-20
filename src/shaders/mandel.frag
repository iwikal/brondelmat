#version 430 core
#pragma precision highp float

layout (location = 1) uniform vec2 position;
layout (location = 2) uniform float scale;
layout (location = 3) uniform ivec2 window_size;

out vec4 FragColor;

const int MAX_ITER = 256;

void main() {
  float x = 0, y = 0;
  vec2 c = ((gl_FragCoord.xy - window_size / 2) * scale - position) / vec2(300);
  for (int i = 0; i < MAX_ITER; i++) {
    float new_x = x * x - y * y;
    y = x * y * 2 + c.y;
    x = new_x + c.x;
    if (x * x + y * y > 2) {
      float col = pow(1.0 - i / float(MAX_ITER), 8);
      FragColor = vec4(0, 1.0 - col, col, 0);
      return;
    }
  }
  FragColor = vec4(0);
}
