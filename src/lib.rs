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

  let mut scn = Scene::new(vec![]);
  scn.load_objects("charliesdf");
  scn.create_objects(&mut eng);

  let mut matgen = MaterialGenerator::new(vec![]);
  matgen.gen_post_vertex();
  matgen.gen_fragpost_beg();
  matgen.fragment_shader += "
  let albedo = pow(textureSample(mainMap, mySampler, in.uv, 0).rgb, vec3f(2.2));
  let WorldPos = textureSample(positionMap, mySampler, in.uv, 0).rgb;
  let norm = textureSample(normalMap, mySampler, in.uv, 0).rgb;
  let mat = textureSample(matMap, mySampler, in.uv, 0).rgb;

  let shadow = shcalc(WorldPos, 0.0);
  let metallic = mat.g;
  let roughness = mat.r;
  let ao = mat.b;

  let color = PBR(norm, albedo, shadow, metallic, roughness, ao, WorldPos);

  return vec4f(color, 1.0);";
  matgen.gen_frag_end();

  let mut renderplane: Object = Object::new(&eng, PLANE.to_vec(), &matgen.generate_material("".to_string(), "".to_string()), engine::render::mesh::MUsages::PostProcessing, true);

  eng.cameras[0].physic_object.pos = Vec3::newdefined(26f32, 80f32, -12f32);
  eng.cameras[0].physic_object.rot = Vec3::newdefined(0f32, -2f32, 0f32);

  eng.lights[0].pos = Vec3::newdefined(26f32, 40f32, -12f32);

  start_loop(Closure::new(move || {
    eng.start();

    eng.cameras[0].physic_object.rot.x += get_mouse_y() as f32/eng.render.get_canvas_size_y()as f32;
    eng.cameras[0].physic_object.rot.y += get_mouse_x() as f32/eng.render.get_canvas_size_x()as f32;
    if is_key_pressed(11){
      eng.cameras[0].physic_object.speed.z = f32::cos(eng.cameras[0].physic_object.rot.x) * f32::cos(eng.cameras[0].physic_object.rot.y) * SPEED;
      eng.cameras[0].physic_object.speed.x = f32::cos(eng.cameras[0].physic_object.rot.x) * f32::sin(eng.cameras[0].physic_object.rot.y) * -SPEED;
    }
    if is_key_pressed(1){
      eng.cameras[0].physic_object.speed.z = f32::cos(eng.cameras[0].physic_object.rot.x) * f32::cos(eng.cameras[0].physic_object.rot.y) * -SPEED;
      eng.cameras[0].physic_object.speed.x = f32::cos(eng.cameras[0].physic_object.rot.x) * f32::sin(eng.cameras[0].physic_object.rot.y) * SPEED;
    }
    if is_key_pressed(12){
      eng.cameras[0].physic_object.speed.x = f32::cos(eng.cameras[0].physic_object.rot.x) * f32::cos(eng.cameras[0].physic_object.rot.y) * SPEED;
      eng.cameras[0].physic_object.speed.z = f32::cos(eng.cameras[0].physic_object.rot.x) * f32::sin(eng.cameras[0].physic_object.rot.y) * SPEED;
    }
    if is_key_pressed(10){
      eng.cameras[0].physic_object.speed.x = f32::cos(eng.cameras[0].physic_object.rot.x) * f32::cos(eng.cameras[0].physic_object.rot.y) * -SPEED;
      eng.cameras[0].physic_object.speed.z = f32::cos(eng.cameras[0].physic_object.rot.x) * f32::sin(eng.cameras[0].physic_object.rot.y) * -SPEED;
    }
    if get_mouse_right_click(){
      let ind = eng.lights.len()-1;
      eng.lights[ind].pos = Vec3::newdefined(eng.cameras[0].physic_object.pos.x, eng.cameras[0].physic_object.pos.y, eng.cameras[0].physic_object.pos.z);
      eng.lights[ind].rot = Vec3::newdefined(eng.cameras[0].physic_object.rot.x, eng.cameras[0].physic_object.rot.y, eng.cameras[0].physic_object.rot.z);
      eng.lights[ind].color = Vec3::newdefined(10f32, 10f32, 9.9f32);
    }
    if get_mouse_middle_click(){
      let ind = eng.lights.len();
      eng.lights.push(Light::new(engine::light::LightType::Spot));
      eng.lights[ind].pos = Vec3::newdefined(eng.cameras[0].physic_object.pos.x, eng.cameras[0].physic_object.pos.y, eng.cameras[0].physic_object.pos.z);
      eng.lights[ind].rot = Vec3::newdefined(eng.cameras[0].physic_object.rot.x, eng.cameras[0].physic_object.rot.y, eng.cameras[0].physic_object.rot.z);
      eng.lights[ind].color = Vec3::newdefined(10f32, 10f32, 9.9f32);
    }
    if is_key_pressed(38){
      eng.renderscale = 0.5f32;
    }
    if is_key_pressed(39){
      eng.renderscale = 1.0f32;
    }

    scn.exec(&mut eng);
    renderplane.exec(&mut eng);
  }));
}