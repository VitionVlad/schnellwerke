use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(module = "/src/engine/input/keyboard.js")]
extern {
    fn getkey(keyid: i32) -> i8;
}


#[allow(dead_code)]
pub fn is_key_pressed(keycode: i32) -> bool{
    return getkey(keycode) == 1;
}