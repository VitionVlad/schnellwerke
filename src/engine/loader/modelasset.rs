#![allow(dead_code)]
#![allow(unused_variables)]

use wasm_bindgen::prelude::wasm_bindgen;

use crate::engine::loader::imageasset::fileopen;

use super::mtlasset::MtlAsset;

#[wasm_bindgen]
extern {
  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);
}

pub struct ModelAsset{
    pub vertices: Vec<Vec<f32>>,
    pub matnam: Vec<String>,
    pub mtl: MtlAsset,
}

impl ModelAsset{
    pub async fn load_obj(path: &str) -> ModelAsset{
        let file = String::from_utf8(fileopen(path).await).unwrap();
        let reader: Vec<&str> = file.split('\n').collect();
        let mut vert: Vec<[f32; 3]> = vec![];
        let mut uv: Vec<[f32; 2]> = vec![];
        let mut norm: Vec<[f32; 3]> = vec![];

        let mut ivert: Vec<u32> = vec![];
        let mut iuv: Vec<u32> = vec![];
        let mut inorm: Vec<u32> = vec![];

        let mut fnvrt: Vec<f32> = vec![];
        let mut fnobj: Vec<Vec<f32>> = vec![];
        
        let mut objcnt = 0usize;
        let mut objbegind: Vec<[usize; 3]> = vec![];

        let mut mtl: MtlAsset = MtlAsset { matinfo: vec![], matnam: vec![] };

        let mut mtsl: Vec<String> = vec![];
        for i in 0..reader.len() {
            let va = reader[i];
            let spl: Vec<&str> = va.split(' ').collect();
            if spl[0] == "mtllib"{
                let pspl: Vec<&str> = path.split('/').collect();
                let mut mtlp: String = "".to_string();
                for i in 0..pspl.len()-1{
                    mtlp += &pspl[i].to_string();
                    mtlp += "/";
                }
                mtlp += &spl[1].to_string();
                mtl = MtlAsset::load_mtl(&mtlp).await;
                continue;
            }
            if spl[0] == "usemtl"{
                mtsl.push(spl[1].to_owned());
                continue;
            }
            if spl[0] == "o"{
                objcnt += 1usize;
                objbegind.push([ivert.len(), iuv.len(), inorm.len()]);
                continue;
            }
            if spl[0] == "v"{
                let spl: Vec<&str> = va.split(' ').collect();
                let pos: [f32; 3] = [spl[1].parse::<f32>().unwrap(), spl[2].parse::<f32>().unwrap(), spl[3].parse::<f32>().unwrap()];
                vert.push(pos);
                continue;
            }
            if spl[0] == "vt"{
                let spl: Vec<&str> = va.split(' ').collect();
                let uvc: [f32; 2] = [spl[1].parse::<f32>().unwrap(), spl[2].parse::<f32>().unwrap()];
                uv.push(uvc);
                continue;
            }
            if spl[0] == "vn"{
                let spl: Vec<&str> = va.split(' ').collect();
                let normal: [f32; 3] = [spl[1].parse::<f32>().unwrap(), spl[2].parse::<f32>().unwrap(), spl[3].parse::<f32>().unwrap()];
                norm.push(normal);
                continue;
            }
            if spl[0] == "f"{
                let spl: Vec<&str> = va.split(' ').collect();
                let spl3: [Vec<&str>; 3] = [spl[1].split('/').collect(), spl[2].split('/').collect(), spl[3].split('/').collect()];
                let posi: [u32; 3] = [spl3[0][0].parse::<u32>().unwrap(), spl3[1][0].parse::<u32>().unwrap(), spl3[2][0].parse::<u32>().unwrap()];
                let uvi: [u32; 3] = [spl3[0][1].parse::<u32>().unwrap(), spl3[1][1].parse::<u32>().unwrap(), spl3[2][1].parse::<u32>().unwrap()];
                let normali: [u32; 3] = [spl3[0][2].parse::<u32>().unwrap(), spl3[1][2].parse::<u32>().unwrap(), spl3[2][2].parse::<u32>().unwrap()];
                inorm.push(normali[0]);
                inorm.push(normali[1]);
                inorm.push(normali[2]);

                ivert.push(posi[0]);
                ivert.push(posi[1]);
                ivert.push(posi[2]);

                iuv.push(uvi[0]);
                iuv.push(uvi[1]);
                iuv.push(uvi[2]);
                continue;
            }
        }
        objbegind.push([ivert.len(), iuv.len(), inorm.len()]);
        for j in 0..objcnt{
            for i in objbegind[j][0]..objbegind[j+1][0]{
                fnvrt.push(vert[ivert[i] as usize - 1][0]);
                fnvrt.push(vert[ivert[i] as usize - 1][1]);
                fnvrt.push(vert[ivert[i] as usize - 1][2]);
            }
            for i in objbegind[j][1]..objbegind[j+1][1]{
                fnvrt.push(uv[iuv[i] as usize - 1][0]);
                fnvrt.push(uv[iuv[i] as usize - 1][1]);
            }
            for i in objbegind[j][2]..objbegind[j+1][2]{
                fnvrt.push(norm[inorm[i] as usize - 1][0]);
                fnvrt.push(norm[inorm[i] as usize - 1][1]);
                fnvrt.push(norm[inorm[i] as usize - 1][2]);
            }
            fnobj.push(fnvrt);
            fnvrt = vec![];
        }
        ModelAsset { 
            vertices: fnobj, 
            matnam: mtsl,
            mtl: mtl,
        }
    }
}