use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(module = "/src/engine/input/mouse.js")]
extern {
    fn jgetx() -> i32;
    fn jgety() -> i32;
    fn getmlc() -> bool;
}

#[allow(dead_code)]
pub fn get_mouse_x() -> i32{
    jgetx()
}

#[allow(dead_code)]
pub fn get_mouse_y() -> i32{
    jgety()
}

#[allow(dead_code)]
pub fn get_mouse_left_click() -> bool{
    getmlc()
}