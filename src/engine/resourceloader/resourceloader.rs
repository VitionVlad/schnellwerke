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

#[allow(dead_code)]
pub struct Objreader{
    load: Jsrelod,
    pub vert: Float32Array,
    pub uv: Float32Array,
    pub norm: Float32Array,
    pub size: i32,
}

impl Objreader{
    #[allow(dead_code)]
    pub fn new(id: &str) -> Objreader{
        let t = Jsrelod::new(id);
        let v = t.getvert();
        let u = t.getuv();
        let n = t.getnorm();
        let l = t.getlen();
        Objreader{
            load: t,
            vert: v,
            uv: u,
            norm: n,
            size: l,
        }
    }
}