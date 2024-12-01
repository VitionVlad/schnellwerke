use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/engine/audio/audio.js")]
extern {
    pub type Jsaudioctx;
    #[wasm_bindgen(constructor)]
    pub fn new() -> Jsaudioctx;
}

#[allow(dead_code)]
pub struct AudioContext{
    pub ctx: Jsaudioctx,
    pub camera_id: usize,
    pub volume: f32,
}

impl AudioContext{
    #[allow(dead_code)]
    pub fn new(id: usize) -> AudioContext{
        AudioContext{
            ctx: Jsaudioctx::new(),
            camera_id: id,
            volume: 1f32,
        }
    }
}