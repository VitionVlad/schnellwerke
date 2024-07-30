use js_sys::Float32Array;

use super::{engine::Engine, math::{vec2::Vec2, vec3::Vec3}, render::mesh::Mesh};

pub struct Object{
    pub mesh: Mesh,
}

impl Object{
    pub fn new(eng: &Engine, vertex_data: Vec<f32>, vertexcode: &str, fragmentcode: &str, ubol: i32, texid: &str, cubeid: &str, for_post: bool) -> Object{
        let size = vertex_data.len()/8;
        let v = Float32Array::new_with_length((size*3) as u32);
        let u = Float32Array::new_with_length((size*2) as u32);
        let n = Float32Array::new_with_length((size*3) as u32);
        for i in 0..size*3{
            v.set_index(i as u32, vertex_data[i]);
        }
        for i in 0..size*2{
            u.set_index(i as u32, vertex_data[i+size*3]);
        }
        for i in 0..size*3{
            n.set_index(i as u32, vertex_data[i+size*5]);
        }
        let mut vcnt: u32 = 0;
        let jst = js_sys::Float32Array::new_with_length((size*3) as u32);
        for i in (0..v.length()).step_by(9){
            let v0 = Vec3::newdefined(v.get_index(i), v.get_index(i+1), v.get_index(i+2));
            let v1 = Vec3::newdefined(v.get_index(i+3), v.get_index(i+4), v.get_index(i+5));
            let v2 = Vec3::newdefined(v.get_index(i+6), v.get_index(i+7), v.get_index(i+8));
            let uv0 = Vec2::newdefined(u.get_index(vcnt), u.get_index(vcnt+1)+1.0);
            let uv1 = Vec2::newdefined(u.get_index(vcnt+2), u.get_index(vcnt+3)+1.0);
            let uv2 = Vec2::newdefined(u.get_index(vcnt+4), u.get_index(vcnt+5)+1.0);
            let deltapos1 = Vec3::newdefined(v1.x-v0.x, v1.y-v0.y, v1.z-v0.z);
            let deltapos2 = Vec3::newdefined(v2.x-v0.x, v2.y-v0.y, v2.z-v0.z);
            let delta_uv1 = Vec2::newdefined(uv1.x-uv0.x, uv1.y-uv0.y);
            let delta_uv2 = Vec2::newdefined(uv2.x-uv0.x, uv2.y-uv0.y);
            let r = 1.0f32 / (delta_uv1.x * delta_uv2.y - delta_uv1.y * delta_uv2.x);
            jst.set_index(i, (deltapos1.x * delta_uv2.y - deltapos2.x * delta_uv1.y)*r);
            jst.set_index(i+1, (deltapos1.y * delta_uv2.y - deltapos2.y * delta_uv1.y)*r);
            jst.set_index(i+2, (deltapos1.z * delta_uv2.y - deltapos2.z * delta_uv1.y)*r);
            jst.set_index(i+3, (deltapos1.x * delta_uv2.y - deltapos2.x * delta_uv1.y)*r);
            jst.set_index(i+4, (deltapos1.y * delta_uv2.y - deltapos2.y * delta_uv1.y)*r);
            jst.set_index(i+5, (deltapos1.z * delta_uv2.y - deltapos2.z * delta_uv1.y)*r);
            jst.set_index(i+6, (deltapos1.x * delta_uv2.y - deltapos2.x * delta_uv1.y)*r);
            jst.set_index(i+7, (deltapos1.y * delta_uv2.y - deltapos2.y * delta_uv1.y)*r);
            jst.set_index(i+8, (deltapos1.z * delta_uv2.y - deltapos2.z * delta_uv1.y)*r);
            vcnt+=6
        }
        Object{
            mesh: Mesh::create(&eng.render, &v, &u, &n, &jst, size, vertexcode, &eng.shadow_code, fragmentcode, ubol, texid, cubeid, &eng.render.magfilter, &eng.render.minfilter, &eng.render.culling_mode, &eng.render.culling_mode_shadow, &eng.render.repeat_mode, for_post)
        }
    }
}