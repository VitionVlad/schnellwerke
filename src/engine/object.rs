use js_sys::Float32Array;

use super::{engine::Engine, material::Material, math::{mat4::Mat4, uniformstruct::{Uniformstruct, Usages}, vec2::Vec2, vec3::Vec3}, physics::{getpoints, PhysicsObject}, render::mesh::{MUsages, Mesh}};

#[allow(dead_code)]
pub struct Object{
    pub mesh: Mesh,
    pub physic_object: PhysicsObject,
    pub ubo: Vec<f32>,
    pub render: bool,
    pub uniforms: Vec<Uniformstruct>,
    index: i32,
    addsize: i32,
    vc: String,
    svc: String,
    fc: String,
}

impl Object{
    #[allow(dead_code)]
    pub fn new(eng: &mut Engine, vertex_data: Vec<f32>, material: &Material, usage: MUsages, is_static: bool) -> Object{
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
        let mut index = -1;
        for i in 0..eng.mindeces.len(){
            if eng.mindeces[i] == false {
                index = i as i32;
                eng.mindeces[i] = true;
            }
        }
        if index == -1 {
            index = eng.mindeces.len() as i32;
            eng.mindeces.push(true);
        }
        Object{
            mesh: Mesh::create(&eng.render, &mut eng.rloop, index, &v, &u, &n, &jst, &jst2, size, &vc, &svc, &fc, 64+material.ubo_size, &material.tex_ids, &material.cube_ids, &material.magfilter, &material.minfilter, &material.culling_mode, &material.culling_mode_shadow, &material.repeat_mode, usage),
            physic_object: PhysicsObject::new(getpoints(v.to_vec()), is_static),
            ubo: vec![0f32, 0f32, 0f32, 0f32],
            uniforms: material.uniforms.clone(),
            index: index,
            render: true,
            addsize: material.ubo_size,
            vc: material.vertex_shader.to_owned(),
            svc: material.uniend.to_owned() + &eng.shadow_code,
            fc: material.fragment_shader.to_owned(),
        }
    }
    #[allow(dead_code)]
    pub fn exec(&mut self, eng: &mut Engine){
        self.mesh.render = self.render;
        let ubeg = eng.uniform_beg.to_owned();
        let mut smats = 0;
        for i in 0..eng.last_light_size{
            if eng.lights[i].shadow {
                smats+=1;
            }
        }
        self.ubo.resize(20*eng.last_cam_size+52+smats*16+eng.last_light_size*8 + self.addsize as usize, 0f32);
        
        for i in 0..(20*eng.last_cam_size+4+smats*16+eng.last_light_size*8){
            self.ubo[i] = eng.ubo_beg_values[i];
        }

        let mut mmat = Mat4::new();
        mmat.trans(self.physic_object.pos);
        mmat.transpose();
        for i in 0..16{
            self.ubo[20*eng.last_cam_size+4+smats*16+eng.last_light_size*8+i] = mmat.mat[i];
        }

        mmat = Mat4::new();
        mmat.yrot(-self.physic_object.rot.y);
        let mut t: Mat4 = Mat4::new();
        t.xrot(-self.physic_object.rot.x);
        mmat.mul(&t);
        t = Mat4::new();
        t.zrot(-self.physic_object.rot.z);
        mmat.mul(&t);
        for i in 0..16{
            self.ubo[20*eng.last_cam_size+20+smats*16+eng.last_light_size*8+i] = mmat.mat[i];
        }


        mmat = Mat4::new();
        mmat.scale(self.physic_object.scale);
        mmat.transpose();
        for i in 0..16{
            self.ubo[20*eng.last_cam_size+36+smats*16+eng.last_light_size*8+i] = mmat.mat[i];
        }

        let mut es = 0;
        for i in 0..self.uniforms.len(){
            self.ubo[20*eng.last_cam_size+52+smats*16+eng.last_light_size*8+i+es] = mmat.mat[i];
            match self.uniforms[i].usage {
                Usages::Float => {
                    self.ubo[20*eng.last_cam_size+52+smats*16+eng.last_light_size*8+i+es] = self.uniforms[i].float;
                    es+=1;
                },
                Usages::Vec2 => {
                    self.ubo[20*eng.last_cam_size+52+smats*16+eng.last_light_size*8+i+es] = self.uniforms[i].vec2.x;
                    self.ubo[20*eng.last_cam_size+52+smats*16+eng.last_light_size*8+i+es+1] = self.uniforms[i].vec2.y;
                    es+=2;
                },
                Usages::Vec3 => {
                    self.ubo[20*eng.last_cam_size+52+smats*16+eng.last_light_size*8+i+es] = self.uniforms[i].vec3.x;
                    self.ubo[20*eng.last_cam_size+52+smats*16+eng.last_light_size*8+i+es+1] = self.uniforms[i].vec3.y;
                    self.ubo[20*eng.last_cam_size+52+smats*16+eng.last_light_size*8+i+es+3] = self.uniforms[i].vec3.z;
                    es+=3;
                },
                Usages::Vec4 => {
                    self.ubo[20*eng.last_cam_size+52+smats*16+eng.last_light_size*8+i+es] = self.uniforms[i].vec4.x;
                    self.ubo[20*eng.last_cam_size+52+smats*16+eng.last_light_size*8+i+es+1] = self.uniforms[i].vec4.y;
                    self.ubo[20*eng.last_cam_size+52+smats*16+eng.last_light_size*8+i+es+2] = self.uniforms[i].vec4.z;
                    self.ubo[20*eng.last_cam_size+52+smats*16+eng.last_light_size*8+i+es+3] = self.uniforms[i].vec4.w;
                    es+=4;
                },
                Usages::Mat => {
                    for j in 0..16 {
                        self.ubo[20*eng.last_cam_size+52+smats*16+eng.last_light_size*8+i+es+j] = self.uniforms[i].mat.mat[j];
                    }
                    es+=16;
                },
            }
        }

        self.mesh.set_ubo(&self.ubo);
        if eng.rec_pipeline {
            self.mesh.jsmesh.queuepipeline(&(eng.uniform_beg.to_owned() + &self.svc),  &(ubeg.to_owned() + &self.vc), &(ubeg.to_owned() + &self.fc), &self.mesh.cullmode, &self.mesh.shcullmode);
        }

        self.physic_object.reset_states();
        self.physic_object.exec();
        for i in 0..eng.last_cam_size{
            eng.cameras[i].physic_object.interact_with_other_object(self.physic_object);
        }
    }
    #[allow(dead_code)]
    pub fn allow_replacing(&mut self, eng: &mut Engine){
        eng.mindeces[self.index as usize] = false;
    }
}