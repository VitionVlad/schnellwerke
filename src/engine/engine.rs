use engine::object::Object;
use crate::*;
use super::camera::Camera;

#[allow(dead_code)]
pub struct Engine{
    pub render: Render,
    pub cameras: Vec<Camera>,
    main_projections: Vec<f32>,
    pub object_to_draw: Vec<Object>,
    uniform_beg: String,
    last_cam_size: usize,
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
            main_projections: vec![0f32, 0f32, 0f32, 0f32],
            object_to_draw: vec![],
            uniform_beg: "
            struct uniforms {
                eng: vec4f,
                mvp: array<mat4x4<f32>, 1>,".to_string(),
            last_cam_size: 1,
            shadow_code: "
            struct uniforms {
              eng: vec4f,
              mvp: mat4x4<f32>
            }
            @group(0) @binding(0) var<uniform> ubo: uniforms;
            @vertex
            fn vertexMain(@location(0) pos: vec3f) -> @builtin(position) vec4f {
              return ubo.mvp * vec4f(pos, 1.0);
            }".to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn start(mut self){
        let render_prepare_loop = Closure::new(move || {
            if self.last_cam_size != self.cameras.len(){
                self.uniform_beg = "
                    struct uniforms {
                        eng: vec4f,
                        mvp: array<mat4x4<f32>, ".to_string();
                self.uniform_beg += &self.cameras.len().to_string();
                self.uniform_beg += ">,";
                self.last_cam_size = self.cameras.len();
            }
            
            let aspect = self.render.get_canvas_size_x() as f32/self.render.get_canvas_size_y() as f32;
            self.main_projections.resize(16*self.cameras.len()+4, 0f32);
            for i1 in 0..self.cameras.len(){
                let ubm = self.cameras[i1].get_projection(aspect);
                for i in 0..16 {
                    self.main_projections[i1*16+i+4] = ubm.mat[i];
                }
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