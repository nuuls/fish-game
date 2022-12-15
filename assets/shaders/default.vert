attribute vec4 coordinates;

uniform vec4 color;
uniform mat4 camera;
uniform vec2 position_offset;

varying lowp vec4 v_color;

void main(void) {
    gl_Position = camera * (coordinates + vec4(position_offset, 0.0, 0.0));
    v_color = color;
}
