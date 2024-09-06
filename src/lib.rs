use engine::engine::{start_loop, Engine};
use engine::input::keyboard::is_key_pressed;
use engine::input::mouse::{get_mouse_middle_click, get_mouse_right_click, get_mouse_x, get_mouse_y};
use engine::light::Light;
use engine::material::MaterialGenerator;
use engine::object::Object;
use engine::plane::PLANE;
use engine::scene::Scene;
use wasm_bindgen::prelude::*;
use engine::render::render::*;
use engine::resourceloader::resourceloader::Objreader;
use engine::math::vec3::Vec3;
mod engine;

#[wasm_bindgen]
extern {
  
  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);
}

#[wasm_bindgen]
pub fn main() {
  const SPEED: f32 = 0.1f32;
  let mut eng: Engine = Engine::new("render");

  let mut matgen = MaterialGenerator::new(vec![]);
  matgen.culling_mode_shadow = "front".to_string();
  matgen.gen_vertex();
  matgen.gen_frag_beg();
  matgen.fragment_shader += "
    output.albedo = textureSample(myTexture, mySampler, in.uv, 0).rgba;
    output.material.r = textureSample(myTexture, mySampler, in.uv, 1).r;
    output.material.g = 1.0 - textureSample(myTexture, mySampler, in.uv, 1).r;
    output.material.b = textureSample(myTexture, mySampler, in.uv, 2).r;

    let TBN = mat3x3<f32>(
      in.tangent,
      in.bitangent,
      in.norm,
    );
    output.normal = vec4f(TBN * (textureSample(myTexture, mySampler, in.uv, 3).rgb * 2.0 - 1.0), 1.0);
    output.position = in.vp;
    return output;";
  matgen.gen_frag_end();

  let mut mesharr: Vec<Object> = vec![];

  let mut res: Objreader = Objreader::new("groundmd");
  mesharr.push(Object::new(&eng, res.arr, matgen.generate_material("alb1ground;roug1ground;ao1ground;norm1ground".to_string(), "".to_string()), engine::render::mesh::MUsages::ShadowAndMain));
  res = Objreader::new("roadmd");
  mesharr.push(Object::new(&eng, res.arr, matgen.generate_material("alb1road;roug1road;ao1road;norm1road".to_string(), "".to_string()), engine::render::mesh::MUsages::ShadowAndMain));
  res = Objreader::new("pavmd");
  mesharr.push(Object::new(&eng, res.arr, matgen.generate_material("alb1pav;roug1pav;ao1pav;norm1pav".to_string(), "".to_string()), engine::render::mesh::MUsages::ShadowAndMain));
  res = Objreader::new("pavmd");
  mesharr.push(Object::new(&eng, res.arr, matgen.generate_material("alb1pav;roug1pav;ao1pav;norm1pav".to_string(), "".to_string()), engine::render::mesh::MUsages::ShadowAndMain));
  res = Objreader::new("fencemd");
  mesharr.push(Object::new(&eng, res.arr, matgen.generate_material("alb1fence;roug1fence;ao1fence;norm1fence".to_string(), "".to_string()), engine::render::mesh::MUsages::ShadowAndMain));
  res = Objreader::new("fencemd");
  mesharr.push(Object::new(&eng, res.arr, matgen.generate_material("alb1fence;roug1fence;ao1fence;norm1fence".to_string(), "".to_string()), engine::render::mesh::MUsages::ShadowAndMain));
  res = Objreader::new("build1md");
  mesharr.push(Object::new(&eng, res.arr, matgen.generate_material("alb1build;roug1build;ao1build;norm1build".to_string(), "".to_string()), engine::render::mesh::MUsages::ShadowAndMain));
  res = Objreader::new("build1md");
  mesharr.push(Object::new(&eng, res.arr, matgen.generate_material("alb1build;roug1build;ao1build;norm1build".to_string(), "".to_string()), engine::render::mesh::MUsages::ShadowAndMain));
  res = Objreader::new("build2md");
  mesharr.push(Object::new(&eng, res.arr, matgen.generate_material("alb1build;roug1build;ao1build;norm1build".to_string(), "".to_string()), engine::render::mesh::MUsages::ShadowAndMain));
  res = Objreader::new("checkpointmd");
  mesharr.push(Object::new(&eng, res.arr, matgen.generate_material("alb1checkpoint;roug1checkpoint;ao1checkpoint;norm1checkpoint".to_string(), "".to_string()), engine::render::mesh::MUsages::ShadowAndMain));
  res = Objreader::new("checkpointmd");
  mesharr.push(Object::new(&eng, res.arr, matgen.generate_material("alb1checkpoint;roug1checkpoint;ao1checkpoint;norm1checkpoint".to_string(), "".to_string()), engine::render::mesh::MUsages::ShadowAndMain));
  res = Objreader::new("b1md");
  mesharr.push(Object::new(&eng, res.arr, matgen.generate_material("alb1b;roug1b;ao1b;norm1b".to_string(), "".to_string()), engine::render::mesh::MUsages::ShadowAndMain));
  res = Objreader::new("b2md");
  mesharr.push(Object::new(&eng, res.arr, matgen.generate_material("alb2b;roug2b;ao2b;norm2b".to_string(), "".to_string()), engine::render::mesh::MUsages::ShadowAndMain));
  res = Objreader::new("s2md");
  mesharr.push(Object::new(&eng, res.arr, matgen.generate_material("alb1s;roug1s;ao1s;norm1s".to_string(), "".to_string()), engine::render::mesh::MUsages::ShadowAndMain));
  res = Objreader::new("s2md");
  mesharr.push(Object::new(&eng, res.arr, matgen.generate_material("alb1s;roug1s;ao1s;norm1s".to_string(), "".to_string()), engine::render::mesh::MUsages::ShadowAndMain));
  res = Objreader::new("s1md");
  mesharr.push(Object::new(&eng, res.arr, matgen.generate_material("alb1s;roug1s;ao1s;norm1s".to_string(), "".to_string()), engine::render::mesh::MUsages::ShadowAndMain));
  res = Objreader::new("s1md");
  mesharr.push(Object::new(&eng, res.arr, matgen.generate_material("alb1s;roug1s;ao1s;norm1s".to_string(), "".to_string()), engine::render::mesh::MUsages::ShadowAndMain));
  res = Objreader::new("s3md");
  mesharr.push(Object::new(&eng, res.arr, matgen.generate_material("alb1s;roug1s;ao1s;norm1s".to_string(), "".to_string()), engine::render::mesh::MUsages::ShadowAndMain));
  res = Objreader::new("s3md");
  mesharr.push(Object::new(&eng, res.arr, matgen.generate_material("alb1s;roug1s;ao1s;norm1s".to_string(), "".to_string()), engine::render::mesh::MUsages::ShadowAndMain));
  res = Objreader::new("s3md");
  mesharr.push(Object::new(&eng, res.arr, matgen.generate_material("alb1s;roug1s;ao1s;norm1s".to_string(), "".to_string()), engine::render::mesh::MUsages::ShadowAndMain));

  matgen.culling_mode_shadow = "none".to_string();
  matgen.gen_vertex();
  matgen.gen_frag_beg();
  matgen.fragment_shader += "
    output.albedo = textureSample(myTexture, mySampler, in.uv, 0).rgba;
    output.normal = vec4f(in.norm, 1.0);
    output.position = in.vp;
    return output;";
  matgen.gen_frag_end();

  res = Objreader::new("f1md");
  mesharr.push(Object::new(&eng, res.arr, matgen.generate_material("us".to_string(), "".to_string()), engine::render::mesh::MUsages::ShadowAndMain));
  res = Objreader::new("f2md");
  mesharr.push(Object::new(&eng, res.arr, matgen.generate_material("ddr".to_string(), "".to_string()), engine::render::mesh::MUsages::ShadowAndMain));
  res = Objreader::new("f3md");
  mesharr.push(Object::new(&eng, res.arr, matgen.generate_material("ussr".to_string(), "".to_string()), engine::render::mesh::MUsages::ShadowAndMain));

  let scn = Scene::new("charliesdf");
  scn.set_objects(&mut mesharr);

  matgen.gen_post_vertex();
  matgen.gen_fragpost_beg();
  matgen.fragment_shader += "
  let albedo = pow(textureSample(mainMap, mySampler, in.uv, 0).rgb, vec3f(2.2));
  let WorldPos = textureSample(positionMap, mySampler, in.uv, 0).rgb;
  let norm = textureSample(normalMap, mySampler, in.uv, 0).rgb;
  let mat = textureSample(matMap, mySampler, in.uv, 0).rgb;

  var visibility = 0.0;
  for (var i = 0; i < LIGHTMN; i++) {
    let smv = ubo.smvp[i] * vec4f(WorldPos, 1.0);
    let proj = vec3f((smv.x / smv.w)*0.5+0.5, (smv.y / smv.w)*-0.5+0.5, smv.z / smv.w);
    let oneOverShadowDepthTextureSize = 1.0 / 1000.0;
    for (var y = -1; y <= 1; y++) {
      for (var x = -1; x <= 1; x++) {
        let offset = vec2f(vec2(x, y)) * oneOverShadowDepthTextureSize;
        visibility += textureSampleCompare(
          shadowMap, shadowSampler,
          proj.xy + offset, i, proj.z
        );
      }
    }
  }
  let shadow = visibility / 9.0;
  let metallic = mat.g;
  let roughness = mat.r;

  let N = normalize(norm);
  let V = normalize(ubo.pos[0].xyz - WorldPos);

  var F0 = vec3f(0.04); 
  F0 = mix(F0, albedo, mat.g);
	         
  var Lo = vec3f(0.0);

  for(var i = 0; i < LIGHTN; i++) {
    let L = normalize(ubo.lpos[i].xyz - WorldPos);
    let H = normalize(V + L);
    let distance    = length(ubo.lpos[i].xyz - WorldPos);
    let attenuation = 1.0 / (distance * distance);
    let radiance     = (ubo.lcolor[i].xyz) * attenuation;        
    
    let NDF = DistributionGGX(N, H, roughness);        
    let G   = GeometrySmith(N, V, L, roughness);      
    let F   = fresnelSchlick(max(dot(H, V), 0.0), F0);       
    
    let kS = F;
    var kD = vec3(1.0) - kS;
    kD *= 1.0 - metallic;	  
    
    let numerator    = NDF * G * F;
    let denominator = 4.0 * max(dot(N, V), 0.0) * max(dot(N, L), 0.0) + 0.0001;
    let specular     = numerator / denominator;  
        
    let NdotL = max(dot(N, L), 0.0);                
    Lo += (kD * albedo / PI + specular) * radiance * NdotL; 
  }

  let ambient = vec3(0.001) * albedo * mat.b;
  var color = ambient + shadow * Lo;

  color = color / (color + vec3(1.0));
  color = pow(color, vec3(1.0/2.2));  

  return vec4f(color, 1.0);";
  matgen.gen_frag_end();

  let mut renderplane: Object = Object::new(&eng, PLANE.to_vec(), matgen.generate_material("".to_string(), "".to_string()), engine::render::mesh::MUsages::PostProcessing);

  eng.cameras[0].pos = Vec3::newdefined(26f32, 4f32, -12f32);
  eng.cameras[0].rot = Vec3::newdefined(0f32, -2f32, 0f32);

  eng.lights[0].pos = Vec3::newdefined(26f32, 40f32, -12f32);
  eng.lights[0].rot = Vec3::newdefined(1.57f32, 0f32, 0f32);

  start_loop(Closure::new(move || {
    eng.cameras[0].rot.x += get_mouse_y() as f32/eng.render.get_canvas_size_y()as f32;
    eng.cameras[0].rot.y += get_mouse_x() as f32/eng.render.get_canvas_size_x()as f32;
    if is_key_pressed(11){
      eng.cameras[0].pos.z += f32::cos(eng.cameras[0].rot.x) * f32::cos(eng.cameras[0].rot.y) * SPEED;
      eng.cameras[0].pos.x += f32::cos(eng.cameras[0].rot.x) * f32::sin(eng.cameras[0].rot.y) * -SPEED;
    }
    if is_key_pressed(1){
      eng.cameras[0].pos.z += f32::cos(eng.cameras[0].rot.x) * f32::cos(eng.cameras[0].rot.y) * -SPEED;
      eng.cameras[0].pos.x += f32::cos(eng.cameras[0].rot.x) * f32::sin(eng.cameras[0].rot.y) * SPEED;
    }
    if is_key_pressed(12){
      eng.cameras[0].pos.x += f32::cos(eng.cameras[0].rot.x) * f32::cos(eng.cameras[0].rot.y) * SPEED;
      eng.cameras[0].pos.z += f32::cos(eng.cameras[0].rot.x) * f32::sin(eng.cameras[0].rot.y) * SPEED;
    }
    if is_key_pressed(10){
      eng.cameras[0].pos.x += f32::cos(eng.cameras[0].rot.x) * f32::cos(eng.cameras[0].rot.y) * -SPEED;
      eng.cameras[0].pos.z += f32::cos(eng.cameras[0].rot.x) * f32::sin(eng.cameras[0].rot.y) * -SPEED;
    }
    if get_mouse_right_click(){
      let ind = eng.lights.len()-1;
      eng.lights[ind].pos = Vec3::newdefined(eng.cameras[0].pos.x, eng.cameras[0].pos.y, eng.cameras[0].pos.z);
      eng.lights[ind].rot = Vec3::newdefined(eng.cameras[0].rot.x, eng.cameras[0].rot.y, eng.cameras[0].rot.z);
      eng.lights[ind].color = Vec3::newdefined(10f32, 10f32, 9.9f32);
    }
    if get_mouse_middle_click(){
      let ind = eng.lights.len();
      eng.lights.push(Light::new(engine::light::LightType::Spot));
      eng.lights[ind].pos = Vec3::newdefined(eng.cameras[0].pos.x, eng.cameras[0].pos.y, eng.cameras[0].pos.z);
      eng.lights[ind].rot = Vec3::newdefined(eng.cameras[0].rot.x, eng.cameras[0].rot.y, eng.cameras[0].rot.z);
      eng.lights[ind].color = Vec3::newdefined(10f32, 10f32, 9.9f32);
    }
    if is_key_pressed(38){
      eng.renderscale = 0.5f32;
    }
    if is_key_pressed(39){
      eng.renderscale = 1.0f32;
    }
    eng.start();
    for i in 0..mesharr.len(){
      mesharr[i].exec(&mut eng);
    }
    renderplane.exec(&mut eng);
  }));
}