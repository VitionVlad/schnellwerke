use wasm_bindgen::prelude::*;

use super::audiocontext::{AudioContext, Jsaudioctx};

#[wasm_bindgen(module = "/src/engine/audio/audio.js")]
extern {
    pub type Jsaudiosource;
    #[wasm_bindgen(constructor)]
    pub fn new(ctx: &Jsaudioctx, id: &str) -> Jsaudiosource;

    #[wasm_bindgen(method)]
    pub fn setrelxy(this: &Jsaudiosource, px: f32);

    #[wasm_bindgen(method)]
    pub fn setvolume(this: &Jsaudiosource, gainValue: f32);

    #[wasm_bindgen(method)]
    pub fn play(this: &Jsaudiosource);

    #[wasm_bindgen(method)]
    pub fn pause(this: &Jsaudiosource);
}

#[allow(dead_code)]
pub struct AudioSource{
    src: Jsaudiosource,
    pub pan: f32,
    pub volume: f32,
}

impl AudioSource{
    #[allow(dead_code)]
    pub fn new(ctx: &AudioContext, id: &str) -> AudioSource{
        AudioSource{
            src: Jsaudiosource::new(&ctx.ctx, id),
            pan: 0f32,
            volume: 1f32,
        }
    }
    #[allow(dead_code)]
    pub fn play(&mut self){
        self.src.setrelxy(self.pan);
        self.src.setvolume(self.volume);
        self.src.play();
    }
    #[allow(dead_code)]
    pub fn pause(&mut self){
        self.src.pause();
    }
}