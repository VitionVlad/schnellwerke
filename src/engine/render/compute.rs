use super::render::*;
use js_sys::Float32Array;

pub struct Compute{
    jsc: Gpucompute,
    workgroupsize: i32,
    ibs: i32,
    obs: i32,
}

impl Compute{
    #[allow(dead_code)]
    pub fn create(ibs: i32, obs: i32, code: &str) -> Compute{
        Compute{
            jsc: Gpucompute::createcompute(ibs, obs, code),
            workgroupsize: 1,
            ibs: ibs,
            obs: obs,
        }
    }
    #[allow(dead_code)]
    pub fn execute(&self, ib: &[f32]){
        let jsi = js_sys::Float32Array::new_with_length((self.ibs) as u32);
        jsi.copy_from(&ib);
    }
}