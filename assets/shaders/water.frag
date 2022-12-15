varying lowp vec4 v_color;
varying lowp float v_time;
varying lowp float v_water_y_level;

void main(void) {
    // gl_FragColor = v_color;
    // gl_FragColor = sin(v_time) * gl_FragColor;

    gl_FragColor = vec4(0.0, 1.0, 0.0, 1.0);
}
