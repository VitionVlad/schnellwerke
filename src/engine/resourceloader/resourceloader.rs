use js_sys::{Float32Array, JsString};
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

    pub type Jsloadsdf;
    #[wasm_bindgen(constructor)]
    pub fn new(iframeid: &str) -> Jsloadsdf;

    #[wasm_bindgen(method)]
    pub fn getmd(this: &Jsloadsdf) -> Float32Array;

    #[wasm_bindgen(method)]
    pub fn getmat(this: &Jsloadsdf) -> Float32Array;

    #[wasm_bindgen(method)]
    pub fn getlight(this: &Jsloadsdf) -> Float32Array;

    #[wasm_bindgen(method)]
    pub fn getspeaker(this: &Jsloadsdf) -> Float32Array;

    pub fn get_text_iframe(id: &str) -> JsString;
    pub fn remove_elem(id: &str);
}

#[allow(dead_code)]
pub fn get_text_from_iframe(id: &str) -> String{
    return get_text_iframe(id).into();
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

#[allow(dead_code)]
pub struct Sdfreader{
    load: Jsloadsdf,
    pub mdd: Vec<f32>,
    pub mat: Vec<f32>,
    pub light: Vec<f32>,
    pub speakers: Vec<f32>,
}

impl Sdfreader{
    #[allow(dead_code)]
    pub fn new(id: &str) -> Sdfreader{
        let t = Jsloadsdf::new(id);
        let v = t.getmd();
        let m = t.getmat();
        let l = t.getlight();
        let s = t.getspeaker();
        let mut a: Vec<f32> = vec![];
        let mut mat: Vec<f32> = vec![];
        let mut light: Vec<f32> = vec![];
        let mut sp: Vec<f32> = vec![];
        for i in 0..v.length(){
            a.push(v.get_index(i as u32));
        }
        for i in 0..m.length(){
            mat.push(m.get_index(i as u32));
        }
        for i in 0..l.length(){
            light.push(l.get_index(i as u32));
        }
        for i in 0..s.length(){
            sp.push(s.get_index(i as u32));
        }
        Sdfreader{
            load: t,
            mdd: a,
            mat: mat,
            light: light,
            speakers: sp,
        }
    }
}