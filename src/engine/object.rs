use super::{engine::Engine, math::{mat4::Mat4, uniformstruct::{getsize, Uniformstruct, Usages}, vec2::Vec2, vec3::Vec3}, physics::physics::PHYSICS_GPU, render::{compute::Compute, mesh::Mesh}, resourceloader::resourceloader::Objreader};
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
    pub modelvert: Vec<f32>,
    pub speed: Vec3,
    pub camera_collision_interact: bool,
    pub is_interacting: f32,
}

impl Object {
    #[allow(dead_code)]
    pub fn new(eng: &Engine, vertices: &[f32], uv: &[f32], normals: &[f32], lenght: i32, vertexcode: &str, shadowvertexcode: &str, fragmentcode: &str, unifroms: &Vec<Uniformstruct>, texid: &str, cubeid: &str, magfilter: &str, minfilter: &str, cull_mode: &str, shcull_mode: &str, repeat_mode: &str, forpost: bool) -> Object{
        let jsvert = js_sys::Float32Array::new_with_length((lenght*4) as u32);
        jsvert.copy_from(&vertices);

        let jsuv = js_sys::Float32Array::new_with_length((lenght*2) as u32);
        jsuv.copy_from(&uv);

        let jsn = js_sys::Float32Array::new_with_length((lenght*3) as u32);
        jsn.copy_from(&normals);
        let ubol: i32 = getsize(unifroms);

        let mut tang: Vec<f32> = Vec::new();
        let mut vcnt: usize = 0;
        for i in (0..vertices.len()).step_by(12){
            let v0 = Vec3::newdefined(vertices[i], vertices[i+1], vertices[i+2]);
            let v1 = Vec3::newdefined(vertices[i+3], vertices[i+4], vertices[i+5]);
            let v2 = Vec3::newdefined(vertices[i+6], vertices[i+7], vertices[i+8]);

            let uv0 = Vec2::newdefined(uv[vcnt], uv[vcnt+1]+1.0);
            let uv1 = Vec2::newdefined(uv[vcnt+2], uv[vcnt+3]+1.0);
            let uv2 = Vec2::newdefined(uv[vcnt+4], uv[vcnt+5]+1.0);

            let deltapos1 = Vec3::newdefined(v1.x-v0.x, v1.y-v0.y, v1.z-v0.z);
            let deltapos2 = Vec3::newdefined(v2.x-v0.x, v2.y-v0.y, v2.z-v0.z);

            let delta_uv1 = Vec2::newdefined(uv1.x-uv0.x, uv1.y-uv0.y);
            let delta_uv2 = Vec2::newdefined(uv2.x-uv0.x, uv2.y-uv0.y);

            let r = 1.0f32 / (delta_uv1.x * delta_uv2.y - delta_uv1.y * delta_uv2.x);

            tang.push((deltapos1.x * delta_uv2.y - deltapos2.x * delta_uv1.y)*r);
            tang.push((deltapos1.y * delta_uv2.y - deltapos2.y * delta_uv1.y)*r);
            tang.push((deltapos1.z * delta_uv2.y - deltapos2.z * delta_uv1.y)*r);

            tang.push((deltapos1.x * delta_uv2.y - deltapos2.x * delta_uv1.y)*r);
            tang.push((deltapos1.y * delta_uv2.y - deltapos2.y * delta_uv1.y)*r);
            tang.push((deltapos1.z * delta_uv2.y - deltapos2.z * delta_uv1.y)*r);

            tang.push((deltapos1.x * delta_uv2.y - deltapos2.x * delta_uv1.y)*r);
            tang.push((deltapos1.y * delta_uv2.y - deltapos2.y * delta_uv1.y)*r);
            tang.push((deltapos1.z * delta_uv2.y - deltapos2.z * delta_uv1.y)*r);
            vcnt+=6
        }
        let jst = js_sys::Float32Array::new_with_length((lenght*3) as u32);
        jst.copy_from(&tang.as_slice());
        Object { 
            mesh: Mesh::create(&eng.ren, &jsvert, &jsuv, &jsn, &jst, lenght, vertexcode, shadowvertexcode, fragmentcode, ubol, texid, cubeid, magfilter, minfilter, cull_mode, shcull_mode, repeat_mode, forpost),
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
            speed: Vec3::new(),
            camera_collision_interact: true,
            is_interacting: 0.0f32,
        }
    }
    #[allow(dead_code)]
    pub fn new_from_obj(eng: &Engine, modelid: &str, vertexcode: &str, shadowvertexcode: &str, fragmentcode: &str, unifroms: &Vec<Uniformstruct>, texid: &str, cubeid: &str, magfilter: &str, minfilter: &str, cull_mode: &str, shcull_mode: &str, repeat_mode: &str, forpost: bool) -> Object{
        let md = Objreader::new(modelid);
        let jsvert = js_sys::Float32Array::new_with_length((md.size*4) as u32);
        jsvert.copy_from(&md.vert.as_slice());

        let jsuv = js_sys::Float32Array::new_with_length((md.size*2) as u32);
        jsuv.copy_from(&md.uv.as_slice());

        let jsn = js_sys::Float32Array::new_with_length((md.size*3) as u32);
        jsn.copy_from(&md.norm.as_slice());
        let ubol: i32 = getsize(unifroms);

        let mut tang: Vec<f32> = Vec::new();
        let mut vcnt: usize = 0;
        for i in (0..md.vert.len()).step_by(12){
            let v0 = Vec3::newdefined(md.vert[i], md.vert[i+1], md.vert[i+2]);
            let v1 = Vec3::newdefined(md.vert[i+3], md.vert[i+4], md.vert[i+5]);
            let v2 = Vec3::newdefined(md.vert[i+6], md.vert[i+7], md.vert[i+8]);

            let uv0 = Vec2::newdefined(md.uv[vcnt], 1.0f32-md.uv[vcnt+1]);
            let uv1 = Vec2::newdefined(md.uv[vcnt+2], 1.0f32-md.uv[vcnt+3]);
            let uv2 = Vec2::newdefined(md.uv[vcnt+4], 1.0f32-md.uv[vcnt+5]);

            let deltapos1 = Vec3::newdefined(v1.x-v0.x, v1.y-v0.y, v1.z-v0.z);
            let deltapos2 = Vec3::newdefined(v2.x-v0.x, v2.y-v0.y, v2.z-v0.z);

            let delta_uv1 = Vec2::newdefined(uv1.x-uv0.x, uv1.y-uv0.y);
            let delta_uv2 = Vec2::newdefined(uv2.x-uv0.x, uv2.y-uv0.y);

            let r = 1.0f32 / (delta_uv1.x * delta_uv2.y - delta_uv1.y * delta_uv2.x);

            tang.push((deltapos1.x * delta_uv2.y - deltapos2.x * delta_uv1.y)*r);
            tang.push((deltapos1.y * delta_uv2.y - deltapos2.y * delta_uv1.y)*r);
            tang.push((deltapos1.z * delta_uv2.y - deltapos2.z * delta_uv1.y)*r);

            tang.push((deltapos1.x * delta_uv2.y - deltapos2.x * delta_uv1.y)*r);
            tang.push((deltapos1.y * delta_uv2.y - deltapos2.y * delta_uv1.y)*r);
            tang.push((deltapos1.z * delta_uv2.y - deltapos2.z * delta_uv1.y)*r);

            tang.push((deltapos1.x * delta_uv2.y - deltapos2.x * delta_uv1.y)*r);
            tang.push((deltapos1.y * delta_uv2.y - deltapos2.y * delta_uv1.y)*r);
            tang.push((deltapos1.z * delta_uv2.y - deltapos2.z * delta_uv1.y)*r);
            vcnt+=6
        }
        let jst = js_sys::Float32Array::new_with_length((tang.len()) as u32);
        jst.copy_from(&tang.as_slice());
        Object { 
            mesh: Mesh::create(&eng.ren, &jsvert, &jsuv, &jsn, &jst, md.size, vertexcode, shadowvertexcode, fragmentcode, ubol, texid, cubeid, magfilter, minfilter, cull_mode, shcull_mode, repeat_mode, forpost),
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
            speed: Vec3::new(),
            camera_collision_interact: true,
            is_interacting: 0.0f32,
        }
    }
    #[allow(dead_code)]
    pub fn new_plane(eng: &Engine, vertexcode: &str, shadowvertexcode: &str, fragmentcode: &str, unifroms: &Vec<Uniformstruct>, texid: &str, cubeid: &str, magfilter: &str, minfilter: &str, cull_mode: &str, shcull_mode: &str, repeat_mode: &str, forpost: bool) -> Object{
        let vertices: [f32; 24] = [
        -1.0, -1.0, 1.0, 1.0,
        -1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0, 1.0,

        -1.0, -1.0, 1.0, 1.0,
        1.0, 1.0, 1.0, 1.0,
        1.0, -1.0, 1.0, 1.0
    ];
    let uv: [f32; 12] = [
        0.0, 1.0,
        0.0, 0.0,
        1.0, 0.0,

        0.0, 1.0,
        1.0, 0.0,
        1.0, 1.0,
    ];
    let normals: [f32; 18] = [
        0.0, 0.0, 1.0,
        0.0, 0.0, 1.0,
        0.0, 0.0, 1.0,

        0.0, 0.0, 1.0,
        0.0, 0.0, 1.0,
        0.0, 0.0, 1.0
    ];
    Object::new(eng, &vertices, &uv, &normals, 6, vertexcode, shadowvertexcode, fragmentcode, unifroms, texid, cubeid, magfilter, minfilter, cull_mode, shcull_mode, repeat_mode, forpost)
    }
    #[allow(dead_code)]
    pub fn new_cube(eng: &Engine, vertexcode: &str, shadowvertexcode: &str, fragmentcode: &str, unifroms: &Vec<Uniformstruct>, texid: &str, cubeid: &str, magfilter: &str, minfilter: &str, cull_mode: &str, shcull_mode: &str, repeat_mode: &str, forpost: bool) -> Object{
        let vertices: [f32; 144] = [
            -1f32, 1f32, -1f32, 1.0f32,
            1f32, 1f32, 1f32, 1.0f32,
            1f32, 1f32, -1f32, 1.0f32,
            1f32, 1f32, 1f32, 1.0f32,
            -1f32, -1f32, 1f32, 1.0f32,
            1f32, -1f32, 1f32, 1.0f32,
            -1f32, 1f32, 1f32, 1.0f32,
            -1f32, -1f32, -1f32, 1.0f32,
            -1f32, -1f32, 1f32, 1.0f32,
            1f32, -1f32, -1f32, 1.0f32,
            -1f32, -1f32, 1f32, 1.0f32,
            -1f32, -1f32, -1f32, 1.0f32,
            1f32, 1f32, -1f32, 1.0f32,
            1f32, -1f32, 1f32, 1.0f32,
            1f32, -1f32, -1f32, 1.0f32,
            -1f32, 1f32, -1f32, 1.0f32,
            1f32, -1f32, -1f32, 1.0f32,
            -1f32, -1f32, -1f32, 1.0f32,
            -1f32, 1f32, -1f32, 1.0f32,
            -1f32, 1f32, 1f32, 1.0f32,
            1f32, 1f32, 1f32, 1.0f32,
            1f32, 1f32, 1f32, 1.0f32,
            -1f32, 1f32, 1f32, 1.0f32,
            -1f32, -1f32, 1f32, 1.0f32,
            -1f32, 1f32, 1f32, 1.0f32,
            -1f32, 1f32, -1f32, 1.0f32,
            -1f32, -1f32, -1f32, 1.0f32,
            1f32, -1f32, -1f32, 1.0f32,
            1f32, -1f32, 1f32, 1.0f32,
            -1f32, -1f32, 1f32, 1.0f32,
            1f32, 1f32, -1f32, 1.0f32,
            1f32, 1f32, 1f32, 1.0f32,
            1f32, -1f32, 1f32, 1.0f32,
            -1f32, 1f32, -1f32, 1.0f32,
            1f32, 1f32, -1f32, 1.0f32,
            1f32, -1f32, -1f32, 1.0f32,
        ];
        let uv: [f32; 72] = [
            0.875f32, 0.5f32, 
            0.625f32, 0.75f32, 
            0.625f32, 0.5f32, 
            0.625f32, 0.75f32, 
            0.375f32, 1f32, 
            0.375f32, 0.75f32, 
            0.625f32, 0f32, 
            0.375f32, 0.25f32, 
            0.375f32, 0f32, 
            0.375f32, 0.5f32, 
            0.125f32, 0.75f32, 
            0.125f32, 0.5f32, 
            0.625f32, 0.5f32, 
            0.375f32, 0.75f32, 
            0.375f32, 0.5f32, 
            0.625f32, 0.25f32, 
            0.375f32, 0.5f32, 
            0.375f32, 0.25f32, 
            0.875f32, 0.5f32, 
            0.875f32, 0.75f32, 
            0.625f32, 0.75f32, 
            0.625f32, 0.75f32, 
            0.625f32, 1f32, 
            0.375f32, 1f32, 
            0.625f32, 0f32, 
            0.625f32, 0.25f32, 
            0.375f32, 0.25f32, 
            0.375f32, 0.5f32, 
            0.375f32, 0.75f32, 
            0.125f32, 0.75f32, 
            0.625f32, 0.5f32, 
            0.625f32, 0.75f32, 
            0.375f32, 0.75f32, 
            0.625f32, 0.25f32, 
            0.625f32, 0.5f32, 
            0.375f32, 0.5f32, 
        ];
        let normals: [f32; 108] = [
            -0f32, 1f32, -0f32, 
            -0f32, 1f32, -0f32, 
            -0f32, 1f32, -0f32, 
            -0f32, -0f32, 1f32, 
            -0f32, -0f32, 1f32, 
            -0f32, -0f32, 1f32, 
            -1f32, -0f32, -0f32, 
            -1f32, -0f32, -0f32, 
            -1f32, -0f32, -0f32, 
            -0f32, -1f32, -0f32, 
            -0f32, -1f32, -0f32, 
            -0f32, -1f32, -0f32, 
            1f32, -0f32, -0f32, 
            1f32, -0f32, -0f32, 
            1f32, -0f32, -0f32, 
            -0f32, -0f32, -1f32, 
            -0f32, -0f32, -1f32, 
            -0f32, -0f32, -1f32, 
            -0f32, 1f32, -0f32, 
            -0f32, 1f32, -0f32, 
            -0f32, 1f32, -0f32, 
            -0f32, -0f32, 1f32, 
            -0f32, -0f32, 1f32, 
            -0f32, -0f32, 1f32, 
            -1f32, -0f32, -0f32, 
            -1f32, -0f32, -0f32, 
            -1f32, -0f32, -0f32, 
            -0f32, -1f32, -0f32, 
            -0f32, -1f32, -0f32, 
            -0f32, -1f32, -0f32, 
            1f32, -0f32, -0f32, 
            1f32, -0f32, -0f32, 
            1f32, -0f32, -0f32, 
            -0f32, -0f32, -1f32, 
            -0f32, -0f32, -1f32, 
            -0f32, -0f32, -1f32, 
        ];
        Object::new(eng, &vertices, &uv, &normals, 36, vertexcode, shadowvertexcode, fragmentcode, unifroms, texid, cubeid, magfilter, minfilter, cull_mode, shcull_mode, repeat_mode, forpost)
    }
    #[allow(dead_code)]
    pub fn new_cube_planeuv(eng: &Engine, vertexcode: &str, shadowvertexcode: &str, fragmentcode: &str, unifroms: &Vec<Uniformstruct>, texid: &str, cubeid: &str, magfilter: &str, minfilter: &str, cull_mode: &str, shcull_mode: &str, repeat_mode: &str, forpost: bool) -> Object{
        let vertices: [f32; 144] = [
            -1f32, 1f32, -1f32, 1.0f32,
            1f32, 1f32, 1f32, 1.0f32,
            1f32, 1f32, -1f32, 1.0f32,
            1f32, 1f32, 1f32, 1.0f32,
            -1f32, -1f32, 1f32, 1.0f32,
            1f32, -1f32, 1f32, 1.0f32,
            -1f32, 1f32, 1f32, 1.0f32,
            -1f32, -1f32, -1f32, 1.0f32,
            -1f32, -1f32, 1f32, 1.0f32,
            1f32, -1f32, -1f32, 1.0f32,
            -1f32, -1f32, 1f32, 1.0f32,
            -1f32, -1f32, -1f32, 1.0f32,
            1f32, 1f32, -1f32, 1.0f32,
            1f32, -1f32, 1f32, 1.0f32,
            1f32, -1f32, -1f32, 1.0f32,
            -1f32, 1f32, -1f32, 1.0f32,
            1f32, -1f32, -1f32, 1.0f32,
            -1f32, -1f32, -1f32, 1.0f32,
            -1f32, 1f32, -1f32, 1.0f32,
            -1f32, 1f32, 1f32, 1.0f32,
            1f32, 1f32, 1f32, 1.0f32,
            1f32, 1f32, 1f32, 1.0f32,
            -1f32, 1f32, 1f32, 1.0f32,
            -1f32, -1f32, 1f32, 1.0f32,
            -1f32, 1f32, 1f32, 1.0f32,
            -1f32, 1f32, -1f32, 1.0f32,
            -1f32, -1f32, -1f32, 1.0f32,
            1f32, -1f32, -1f32, 1.0f32,
            1f32, -1f32, 1f32, 1.0f32,
            -1f32, -1f32, 1f32, 1.0f32,
            1f32, 1f32, -1f32, 1.0f32,
            1f32, 1f32, 1f32, 1.0f32,
            1f32, -1f32, 1f32, 1.0f32,
            -1f32, 1f32, -1f32, 1.0f32,
            1f32, 1f32, -1f32, 1.0f32,
            1f32, -1f32, -1f32, 1.0f32,
        ];
        let uv: [f32; 72] = [
            0.0, 1.0,
            0.0, 0.0,
            1.0, 0.0,
            0.0, 1.0,
            1.0, 0.0,
            1.0, 1.0,
            0.0, 1.0,
            0.0, 0.0,
            1.0, 0.0,
            0.0, 1.0,
            1.0, 0.0,
            1.0, 1.0,
            0.0, 1.0,
            0.0, 0.0,
            1.0, 0.0,
            0.0, 1.0,
            1.0, 0.0,
            1.0, 1.0,
            0.0, 1.0,
            0.0, 0.0,
            1.0, 0.0,
            0.0, 1.0,
            1.0, 0.0,
            1.0, 1.0,
            0.0, 1.0,
            0.0, 0.0,
            1.0, 0.0,
            0.0, 1.0,
            1.0, 0.0,
            1.0, 1.0,
            0.0, 1.0,
            0.0, 0.0,
            1.0, 0.0,
            0.0, 1.0,
            1.0, 0.0,
            1.0, 1.0,
        ];
        let normals: [f32; 108] = [
            -0f32, 1f32, -0f32, 
            -0f32, 1f32, -0f32, 
            -0f32, 1f32, -0f32, 
            -0f32, -0f32, 1f32, 
            -0f32, -0f32, 1f32, 
            -0f32, -0f32, 1f32, 
            -1f32, -0f32, -0f32, 
            -1f32, -0f32, -0f32, 
            -1f32, -0f32, -0f32, 
            -0f32, -1f32, -0f32, 
            -0f32, -1f32, -0f32, 
            -0f32, -1f32, -0f32, 
            1f32, -0f32, -0f32, 
            1f32, -0f32, -0f32, 
            1f32, -0f32, -0f32, 
            -0f32, -0f32, -1f32, 
            -0f32, -0f32, -1f32, 
            -0f32, -0f32, -1f32, 
            -0f32, 1f32, -0f32, 
            -0f32, 1f32, -0f32, 
            -0f32, 1f32, -0f32, 
            -0f32, -0f32, 1f32, 
            -0f32, -0f32, 1f32, 
            -0f32, -0f32, 1f32, 
            -1f32, -0f32, -0f32, 
            -1f32, -0f32, -0f32, 
            -1f32, -0f32, -0f32, 
            -0f32, -1f32, -0f32, 
            -0f32, -1f32, -0f32, 
            -0f32, -1f32, -0f32, 
            1f32, -0f32, -0f32, 
            1f32, -0f32, -0f32, 
            1f32, -0f32, -0f32, 
            -0f32, -0f32, -1f32, 
            -0f32, -0f32, -1f32, 
            -0f32, -0f32, -1f32, 
        ];
        Object::new(eng, &vertices, &uv, &normals, 36, vertexcode, shadowvertexcode, fragmentcode, unifroms, texid, cubeid, magfilter, minfilter, cull_mode, shcull_mode, repeat_mode, forpost)
    }
    #[allow(dead_code)]
    pub fn collision_calc(&mut self, mmat: Mat4, pos: Vec3, size: Vec3, speed: Vec3){
        self.incomp.resize(self.comp.ibs as usize, 0f32);
            for i in 0..16 {
                self.incomp[i] = mmat.mat[i];
            }
            self.incomp[16] = pos.x;
            self.incomp[17] = pos.y;
            self.incomp[18] = pos.z;
            self.incomp[19] = size.x;
            self.incomp[20] = size.y;
            self.incomp[21] = size.z;
            self.incomp[22] = speed.x;
            self.incomp[23] = speed.y;
            self.incomp[24] = speed.z;
            self.incomp[25] = self.incomp.len() as f32;
            for i in 26..self.modelvert.len()+26 {
                self.incomp[i] = self.modelvert[i-26];
            }
            self.comp.execute(&self.incomp);
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
           self.collision_calc(mmat, eng.pos, eng.size, eng.speed)
        }
        for i in 0..unifroms.len(){
            self.mat = eng.projection;
            self.smat = eng.shadowprojection;

            self.smat.mul(&mmat);

            self.mat.transpose();
            self.smat.transpose();
            mmat.transpose();
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
                    for b in 0..16{
                        self.jsarr.set_index(b+self.inuniform, mmat.mat[b as usize]);
                    }
                    self.inuniform+=16;
                },
                Usages::Smvpmat => {
                    for b in 0..16{
                        self.jsarr.set_index(b+self.inuniform, self.smat.mat[b as usize]);
                    }
                    self.inuniform+=16;
                },
            }
        }
        self.mesh.draw(&eng.ren, self.jsarr.clone());
        self.is_interacting = self.comp.out_buf[0];
        if self.comp.out_buf[0] == 1f32 && !eng.inshadow && self.camera_collision_interact{
            eng.speed.y = 0f32;
        }
        if self.comp.out_buf[0] == 2f32 && !eng.inshadow && self.camera_collision_interact{
            eng.speed.y = 0f32;
            eng.speed.x = 0f32;
            eng.speed.z = 0f32;
        }
        self.pos.sum(self.speed);
        self.speed = Vec3::new();
    }
    #[allow(dead_code)]
    pub fn replace_vertices(&mut self, vertices: &[f32], lenght: i32){
        self.modelvert = vertices.to_vec();
        let jsvert = js_sys::Float32Array::new_with_length((lenght*4) as u32);
        jsvert.copy_from(&vertices);
        self.mesh.replace_vertices(&jsvert);
    }
}