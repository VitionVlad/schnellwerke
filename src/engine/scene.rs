use super::{engine::Engine, material::Material, math::vec3::Vec3, object::Object, render::mesh::MUsages, resourceloader::resourceloader::{Objreader, Sdfreader}};

#[allow(dead_code)]
pub struct ObjectM{
    pub md: String,
    pub mat: Material,
    pub usage: MUsages,
}

#[allow(dead_code)]
pub struct Scene{
    pub model_objects: Vec<ObjectM>,
    pub all_objects: Vec<Object>,
    mdd: Vec<f32>,
    cdd: Vec<f32>,
}

impl Scene {
    #[allow(dead_code)]
    pub fn new(id: &str) -> Scene{
        let ld = Sdfreader::new(id);
        Scene{
            model_objects: vec![],
            all_objects: vec![],
            mdd: ld.mdd,
            cdd: ld.cdd,
        }
    }
    #[allow(dead_code)]
    pub fn create_objects(&mut self, eng: &mut Engine){
        let mut md: Objreader;
        for i in 0..self.model_objects.len(){
            md = Objreader::new(&self.model_objects[i].md);
            self.all_objects.push(Object::new(eng, md.arr, &self.model_objects[i].mat, self.model_objects[i].usage));
        }
    }
    #[allow(dead_code)]
    pub fn set_objects(&mut self){
        for i in (0..self.mdd.len()).step_by(9){
            self.all_objects[i/9].pos = Vec3::newdefined(self.mdd[i], self.mdd[i+1], self.mdd[i+2]);
            self.all_objects[i/9].rot = Vec3::newdefined(self.mdd[i+3], self.mdd[i+4], self.mdd[i+5]);
            self.all_objects[i/9].scale = Vec3::newdefined(self.mdd[i+6], self.mdd[i+7], self.mdd[i+8]);
        }
    }
    #[allow(dead_code)]
    pub fn exec(&mut self, eng: &mut Engine){
        for i in 0..self.all_objects.len(){
            self.all_objects[i].exec(eng);
        }
    }
}