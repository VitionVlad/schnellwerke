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

    pub type Jsloadsdf;
    #[wasm_bindgen(constructor)]
    pub fn new(iframeid: &str) -> Jsloadsdf;

    #[wasm_bindgen(method)]
    pub fn getmd(this: &Jsloadsdf) -> Float32Array;

    #[wasm_bindgen(method)]
    pub fn getcb(this: &Jsloadsdf) -> Float32Array;

    #[wasm_bindgen(method)]
    pub fn getcu(this: &Jsloadsdf) -> Float32Array;

    #[wasm_bindgen(method)]
    pub fn getpl(this: &Jsloadsdf) -> Float32Array;
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
    pub cdd: Vec<f32>,
    pub cdu: Vec<f32>,
    pub pl: Vec<f32>,
}

impl Sdfreader{
    #[allow(dead_code)]
    pub fn new(id: &str) -> Sdfreader{
        let t = Jsloadsdf::new(id);
        let v = t.getmd();
        let c = t.getcb();
        let u = t.getcu();
        let p = t.getpl();
        let mut a: Vec<f32> = vec![];
        let mut b: Vec<f32> = vec![];
        let mut cu: Vec<f32> = vec![];
        let mut pl: Vec<f32> = vec![];
        for i in 0..v.length(){
            a.push(v.get_index(i as u32));
        }
        for i in 0..c.length(){
            b.push(c.get_index(i as u32));
        }
        for i in 0..u.length(){
            cu.push(u.get_index(i as u32));
        }
        for i in 0..p.length(){
            pl.push(p.get_index(i as u32));
        }
        Sdfreader{
            load: t,
            mdd: a,
            cdd: b,
            cdu: cu,
            pl: pl,
        }
    }
}