use js_sys::Float32Array;

use super::{engine::Engine, light::LightType, material::Material, math::{mat4::Mat4, vec2::Vec2, vec3::Vec3}, render::mesh::{MUsages, Mesh}};

#[allow(dead_code)]
pub struct Object{
    pub mesh: Mesh,
    pub pos: Vec3,
    pub rot: Vec3,
    pub scale: Vec3,
    pub ubo: Vec<f32>,
    startsize: i32,
    addsize: i32,
    vc: String,
    svc: String,
    fc: String,
}

impl Object{
    #[allow(dead_code)]
    pub fn new(eng: &Engine, vertex_data: Vec<f32>, material: Material, usage: MUsages) -> Object{
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
        let jst2 = js_sys::Float32Array::new_with_length((size*3) as u32);
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

            jst2.set_index(i,   (deltapos2.x * delta_uv1.x - deltapos1.x * delta_uv2.x)*r);
            jst2.set_index(i+1, (deltapos2.y * delta_uv1.x - deltapos1.y * delta_uv2.x)*r);
            jst2.set_index(i+2, (deltapos2.z * delta_uv1.x - deltapos1.z * delta_uv2.x)*r);
            jst2.set_index(i+3, (deltapos2.x * delta_uv1.x - deltapos1.x * delta_uv2.x)*r);
            jst2.set_index(i+4, (deltapos2.y * delta_uv1.x - deltapos1.y * delta_uv2.x)*r);
            jst2.set_index(i+5, (deltapos2.z * delta_uv1.x - deltapos1.z * delta_uv2.x)*r);
            jst2.set_index(i+6, (deltapos2.x * delta_uv1.x - deltapos1.x * delta_uv2.x)*r);
            jst2.set_index(i+7, (deltapos2.y * delta_uv1.x - deltapos1.y * delta_uv2.x)*r);
            jst2.set_index(i+8, (deltapos2.z * delta_uv1.x - deltapos1.z * delta_uv2.x)*r);
            vcnt+=6
        }
        let vc = eng.uniform_beg.to_string() + &material.vertex_shader;
        let fc = eng.uniform_beg.to_string() + &material.fragment_shader;
        let svc = eng.uniform_beg.to_string() + &material.uniend + &eng.shadow_code;
        let mut smats = 0;
        for i in 0..eng.lights.len(){
            smats+=1;
            if eng.lights[i].light_type == LightType::Point{
                smats+=5;
            }
        }
        let startsize: i32 = (20*eng.cameras.len()+20+smats*16+eng.lights.len()*8) as i32;
        Object{
            mesh: Mesh::create(&eng.render, &v, &u, &n, &jst, &jst2, size, &vc, &svc, &fc, 64+material.ubo_size, &material.tex_ids, &material.cube_ids, &material.magfilter, &material.minfilter, &material.culling_mode, &material.culling_mode_shadow, &material.repeat_mode, usage),
            pos: Vec3::new(),
            rot: Vec3::new(),
            scale: Vec3::newdefined(1f32, 1f32, 1f32),
            ubo: vec![0f32, 0f32, 0f32, 0f32],
            startsize: startsize,
            addsize: material.ubo_size,
            vc: material.vertex_shader,
            svc: material.uniend + &eng.shadow_code,
            fc: material.fragment_shader,
        }
    }
    #[allow(dead_code)]
    pub fn exec(&mut self, eng: &mut Engine){
        let ubeg = eng.uniform_beg.to_owned();
        let mut smats = 0;
        for i in 0..eng.lights.len(){
            smats+=1;
            if eng.lights[i].light_type == LightType::Point{
                smats+=5;
            }
        }
        self.ubo.resize(20*eng.cameras.len()+20+smats*16+eng.lights.len()*8 + self.addsize as usize, 0f32);
        self.startsize = (20*eng.cameras.len()+20+smats*16+eng.lights.len()*8) as i32;
        
        for i in 0..(20*eng.cameras.len()+4+smats*16+eng.lights.len()*8){
            self.ubo[i] = eng.ubo_beg_values[i];
        }

        let mut mmat = Mat4::new();
        mmat.trans(self.pos);
        let mut t: Mat4 = Mat4::new();
        t.yrot(self.rot.y);
        mmat.mul(&t);
        t = Mat4::new();
        t.xrot(self.rot.x);
        mmat.mul(&t);
        t = Mat4::new();
        t.zrot(self.rot.z);
        mmat.mul(&t);
        t = Mat4::new();
        t.scale(self.scale);
        mmat.mul(&t);
        mmat.transpose();

        for i in 0..16{
            self.ubo[20*eng.cameras.len()+4+smats*16+eng.lights.len()*8+i] = mmat.mat[i];
        }
        self.mesh.set_ubo(&self.ubo);
        if eng.rec_pipeline {
            self.mesh.jsmesh.queuepipeline(&(eng.uniform_beg.to_owned() + &self.svc),  &(ubeg.to_owned() + &self.vc), &(ubeg.to_owned() + &self.fc), &self.mesh.cullmode, &self.mesh.shcullmode);
        }
    }
}