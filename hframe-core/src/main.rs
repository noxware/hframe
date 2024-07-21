use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/lib.js")]
extern "C" {
    #[wasm_bindgen(js_name = doSomething)]
    fn do_something();
}

#[wasm_bindgen(start)]
fn run() {
    do_something();
}

fn main() {
    console_error_panic_hook::set_once();

    do_something();
}
