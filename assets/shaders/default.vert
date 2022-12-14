attribute vec4 coordinates;

uniform vec4 color;
uniform mat4 camera;

varying lowp vec4 vColor;

void main(void) {
    gl_Position = camera * coordinates;
    vColor = color;
}
