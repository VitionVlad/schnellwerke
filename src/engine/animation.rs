use super::{engine::Engine, math::vec3::Vec3, object::Object};

#[allow(dead_code)]
pub struct Keytiming{
    pub tim: i32,
    pub fpos: Vec3,
    pub frot: Vec3,
    pub fscale: Vec3,
    pub current_time: i32,
    pfpos: Vec3,
    pfrot: Vec3,
    pfscale: Vec3,
}

impl Keytiming {
    #[allow(dead_code)]
    pub fn new(tim: i32, object: &Object, fpos: Vec3, frot: Vec3, fscale: Vec3) -> Keytiming{
        let ofpos = object.pos;
        let ofrot = object.rot;
        let ofscale = object.scale;
        let posampl = Vec3::newdefined(
            (fpos.x - ofpos.x)/tim as f32, 
            (fpos.y - ofpos.y)/tim as f32, 
            (fpos.z - ofpos.z)/tim as f32
        );
        let rotampl = Vec3::newdefined(
            (frot.x - ofrot.x)/tim as f32, 
            (frot.y - ofrot.y)/tim as f32, 
            (frot.z - ofrot.z)/tim as f32
        );
        let scaleampl = Vec3::newdefined(
            (fscale.x - ofscale.x)/tim as f32, 
            (fscale.y - ofscale.y)/tim as f32, 
            (fscale.z - ofscale.z)/tim as f32
        );
        Keytiming{
            tim: tim,
            fpos: fpos,
            frot: frot,
            fscale: fscale,
            pfpos: posampl,
            pfrot: rotampl,
            pfscale: scaleampl,
            current_time: 0,
        }
    }
    #[allow(dead_code)]
    pub fn play(&mut self, eng: &Engine, object: &mut Object){
        if eng.norm > 1 && self.tim > self.current_time{
            let dst = self.tim - self.current_time;
            if dst as f64 <= eng.frametime {
                let opos = Vec3::newdefined(
                    object.pos.x + self.pfpos.x * dst as f32, 
                    object.pos.y + self.pfpos.y * dst as f32, 
                    object.pos.z + self.pfpos.z * dst as f32
                );
                object.pos = opos;

                let orot = Vec3::newdefined(
                    object.rot.x + self.pfrot.x * dst as f32, 
                    object.rot.y + self.pfrot.y * dst as f32, 
                    object.rot.z + self.pfrot.z * dst as f32
                );
                object.rot = orot;

                let oscale = Vec3::newdefined(
                    object.scale.x + self.pfscale.x * dst as f32, 
                    object.scale.y + self.pfscale.y * dst as f32, 
                    object.scale.z + self.pfscale.z * dst as f32
                );
                object.scale = oscale;
                self.current_time += dst;
            }else{
                let opos = Vec3::newdefined(
                    object.pos.x + self.pfpos.x * eng.frametime as f32, 
                    object.pos.y + self.pfpos.y * eng.frametime as f32, 
                    object.pos.z + self.pfpos.z * eng.frametime as f32
                );
                object.pos = opos;

                let orot = Vec3::newdefined(
                    object.rot.x + self.pfrot.x * eng.frametime as f32, 
                    object.rot.y + self.pfrot.y * eng.frametime as f32, 
                    object.rot.z + self.pfrot.z * eng.frametime as f32
                );
                object.rot = orot;

                let oscale = Vec3::newdefined(
                    object.scale.x + self.pfscale.x * eng.frametime as f32, 
                    object.scale.y + self.pfscale.y * eng.frametime as f32, 
                    object.scale.z + self.pfscale.z * eng.frametime as f32
                );
                object.scale = oscale;
                self.current_time += eng.frametime as i32;
            }
        }
    }
}