use super::{cube::{CUBE, CUBEUV}, engine::Engine, material::Material, math::vec3::Vec3, object::Object, plane::PLANE, render::mesh::MUsages, resourceloader::resourceloader::{Objreader, Sdfreader}};

#[allow(dead_code)]
#[derive(PartialEq)]
pub enum ObjectType{
    Model,
    Cube,
    CubeUV,
    Plane,
}

#[allow(dead_code)]
pub struct ObjectCreateInfo{
    pub md: String,
    pub mat: Material,
    pub usage: MUsages,
    pub object_type: ObjectType,
    pub is_static: bool,
}

#[allow(dead_code)]
pub struct Scene{
    pub model_objects: Vec<ObjectCreateInfo>,
    pub all_objects: Vec<Object>,
    mdd: Vec<f32>,
    cdd: Vec<f32>,
    cdu: Vec<f32>,
    pl: Vec<f32>,
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
            cdu: ld.cdu,
            pl: ld.pl,
        }
    }
    #[allow(dead_code)]
    pub fn create_objects(&mut self, eng: &mut Engine){
        let mut md: Objreader;
        for i in 0..self.model_objects.len(){
            if self.model_objects[i].object_type == ObjectType::Model {
                md = Objreader::new(&self.model_objects[i].md);
                self.all_objects.push(Object::new(eng, md.arr, &self.model_objects[i].mat, self.model_objects[i].usage, self.model_objects[i].is_static));    
            }
            if self.model_objects[i].object_type == ObjectType::Cube {
                self.all_objects.push(Object::new(eng, CUBE.to_vec(), &self.model_objects[i].mat, self.model_objects[i].usage, self.model_objects[i].is_static));    
            }
            if self.model_objects[i].object_type == ObjectType::CubeUV {
                self.all_objects.push(Object::new(eng, CUBEUV.to_vec(), &self.model_objects[i].mat, self.model_objects[i].usage, self.model_objects[i].is_static));    
            }
            if self.model_objects[i].object_type == ObjectType::Plane {
                self.all_objects.push(Object::new(eng, PLANE.to_vec(), &self.model_objects[i].mat, self.model_objects[i].usage, self.model_objects[i].is_static));    
            }
        }
    }
    #[allow(dead_code)]
    pub fn set_objects(&mut self){
        let mut it = 0;
        for i in 0..self.model_objects.len(){
            if self.model_objects[i].object_type == ObjectType::Model{
                self.all_objects[i].physic_object.pos = Vec3::newdefined(self.mdd[it], self.mdd[it+1], self.mdd[it+2]);
                self.all_objects[i].physic_object.rot = Vec3::newdefined(self.mdd[it+3], self.mdd[it+4], self.mdd[it+5]);
                self.all_objects[i].physic_object.scale = Vec3::newdefined(self.mdd[it+6], self.mdd[it+7], self.mdd[it+8]);
                it += 9;
            }
        }
        it = 0;
        for i in 0..self.model_objects.len(){
            if self.model_objects[i].object_type == ObjectType::Cube{
                self.all_objects[i].physic_object.pos = Vec3::newdefined(self.cdd[it], self.cdd[it+1], self.cdd[it+2]);
                self.all_objects[i].physic_object.rot = Vec3::newdefined(self.cdd[it+3], self.cdd[it+4], self.cdd[it+5]);
                self.all_objects[i].physic_object.scale = Vec3::newdefined(self.cdd[it+6], self.cdd[it+7], self.cdd[it+8]);
                it += 9;
            }
        }
        it = 0;
        for i in 0..self.model_objects.len(){
            if self.model_objects[i].object_type == ObjectType::CubeUV{
                self.all_objects[i].physic_object.pos = Vec3::newdefined(self.cdu[it], self.cdu[it+1], self.cdu[it+2]);
                self.all_objects[i].physic_object.rot = Vec3::newdefined(self.cdu[it+3], self.cdu[it+4], self.cdu[it+5]);
                self.all_objects[i].physic_object.scale = Vec3::newdefined(self.cdu[it+6], self.cdu[it+7], self.cdu[it+8]);
                it += 9;
            }
        }
        it = 0;
        for i in 0..self.model_objects.len(){
            if self.model_objects[i].object_type == ObjectType::Plane{
                self.all_objects[i].physic_object.pos = Vec3::newdefined(self.pl[it], self.pl[it+1], self.pl[it+2]);
                self.all_objects[i].physic_object.rot = Vec3::newdefined(self.pl[it+3], self.pl[it+4], self.pl[it+5]);
                self.all_objects[i].physic_object.scale = Vec3::newdefined(self.pl[it+6], self.pl[it+7], self.pl[it+8]);
                it += 9;
            }
        }
    }
    #[allow(dead_code)]
    pub fn exec(&mut self, eng: &mut Engine){
        for i in 0..self.all_objects.len(){
            self.all_objects[i].exec(eng);
        }
        for i in 0..self.all_objects.len(){
            for b in 0..self.all_objects.len(){
                let o = self.all_objects[b].physic_object.to_owned();
                self.all_objects[i].physic_object.interact_with_other_object(o);
            }
        }
    }
}