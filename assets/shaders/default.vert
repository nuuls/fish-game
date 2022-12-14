attribute vec4 coordinates;

void main(void) {
    gl_Position = vec4(coordinates.x / 500.0, coordinates.y / 500.0, 0.0, 1.0);
}