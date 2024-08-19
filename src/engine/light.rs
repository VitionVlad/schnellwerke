use super::{camera::Camera, math::vec3::Vec3};

#[allow(dead_code)]
#[derive(PartialEq)]
pub enum LightType{
    Directional,
    Spot,
    Point,
}

#[allow(dead_code)]
pub struct Light{
    pub light_type: LightType,
    pub pos: Vec3,
    pub rot: Vec3,
    pub color: Vec3,
    pub shadow: bool,
    cameras: Vec<Camera>,
}

impl Light{
    #[allow(dead_code)]
    pub fn new(light_type: LightType) -> Light{
        let mut cams: Vec<Camera> = vec![Camera{ pos: Vec3::new(), rot: Vec3::new(), fov: 90f32, znear: 0.1f32, zfar: 100f32, is_orthographic: false }];
        if light_type == LightType::Point{
            for _i in 0..5 {
                cams.push(Camera{ pos: Vec3::new(), rot: Vec3::new(), fov: 90f32, znear: 0.1f32, zfar: 100f32, is_orthographic: false })
            }
        }
        Light{
            light_type: light_type,
            pos: Vec3::new(),
            rot: Vec3::new(),
            color: Vec3::newdefined(1f32, 1f32, 1f32),
            shadow: true,
            cameras: cams,
        }
    }
    #[allow(dead_code)]
    pub fn getvec(&self) -> Vec<f32>{
        let mut ret: Vec<f32> = vec![];
        for l in 0..self.cameras.len(){
            let cmm = self.cameras[l].get_projection(1f32);
            for i in 0..16{
                ret.push(cmm.mat[i]);
            }
        }
        return ret;
    }
}