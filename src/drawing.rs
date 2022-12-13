use wasm_bindgen::JsValue;
use web_sys::{WebGlBuffer, WebGlProgram, WebGlRenderingContext};

use crate::utils::{float_32_array, uint_16_array};

#[derive(Debug, Clone)]
pub struct Shader {
    pub program: WebGlProgram,
    pub coordinate_index: u32,
}

pub struct Renderer {
    pub coordinate_buffer: WebGlBuffer,
    pub index_buffer: WebGlBuffer,
    pub shader: Shader,
    pub gl: WebGlRenderingContext,
}

impl Renderer {
    pub fn triangle(&self, coordinates: &[f32; 9], indices: &[u16; 3]) -> Result<(), JsValue> {
        const NUM_COORDINATES: i32 = 3;
        use WebGlRenderingContext as GL;
        let gl = &self.gl;

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.coordinate_buffer));

        gl.buffer_data_with_array_buffer_view(
            GL::ARRAY_BUFFER,
            &float_32_array(coordinates)?.into(),
            GL::STATIC_DRAW,
        );
        gl.bind_buffer(GL::ARRAY_BUFFER, None);

        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&self.index_buffer));
        gl.buffer_data_with_array_buffer_view(
            GL::ELEMENT_ARRAY_BUFFER,
            &uint_16_array(indices)?.into(),
            GL::STATIC_DRAW,
        );
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, None);

        // bind buffer
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.coordinate_buffer));
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&self.index_buffer));
        gl.vertex_attrib_pointer_with_i32(self.shader.coordinate_index, 3, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(self.shader.coordinate_index);

        // Tell WebGL to use our program when drawing
        gl.use_program(Some(&self.shader.program));

        // draw
        gl.draw_elements_with_i32(GL::TRIANGLES, NUM_COORDINATES, GL::UNSIGNED_SHORT, 0);

        Ok(())
    }
}
