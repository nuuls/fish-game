precision lowp float;

varying vec4 v_color;
varying float v_time;
varying float v_water_y_level;
varying vec3 v_world_position;

const float water_height = 0.1;
const vec4 blue_1 = vec4(1.0, 1.0, 1.0, 0.3);
const vec4 blue_2 = vec4(0.26, 0.47, 0.9, 0.5);

void main(void) {
    // gl_FragColor = v_color;
    // gl_FragColor = sin(v_time) * gl_FragColor;
    float water_start = v_water_y_level + sin(sin(v_world_position.x * 0.3) * 0.3 + v_time * 2.0) * water_height + water_height;
    if(v_world_position.y < water_start) {
        gl_FragColor = vec4(0.0, 0.0, 1.0, 0.0);
    } else {

        float a = clamp((v_world_position.y - water_start) * 1.0, 0.0, 1.0);

        gl_FragColor = mix(blue_1, blue_2, a);
    }

    // gl_FragColor = vec4(sin(v_time) * vec3(0.0, 1.0, 0.0), 1.0);
}
