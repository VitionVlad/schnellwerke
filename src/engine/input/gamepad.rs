use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(module = "/src/engine/input/input.js")]
extern {
    pub type Jsgamepad;

    #[wasm_bindgen(constructor)]
    pub fn new() -> Jsgamepad;

    #[wasm_bindgen(method)]
    fn getgamepadnum(this: &Jsgamepad) -> i32;

    #[wasm_bindgen(method)]
    fn getgamepadbnum(this: &Jsgamepad, gi: i32) -> i32;

    #[wasm_bindgen(method)]
    fn getgamepadanum(this: &Jsgamepad, gi: i32) -> i32;

    #[wasm_bindgen(method)]
    fn getgamepadaxis(this: &Jsgamepad, gi: i32, ai: i32) -> f32;

    #[wasm_bindgen(method)]
    fn getbuttonpressed(this: &Jsgamepad, gi: i32, bi: i32) -> bool;
}

#[allow(dead_code)]
pub struct Gamepad{
    jsg: Jsgamepad,
}

impl Gamepad{
    #[allow(dead_code)]
    pub fn new() -> Gamepad{
        Gamepad{
            jsg: Jsgamepad::new(),
        }
    }
    #[allow(dead_code)]
    pub fn get_gamepad_number(&mut self) -> i32{
        return self.jsg.getgamepadnum();  
    }
    #[allow(dead_code)]
    pub fn get_gamepad_buttons_number(&mut self, gi: i32) -> i32{
        let gl = self.get_gamepad_number();
        if gl == 0 {
            return  0;
        }
        if gi < gl{
            return self.jsg.getgamepadbnum(gi);  
        }
        return self.jsg.getgamepadbnum(gl-1);  
    }
    #[allow(dead_code)]
    pub fn get_gamepad_axis_number(&mut self, gi: i32) -> i32{
        let gl = self.get_gamepad_number();
        if gl == 0 {
            return  0;
        }
        if gi < gl{
            return self.jsg.getgamepadanum(gi);  
        }
        return self.jsg.getgamepadanum(gl-1);  
    }
    #[allow(dead_code)]
    pub fn is_gamepad_button_pressed(&mut self, gi: i32, bi: i32) -> bool{
        let gl = self.get_gamepad_number();
        let l = self.get_gamepad_buttons_number(gi);
        if gl == 0{
            return false;
        }
        if gl > gi {
            if l == 0 {
                return false;
            }
            if bi < l {
                return self.jsg.getbuttonpressed(gi, bi);  
            }
            return self.jsg.getbuttonpressed(gi, l-1);
        }  
        if l == 0 {
            return false;
        }
        if bi < l {
            return self.jsg.getbuttonpressed(gl-1, bi);  
        }
        return self.jsg.getbuttonpressed(gl-1, l-1);
    }
    #[allow(dead_code)]
    pub fn get_gamepad_axis_value(&mut self, gi: i32, ai: i32) -> f32{
        let gl = self.get_gamepad_number();
        let l = self.get_gamepad_axis_number(gi);
        if gl == 0{
            return 0f32;
        }
        if gl > gi {
            if l == 0 {
                return 0f32;
            }
            if ai < l {
                return self.jsg.getgamepadaxis(gi, ai);  
            }
            return self.jsg.getgamepadaxis(gi, l-1);
        }  
        if l == 0 {
            return 0f32;
        }
        if ai < l {
            return self.jsg.getgamepadaxis(gl-1, ai);  
        }
        return self.jsg.getgamepadaxis(gl-1, l-1);
    }
}