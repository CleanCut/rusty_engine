#version 140

in vec3 v_color;
in vec2 v_position;
out vec4 color;
const float border_dist = 0.81;

uniform float local_scale;

void main() {
    // Get the distance from the center of the circle
    float dist = sqrt(pow(v_position.x, 2) + pow(v_position.y, 2));

    // Fake anti-alias -- should we do this? It opens up a can of worms if you scale up a lot
    //float alpha = 1 - smoothstep(0.98 * local_scale, 1.0 * local_scale, dist);

    // This is the simple, aliased approach.
    float alpha = 1.0;
    if (dist > 1.0 * local_scale) {
        discard;
    }
    color = vec4(v_color, alpha);
}