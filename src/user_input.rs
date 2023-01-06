use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::JsCast;

use wasm_bindgen::prelude::Closure;

#[derive(Default, Clone)]
pub struct UserInput {
    pub move_left: bool,
    pub move_right: bool,
    pub throw_rod: bool,
}

pub struct InputHandler {
    current_state: Rc<RefCell<UserInput>>,
}

impl InputHandler {
    pub fn new() -> Self {
        InputHandler {
            current_state: Rc::new(RefCell::new(UserInput::default())),
        }
    }

    pub fn attach(&mut self) {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();

        {
            let state = self.current_state.clone();
            let closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
                let mut s = state.borrow_mut();

                s.move_left = event.key() == "a";
                s.move_right = event.key() == "d";
                s.throw_rod = event.key() == " ";
            }) as Box<dyn FnMut(_)>);
            canvas
                .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())
                .unwrap();
            closure.forget();
        }
        {
            let state = self.current_state.clone();
            let closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
                let mut s = state.borrow_mut();

                if s.move_left && event.key() == "a" {
                    s.move_left = false;
                }
                if s.move_right && event.key() == "d" {
                    s.move_right = false;
                }
            }) as Box<dyn FnMut(_)>);
            canvas
                .add_event_listener_with_callback("keyup", closure.as_ref().unchecked_ref())
                .unwrap();
            closure.forget();
        }
    }

    pub fn current_state(&self) -> UserInput {
        let state = self.current_state.borrow();
        state.clone()
    }

    pub fn after_update(&mut self) {
        let mut state = self.current_state.borrow_mut();
        state.throw_rod = false;
    }
}
