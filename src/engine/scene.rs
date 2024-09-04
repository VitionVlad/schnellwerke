use super::{math::vec3::Vec3, resourceloader::resourceloader::Sdfreader, object::Object};

#[allow(dead_code)]
pub struct Scene{
    mdd: Vec<f32>,
    cdd: Vec<f32>,
}

impl Scene {
    #[allow(dead_code)]
    pub fn new(id: &str) -> Scene{
        let ld = Sdfreader::new(id);
        Scene{
            mdd: ld.mdd,
            cdd: ld.cdd,
        }
    }
    #[allow(dead_code)]
    pub fn set_objects(&self, objects: &mut Vec<Object>){
        for i in (0..self.mdd.len()).step_by(9){
            objects[i/9].pos = Vec3::newdefined(self.mdd[i], self.mdd[i+1], self.mdd[i+2]);
            objects[i/9].rot = Vec3::newdefined(self.mdd[i+3], self.mdd[i+4], self.mdd[i+5]);
            objects[i/9].scale = Vec3::newdefined(self.mdd[i+6], self.mdd[i+7], self.mdd[i+8]);
        }
    }
}