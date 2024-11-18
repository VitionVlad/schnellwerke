use super::{render::*, rloop::Rloop};
use js_sys::Float32Array;

pub struct Mesh{
    pub jsmesh: Gfxmesh,
    pub cullmode: String,
    pub shcullmode: String,
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum MUsages{
    ShadowAndMain,
    OnlyMain,
    OnlyShadow,
    PostProcessing,
}

impl Mesh{
    #[allow(dead_code)]
    pub fn create(gfx: &Render, rl: &mut Rloop, vertices: &Float32Array, uv: &Float32Array, normals: &Float32Array, tang: &Float32Array, bitang: &Float32Array, lenght: usize, vertexcode: &str, shadowvertexcode: &str, fragmentcode: &str, ubol: i32, texid: &str, cubeid: &str, magfilter: &str, minfilter: &str, cull_mode: &str, shcull_mode: &str, repeat_mode: &str, usage: MUsages) -> Mesh{
        let m = Gfxmesh::create(&gfx.jsren, &vertices, &uv, &normals, &tang, &bitang, lenght, vertexcode, shadowvertexcode, fragmentcode, ubol, texid, cubeid, magfilter, minfilter, cull_mode, shcull_mode, repeat_mode, usage as u32 + 1);
        rl.push_mesh(&m);
        Mesh{
            jsmesh: m,
            cullmode: cull_mode.to_string(),
            shcullmode: shcull_mode.to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn set_ubo(&mut self, ubo: &[f32]){
        let ub = Float32Array::new_with_length(ubo.len() as u32);
        ub.copy_from(&ubo);
        self.jsmesh.set_ubo(&ub);
    }
}