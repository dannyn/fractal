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

    float intensity = (i == iter ? 0.0 : float(i) / iter);
    float r = intensity * 0.70; 
    float g = intensity * 1.00; 
    float b = intensity * 0.25; 

    color = vec4(r, g, b, 1.0);
}
