#![allow(dead_code)]
#![allow(unused_variables)]

use js_sys::{Float32Array, Uint8Array};
use wasm_bindgen::prelude::{wasm_bindgen, Closure};

#[wasm_bindgen(module = "/src/engine/render/gauss.js")]
unsafe extern "C"{
    fn get_frametime(eh: u32) -> f32;
    fn get_resx(eh: u32) -> u32;
    fn get_resy(eh: u32) -> u32;
    fn setresolution(eh: u32, xs: u32, ys: u32);
    fn seticon(eh: u32, xs: u32, ys: u32, pixels: Uint8Array);
    fn settitle(title: &str);
    fn setfullscreen(eh: u32);
    fn quitfullscreen(eh: u32);
    fn getKeyPressed(index: u32) -> bool;
    fn getmr() -> bool;
    fn getml() -> bool;
    fn getmm() -> bool;
    fn get_mouse_posx()  -> f32;
    fn get_mouse_posy()  -> f32;
    fn get_mouse_stat()  -> bool;
    fn req_mouse_lock(eh: u32);
    fn req_mouse_unlock(eh: u32);
    fn modifyshadowdata(eh: u32, ncnt: u32, nres: u32);
    fn modifydeffereddata(eh: u32, ncnt: u32, nres: f32);
    fn modifyshadowuniform(eh: u32, pos: u32, value: f32);
    fn modifydeffereduniform(eh: u32, pos: u32, value: f32);
    fn neweng(canvasid: &str) -> u32;
    fn destroy(eh: u32);
    fn newmaterial(vert: &str, frag: &str, shadow: &str, cullmode: u32, scullmode: u32) -> u32;
    fn newmodel(vert: Float32Array, uv: Float32Array, normals: Float32Array, tan: Float32Array, ctan: Float32Array) -> u32;
    fn setrendercamera(eme: u32, val: i8);
    fn setmeshbuf(eme: u32, i: u32, val: f32);
    fn setdrawable(eme: u32, val: i8);
    fn newmesh(eh: u32, es: u32, em: u32, te: u32, usage: u32) -> u32;
    fn newtexture(xsize: u32, ysize: u32, zsize: u32, pixels: Uint8Array) -> u32;
    fn rn(eh: u32);
    fn renderloop(fun: &Closure<dyn FnMut()>);
}

pub fn render_loop(fun: Closure<dyn FnMut()>){
    renderloop(&fun);
    fun.forget();
}

#[derive(Copy, Clone)]
pub struct Render{
    pub euclid: u32,
    pub shadow_map_resolution: u32,
    pub shadow_map_count: u32,
    pub camera_count: u32,
    pub resolution_scale: f32,
    pub resolution_x: u32,
    pub resolution_y: u32,
    pub fullscreen: bool,
    pub frametime: f32,
    fullscreeno: bool,
}

impl Render{
    pub fn new(canvasid: &str) -> Render{
        Render { 
            euclid: neweng(canvasid),
            shadow_map_count: 1,
            shadow_map_resolution: 1000,
            camera_count: 1,
            resolution_scale: 1f32,
            resolution_x: 800,
            resolution_y: 600,
            fullscreen: false,
            fullscreeno: false,
            frametime: 0.0,
        }
    }
    pub fn exec(&mut self){
        rn(self.euclid);
        self.resolution_x = get_resx(self.euclid);
        self.resolution_y = get_resy(self.euclid);
        if self.fullscreen != self.fullscreeno {
            match self.fullscreen{
                true => setfullscreen(self.euclid),
                false => quitfullscreen(self.euclid),
            }
            self.fullscreeno = self.fullscreen;
        }
        modifyshadowdata(self.euclid, self.shadow_map_count, self.shadow_map_resolution);
        modifydeffereddata(self.euclid, self.camera_count, self.resolution_scale);
        self.frametime = get_frametime(self.euclid);
    }
    pub fn set_shadow_uniform_data(&self, i: u32, value: f32){
        modifyshadowuniform(self.euclid, i, value);
    }
    pub fn set_deffered_uniform_data(&self, i: u32, value: f32){
        modifydeffereduniform(self.euclid, i, value);
    }
    pub fn set_new_resolution(&self, resx: u32, resy: u32){
        setresolution(self.euclid, resx, resy);
    }
    pub fn set_icon(&self, resx: u32, resy: u32, data: Vec<i8>){
    }
    pub fn set_title(&self, title: &str){
        settitle(title);
    }
    pub fn destroy(&self){
        destroy(self.euclid);
    }
}

#[derive(Copy, Clone)]
pub struct Control{
    euclid: u32,
    pub xpos: f32,
    pub ypos: f32,
    pub mouse_lock: bool,
    old_mouse_lock: bool,
    pub mousebtn: [bool; 3],
}

impl Control{
    pub fn new(render: Render) -> Control{
        Control {
            euclid: render.euclid,
            xpos: 0.0f32,
            ypos: 0.0f32,
            mouse_lock: false,
            old_mouse_lock: false,
            mousebtn: [false, false, false],
        }
    }
    pub fn get_key_state(&self, keyindex: u32) -> bool{
        return getKeyPressed(keyindex);
    }
    pub fn get_mouse_pos(&mut self){
        if self.mouse_lock != self.old_mouse_lock{
            match self.mouse_lock {
                true => req_mouse_lock(self.euclid),
                false => req_mouse_unlock(self.euclid),
            }
        }else{
            self.mouse_lock = get_mouse_stat();
        }
        self.old_mouse_lock = self.mouse_lock;
        self.xpos = get_mouse_posx();
        self.ypos = get_mouse_posy();
        self.mousebtn = [ getmr(), getmm(), getml()];
    }
}

#[derive(Copy, Clone)]
pub enum CullMode {
    CullModeNone = 0,
    CullModeFrontBit = 0x00000001,
    CullModeBackBit = 0x00000002,
    CullModeFrontAndBack = 0x00000003,
}

#[derive(Copy, Clone)]
pub struct MaterialShaders{
    pub materialid: u32,
}

impl MaterialShaders{
    pub fn new(ren: Render, vert: Vec<u8>, frag: Vec<u8>, shadow: Vec<u8>, cullmode: CullMode, shadow_cullmode: CullMode) -> MaterialShaders{
        MaterialShaders { 
            materialid: newmaterial(&String::from_utf8(vert).unwrap(), &String::from_utf8(frag).unwrap(), &String::from_utf8(shadow).unwrap(), cullmode as u32, shadow_cullmode as u32)
        }
    }
}

#[derive(Copy, Clone)]
pub struct Vertexes{
    pub modelid: u32,
}

impl Vertexes{
    pub fn new(ren: Render, vertices: Vec<f32>) -> Vertexes{
        let size = vertices.len()/8;
        let mut v: Vec<f32> = vec![];
        let mut uv: Vec<f32> = vec![];
        let mut n: Vec<f32> = vec![];
        let mut tg: Vec<f32> = vec![];
        let mut ctg: Vec<f32> = vec![];
        for i in 0..size*3 {
            v.push(vertices[i]);
        }
        for i in 0..size*2 {
            uv.push(vertices[i+size*3]);
        }
        for i in 0..size*3 {
            n.push(vertices[i+size*5]);
        }
        let mut u = 0;
        for i in (0..size*3).step_by(9){
            let v0: [f32; 3] = [ v[i], v[i+1], v[i+2] ];
            let v1: [f32; 3] = [ v[i+3], v[i+4], v[i+5] ];
            let v2: [f32; 3] = [ v[i+6], v[i+7], v[i+8] ];
            let uv0: [f32; 2] = [ uv[u], uv[u+1]+1.0f32 ];
            let uv1: [f32; 2] = [ uv[u+2], uv[u+3]+1.0f32 ];
            let uv2: [f32; 2] = [ uv[u+4], uv[u+5]+1.0f32 ];
            let deltapos1: [f32; 3] = [ v1[0]-v0[0], v1[1]-v0[1], v1[2]-v0[2]];
            let deltapos2: [f32; 3] = [ v2[0]-v0[0], v2[1]-v0[1], v2[2]-v0[2]];
            let delta_uv1: [f32; 2] = [uv1[0]-uv0[0], uv1[1]-uv0[1]];
            let delta_uv2: [f32; 2] = [uv2[0]-uv0[0], uv2[1]-uv0[1]];
            let r = 1.0 / (delta_uv1[0] * delta_uv2[1] - delta_uv1[1] * delta_uv2[0]);
            tg.push((deltapos1[0] * delta_uv2[1] - deltapos2[0] * delta_uv1[1])*r);
            tg.push((deltapos1[1] * delta_uv2[1] - deltapos2[1] * delta_uv1[1])*r);
            tg.push((deltapos1[2] * delta_uv2[1] - deltapos2[2] * delta_uv1[1])*r);
            tg.push((deltapos1[0] * delta_uv2[1] - deltapos2[0] * delta_uv1[1])*r);
            tg.push((deltapos1[1] * delta_uv2[1] - deltapos2[1] * delta_uv1[1])*r);
            tg.push((deltapos1[2] * delta_uv2[1] - deltapos2[2] * delta_uv1[1])*r);
            tg.push((deltapos1[0] * delta_uv2[1] - deltapos2[0] * delta_uv1[1])*r);
            tg.push((deltapos1[1] * delta_uv2[1] - deltapos2[1] * delta_uv1[1])*r);
            tg.push((deltapos1[2] * delta_uv2[1] - deltapos2[2] * delta_uv1[1])*r);
            ctg.push((deltapos2[0] * delta_uv1[0] - deltapos1[0] * delta_uv2[0])*r);
            ctg.push((deltapos2[1] * delta_uv1[0] - deltapos1[1] * delta_uv2[0])*r);
            ctg.push((deltapos2[2] * delta_uv1[0] - deltapos1[2] * delta_uv2[0])*r);
            ctg.push((deltapos2[0] * delta_uv1[0] - deltapos1[0] * delta_uv2[0])*r);
            ctg.push((deltapos2[1] * delta_uv1[0] - deltapos1[1] * delta_uv2[0])*r);
            ctg.push((deltapos2[2] * delta_uv1[0] - deltapos1[2] * delta_uv2[0])*r);
            ctg.push((deltapos2[0] * delta_uv1[0] - deltapos1[0] * delta_uv2[0])*r);
            ctg.push((deltapos2[1] * delta_uv1[0] - deltapos1[1] * delta_uv2[0])*r);
            ctg.push((deltapos2[2] * delta_uv1[0] - deltapos1[2] * delta_uv2[0])*r);
            u+=6;
        }

        let vjs = Float32Array::new_with_length((size*3) as u32);
        vjs.copy_from(&v);
        let ujs = Float32Array::new_with_length((size*2) as u32);
        ujs.copy_from(&uv);
        let njs = Float32Array::new_with_length((size*3) as u32);
        njs.copy_from(&n);
        let tjs = Float32Array::new_with_length((size*3) as u32);
        tjs.copy_from(&tg);
        let cjs = Float32Array::new_with_length((size*3) as u32);
        cjs.copy_from(&ctg);

        Vertexes { 
            modelid: newmodel(vjs, ujs, njs, tjs, cjs),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Texture{
    pub texid: u32,
}

impl Texture {
    pub fn new(render: Render, xs: u32, ys: u32, texnm: u32, data: Vec<i8>) -> Texture{
        let mut u8d: Vec<u8> = vec![];
        for i in 0..data.len(){
            u8d.push(data[i] as u8);
        }
        let jsi = Uint8Array::new_with_length(xs*ys*texnm*4);
        jsi.copy_from(&u8d);
        Texture { 
            texid: newtexture(xs, ys, texnm, jsi),
        }
    }
}

#[derive(Copy, Clone)]
#[derive(PartialEq)]
pub enum MeshUsage {
    LightingPass = 0,
    DefferedPass = 1,
    ShadowPass = 2,
    ShadowAndDefferedPass = 3,
}

#[derive(Copy, Clone)]
pub struct Mesh{
    pub meshid: u32,
    pub ubo: [f32; 20],
    pub draw: bool,
    pub draw_shadow: bool,
    pub keep_shadow: bool,
    pub render_all_cameras: bool,
    pub exclude_selected_camera: bool,
    pub camera_number: i8,
}

impl Mesh{
    pub fn new(ren: Render, model: Vertexes, material: MaterialShaders, texture: Texture, usage: MeshUsage) -> Mesh{
        Mesh { 
            meshid: newmesh(ren.euclid, material.materialid, model.modelid, texture.texid, usage as u32),
            ubo: [1.0; 20],
            draw: true,
            draw_shadow: true,
            keep_shadow: true,
            render_all_cameras: true,
            exclude_selected_camera: false,
            camera_number: 0,
        }
    }

    pub fn exec(&self){
        for i in 0..20{
            setmeshbuf(self.meshid, i, self.ubo[i as usize]);
        }
        setdrawable(self.meshid, match self.draw {
            true => match self.draw_shadow {
                true => 1,
                false => 3,
            },
            false => match self.keep_shadow {
                true => 2,
                false => 0,
            },
        });
        setrendercamera(self.meshid, match self.render_all_cameras{
            true => -1,
            false => {
                if self.exclude_selected_camera {
                    self.camera_number + 10
                }else{
                    self.camera_number
                }
            }
        });
    }
}