use std::sync::atomic::AtomicU64;

use js_sys::WebAssembly;
use wasm_bindgen::JsCast;
use wasm_bindgen::{prelude::Closure, JsValue};
use web_sys::{WebGlBuffer, WebGlProgram, WebGlRenderingContext, WebGlShader};
pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
#[macro_export]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

pub fn float_32_array(slice: &[f32]) -> Result<js_sys::Float32Array, JsValue> {
    let memory_buffer = wasm_bindgen::memory()
        .dyn_into::<WebAssembly::Memory>()?
        .buffer();
    let arr_location = slice.as_ptr() as u32 / 4;
    let array = js_sys::Float32Array::new(&memory_buffer)
        .subarray(arr_location, arr_location + slice.len() as u32);
    Ok(array)
}

pub fn uint_16_array(slice: &[u16]) -> Result<js_sys::Uint16Array, JsValue> {
    let memory_buffer = wasm_bindgen::memory()
        .dyn_into::<WebAssembly::Memory>()?
        .buffer();
    let arr_location = slice.as_ptr() as u32 / 2;
    let array = js_sys::Uint16Array::new(&memory_buffer)
        .subarray(arr_location, arr_location + slice.len() as u32);
    Ok(array)
}

pub fn compile_shader(
    context: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

pub fn link_program(
    context: &WebGlRenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}

pub fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

pub fn request_animation_frame(f: &Closure<dyn FnMut(f32)>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

pub fn bind_buffer_to_attribute(
    gl: &WebGlRenderingContext,
    buffer: &WebGlBuffer,
    attribute_index: u32,
    num_components: i32,
) {
    let type_ = WebGlRenderingContext::FLOAT;
    let normalize = false;
    let stride = 0;
    let offset = 0;
    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

    gl.vertex_attrib_pointer_with_i32(
        attribute_index,
        num_components,
        type_,
        normalize,
        stride,
        offset,
    );
    gl.enable_vertex_attrib_array(attribute_index);
    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, None);
}

static CURRENT_ID: AtomicU64 = AtomicU64::new(0);

pub fn next_id() -> String {
    format!(
        "id-{}",
        CURRENT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    )
}
