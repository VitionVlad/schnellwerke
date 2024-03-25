use super::{audio::audiosource::Audiosource, engine::Engine, math::vec3::Vec3};

#[allow(dead_code)]
pub struct Audiosource3d{
    pub audsrc: Audiosource,
    pub pos: Vec3,
    pub power: f32,
}

impl Audiosource3d{
    #[allow(dead_code)]
    pub fn new(url: &str, pos: Vec3, power: f32) -> Audiosource3d{
        Audiosource3d{
            audsrc: Audiosource::new(url),
            pos: pos,
            power: power,
        }
    }
    #[allow(dead_code)]
    pub fn play(&mut self, eng: &Engine){
        self.audsrc.play();
        let powervol = (self.power - f32::sqrt(f32::powi(self.pos.x-eng.pos.x, 2)+f32::powi(self.pos.y-eng.pos.y, 2)+f32::powi(self.pos.z-eng.pos.z, 2)))/self.power;
        self.audsrc.volume = f32::min(f32::max(eng.volume * powervol, 0.0f32), 1.0f32);
    }
    #[allow(dead_code)]
    pub fn stop(&mut self){
        self.audsrc.stop();
    }
    #[allow(dead_code)]
    pub fn pause(&mut self){
        self.audsrc.pause();
    }
}