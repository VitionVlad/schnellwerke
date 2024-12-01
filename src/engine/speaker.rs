use super::{audio::audiosource::AudioSource, engine::Engine, math::{vec2::Vec2, vec3::Vec3}};

#[allow(dead_code)]
pub struct Speaker{
    audio: AudioSource,
    pub pos: Vec3,
    pub volume: f32,
    pub power: f32,
}

impl Speaker {
    #[allow(dead_code)]
    pub fn new(eng: &mut Engine, id: &str, pos: Vec3, power: f32) -> Speaker{
        Speaker{
            audio: AudioSource::new(&eng.audioctx, id),
            pos: pos,
            volume: 1f32,
            power: power,
        }
    }
    #[allow(dead_code)]
    fn calcpan(src: Vec2, listp: Vec2, listr: Vec2) -> f32{
        let vec_d = Vec2::newdefined(src.x - listp.x, src.y - listp.y);
        let vec_r_perp = Vec2::newdefined(listr.y, listr.x);
        let dot = vec_d.x * vec_r_perp.x + vec_d.y * vec_r_perp.y;
        let magnitude_d = (vec_d.x.powi(2) + vec_d.y.powi(2)).sqrt();
        if magnitude_d == 0.0 {
            0.0
        } else {
            dot / magnitude_d
        }
    }
    #[allow(dead_code)]
    pub fn play(&mut self, eng: &mut Engine){
        self.audio.volume =  (self.power - f32::sqrt(f32::powi(eng.cameras[eng.audioctx.camera_id].physic_object.pos.x - self.pos.x, 2) + f32::powi(eng.cameras[eng.audioctx.camera_id].physic_object.pos.y - self.pos.y, 2) + f32::powi(eng.cameras[eng.audioctx.camera_id].physic_object.pos.z - self.pos.z, 2)))/self.power * self.volume * eng.audioctx.volume;
        self.audio.pan = Self::calcpan(Vec2::newdefined(self.pos.x, self.pos.z), Vec2::newdefined(eng.cameras[eng.audioctx.camera_id].physic_object.pos.x, eng.cameras[eng.audioctx.camera_id].physic_object.pos.z), Vec2::newdefined(eng.cameras[eng.audioctx.camera_id].physic_object.rot.y.sin(), eng.cameras[eng.audioctx.camera_id].physic_object.rot.y.cos()));
        self.audio.play();
    }
    #[allow(dead_code)]
    pub fn pause(&mut self){
        self.audio.pause();
    }
}