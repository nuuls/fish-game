attribute vec4 coordinates;
uniform mat4 camera;

void main(void) {
    gl_Position = camera * coordinates;
}