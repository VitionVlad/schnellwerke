use engine::{audio::audiocontext::AudioContext, input::{gamepad::Gamepad, keyboard::Keyboard, mouse::Mouse, touch::Touch}, light::{Light, LightType}, physics::PhysicsObject, render::rloop::Rloop};
use crate::*;
use super::camera::Camera;

#[allow(dead_code)]
pub struct Engine{
    pub rloop: Rloop,
    pub render: Render,
    pub renderscale: f32,
    pub shadowmap_resolution: i32,
    pub cameras: Vec<Camera>,
    pub lights: Vec<Light>,
    pub ubo_beg_values: Vec<f32>,
    pub uniform_beg: String,
    pub last_cam_size: usize,
    pub last_light_size: usize,
    last_renderscale: f32,
    last_shs: i32,
    last_smat: usize,
    pub shadow_code: String,
    pub rec_pipeline: bool,
    pub keyboard: Keyboard,
    pub mouse: Mouse,
    pub touch: Touch,
    pub gamepads: Gamepad,
    sr: bool,
    pub audioctx: AudioContext,
}

impl Engine {
    #[allow(dead_code)]
    pub fn new(canvasid: &str) -> Engine{
        let ren = Render::init(canvasid, 1f32, 1000); 
        Engine{
            rloop: Rloop::new(&ren),
            render: ren,
            renderscale: 1.0f32,
            shadowmap_resolution: 1000,
            cameras: vec![Camera{ physic_object: PhysicsObject::new(vec![Vec3::newdefined(0.1, 0f32, 0.1), Vec3::newdefined(-0.1, -5f32, -0.1)], false), fov: 90f32, znear: 0.1f32, zfar: 100f32, is_orthographic: false }],
            lights: vec![Light::new(LightType::Directional)],
            ubo_beg_values: vec![0f32, 0f32, 0f32, 0f32],
            uniform_beg: "
            const LIGHTN = 1;
            const LIGHTMN = 1;
            const CAMN = 1;
            struct uniforms {
                eng: vec4f,
                mvp: array<mat4x4<f32>, 1>,
                pos: array<vec4f, 1>,
                smvp: array<mat4x4<f32>, 1>,
                lpos: array<vec4f, 1>,
                lcolor: array<vec4f, 1>,
                model: mat4x4<f32>,".to_string(),
            last_cam_size: 1,
            last_light_size: 1,
            last_renderscale: 1f32,
            last_shs: 1000,
            last_smat: 1,
            shadow_code: "
            @group(0) @binding(0) var<uniform> ubo: uniforms;
            @vertex
            fn vertexMain(@location(0) pos: vec3f) -> @builtin(position) vec4f {
              return ubo.smvp[i32(ubo.eng.a)] * ubo.model * vec4f(pos, 1.0);
            }".to_string(),
            rec_pipeline: false,
            keyboard: Keyboard::new(),
            mouse: Mouse::new(),
            touch: Touch::new(),
            gamepads: Gamepad::new(),
            sr: false,
            audioctx: AudioContext::new(0),
        }
    }
    #[allow(dead_code)]
    pub fn start(&mut self){
        if !self.sr {
            self.rloop.drawloop();
            self.sr = !self.sr;
        }
        self.rec_pipeline = false;
        let mut smats = 0;
        for i in 0..self.lights.len(){
            if self.lights[i].shadow {
                smats+=1;
            }
        }
        if self.last_cam_size != self.cameras.len() || self.last_light_size != self.lights.len() || self.last_smat != smats{
            self.uniform_beg = "
            const LIGHTN = ".to_string();
            self.uniform_beg += &self.lights.len().to_string();
            self.uniform_beg += ";
            const LIGHTMN = ";
            self.uniform_beg += &smats.to_string();
            self.uniform_beg += ";
            const CAMN = ";
            self.uniform_beg += &self.cameras.len().to_string();
            self.uniform_beg += ";
            struct uniforms {
                eng: vec4f,
                mvp: array<mat4x4<f32>, ";
            self.uniform_beg += &self.cameras.len().to_string();
            self.uniform_beg += ">,
                    pos: array<vec4f, ";
            self.uniform_beg += &self.cameras.len().to_string();
            self.uniform_beg += ">,
                    smvp: array<mat4x4<f32>, ";
            self.uniform_beg += &smats.to_string();
            self.uniform_beg += ">,
                    lpos: array<vec4f, ";
            self.uniform_beg += &self.lights.len().to_string();
            self.uniform_beg += ">,
                    lcolor: array<vec4f, ";
            self.uniform_beg += &self.lights.len().to_string();
            self.uniform_beg += ">,
                    model: mat4x4<f32>,";
            self.last_cam_size = self.cameras.len();
            self.last_light_size = self.lights.len();
            self.rec_pipeline = true;
            self.last_smat = smats;
            self.render.change_render_scale(self.renderscale, self.last_cam_size as u32);
            self.render.change_shadow_map_resolution(self.shadowmap_resolution, smats as u32);
        }
        if self.last_renderscale != self.renderscale {
            self.render.change_render_scale(self.renderscale, self.last_cam_size as u32);
            self.last_renderscale = self.renderscale;
        }
        if self.last_shs != self.shadowmap_resolution {
            self.render.change_shadow_map_resolution(self.shadowmap_resolution, smats as u32);
            self.last_shs = self.shadowmap_resolution;
        }
            
        let aspect = self.render.get_canvas_size_x() as f32/self.render.get_canvas_size_y() as f32;
        self.ubo_beg_values.resize(20*self.cameras.len()+4+smats*16+self.lights.len()*8, 0f32);
        for i1 in 0..self.cameras.len(){
            self.cameras[i1].physic_object.exec();
            self.cameras[i1].physic_object.reset_states();
            let ubm = self.cameras[i1].get_projection(aspect);
            for i in 0..16 {
                self.ubo_beg_values[i1*16+i+4] = ubm.mat[i];
            }
        }
        for i in 0..self.cameras.len(){
            self.ubo_beg_values[16*self.cameras.len()+4+i*4] = self.cameras[i].physic_object.pos.x;
            self.ubo_beg_values[16*self.cameras.len()+5+i*4] = self.cameras[i].physic_object.pos.y;
            self.ubo_beg_values[16*self.cameras.len()+6+i*4] = self.cameras[i].physic_object.pos.z;
            self.ubo_beg_values[16*self.cameras.len()+7+i*4] = 0f32;
        }
        
        let mut fl = 0;
        for l in 0..self.lights.len(){
            if self.lights[l].shadow{
                let vc = self.lights[l].getvec();
                for i in 0..vc.len(){
                    self.ubo_beg_values[20*self.cameras.len()+4+fl+i] = vc[i];
                }
                fl+=16;
            }
        }

        for i in 0..self.lights.len(){
            self.ubo_beg_values[20*self.cameras.len()+4+smats*16+i*4] = self.lights[i].pos.x;
            self.ubo_beg_values[20*self.cameras.len()+5+smats*16+i*4] = self.lights[i].pos.y;
            self.ubo_beg_values[20*self.cameras.len()+6+smats*16+i*4] = self.lights[i].pos.z;
            self.ubo_beg_values[20*self.cameras.len()+7+smats*16+i*4] = self.lights[i].light_type as i32 as f32;
        }

        for i in 0..self.lights.len(){
            self.ubo_beg_values[20*self.cameras.len()+4+smats*16+self.lights.len()*4+i*4] = self.lights[i].color.x;
            self.ubo_beg_values[20*self.cameras.len()+5+smats*16+self.lights.len()*4+i*4] = self.lights[i].color.y;
            self.ubo_beg_values[20*self.cameras.len()+6+smats*16+self.lights.len()*4+i*4] = self.lights[i].color.z;
            self.ubo_beg_values[20*self.cameras.len()+7+smats*16+self.lights.len()*4+i*4] = 0f32;
        }
    }
}