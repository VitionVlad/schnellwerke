use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/engine/audio/audio.js")]
extern {
    pub type Jsaudio;
    #[wasm_bindgen(constructor)]
    pub fn new(url: &str) -> Jsaudio;

    #[wasm_bindgen(method)]
    pub fn play(this: &Jsaudio);

    #[wasm_bindgen(method)]
    pub fn stop(this: &Jsaudio);

    #[wasm_bindgen(method)]
    pub fn pause(this: &Jsaudio);

    #[wasm_bindgen(method)]
    pub fn setvolume(this: &Jsaudio, vol: f32);

    #[wasm_bindgen(method)]
    pub fn settime(this: &Jsaudio, time: f32);

    #[wasm_bindgen(method)]
    pub fn ended(this: &Jsaudio) -> bool;
}

#[allow(dead_code)]
pub struct Audiosource{
    jsa: Jsaudio,
    pub volume: f32,
    pub playng: bool,
}

impl Audiosource{
    #[allow(dead_code)]
    pub fn new(url: &str) -> Audiosource{
        Audiosource { 
            jsa: Jsaudio::new(url),
            volume: 1.0f32,
            playng: true,
        }
    }
    #[allow(dead_code)]
    pub fn play(&mut self){
        self.jsa.setvolume(self.volume);
        if !self.playng {
            self.jsa.play();
        }
    }
    #[allow(dead_code)]
    pub fn stop(&mut self){
        self.jsa.setvolume(self.volume);
        self.jsa.stop();
        self.playng = false;
    }
    #[allow(dead_code)]
    pub fn pause(&mut self){
        self.jsa.setvolume(self.volume);
        self.jsa.pause();
        self.playng = false;
    }
    #[allow(dead_code)]
    pub fn set_time(&self, time: f32){
        self.jsa.settime(time);
    }
}