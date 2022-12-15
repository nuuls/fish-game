#![feature(generic_arg_infer)]
#![feature(array_chunks)]

extern crate js_sys;
extern crate mat4;
extern crate wasm_bindgen;
extern crate web_sys;
use drawing::Shader;
use drawing::WaterShader;
use std::cell::RefCell;
use std::rc::Rc;
use types::Entity;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, WebGlBuffer, WebGlProgram, WebGlRenderingContext};
mod drawing;
mod game;
mod level;
mod player;
mod types;
use game::Game;

#[allow(dead_code)]
mod utils;
use utils::{compile_shader, link_program, request_animation_frame, set_panic_hook};

const AMORTIZATION: f32 = 0.95;

// vertex + fragment shader

#[derive(Debug, Clone)]
struct Buffers(WebGlBuffer, WebGlBuffer, WebGlBuffer);

fn into_shader(gl: &WebGlRenderingContext, program: WebGlProgram) -> Shader {
    Shader {
        camera_index: gl.get_uniform_location(&program, "camera").unwrap(),
        color_index: gl.get_uniform_location(&program, "color").unwrap(),
        coordinate_index: gl.get_attrib_location(&program, "coordinates") as u32,
        program: program,
    }
}

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

    // Shader
    let program = initShaderProgram(
        &gl,
        include_str!("../assets/shaders/default.vert"),
        include_str!("../assets/shaders/default.frag"),
    )?;
    let water_program = initShaderProgram(
        &gl,
        include_str!("../assets/shaders/water.vert"),
        include_str!("../assets/shaders/water.frag"),
    )?;

    // Draw the scene repeatedly
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    let drag = Rc::new(RefCell::new(false));
    let dX = Rc::new(RefCell::new(0.0));
    let dY = Rc::new(RefCell::new(0.0));
    let _canvas_width = Rc::new(RefCell::new(canvas.width() as f32));
    let _canvas_height = Rc::new(RefCell::new(canvas.height() as f32));
    let mut renderer = drawing::Renderer {
        coordinate_buffer: gl.create_buffer().ok_or("failed to create buffer")?,
        index_buffer: gl.create_buffer().ok_or("failed to create buffer")?,
        shader: into_shader(&gl, program),
        water_shader: WaterShader {
            time_index: gl.get_uniform_location(&water_program, "time").unwrap(),
            water_y_level_index: gl
                .get_uniform_location(&water_program, "water_y_level")
                .unwrap(),
            base: into_shader(&gl, water_program),
        },
        gl,
        camera: mat4::new_identity(),
        time: 0.0,
    };

    // get canvas as event target
    let _event_target: EventTarget = canvas.into();

    let mut game = Game::new();
    let window = web_sys::window().expect("should have a window in this context");
    let performance = window
        .performance()
        .expect("performance should be available");

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
            renderer.time = performance.now() as f32 / 1000.0;
            drawScene(&mut renderer, &mut game).unwrap();
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
fn drawScene(renderer: &mut drawing::Renderer, game: &mut Game) -> Result<(), JsValue> {
    use WebGlRenderingContext as xD;
    let gl = &renderer.gl;

    gl.enable(xD::BLEND);
    gl.blend_func(xD::SRC_ALPHA, xD::ONE_MINUS_SRC_ALPHA);
    // gl.enable(WebGlRenderingContext::DEPTH_TEST); // Enable depth testing
    // gl.depth_func(WebGlRenderingContext::LEQUAL); // Near things obscure far things

    gl.clear(xD::COLOR_BUFFER_BIT | xD::DEPTH_BUFFER_BIT);
    gl.clear_color(0.4, 0.7, 0.9, 1.0);
    gl.clear_depth(1.0);

    let canvas: web_sys::HtmlCanvasElement = gl
        .canvas()
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    gl.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);
    let aspect: f32 = canvas.width() as f32 / canvas.height() as f32;
    let zoom = 1.0 / 20.0;

    let mut tmp1 = mat4::new_identity();
    let mut tmp2 = mat4::new_identity();
    mat4::scale(&mut tmp1, &tmp2, &[zoom, -zoom * aspect, 1.0]);
    mat4::translate(&mut tmp2, &tmp1, &[-30.0, -15.0, 0.0]);
    renderer.camera = tmp2;

    for tri in game.next_frame() {
        renderer.triangle(&tri)?;
    }

    // buffers

    Ok(())
}
