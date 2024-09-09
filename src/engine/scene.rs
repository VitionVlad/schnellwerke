use super::{engine::Engine, math::vec3::Vec3, object::Object, resourceloader::resourceloader::Sdfreader};

#[allow(dead_code)]
pub struct Scene{
    pub objects: Vec<Object>,
    mdd: Vec<f32>,
    cdd: Vec<f32>,
}

impl Scene {
    #[allow(dead_code)]
    pub fn new(id: &str) -> Scene{
        let ld = Sdfreader::new(id);
        Scene{
            objects: vec![],
            mdd: ld.mdd,
            cdd: ld.cdd,
        }
    }
    #[allow(dead_code)]
    pub fn set_objects(&mut self){
        for i in (0..self.mdd.len()).step_by(9){
            self.objects[i/9].pos = Vec3::newdefined(self.mdd[i], self.mdd[i+1], self.mdd[i+2]);
            self.objects[i/9].rot = Vec3::newdefined(self.mdd[i+3], self.mdd[i+4], self.mdd[i+5]);
            self.objects[i/9].scale = Vec3::newdefined(self.mdd[i+6], self.mdd[i+7], self.mdd[i+8]);
        }
    }
    #[allow(dead_code)]
    pub fn exec(&mut self, eng: &mut Engine){
        for i in 0..self.objects.len(){
            self.objects[i].exec(eng);
        }
    }
}