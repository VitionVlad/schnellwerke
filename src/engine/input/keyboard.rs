use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(module = "/src/engine/input/input.js")]
extern {
    pub type Jskeyboard;

    #[wasm_bindgen(constructor)]
    pub fn new() -> Jskeyboard;

    #[wasm_bindgen(method)]
    fn getkey(this: &Jskeyboard, keyid: i32) -> i8;

    #[wasm_bindgen(method)]
    fn getlastkey(this: &Jskeyboard) -> i32;
}

pub struct Keyboard{
    jsk: Jskeyboard,
}

impl Keyboard{
    #[allow(dead_code)]
    pub fn new() -> Keyboard{
        Keyboard{
            jsk: Jskeyboard::new(),
        }
    }
    #[allow(dead_code)]
    pub fn is_key_pressed(&mut self, keycode: i32) -> bool{
        return self.jsk.getkey(keycode) == 1;  
    }
    #[allow(dead_code)]
    pub fn last_key_pressed(&mut self) -> i32{
        return self.jsk.getlastkey();
    }
}