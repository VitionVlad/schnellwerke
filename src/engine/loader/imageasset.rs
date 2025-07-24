#![allow(dead_code)]
#![allow(unused_variables)]

use std::{i8, vec};

use js_sys::Uint8Array;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen(module = "/src/engine/loader/loader.js")]
unsafe extern "C"{
    async fn openfs(path: &str) -> JsValue;
}

pub async fn fileopen(path: &str) -> Vec<u8>{
    return Uint8Array::from(openfs(path).await).to_vec();
}

pub struct ImageAsset{
    pub data: Vec<i8>,
    pub size: [u32; 2],
}

impl ImageAsset{
    pub async fn load_tga(path: &str) -> ImageAsset{
        let tga = fileopen(path).await;
        let size16: [u32; 2] = [ ((tga[12] as u32) << 16) | ((tga[13] as u32) << 8) , ((tga[14] as u32) << 16) | ((tga[15] as u32) << 8)];
        let mut data: Vec<i8> = vec![];
        let sz = size16[0] * size16[1] * 3;
        for i in (0..sz as usize).step_by(3) {
            data.push(tga[i+18] as i8);
            data.push(tga[i+19] as i8);
            data.push(tga[i+20] as i8);
            data.push(i8::MAX);
        }
        ImageAsset { 
            data: data, 
            size: [size16[0], size16[1]] 
        }
    }
    pub async fn load_tiff(path: &str) -> ImageAsset{
        let tiff = fileopen(path).await;
        let mut size: [u32; 2] = [0, 0];
        let idfoffset: u32 = (tiff[7] as u32) << 24 | (tiff[6] as u32) << 16 | (tiff[5] as u32) << 8 | (tiff[4] as u32);
        let mut begoff = 8u32;
        let mut componentscnt = 3;
        let argcnt = ((tiff[idfoffset as usize + 1] as u32) << 8) | (tiff[idfoffset as usize] as u32);
        for i in (idfoffset+2..idfoffset+2+argcnt*12).step_by(12){
            let tag = ((tiff[i as usize+1] as u16) << 8) | tiff[i as usize] as u16;
            if tag == 256 {
                size[0] = (tiff[i as usize + 11] as u32) << 24 | (tiff[i as usize + 10] as u32) << 16 | (tiff[i as usize + 9] as u32) << 8 | (tiff[i as usize + 8] as u32);
            }
            if tag == 257 {
                size[1] = (tiff[i as usize + 11] as u32) << 24 | (tiff[i as usize + 10] as u32) << 16 | (tiff[i as usize + 9] as u32) << 8 | (tiff[i as usize + 8] as u32);
            }
            if tag == 273 {
                let stripcnt = (tiff[i as usize + 7] as u32) << 24 | (tiff[i as usize + 6] as u32) << 16 | (tiff[i as usize + 5] as u32) << 8 | (tiff[i as usize + 4] as u32);
                let stripoff = (tiff[i as usize + 11] as u32) << 24 | (tiff[i as usize + 10] as u32) << 16 | (tiff[i as usize + 9] as u32) << 8 | (tiff[i as usize + 8] as u32);
                if stripcnt != 1 {
                    begoff = (tiff[stripoff as usize + 3] as u32) << 24 | (tiff[stripoff as usize + 2] as u32) << 16 | (tiff[stripoff as usize + 1] as u32) << 8 | (tiff[stripoff as usize] as u32);
                }else{
                    begoff = stripoff;
                }
            }
            if tag == 277 {
                componentscnt = (tiff[i as usize + 11] as u32) << 24 | (tiff[i as usize + 10] as u32) << 16 | (tiff[i as usize + 9] as u32) << 8 | (tiff[i as usize + 8] as u32);
            }
        }
        let mut data: Vec<i8> = vec![];
        let esz: u32 = size[0]*size[1]*componentscnt + begoff;
        for i in (begoff..esz).step_by(componentscnt as usize){
            for j in 0..componentscnt{
                data.push(tiff[i as usize + j as usize] as i8);
            }
            for _ in 0..(4 - componentscnt){
                data.push(tiff[i as usize] as i8);
            }
        }
        ImageAsset { 
            data: data, 
            size: size, 
        }
    }
}