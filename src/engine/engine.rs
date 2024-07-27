use crate::*;
use super::camera::Camera;

#[allow(dead_code)]
pub struct Engine{
    pub render: Render,
    pub cameras: Vec<Camera>,
    main_projections: Vec<f32>,
    pub mesh_to_draw: Vec<Mesh>,
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
            mesh_to_draw: vec![],
        }
    }
    #[allow(dead_code)]
    pub fn start(mut self){
        let render_prepare_loop = Closure::new(move || {
            let aspect = self.render.get_canvas_size_x() as f32/self.render.get_canvas_size_y() as f32;
            self.main_projections.resize(16*self.cameras.len()+4, 0f32);
            for i1 in 0..self.cameras.len(){
                let ubm = self.cameras[i1].get_projection(aspect);
                for i in 0..16 {
                    self.main_projections[i1*16+i+4] = ubm.mat[i];
                }
            }

            for i in 0..self.mesh_to_draw.len(){
                self.mesh_to_draw[i].set_ubo(&self.main_projections);
            }
          });
          set_func(&render_prepare_loop);
          drawloop();
          render_prepare_loop.forget();
    }
}