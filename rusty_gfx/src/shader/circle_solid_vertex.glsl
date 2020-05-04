// For drawing a shader shape circle (use a shader to draw a circle in a triangle)
// Inspiration:
// - https://blog.roboinstruct.us/2017/06/09/rust-and-opengl-in-orbit.html
// - https://github.com/big-ab-games/prototype-orbit/blob/master/src/orbitbody/shader/frag.glsl
#version 140

in vec2 position;
in vec3 color;
out vec3 v_color;
out vec2 v_position;

uniform mat4 matrix;

void main() {
    v_color = color;
    v_position = position;
    // TODO: Combine projection, view, and local here (and in equivalent shaders)
    // Like this: https://github.com/big-ab-games/prototype-orbit/blob/master/src/orbitbody/shader/vert.glsl
    gl_Position = matrix * vec4(position, 0.0, 1.0);
}