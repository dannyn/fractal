#version 140
in vec2 position;
in vec2 tex;

out vec2 tex_coords;

void main() {
	tex_coords = tex;
    gl_Position = vec4(position, 0.0, 1.0);
}
