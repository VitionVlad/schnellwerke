use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(module = "/src/engine/input/input.js")]
extern {
    pub type Jstouch;

    #[wasm_bindgen(constructor)]
    pub fn new() -> Jstouch;

    #[wasm_bindgen(method)]
    fn jgettx(this: &Jstouch) -> i32;

    #[wasm_bindgen(method)]
    fn jgetty(this: &Jstouch) -> i32;

    #[wasm_bindgen(method)]
    fn jgetuse(this: &Jstouch) -> bool;

    #[wasm_bindgen(method)]
    fn jsettouchindex(this: &Jstouch, lindex: i32);
}

#[allow(dead_code)]
pub struct Touch{
    jstouch: Jstouch,
}

impl Touch {
    #[allow(dead_code)]
    pub fn new() -> Touch {
        Touch{
            jstouch: Jstouch::new(),
        }
    }
    #[allow(dead_code)]
    pub fn set_touch_index(&mut self, index: i32){
        self.jstouch.jsettouchindex(index);
    }
    #[allow(dead_code)]
    pub fn is_touching(&mut self) -> bool{
        self.jstouch.jgetuse()
    }
    #[allow(dead_code)]
    pub fn get_x_touch(&mut self) -> i32{
        self.jstouch.jgettx()
    }
    #[allow(dead_code)]
    pub fn get_y_touch(&mut self) -> i32{
        self.jstouch.jgetty()
    }
}