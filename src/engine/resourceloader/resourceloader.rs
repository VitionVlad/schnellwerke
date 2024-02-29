use js_sys::Float32Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/engine/resourceloader/resloader.js")]
extern {
    pub type Jsrelod;
    #[wasm_bindgen(constructor)]
    pub fn new(iframeid: &str) -> Jsrelod;

    #[wasm_bindgen(method)]
    pub fn getvert(this: &Jsrelod) -> Float32Array;

    #[wasm_bindgen(method)]
    pub fn getuv(this: &Jsrelod) -> Float32Array;

    #[wasm_bindgen(method)]
    pub fn getnorm(this: &Jsrelod) -> Float32Array;

    #[wasm_bindgen(method)]
    pub fn getlen(this: &Jsrelod) -> i32;
}