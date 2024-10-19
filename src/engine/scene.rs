use crate::log;

use super::{cube::{CUBE, CUBEUV}, engine::Engine, material::{Material, MaterialGenerator}, math::{uniformstruct::Uniformstruct, vec3::Vec3}, object::Object, plane::PLANE, render::mesh::MUsages, resourceloader::resourceloader::{get_text_from_iframe, Objreader, Sdfreader}};

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
    pub pos: Vec3,
    pub rot: Vec3,
    pub scale: Vec3,
}

#[allow(dead_code)]
pub struct Scene{
    pub material_gen: MaterialGenerator,
    pub objects_to_create: Vec<ObjectCreateInfo>,
    pub all_objects: Vec<Object>,
}

impl Scene {
    #[allow(dead_code)]
    pub fn new(uniform_struct: Vec<Uniformstruct>) -> Scene{
        Scene{
            material_gen: MaterialGenerator::new(uniform_struct),
            objects_to_create: vec![],
            all_objects: vec![],
        }
    }
    #[allow(dead_code)]
    pub fn create_objects(&mut self, eng: &mut Engine){
        let mut md: Objreader;
        for i in 0..self.objects_to_create.len(){
            if self.objects_to_create[i].object_type == ObjectType::Model {
                md = Objreader::new(&self.objects_to_create[i].md);
                self.all_objects.push(Object::new(eng, md.arr, &self.objects_to_create[i].mat, self.objects_to_create[i].usage, self.objects_to_create[i].is_static)); 
            }
            if self.objects_to_create[i].object_type == ObjectType::Cube {
                self.all_objects.push(Object::new(eng, CUBE.to_vec(), &self.objects_to_create[i].mat, self.objects_to_create[i].usage, self.objects_to_create[i].is_static));    
            }
            if self.objects_to_create[i].object_type == ObjectType::CubeUV {
                self.all_objects.push(Object::new(eng, CUBEUV.to_vec(), &self.objects_to_create[i].mat, self.objects_to_create[i].usage, self.objects_to_create[i].is_static));    
            }
            if self.objects_to_create[i].object_type == ObjectType::Plane {
                self.all_objects.push(Object::new(eng, PLANE.to_vec(), &self.objects_to_create[i].mat, self.objects_to_create[i].usage, self.objects_to_create[i].is_static));    
            }
            self.all_objects[i].physic_object.pos = self.objects_to_create[i].pos;
            self.all_objects[i].physic_object.rot = self.objects_to_create[i].rot;
            self.all_objects[i].physic_object.scale = self.objects_to_create[i].scale;   
        }
    }
    #[allow(dead_code)]
    pub fn load_objects(&mut self, id: &str){
        let sdf = Sdfreader::new(id);
        let mut mv: Vec<Material> = vec![];
        let mut it = 0usize;
        while sdf.mat.len() > it{
            log(&("SDFParser: vertex shader id = ".to_string() + &(sdf.mat[it] as i32).to_string() + " at location = " + &it.to_string()));
            if sdf.mat[it] as i32 > 0{
                self.material_gen.gen_vertex_beg();
                self.material_gen.vertex_shader += &get_text_from_iframe(&("vsh".to_string()+&(sdf.mat[it] as i32).to_string()));
                self.material_gen.gen_vert_end();
            }else{
                self.material_gen.gen_vertex();
            }
            it+=1;
            log(&("SDFParser: pixel shader id = ".to_string() + &(sdf.mat[it] as i32).to_string() + " at location = " + &it.to_string()));
            if sdf.mat[it] as i32 > 0{
                self.material_gen.gen_frag_beg();
                self.material_gen.fragment_shader += &get_text_from_iframe(&("psh".to_string()+&(sdf.mat[it] as i32).to_string()));
                self.material_gen.gen_frag_end();
            }
            it+=1;
            let tn = sdf.mat[it] as i32;
            log(&("SDFParser: texture array length = ".to_string() + &(sdf.mat[it] as i32).to_string() + " at location = " + &it.to_string()));
            let mut ids = "".to_string();
            for _ in 0..tn-1{
                it+=1;
                log(&("SDFParser: texture id = ".to_string() + &(sdf.mat[it] as i32).to_string() + " at location = " + &it.to_string() + " html id = " + &(sdf.mat[it] as i32).to_string()));
                ids += &("tex".to_string() + &(sdf.mat[it] as i32).to_string() + ";");
            }
            it+=1;
            ids += &("tex".to_string() + &(sdf.mat[it] as i32).to_string());
            log(&("SDFParser: texture array = ".to_string() + &ids));
            mv.push(self.material_gen.generate_material(ids, "".to_string()));
            it+=1;
        }
        let mut mt: usize = 0;
        for i in (0..sdf.mdd.len()).step_by(10){
            self.objects_to_create.push(ObjectCreateInfo{ 
                md: ("md".to_string() + &(sdf.mdd[i] as i32).to_string()), 
                mat: mv[mt].clone(),
                usage: MUsages::ShadowAndMain,
                object_type: ObjectType::Model,
                is_static: true,
                pos: Vec3::newdefined(sdf.mdd[i+1], sdf.mdd[i+2], sdf.mdd[i+3]),
                rot: Vec3::newdefined(sdf.mdd[i+4], sdf.mdd[i+5], sdf.mdd[i+6]),
                scale: Vec3::newdefined(sdf.mdd[i+7], sdf.mdd[i+8], sdf.mdd[i+9]),
            });
            mt+=1;
        }
        for i in (0..sdf.cdd.len()).step_by(9){
            self.objects_to_create.push(ObjectCreateInfo{ 
                md: "".to_string(), 
                mat: mv[mt].clone(),
                usage: MUsages::ShadowAndMain,
                object_type: ObjectType::Cube,
                is_static: true,
                pos: Vec3::newdefined(sdf.cdd[i], sdf.cdd[i+1], sdf.cdd[i+2]),
                rot: Vec3::newdefined(sdf.cdd[i+3], sdf.cdd[i+4], sdf.cdd[i+5]),
                scale: Vec3::newdefined(sdf.cdd[i+6], sdf.cdd[i+7], sdf.cdd[i+8]),
            });
            mt+=1;
        }
        for i in (0..sdf.cdu.len()).step_by(9){
            self.objects_to_create.push(ObjectCreateInfo{ 
                md: "".to_string(), 
                mat: mv[mt].clone(),
                usage: MUsages::ShadowAndMain,
                object_type: ObjectType::CubeUV,
                is_static: true,
                pos: Vec3::newdefined(sdf.cdu[i], sdf.cdu[i+1], sdf.cdu[i+2]),
                rot: Vec3::newdefined(sdf.cdu[i+3], sdf.cdu[i+4], sdf.cdu[i+5]),
                scale: Vec3::newdefined(sdf.cdu[i+6], sdf.cdu[i+7], sdf.cdu[i+8]),
            });
            mt+=1;
        }
        for i in (0..sdf.pl.len()).step_by(9){
            self.objects_to_create.push(ObjectCreateInfo{ 
                md: "".to_string(), 
                mat: mv[mt].clone(),
                usage: MUsages::ShadowAndMain,
                object_type: ObjectType::Plane,
                is_static: true,
                pos: Vec3::newdefined(sdf.pl[i], sdf.pl[i+1], sdf.pl[i+2]),
                rot: Vec3::newdefined(sdf.pl[i+3], sdf.pl[i+4], sdf.pl[i+5]),
                scale: Vec3::newdefined(sdf.pl[i+6], sdf.pl[i+7], sdf.pl[i+8]),
            });
            mt+=1;
        }
    }
    #[allow(dead_code)]
    pub fn exec(&mut self, eng: &mut Engine){
        for i in 0..self.all_objects.len(){
            self.all_objects[i].exec(eng);
        }
        for i in 0..self.all_objects.len(){
            for b in 0..self.all_objects.len(){
                if b != i {
                    let o = self.all_objects[b].physic_object.to_owned();
                    self.all_objects[i].physic_object.interact_with_other_object(o);
                }
            }
        }
    }
}