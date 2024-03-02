use super::{engine::Engine, math::{mat4::Mat4, uniformstruct::{getsize, Uniformstruct, Usages}, vec3::Vec3}, render::mesh::Mesh, resourceloader::resourceloader::Objreader, render::compute::Compute, physics::physics::PHYSICS_GPU};
use js_sys::Float32Array;

#[allow(dead_code)]
pub struct Object{
    pub mesh: Mesh,
    jsarr: Float32Array,
    inuniform: u32,
    pub pos: Vec3,
    pub rot: Vec3,
    pub scale: Vec3,
    mat: Mat4,
    smat: Mat4,
    pub comp: Compute,
    incomp: Vec<f32>,
    pub collision_detect: bool,
    modelvert: Vec<f32>,
}

impl Object {
    #[allow(dead_code)]
    pub fn new(eng: &Engine, vertices: &[f32], uv: &[f32], normals: &[f32], lenght: i32, vertexcode: &str, shadowvertexcode: &str, fragmentcode: &str, unifroms: &Vec<Uniformstruct>, texid: &str, magfilter: &str, minfilter: &str, forpost: bool) -> Object{
        let jsvert = js_sys::Float32Array::new_with_length((lenght*4) as u32);
        jsvert.copy_from(&vertices);

        let jsuv = js_sys::Float32Array::new_with_length((lenght*2) as u32);
        jsuv.copy_from(&uv);

        let jsn = js_sys::Float32Array::new_with_length((lenght*3) as u32);
        jsn.copy_from(&normals);
        let ubol: i32 = getsize(unifroms);
        Object { 
            mesh: Mesh::create(&eng.ren, jsvert, jsuv, jsn, lenght, vertexcode, shadowvertexcode, fragmentcode, ubol, texid, magfilter, minfilter, forpost),
            jsarr: Float32Array::new_with_length((ubol/4) as u32),
            inuniform: 0,
            pos: Vec3::new(),
            rot: Vec3::new(),
            scale: Vec3::newdefined(1f32, 1f32, 1f32),
            mat: Mat4::new(),
            smat: Mat4::new(),
            comp: Compute::create((lenght*4+23) as u32, 4, PHYSICS_GPU),
            incomp: Vec::with_capacity((lenght*4+23) as usize),
            collision_detect: true,
            modelvert: vertices.to_vec(),
        }
    }
    #[allow(dead_code)]
    pub fn new_from_obj(eng: &Engine, modelid: &str, vertexcode: &str, shadowvertexcode: &str, fragmentcode: &str, unifroms: &Vec<Uniformstruct>, texid: &str, magfilter: &str, minfilter: &str, forpost: bool) -> Object{
        let md = Objreader::new(modelid);
        let jsvert = js_sys::Float32Array::new_with_length((md.size*4) as u32);
        jsvert.copy_from(&md.vert.as_slice());

        let jsuv = js_sys::Float32Array::new_with_length((md.size*2) as u32);
        jsuv.copy_from(&md.uv.as_slice());

        let jsn = js_sys::Float32Array::new_with_length((md.size*3) as u32);
        jsn.copy_from(&md.norm.as_slice());
        let ubol: i32 = getsize(unifroms);
        Object { 
            mesh: Mesh::create(&eng.ren, jsvert, jsuv, jsn, md.size, vertexcode, shadowvertexcode, fragmentcode, ubol, texid, magfilter, minfilter, forpost),
            jsarr: Float32Array::new_with_length((ubol/4) as u32),
            inuniform: 0,
            pos: Vec3::new(),
            rot: Vec3::new(),
            scale: Vec3::newdefined(1f32, 1f32, 1f32),
            mat: Mat4::new(),
            smat: Mat4::new(),
            comp: Compute::create((md.size*4+26) as u32, 4, PHYSICS_GPU),
            incomp: Vec::new(),
            collision_detect: true,
            modelvert: md.vert,
        }
    }
    #[allow(dead_code)]
    pub fn draw(&mut self, eng: &mut Engine, unifroms: &Vec<Uniformstruct>){
        self.inuniform = 0;
        let mut mmat = Mat4::new();
        mmat.scale(self.scale);

        let mut t: Mat4 = Mat4::new();
        t.xrot(self.rot.x);
        mmat.mul(&t);

        t = Mat4::new();
        t.yrot(self.rot.y);
        mmat.mul(&t);

        t = Mat4::new();
        t.zrot(self.rot.z);
        mmat.mul(&t);

        t = Mat4::new();
        t.trans(self.pos);
        mmat.mul(&t);
        if self.collision_detect && !eng.inshadow{
            self.incomp.resize(self.comp.ibs as usize, 0f32);
            for i in 0..16 {
                self.incomp[i] = mmat.mat[i];
            }
            self.incomp[16] = eng.pos.x;
            self.incomp[17] = eng.pos.y;
            self.incomp[18] = eng.pos.z;
            self.incomp[19] = eng.size.x;
            self.incomp[20] = eng.size.y;
            self.incomp[21] = eng.size.z;
            self.incomp[22] = eng.speed.x;
            self.incomp[23] = eng.speed.y;
            self.incomp[24] = eng.speed.z;
            self.incomp[25] = self.incomp.len() as f32;
            for i in 26..self.modelvert.len()+26 {
                self.incomp[i] = self.modelvert[i-26];
            }
            self.comp.execute(&self.incomp);
        }
        for i in 0..unifroms.len(){
            self.mat = eng.projection;
            self.smat = eng.shadowprojection;

            self.mat.mul(&mmat);
            self.smat.mul(&mmat);

            self.mat.transpose();
            self.smat.transpose();
            match unifroms[i].usage {
                Usages::Float => {
                    self.jsarr.set_index(self.inuniform, unifroms[i].float);
                    self.inuniform+=1;
                },
                Usages::Vec2 => {
                    self.jsarr.set_index(self.inuniform, unifroms[i].vec2.x);
                    self.inuniform+=1;
                    self.jsarr.set_index(self.inuniform, unifroms[i].vec2.y);
                    self.inuniform+=1;
                },
                Usages::Vec3 => {
                    self.jsarr.set_index(self.inuniform, unifroms[i].vec3.x);
                    self.inuniform+=1;
                    self.jsarr.set_index(self.inuniform, unifroms[i].vec3.y);
                    self.inuniform+=1;
                    self.jsarr.set_index(self.inuniform, unifroms[i].vec3.z);
                    self.inuniform+=1;
                },
                Usages::Vec4 => {
                    self.jsarr.set_index(self.inuniform, unifroms[i].vec4.x);
                    self.inuniform+=1;
                    self.jsarr.set_index(self.inuniform, unifroms[i].vec4.y);
                    self.inuniform+=1;
                    self.jsarr.set_index(self.inuniform, unifroms[i].vec4.z);
                    self.inuniform+=1;
                    self.jsarr.set_index(self.inuniform, unifroms[i].vec4.w);
                    self.inuniform+=1;
                },
                Usages::Mat => {
                    for b in 0..16{
                        self.jsarr.set_index(b+self.inuniform, unifroms[i].mat.mat[b as usize]);
                    }
                    self.inuniform+=16;
                },
                Usages::Mvpmat => {
                    for b in 0..16{
                        self.jsarr.set_index(b+self.inuniform, self.mat.mat[b as usize]);
                    }
                    self.inuniform+=16;
                },
                Usages::Smvpmat => {
                    for b in 0..16{
                        self.jsarr.set_index(b+self.inuniform, self.mat.mat[b as usize]);
                    }
                    self.inuniform+=16;
                },
            }
        }
        self.mesh.draw(&eng.ren, self.jsarr.clone());
        if self.comp.out_buf[0] == 1f32 && !eng.inshadow{
            eng.speed.y = 0f32;
        }
        if self.comp.out_buf[0] == 2f32 && !eng.inshadow{
            eng.speed.y = 0f32;
            eng.speed.x = 0f32;
            eng.speed.z = 0f32;
        }
    }
}