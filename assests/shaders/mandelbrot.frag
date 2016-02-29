#version 140
in vec2 tex_coords;

out vec4 color;

uniform float scale;
uniform vec2 center;
uniform int iter;

void main() {
    vec2 z, c;

    c.x = 1.3333 * (tex_coords.x - 0.5) * scale - center.x;
    c.y = (tex_coords.y - 0.5) * scale - center.y;

    int i;
    z = c;
    for(i=0; i<iter; i++) {
        float x = (z.x * z.x - z.y * z.y) + c.x;
        float y = (z.y * z.x + z.x * z.y) + c.y;

        if((x * x + y * y) > 4.0) break;
        z.x = x;
        z.y = y;
    }

    float r = (i == iter ? 0.0 : 0.70 * float(i) / iter);
    float g = (i == iter ? 0.0 : 1.00 * float(i) / iter);
    float b = (i == iter ? 0.0 : 0.25 * float(i) / iter);

    color = vec4(r, g, b, 1.0);

}
