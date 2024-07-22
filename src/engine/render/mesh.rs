use super::render::*;
use js_sys::Float32Array;

pub struct Mesh{
    pub jsmesh: Gfxmesh,
    len: i32,
}

impl Mesh{
    #[allow(dead_code)]
    pub fn create(gfx: &Render, vertices: &Float32Array, uv: &Float32Array, normals: &Float32Array, tang: &Float32Array, lenght: i32, vertexcode: &str, shadowvertexcode: &str, fragmentcode: &str, ubol: i32, texid: &str, cubeid: &str, magfilter: &str, minfilter: &str, cull_mode: &str, shcull_mode: &str, repeat_mode: &str, forpost: bool) -> Mesh{
        Mesh{
            jsmesh: Gfxmesh::create(&gfx.jsren, &vertices, &uv, &normals, &tang, lenght, vertexcode, shadowvertexcode, fragmentcode, ubol, texid, cubeid, magfilter, minfilter, cull_mode, shcull_mode, repeat_mode, forpost),
            len: ubol,
        }
    }
    #[allow(dead_code)]
    pub fn replace_vertices(&self, vertices: &Float32Array){
        self.jsmesh.writenewvertices(vertices);
    }
    #[allow(dead_code)]
    pub fn set_ubo(&self, ubo: &[f32]){
        let ub = Float32Array::new_with_length(self.len as u32);
        ub.copy_from(&ubo);
        self.jsmesh.set_ubo(&ub);
    }
}