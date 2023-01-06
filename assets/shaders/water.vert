attribute vec4 coordinates;

uniform vec4 color;
uniform mat4 camera;
uniform mat4 transform;
uniform float water_y_level;
uniform float time;

varying lowp vec4 v_color;
varying lowp float v_time;
varying lowp float v_water_y_level;
varying lowp vec3 v_world_position;

void main(void) {
    gl_Position = camera * transform * coordinates;
    v_color = color;
    v_time = time;
    v_water_y_level = water_y_level;
    v_world_position = coordinates.xyz;
}
