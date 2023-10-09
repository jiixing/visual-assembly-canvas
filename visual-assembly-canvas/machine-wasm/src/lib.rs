extern crate console_error_panic_hook;

mod utils;

mod controller;

use std::panic;
use machine::{Execute, Machine};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(text: &str);
}

#[wasm_bindgen]
pub fn setup_system() {
    // Setup panic hook.
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
pub fn load_machine(source: &str) -> Vec<u16> {
    let mut m: Machine = source.into();
    m.handlers.print.push(Box::new(|s| log(s)));
    m.run();

    let stack = m.mem.read_stack(10);
    stack
}