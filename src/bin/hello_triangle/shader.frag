// this was the vertex shader output; it’s now our (rasterized and interpolated) input!
in vec3 v_color;

// we will output a single color
out vec4 frag_color;

void main() {
  // KISS
frag_color = vec4(v_color, 1.0);
}
