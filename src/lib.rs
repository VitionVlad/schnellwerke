use engine::camera::Camera;
use engine::engine::{start_loop, Engine};
use engine::input::keyboard::is_key_pressed;
use engine::input::mouse::{get_mouse_x, get_mouse_y};
use engine::material::MaterialGenerator;
use engine::object::Object;
use engine::plane::PLANE;
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
  eng.cameras.push(Camera{ pos: Vec3::newdefined(-2f32, 0f32, 4f32), rot: Vec3::newdefined(0f32, 0.5f32, 0f32), fov: 60f32, znear: 0.1f32, zfar: 100f32, is_orthographic: false});
  let res: Objreader = Objreader::new("cube");

  let mut matgen = MaterialGenerator::new(vec![]);
  matgen.gen_vertex();
  matgen.gen_frag_beg();
  matgen.fragment_shader += "
    output.albedo = textureSample(myTexture, mySampler, in.uv, 0).rgba;
    output.normal = vec4f(in.norm, 1.0);
    output.position = in.vp;
    return output;";
  matgen.gen_frag_end();

  let mut mesh1: Object = Object::new(&eng, res.arr, matgen.generate_material("tex".to_string(), "".to_string()), engine::render::mesh::MUsages::ShadowAndMain);

  matgen.gen_frag_beg();
  matgen.fragment_shader += "
    output.albedo = vec4f(in.uv, -1.0, 1.0);
    output.normal = vec4f(in.norm, 1.0);
    output.position = in.vp;
    return output;";
  matgen.gen_frag_end();

  let mut mesh2: Object = Object::new(&eng, PLANE.to_vec(), matgen.generate_material("".to_string(), "".to_string()), engine::render::mesh::MUsages::ShadowAndMain);
  mesh2.pos.x = -2f32;

  matgen.gen_post_vertex();
  matgen.gen_fragpost_beg();
  matgen.fragment_shader += "
  let col = textureSample(mainMap, mySampler, in.uv, 0).rgb;
  let colind = textureSample(mainMap, mySampler, vec2f(col.r, -col.g), 1).rgb;
  if col.b < 0.0 {
    return vec4f(colind, 1);
  }
  return vec4f(col.rgb, 1);";
  matgen.gen_frag_end();

  let mut renderplane: Object = Object::new(&eng, PLANE.to_vec(), matgen.generate_material("".to_string(), "".to_string()), engine::render::mesh::MUsages::PostProcessing);

  mesh1.scale.y = 2f32;
  mesh1.rot.x = 0.5f32;

  eng.cameras[0].pos = Vec3::newdefined(-2f32, 0f32, 4f32);
  eng.cameras[0].rot = Vec3::newdefined(0f32, 0.5f32, 0f32);

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
    eng.start();
    mesh1.rot.x += 0.01f32;
    mesh1.exec(&mut eng);
    mesh2.exec(&mut eng);
    renderplane.exec(&mut eng);
  }));
}