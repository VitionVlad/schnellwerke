use wasm_bindgen::prelude::*;

use crate::{Gfxmesh, Gfxrender, Render};

#[wasm_bindgen(module = "/src/engine/render/gfx.js")]
extern {
    pub type Jsloop;
    #[wasm_bindgen(constructor)]
    pub fn new(gfxr: &Gfxrender) -> Jsloop;

    #[wasm_bindgen(method)]
    pub fn push_mesh(this: &Jsloop, mesh: &Gfxmesh, index: i32);

    #[wasm_bindgen(method)]
    pub fn set_render(this: &Jsloop, reb: &Gfxrender);

    #[wasm_bindgen(method)]
    pub fn drawloop(this: &Jsloop);

    pub fn snlll(fun: &Closure<dyn FnMut()>, to: u32);
}

#[allow(dead_code)]
pub struct Rloop{
    rloop: Jsloop,
}

impl Rloop {
    #[allow(dead_code)]
    pub fn new(ren: &Render) -> Rloop{
        Rloop{
            rloop: Jsloop::new(&ren.jsren)
        }
    }
    #[allow(dead_code)]
    pub fn push_mesh(&mut self, mesh: &Gfxmesh, index: i32){
        self.rloop.push_mesh(&mesh, index);
    }
    #[allow(dead_code)]
    pub fn set_render(&mut self, ren: Render){
        self.rloop.set_render(&ren.jsren);
    }
    #[allow(dead_code)]
    pub fn drawloop(&mut self){
        self.rloop.drawloop();
    }
}

pub fn logic_loop(fun: Closure<dyn FnMut()>, to: u32){
    snlll(&fun, to);
    fun.forget();
}