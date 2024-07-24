use crate::*;
use super::camera::Camera;

#[allow(dead_code)]
pub struct Engine{
    pub render: Render,
    pub cameras: Vec<Camera>,
}

impl Engine {
    #[allow(dead_code)]
    pub fn new(canvasid: &str) -> Engine{
        let ren = Render::init(canvasid, 1f32, 1000);
        set_render(&ren.jsren);
        Engine{
            render: ren,
            cameras: vec![Camera{ pos: Vec3::new(), rot: Vec3::new(), fov: 90f32, znear: 0.1f32, zfar: 100f32, is_orthographic: false }],
        }
    }
    #[allow(dead_code)]
    pub fn start(){
        let render_prepare_loop = Closure::new(move || {
            //let mut ubm = Mat4::new();
            //ubm.perspective(90f32, 100f32, 0.1f32, ren.get_canvas_size_x() as f32/ren.get_canvas_size_y() as f32);
            //let mut t: Mat4 = Mat4::new();
            //t.xrot(0f32);
            //ubm.mul(&t);
            //t = Mat4::new();
            //t.yrot(0.5f32);
            //ubm.mul(&t);
            //t = Mat4::new();
            //t.zrot(0f32);
            //ubm.mul(&t);
            //t = Mat4::new();
            //t.trans(Vec3::newdefined(2f32, 0f32, -4f32));
            //ubm.mul(&t);
            //ubm.transpose();
            //mesh1.set_ubo(&ubm.mat);
          });
          set_func(&render_prepare_loop);
          drawloop();
          render_prepare_loop.forget();
    }
}