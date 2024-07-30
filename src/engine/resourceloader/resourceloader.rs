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
    pub arr: Vec<f32>,
}

impl Objreader{
    #[allow(dead_code)]
    pub fn new(id: &str) -> Objreader{
        let t = Jsrelod::new(id);
        let v = t.getvert();
        let u = t.getuv();
        let n = t.getnorm();
        let mut a: Vec<f32> = vec![];
        for i in 0..t.getlen()*3{// n/8
            a.push(v.get_index(i as u32));
        }
        for i in 0..t.getlen()*2{
            a.push(u.get_index(i as u32));
        }
        for i in 0..t.getlen()*3{
            a.push(n.get_index(i as u32));
        }
        Objreader{
            load: t,
            arr: a,
        }
    }
}