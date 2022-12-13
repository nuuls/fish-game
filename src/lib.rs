#![feature(generic_arg_infer)]

extern crate js_sys;
extern crate mat4;
extern crate wasm_bindgen;
extern crate web_sys;
use js_sys::ArrayBuffer;
use js_sys::WebAssembly;
use std::cell::RefCell;
use std::f32::consts::PI;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    EventTarget, MouseEvent, WebGlBuffer, WebGlProgram, WebGlRenderingContext, WebGlUniformLocation,
};

#[allow(dead_code)]
mod utils;
use utils::{compile_shader, link_program, request_animation_frame, set_panic_hook};

const AMORTIZATION: f32 = 0.95;

// vertex + fragment shader
#[derive(Debug, Clone)]
struct Shader {
    pub program: WebGlProgram,
    pub coordinateIndex: u32,
}

#[derive(Debug, Clone)]
struct Buffers(WebGlBuffer, WebGlBuffer, WebGlBuffer);

#[allow(non_snake_case)]
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    set_panic_hook();
    /*============ Creating a canvas =================*/
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let gl = canvas
        .get_context("webgl")?
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()?;

    // Vertex shader program

    let vsSource = r#"
    attribute vec4 coordinates;

    void main(void) {
        gl_Position = coordinates;
    }
  "#;

    // Fragment shader program

    let fsSource = r#"
    void main(void) {
        gl_FragColor = vec4(1.0, 0.0, 0.0, 1.0);
    }
  "#;
    let shaderProgram = initShaderProgram(&gl, vsSource, fsSource)?;
    let shader = Shader {
        coordinateIndex: gl.get_attrib_location(&shaderProgram, "coordinates") as u32,
        program: shaderProgram,
    };

    // Draw the scene repeatedly
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    let drag = Rc::new(RefCell::new(false));
    let dX = Rc::new(RefCell::new(0.0));
    let dY = Rc::new(RefCell::new(0.0));
    let canvas_width = Rc::new(RefCell::new(canvas.width() as f32));
    let canvas_height = Rc::new(RefCell::new(canvas.height() as f32));

    // get canvas as event target
    let event_target: EventTarget = canvas.into();

    // RequestAnimationFrame
    {
        let dX = dX.clone();
        let dY = dY.clone();
        let drag = drag.clone();
        // Request animation frame
        *g.borrow_mut() = Some(Closure::wrap(Box::new(move |_d| {
            if !*drag.borrow() {
                *dX.borrow_mut() *= AMORTIZATION;
                *dY.borrow_mut() *= AMORTIZATION;
            }
            // drawScene(&gl.clone(), programmInfo.clone(), buffers.clone()).unwrap();
            drawScene(&gl.clone(), shader.clone()).unwrap();
            // Schedule ourself for another requestAnimationFrame callback.
            request_animation_frame(f.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut(f32)>));

        request_animation_frame(g.borrow().as_ref().unwrap());
    }
    Ok(())
}

#[allow(non_snake_case)]
fn initShaderProgram(
    gl: &WebGlRenderingContext,
    vsSource: &str,
    fsSource: &str,
) -> Result<WebGlProgram, String> {
    let v_shader = compile_shader(gl, WebGlRenderingContext::VERTEX_SHADER, vsSource);
    let f_shader = compile_shader(gl, WebGlRenderingContext::FRAGMENT_SHADER, fsSource);

    link_program(gl, &v_shader?, &f_shader?)
}

#[allow(non_snake_case)]
#[allow(dead_code)]
fn drawScene(gl: &WebGlRenderingContext, shader: Shader) -> Result<(), JsValue> {
    use WebGlRenderingContext as xD;

    // let Buffers(positionBuffer, colorBuffer, indexBuffer) = buffers;
    gl.clear_color(0.0, 0.0, 0.0, 1.0);
    gl.clear_depth(1.0);
    // gl.enable(WebGlRenderingContext::DEPTH_TEST); // Enable depth testing
    // gl.depth_func(WebGlRenderingContext::LEQUAL); // Near things obscure far things

    gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT | WebGlRenderingContext::DEPTH_BUFFER_BIT);

    let fieldOfView = 45.0 * PI / 180.0; // in radians
    let canvas: web_sys::HtmlCanvasElement = gl
        .canvas()
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    gl.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);
    let aspect: f32 = canvas.width() as f32 / canvas.height() as f32;

    // Tell WebGL to use our program when drawing
    gl.use_program(Some(&shader.program));

    // data
    let num_coordinates = 3;
    let coordinates: [f32; _] = [
        -0.5, 0.5, 0.0, //
        -0.5, -0.5, 0.0, //
        0.5, -0.5, 0.0,
    ];
    let indices: [i16; 3] = [0, 1, 2];

    // buffers
    let coordinate_buffer = gl
        .create_buffer()
        .ok_or("failed to create vertex_buffer buffer")?;
    gl.bind_buffer(xD::ARRAY_BUFFER, Some(&coordinate_buffer));

    gl.buffer_data_with_array_buffer_view(
        xD::ARRAY_BUFFER,
        &*float_32_array!(coordinates),
        xD::STATIC_DRAW,
    );
    gl.bind_buffer(xD::ARRAY_BUFFER, None);

    let index_buffer = gl
        .create_buffer()
        .ok_or("failed to create index_buffer buffer")?;
    gl.bind_buffer(xD::ELEMENT_ARRAY_BUFFER, Some(&index_buffer));
    gl.buffer_data_with_array_buffer_view(
        xD::ELEMENT_ARRAY_BUFFER,
        &*uint_16_array!(indices),
        xD::STATIC_DRAW,
    );
    gl.bind_buffer(xD::ELEMENT_ARRAY_BUFFER, None);

    // bind buffer
    gl.bind_buffer(xD::ARRAY_BUFFER, Some(&coordinate_buffer));
    gl.bind_buffer(xD::ELEMENT_ARRAY_BUFFER, Some(&index_buffer));
    gl.vertex_attrib_pointer_with_i32(shader.coordinateIndex, 3, xD::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(shader.coordinateIndex);

    // draw
    gl.draw_elements_with_i32(xD::TRIANGLES, num_coordinates, xD::UNSIGNED_SHORT, 0);

    Ok(())
}
