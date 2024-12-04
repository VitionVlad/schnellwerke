use engine::engine::Engine;
use engine::light::Light;
use engine::material::MaterialGenerator;
use engine::object::Object;
use engine::plane::PLANE;
use engine::render::rloop::logic_loop;
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
  const SPEED: f32 = 0.01f32;
  let mut eng: Engine = Engine::new("render");
  eng.lights = vec![];

  let mut scn = Scene::new(vec![]);
  scn.load_objects(&mut eng, "charliesdf");
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

  let mut renderplane: Object = Object::new(&mut eng, PLANE.to_vec(), &matgen.generate_material("".to_string(), "".to_string()), engine::render::mesh::MUsages::PostProcessing, true);

  eng.cameras[0].physic_object.pos = Vec3::newdefined(26f32, 80f32, -12f32);
  eng.cameras[0].physic_object.rot = Vec3::newdefined(0f32, -2f32, 0f32);

  logic_loop(Closure::new(move || {
    eng.start();

    if eng.touch.is_touching(){
      let xp = eng.touch.get_x_touch() as f32 / eng.render.get_canvas_size_x()as f32;
      let yp = 1.0 - eng.touch.get_y_touch() as f32 / eng.render.get_canvas_size_y()as f32;
      if xp <= 0.5{
        eng.cameras[0].physic_object.speed.z += f32::cos(eng.cameras[0].physic_object.rot.x) * f32::cos(eng.cameras[0].physic_object.rot.y) * ((yp*2.0)-1.0)*-SPEED;
        eng.cameras[0].physic_object.speed.x += f32::cos(eng.cameras[0].physic_object.rot.x) * f32::sin(eng.cameras[0].physic_object.rot.y) * ((yp*2.0)-1.0)*SPEED;
        eng.cameras[0].physic_object.speed.x += f32::cos(eng.cameras[0].physic_object.rot.x) * f32::cos(eng.cameras[0].physic_object.rot.y) * ((xp*4.0)-1.0)* SPEED;
        eng.cameras[0].physic_object.speed.z += f32::cos(eng.cameras[0].physic_object.rot.x) * f32::sin(eng.cameras[0].physic_object.rot.y) * ((xp*4.0)-1.0)* SPEED;
      }else{
        eng.cameras[0].physic_object.rot.x += -((yp*2.0)-1.0)/100.0;
        eng.cameras[0].physic_object.rot.y += (((xp-0.5)*4.0)-1.0)/100.0;
      }
    }
    eng.cameras[0].physic_object.rot.x += eng.mouse.get_y_coords() as f32/eng.render.get_canvas_size_y()as f32;
    eng.cameras[0].physic_object.rot.y += eng.mouse.get_x_coords() as f32/eng.render.get_canvas_size_x()as f32;
    if eng.keyboard.is_key_pressed(11){
      eng.cameras[0].physic_object.speed.z += f32::cos(eng.cameras[0].physic_object.rot.x) * f32::cos(eng.cameras[0].physic_object.rot.y) * SPEED;
      eng.cameras[0].physic_object.speed.x += f32::cos(eng.cameras[0].physic_object.rot.x) * f32::sin(eng.cameras[0].physic_object.rot.y) * -SPEED;
    }
    if eng.keyboard.is_key_pressed(1){
      eng.cameras[0].physic_object.speed.z += f32::cos(eng.cameras[0].physic_object.rot.x) * f32::cos(eng.cameras[0].physic_object.rot.y) * -SPEED;
      eng.cameras[0].physic_object.speed.x += f32::cos(eng.cameras[0].physic_object.rot.x) * f32::sin(eng.cameras[0].physic_object.rot.y) * SPEED;
    }
    if eng.keyboard.is_key_pressed(12){
      eng.cameras[0].physic_object.speed.x += f32::cos(eng.cameras[0].physic_object.rot.x) * f32::cos(eng.cameras[0].physic_object.rot.y) * SPEED;
      eng.cameras[0].physic_object.speed.z += f32::cos(eng.cameras[0].physic_object.rot.x) * f32::sin(eng.cameras[0].physic_object.rot.y) * SPEED;
    }
    if eng.keyboard.is_key_pressed(10){
      eng.cameras[0].physic_object.speed.x += f32::cos(eng.cameras[0].physic_object.rot.x) * f32::cos(eng.cameras[0].physic_object.rot.y) * -SPEED;
      eng.cameras[0].physic_object.speed.z += f32::cos(eng.cameras[0].physic_object.rot.x) * f32::sin(eng.cameras[0].physic_object.rot.y) * -SPEED;
    }
    if eng.mouse.get_right_mouse_button(){
      let ind = eng.lights.len()-1;
      eng.lights[ind].pos = Vec3::newdefined(eng.cameras[0].physic_object.pos.x, eng.cameras[0].physic_object.pos.y, eng.cameras[0].physic_object.pos.z);
      eng.lights[ind].rot = Vec3::newdefined(eng.cameras[0].physic_object.rot.x, eng.cameras[0].physic_object.rot.y, eng.cameras[0].physic_object.rot.z);
      eng.lights[ind].color = Vec3::newdefined(10f32, 10f32, 9.9f32);
      log(&("lstat ".to_string() + &eng.lights[ind].pos.x.to_string() + &" ".to_string() + &eng.lights[ind].pos.y.to_string() + &" ".to_string() + &eng.lights[ind].pos.z.to_string() + &" ".to_string() + &eng.lights[ind].rot.x.to_string() + &" ".to_string() + &eng.lights[ind].rot.y.to_string() + &" ".to_string() + &eng.lights[ind].rot.z.to_string()));
    }
    if eng.mouse.get_middle_mouse_button(){
      let ind = eng.lights.len();
      eng.lights.push(Light::new(engine::light::LightType::Spot));
      eng.lights[ind].pos = Vec3::newdefined(eng.cameras[0].physic_object.pos.x, eng.cameras[0].physic_object.pos.y, eng.cameras[0].physic_object.pos.z);
      eng.lights[ind].rot = Vec3::newdefined(eng.cameras[0].physic_object.rot.x, eng.cameras[0].physic_object.rot.y, eng.cameras[0].physic_object.rot.z);
      eng.lights[ind].color = Vec3::newdefined(10f32, 10f32, 9.9f32);
      log(&("lstat ".to_string() + &eng.lights[ind].pos.x.to_string() + &" ".to_string() + &eng.lights[ind].pos.y.to_string() + &" ".to_string() + &eng.lights[ind].pos.z.to_string() + &" ".to_string() + &eng.lights[ind].rot.x.to_string() + &" ".to_string() + &eng.lights[ind].rot.y.to_string() + &" ".to_string() + &eng.lights[ind].rot.z.to_string()));
    }
    if eng.keyboard.is_key_pressed(38){
      eng.renderscale = 0.5f32;
    }
    if eng.keyboard.is_key_pressed(39){
      eng.renderscale = 1.0f32;
    }

    scn.exec(&mut eng);
    renderplane.exec(&mut eng);
  }), 4);
}