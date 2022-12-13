#![feature(generic_arg_infer)]

extern crate js_sys;
extern crate mat4;
extern crate wasm_bindgen;
extern crate web_sys;
use drawing::Shader;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, WebGlBuffer, WebGlProgram, WebGlRenderingContext};
mod drawing;
mod game;
mod level;
use game::Game;

#[allow(dead_code)]
mod utils;
use utils::{compile_shader, link_program, request_animation_frame, set_panic_hook};

const AMORTIZATION: f32 = 0.95;

// vertex + fragment shader

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
        gl_FragColor = vec4(1.0, 0.0, 0.0, 0.5);
    }
  "#;
    let shaderProgram = initShaderProgram(&gl, vsSource, fsSource)?;

    // Draw the scene repeatedly
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    let drag = Rc::new(RefCell::new(false));
    let dX = Rc::new(RefCell::new(0.0));
    let dY = Rc::new(RefCell::new(0.0));
    let canvas_width = Rc::new(RefCell::new(canvas.width() as f32));
    let canvas_height = Rc::new(RefCell::new(canvas.height() as f32));
    let renderer = drawing::Renderer {
        shader: Shader {
            coordinate_index: gl.get_attrib_location(&shaderProgram, "coordinates") as u32,
            program: shaderProgram,
        },
        coordinate_buffer: gl.create_buffer().ok_or("failed to create buffer")?,
        index_buffer: gl.create_buffer().ok_or("failed to create buffer")?,
        gl,
    };

    // get canvas as event target
    let event_target: EventTarget = canvas.into();

    let mut game = Game::new();

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
            drawScene(&renderer, &mut game).unwrap();
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
fn drawScene(renderer: &drawing::Renderer, game: &mut Game) -> Result<(), JsValue> {
    use WebGlRenderingContext as xD;
    let gl = &renderer.gl;

    // let Buffers(positionBuffer, colorBuffer, indexBuffer) = buffers;
    gl.clear_color(0.0, 0.0, 0.0, 1.0);
    gl.clear_depth(1.0);
    // gl.enable(WebGlRenderingContext::DEPTH_TEST); // Enable depth testing
    // gl.depth_func(WebGlRenderingContext::LEQUAL); // Near things obscure far things

    gl.clear(xD::COLOR_BUFFER_BIT | xD::DEPTH_BUFFER_BIT);

    let canvas: web_sys::HtmlCanvasElement = gl
        .canvas()
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    gl.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);
    let aspect: f32 = canvas.width() as f32 / canvas.height() as f32;

    let indices: [u16; _] = [0, 1, 2];

    for tri in game.next_frame() {
        renderer.triangle(&tri, &indices)?;
    }

    // buffers

    Ok(())
}
