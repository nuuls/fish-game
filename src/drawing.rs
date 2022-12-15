use wasm_bindgen::JsValue;
use web_sys::{WebGlBuffer, WebGlProgram, WebGlRenderingContext, WebGlUniformLocation};

use crate::{
    log,
    types::Color,
    types::Triangle,
    utils::{float_32_array, uint_16_array},
};

type Mat4 = [f32; 16];

#[derive(Debug, Clone)]
pub struct Shader {
    pub program: WebGlProgram,
    pub coordinate_index: u32,
    pub camera_index: WebGlUniformLocation,
    pub color_index: WebGlUniformLocation,
}

pub struct WaterShader {
    pub base: Shader,
    pub time_index: WebGlUniformLocation,
    pub water_y_level_index: WebGlUniformLocation,
}

pub struct Renderer {
    pub coordinate_buffer: WebGlBuffer,
    pub index_buffer: WebGlBuffer,
    pub gl: WebGlRenderingContext,
    pub camera: Mat4,
    pub time: f32,

    pub shader: Shader,
    pub water_shader: WaterShader,
}

impl Renderer {
    pub fn use_shader(&self, color: &Color) -> Result<(), JsValue> {
        let shader = &self.shader;

        self.gl.use_program(Some(&shader.program));
        self.gl
            .uniform_matrix4fv_with_f32_array(Some(&shader.camera_index), false, &self.camera);
        self.gl
            .uniform4fv_with_f32_array(Some(&shader.color_index), color);
        Ok(())
    }

    pub fn use_water_shader(&self, color: &Color, water_y_level: f32) -> Result<(), JsValue> {
        let shader = &self.water_shader.base;

        self.gl.use_program(Some(&shader.program));
        self.gl
            .uniform_matrix4fv_with_f32_array(Some(&shader.camera_index), false, &self.camera);
        self.gl
            .uniform4fv_with_f32_array(Some(&shader.color_index), color);

        self.gl
            .uniform1f(Some(&self.water_shader.water_y_level_index), water_y_level);
        self.gl
            .uniform1f(Some(&self.water_shader.time_index), self.time);

        Ok(())
    }

    pub fn triangle(&self, triangle: &Triangle) -> Result<(), JsValue> {
        const NUM_COORDINATES: i32 = 3;
        use WebGlRenderingContext as GL;
        let gl = &self.gl;

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.coordinate_buffer));

        gl.buffer_data_with_array_buffer_view(
            GL::ARRAY_BUFFER,
            &float_32_array(&triangle.coords)?.into(),
            GL::STATIC_DRAW,
        );
        gl.bind_buffer(GL::ARRAY_BUFFER, None);

        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&self.index_buffer));
        gl.buffer_data_with_array_buffer_view(
            GL::ELEMENT_ARRAY_BUFFER,
            &uint_16_array(&[0, 1, 2])?.into(),
            GL::STATIC_DRAW,
        );
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, None);

        // bind buffer
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.coordinate_buffer));
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&self.index_buffer));
        gl.vertex_attrib_pointer_with_i32(self.shader.coordinate_index, 3, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(self.shader.coordinate_index);

        // use shader
        if triangle.color[0] < 0.0001 && triangle.color[1] < triangle.color[2] {
            // spaghetti is served 🍝
            let water_y_level = triangle.coords[1]
                .min(triangle.coords[4])
                .min(triangle.coords[7]);

            // let water_y_level = mat4::(self.camera,

            self.use_water_shader(&triangle.color, water_y_level)?;
        } else {
            self.use_shader(&triangle.color)?;
        }

        // draw
        gl.draw_elements_with_i32(GL::TRIANGLES, NUM_COORDINATES, GL::UNSIGNED_SHORT, 0);

        Ok(())
    }
}
