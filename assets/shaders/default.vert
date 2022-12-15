attribute vec4 coordinates;

uniform vec4 color;
uniform mat4 camera;

varying lowp vec4 v_color;

void main(void) {
    gl_Position = camera * coordinates;
    v_color = color;
}
