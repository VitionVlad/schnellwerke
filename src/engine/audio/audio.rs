#![allow(dead_code)]
#![allow(unused_variables)]

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(module = "/src/engine/audio/audio.js")]
unsafe extern "C"{
    fn newmozart() -> u32;
    fn mozartsetvolume(mhi: u32, vol: f32);
    fn newsound(mhi: u32, path: &str) -> u32;
    fn soundplay(msn: u32, pan: f32, vol: f32);
    fn soundsetloop(msn: u32, val: bool);
    fn soundsetpos(msn: u32, val: f32);
    fn cend(msn: u32) -> u8;
    fn soundstop(msn: u32);
    fn destroymozart(mhi: u32);
}

#[derive(Copy, Clone)]
pub struct AudioEngine{
    pub index: u32,
    pub vol: f32,
    pub spacial: bool,
}

impl AudioEngine{
    pub fn new() -> AudioEngine{
        AudioEngine{
            index: newmozart(),
            vol: 1.0f32,
            spacial: true,
        }
    }
    pub fn exec(&mut self){
        mozartsetvolume(self.index, self.vol);
    }
    pub fn destroy(&mut self){
        destroymozart(self.index);
    }
}

pub struct Sound{
    index: u32,
    pub vol: f32,
    pub pan: f32,
    pub loopsound: bool,
}

impl Sound{
    pub fn new(ae: AudioEngine, path: &str) -> Sound{
        Sound { 
            index: newsound(ae.index, path), 
            vol: 1.0, 
            pan: 0.0,
            loopsound: true,
        }
    }
    pub fn play(&mut self){
        soundsetloop(self.index, self.loopsound);
        soundplay(self.index, self.pan, self.vol);
    }
    pub fn stop(&mut self){
        soundstop(self.index)
    }
    pub fn check_end(&mut self) -> bool{
        cend(self.index) == 1
    }
    pub fn set_new_pos(&mut self, newpos: f32){
        soundsetpos(self.index, newpos);
    }
}