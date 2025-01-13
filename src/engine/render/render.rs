use js_sys::Float32Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/engine/render/gfx.js")]
extern {
    pub type Gfxrender;
    #[wasm_bindgen(constructor)]
    pub fn new(canvasid: &str, renderscale: f32, shadowmapres: i32) -> Gfxrender;

    #[wasm_bindgen(method)]
    pub fn gfxgetcanvassizex(this: &Gfxrender) -> i32;

    #[wasm_bindgen(method)]
    pub fn gfxgetcanvassizey(this: &Gfxrender) -> i32;

    #[wasm_bindgen(method)]
    pub fn gfxsetrenderscale(this: &Gfxrender,renderscale: f32, mainpasslayers: u32);

    #[wasm_bindgen(method)]
    pub fn gfxsetshadowmapres(this: &Gfxrender,shadowmapres: i32, shadowmapcnt: u32);

    pub type Gfxmesh;
    #[wasm_bindgen(method)]
    pub fn preparesh(this: &Gfxmesh, shadowvertexcode: &str, cullMode: &str);

    #[wasm_bindgen(method)]
    pub fn createub(this: &Gfxmesh, ubol: i32);

    #[wasm_bindgen(method)]
    pub fn createshpipeline(this: &Gfxmesh, vertexcode: &str, cullMode: &str);

    #[wasm_bindgen(method)]
    pub fn queuepipeline(this: &Gfxmesh, svertexcode: &str, vertexcode: &str, fragmentcode: &str, cullMode: &str, shcullMode: &str);

    #[wasm_bindgen(method)]
    pub fn createpipeline(this: &Gfxmesh, gfx: &Gfxrender, vertexcode: &str, fragmentcode: &str, cullMode: &str);

    #[wasm_bindgen(constructor)]
    pub fn create(gfx: &Gfxrender, vertices: &Float32Array, uv: &Float32Array, normals: &Float32Array, tang: &Float32Array, bitang: &Float32Array, lenght: usize, vertexcode: &str, shadowvertexcode: &str, fragmentcode: &str, ubol: i32, texid: &str, cubeid: &str, magfilter: &str, minfilter: &str, cullMode: &str, shcullMode: &str, repeatmode: &str, usage: u32) -> Gfxmesh;

    #[wasm_bindgen(method)]
    pub fn set_ubo(this: &Gfxmesh, uniformValues: &Float32Array);

    #[wasm_bindgen(method)]
    pub fn will_render(this: &Gfxmesh, render: bool);

    pub type Gpucompute;
    #[wasm_bindgen(constructor)]
    pub fn createcompute(ibs: i32, obs: i32, code: &str) -> Gpucompute;

    #[wasm_bindgen(method)]
    pub fn execute(this: &Gpucompute, ib: &Float32Array, workgroupsize: i32);

    #[wasm_bindgen(method)]
    pub fn getstate(this: &Gpucompute) -> bool;

    #[wasm_bindgen(method)]
    pub fn getresult(this: &Gpucompute) -> Float32Array;
}

pub struct Render{
    pub jsren: Gfxrender,
}

impl Render{
    #[allow(dead_code)]
    pub fn init(canvasid: &str, renderscale: f32, shadowmapres: i32) -> Render{
        Render{
            jsren: Gfxrender::new(canvasid, renderscale, shadowmapres),
        }
    }
    #[allow(dead_code)]
    pub fn get_canvas_size_x(&self) -> i32{
        self.jsren.gfxgetcanvassizex()
    }
    #[allow(dead_code)]
    pub fn get_canvas_size_y(&self) -> i32{
        self.jsren.gfxgetcanvassizey()
    }
    #[allow(dead_code)]
    pub fn change_render_scale(&self, renderscale: f32, mainpasslayers: u32){
        self.jsren.gfxsetrenderscale(renderscale, mainpasslayers);
    }
    #[allow(dead_code)]
    pub fn change_shadow_map_resolution(&self, renderscale: i32, shadowmapcnt: u32){
        self.jsren.gfxsetshadowmapres(renderscale, shadowmapcnt);
    }
}