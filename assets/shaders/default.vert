attribute vec4 coordinates;

uniform vec4 color;
uniform mat4 camera;
uniform mat4 transform;

varying lowp vec4 v_color;

void main(void) {
    gl_Position = camera * transform * coordinates;
    v_color = color;
}
