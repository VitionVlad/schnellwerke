use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(module = "/src/engine/input/input.js")]
extern {
    pub type Jsmouse;

    #[wasm_bindgen(constructor)]
    pub fn new() -> Jsmouse;

    #[wasm_bindgen(method)]
    fn jgetx(this: &Jsmouse) -> i32;

    #[wasm_bindgen(method)]
    fn jgety(this: &Jsmouse) -> i32;

    #[wasm_bindgen(method)]
    fn getmlc(this: &Jsmouse) -> bool;

    #[wasm_bindgen(method)]
    fn getmmc(this: &Jsmouse) -> bool;

    #[wasm_bindgen(method)]
    fn getmrc(this: &Jsmouse) -> bool;
}

pub struct Mouse{
    jmouse: Jsmouse,
}

impl Mouse{
    #[allow(dead_code)]
    pub fn new() -> Mouse{
        Mouse{
            jmouse: Jsmouse::new(),
        }
    }
    #[allow(dead_code)]
    pub fn get_x_coords(&mut self) -> i32{
        self.jmouse.jgetx()
    }
    #[allow(dead_code)]
    pub fn get_y_coords(&mut self) -> i32{
        self.jmouse.jgety()
    }
    #[allow(dead_code)]
    pub fn get_left_mouse_button(&mut self) -> bool{
        self.jmouse.getmlc()
    }
    #[allow(dead_code)]
    pub fn get_middle_mouse_button(&mut self) -> bool{
        self.jmouse.getmmc()
    }
    #[allow(dead_code)]
    pub fn get_right_mouse_button(&mut self) -> bool{
        self.jmouse.getmrc()
    }
}