use engine::{light::{Light, LightType}, object::Object};
use crate::*;
use super::camera::Camera;

#[allow(dead_code)]
pub struct Engine{
    pub render: Render,
    pub cameras: Vec<Camera>,
    pub lights: Vec<Light>,
    main_projections: Vec<f32>,
    pub object_to_draw: Vec<Object>,
    uniform_beg: String,
    last_cam_size: usize,
    last_light_size: usize,
    pub shadow_code: String,
}

impl Engine {
    #[allow(dead_code)]
    pub fn new(canvasid: &str) -> Engine{
        let ren = Render::init(canvasid, 1f32, 1000); 
        set_render(&ren.jsren);
        Engine{
            render: ren,
            cameras: vec![Camera{ pos: Vec3::new(), rot: Vec3::new(), fov: 90f32, znear: 0.1f32, zfar: 100f32, is_orthographic: false }],
            lights: vec![Light::new(LightType::Directional)],
            main_projections: vec![0f32, 0f32, 0f32, 0f32],
            object_to_draw: vec![],
            uniform_beg: "
            struct uniforms {
                eng: vec4f,
                mvp: array<mat4x4<f32>, 1>,
                pos: array<vec4f, 1>,
                smvp: array<mat4x4<f32>, 1>,
                lpos: array<vec4f, 1>,
                lcolor: array<vec4f, 1>,".to_string(),
            last_cam_size: 1,
            last_light_size: 1,
            shadow_code: "
            struct uniforms {
              eng: vec4f,
              mvp: array<mat4x4<f32>, 1>,
              pos: array<vec4f, 1>,
              smvp: array<mat4x4<f32>, 1>,
              lpos: array<vec4f, 1>,
              lcolor: array<vec4f, 1>,
            }
            @group(0) @binding(0) var<uniform> ubo: uniforms;
            @vertex
            fn vertexMain(@location(0) pos: vec3f) -> @builtin(position) vec4f {
              return ubo.mvp[0] * vec4f(pos, 1.0);
            }".to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn start(mut self){
        let render_prepare_loop = Closure::new(move || {
            let mut smats = 0;
            for i in 0..self.lights.len(){
                smats+=1;
                if self.lights[i].light_type == LightType::Point{
                    smats+=5;
                }
            }
            if self.last_cam_size != self.cameras.len() || self.last_light_size != self.lights.len(){
                self.uniform_beg = "
                    struct uniforms {
                        eng: vec4f,
                        mvp: array<mat4x4<f32>, ".to_string();
                self.uniform_beg += &self.cameras.len().to_string();
                self.uniform_beg += ">,
                        cam: array<vec4f, ";
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
                self.uniform_beg += ">,";
                self.last_cam_size = self.cameras.len();
                self.last_light_size = self.lights.len();
            }
            
            let aspect = self.render.get_canvas_size_x() as f32/self.render.get_canvas_size_y() as f32;
            self.main_projections.resize(20*self.cameras.len()+4+smats*16+self.lights.len()*8, 0f32);
            for i1 in 0..self.cameras.len(){
                let ubm = self.cameras[i1].get_projection(aspect);
                for i in 0..16 {
                    self.main_projections[i1*16+i+4] = ubm.mat[i];
                }
            }
            for i in (0..self.cameras.len()*4).step_by(4){
                self.main_projections[16*self.cameras.len()+4+i] = self.cameras[i].pos.x;
                self.main_projections[16*self.cameras.len()+5+i] = self.cameras[i].pos.y;
                self.main_projections[16*self.cameras.len()+6+i] = self.cameras[i].pos.z;
                self.main_projections[16*self.cameras.len()+7+i] = 0f32;
            }
            
            for l in 0..self.lights.len(){
                let vc = self.lights[l].getvec();
                for i in 0..vc.len(){
                    self.main_projections[20*self.cameras.len()+4+l*16+i] = vc[i];
                }
            }

            for i in 0..self.lights.len(){
                self.main_projections[20*self.cameras.len()+4+smats*16] = self.lights[i].pos.x;
                self.main_projections[20*self.cameras.len()+5+smats*16] = self.lights[i].pos.y;
                self.main_projections[20*self.cameras.len()+6+smats*16] = self.lights[i].pos.z;
                self.main_projections[20*self.cameras.len()+7+smats*16] = 0f32;
            }

            for i in 0..self.lights.len(){
                self.main_projections[20*self.cameras.len()+4+smats*16+self.lights.len()*4] = self.lights[i].color.x;
                self.main_projections[20*self.cameras.len()+5+smats*16+self.lights.len()*4] = self.lights[i].color.y;
                self.main_projections[20*self.cameras.len()+6+smats*16+self.lights.len()*4] = self.lights[i].color.z;
                self.main_projections[20*self.cameras.len()+7+smats*16+self.lights.len()*4] = 0f32;
            }

            for i in 0..self.object_to_draw.len(){
                self.object_to_draw[i].mesh.set_ubo(&self.main_projections);
            }
          });
          set_func(&render_prepare_loop);
          drawloop();
          render_prepare_loop.forget();
    }
}