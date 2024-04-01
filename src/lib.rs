use engine::engine::Engine;
use engine::math::uniformstruct::*;
use crate::engine::math::vec4::Vec4;
use engine::object::Object;
use engine::input::keyboard::is_key_pressed;
use engine::input::mouse::{get_mouse_x, get_mouse_y};
use engine::input::touch::*;
use wasm_bindgen::prelude::*;
use crate::engine::audiosource3d::Audiosource3d;
use crate::engine::math::vec3::Vec3;
use crate::engine::math::vec2::Vec2;
use crate::engine::shader_builder::ShaderBuilder;
use engine::animation::Keytiming;
mod engine;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn main() {
    const SPEED: f32 = 3.8f32;
    let mut eng: Engine = Engine::new("render", 1f32, 4000);

    let mut uniforms: Vec<Uniformstruct> = vec![];
    uniforms.push(createvec4(Vec4::newdefined(0.8f32, -1.0f32, -0.8f32, 0.0f32), "lightpos"));
    uniforms.push(createvec4(Vec4::newdefined(1f32, 1f32, 1f32, 0.2f32), "lightcolor"));

    let mut shaders = ShaderBuilder::new(&uniforms);
    shaders.new_fragment_shader();
    shaders.fragment_begin_main();
    shaders.fragment_add_light(true, "lightcolor", "lightpos");
    shaders.fragment_end_main();

    let mut mesh: Object = Object::new_from_obj(&eng, "md1", &shaders.vertex_code, &shaders.shadow_vertex_code, &shaders.fragment_code, &uniforms, "tex;stex;ntex", "", "linear", "linear", "none", "none", "repeat", false);
    mesh.scale = Vec3::newdefined(0.025, 0.025, 0.025);

    let mut mesh2: Object = Object::new_from_obj(&eng, "md2", &shaders.vertex_code, &shaders.shadow_vertex_code, &shaders.fragment_code, &uniforms, "tex2;stex2;ntex2", "", "linear", "linear", "none", "none", "repeat", false);
    mesh2.scale = Vec3::newdefined(0.025, 0.025, 0.025);
    let mut mesh3: Object = Object::new_from_obj(&eng, "md3", &shaders.vertex_code, &shaders.shadow_vertex_code, &shaders.fragment_code, &uniforms, "tex3;stex3;ntex3", "", "linear", "linear", "none", "none", "repeat", false);
    mesh3.scale = Vec3::newdefined(0.025, 0.025, 0.025); 
    mesh3.collision_detect = false;

    let mut mesh4: Object = Object::new_from_obj(&eng, "md4", &shaders.vertex_code, &shaders.shadow_vertex_code, &shaders.fragment_code, &uniforms, "tex4;stex4;ntex4", "", "linear", "linear", "none", "none", "repeat", false);
    mesh4.scale = Vec3::newdefined(0.025, 0.025, 0.025);
    mesh4.collision_detect = false;

    let mut mesh5: Object = Object::new_from_obj(&eng, "md5", &shaders.vertex_code, &shaders.shadow_vertex_code, &shaders.fragment_code, &uniforms, "tex5;stex5;ntex5", "", "linear", "linear", "none", "none", "repeat", false);
    mesh5.scale = Vec3::newdefined(0.025, 0.025, 0.025);
    
    let mut mesh6: Object = Object::new_from_obj(&eng, "md6", &shaders.vertex_code, &shaders.shadow_vertex_code, &shaders.fragment_code, &uniforms, "tex6;stex6;ntex6", "", "linear", "linear", "none", "none", "repeat", false);
    mesh6.scale = Vec3::newdefined(0.025, 0.025, 0.025);
    mesh6.collision_detect = false;

    let mut mesh7: Object = Object::new_from_obj(&eng, "md7", &shaders.vertex_code, &shaders.shadow_vertex_code, &shaders.fragment_code, &uniforms, "tex7;stex7;ntex7", "", "linear", "linear", "none", "none", "repeat", false);
    mesh7.scale = Vec3::newdefined(0.025, 0.025, 0.025);
    mesh7.collision_detect = false;

    let mut mesh8: Object = Object::new_from_obj(&eng, "md8", &shaders.vertex_code, &shaders.shadow_vertex_code, &shaders.fragment_code, &uniforms, "tex8;stex8;ntex8", "", "linear", "linear", "none", "none", "repeat", false);
    mesh8.scale = Vec3::newdefined(0.025, 0.025, 0.025);

    let mut mesh9: Object = Object::new_from_obj(&eng, "md9", &shaders.vertex_code, &shaders.shadow_vertex_code, &shaders.fragment_code, &uniforms, "tex9;stex9;ntex9", "", "linear", "linear", "none", "none", "repeat", false);
    mesh9.scale = Vec3::newdefined(0.025, 0.025, 0.025);
    mesh9.collision_detect = false;

    let mut mesh10: Object = Object::new_from_obj(&eng, "md10", &shaders.vertex_code, &shaders.shadow_vertex_code, &shaders.fragment_code, &uniforms, "tex10;stex10;ntex10", "", "linear", "linear", "none", "none", "repeat", false);
    mesh10.scale = Vec3::newdefined(0.025, 0.025, 0.025);

    let mut mesh11: Object = Object::new_from_obj(&eng, "md11", &shaders.vertex_code, &shaders.shadow_vertex_code, &shaders.fragment_code, &uniforms, "tex11;stex11;ntex11", "", "linear", "linear", "none", "none", "repeat", false);
    mesh11.scale = Vec3::newdefined(0.025, 0.025, 0.025);
    mesh11.collision_detect = false;

    let mut mesh12: Object = Object::new_cube(&eng, &shaders.vertex_code, &shaders.shadow_vertex_code, &shaders.fragment_code, &uniforms, "tex11;stex11;ntex11", "", "linear", "linear", "none", "none", "repeat", false);

    let mut anim = Keytiming::new(10000, &mesh12, Vec3::newdefined(0f32, 8f32, -5f32), Vec3::newdefined(10f32, 5f32, 2.5f32), Vec3::newdefined(1f32, 1.5f32, 1f32));

    shaders.new_fragment_shader();
    shaders.fragment_begin_main();
    shaders.fragment_code += "
      col += vec4(textureSample(mainMap, mySampler, in.uv).rgb, 1.0);
    ";
    shaders.fragment_end_main();

    let mut reshnquad: Object = Object::new_plane(&eng, &shaders.vertex_code, &shaders.shadow_vertex_code, &shaders.fragment_code, &uniforms, "", "", "nearest", "nearest", "none", "none", "clamp-to-edge", false);
    reshnquad.pos.x = 5f32;
    reshnquad.rot.z = 1.5708f32;
    reshnquad.rot.y = 1.5708f32;
    reshnquad.rot.x = 1.5708f32;
    reshnquad.pos.y = 3.8f32;
    reshnquad.scale = Vec3::newdefined(3.8f32, 3.8f32, 1f32);

    shaders = ShaderBuilder::new_skybox(&uniforms);
    shaders.new_fragment_shader();
    shaders.fragment_begin_main();
    shaders.fragment_code += "
      col += vec4f(textureSample(mycube, mySampler, in.vertex.xyz).rgb+0.5, 1);
    ";
    shaders.fragment_end_main();

    let mut skybox: Object = Object::new_cube(&eng, &shaders.vertex_code, &shaders.shadow_vertex_code, &&shaders.fragment_code, &uniforms, "", "right;left;top;bottom;front;back", "linear", "linear", "front", "back", "repeat", false);
    skybox.collision_detect = false;
    skybox.scale = Vec3::newdefined(1000f32, 1000f32, 1000f32);

    shaders = ShaderBuilder::new_post_procces(&uniforms);
    shaders.new_fragment_shader();
    shaders.fragment_begin_main();
    shaders.fragment_add_bloom();
    shaders.fragment_add_kbao();
    shaders.fragment_add_mainframebuffer();
    shaders.fragment_end_main();

    let mut renquad: Object = Object::new_plane(&eng, &shaders.vertex_code, &shaders.shadow_vertex_code, &&shaders.fragment_code, &uniforms, "", "", "nearest", "nearest", "none", "none", "clamp-to-edge", true);
    renquad.collision_detect = false;

    eng.pos.y = -20f32;

    let mut as1 = Audiosource3d::new("assets/sample.mp3", Vec3::newdefined(0f32, -4f32, 0f32), 10f32);

    eng.shadowpos = Vec3::newdefined(80f32, -142f32, -35f32);
    eng.shadoworthographic = true;
    eng.shadowfov = 50f32;
    eng.shadow_z_far = 220f32;
    eng.shadowrot = Vec2::newdefined(1.05f32, 1.05f32);

    let drawloop = move || {
      eng.speed.y = SPEED;
      {
        eng.rot.x += get_mouse_y() as f32/eng.ren.get_canvas_size_y()as f32;
        eng.rot.y += get_mouse_x() as f32/eng.ren.get_canvas_size_x()as f32;
        if is_key_pressed(87){
          eng.speed.z = f32::cos(eng.rot.x) * f32::cos(eng.rot.y) * SPEED;
          eng.speed.x = f32::cos(eng.rot.x) * f32::sin(eng.rot.y) * -SPEED;
        }
        if is_key_pressed(83){
          eng.speed.z = f32::cos(eng.rot.x) * f32::cos(eng.rot.y) * -SPEED;
          eng.speed.x = f32::cos(eng.rot.x) * f32::sin(eng.rot.y) * SPEED;
        }
        if is_key_pressed(65){
          eng.speed.x = f32::cos(eng.rot.x) * f32::cos(eng.rot.y) * SPEED;
          eng.speed.z = f32::cos(eng.rot.x) * f32::sin(eng.rot.y) * SPEED;
        }
        if is_key_pressed(68){
          eng.speed.x = f32::cos(eng.rot.x) * f32::cos(eng.rot.y) * -SPEED;
          eng.speed.z = f32::cos(eng.rot.x) * f32::sin(eng.rot.y) * -SPEED;
        }
        if is_key_pressed(77){
          as1.audsrc.playng = !as1.audsrc.playng;
        }
        as1.play(&eng);
        if is_key_pressed(82){
          eng.pos.y = -20f32;
          eng.pos.x = 0f32;
          eng.pos.z = 0f32;
        }
        if is_key_pressed(75){
          if eng.renderscale > 0.1f32{
            eng.renderscale-=0.1;
          }
        }
        if is_key_pressed(76){
          eng.renderscale+=0.1;
        }
        set_touch_index(0);
        if get_is_touching(){
          eng.rot.y += ((get_touch_x() as f32/eng.ren.get_canvas_size_x()as f32)*2.0f32 - 1.0f32) / 100f32;
        }
      }

      eng.begin_shadow("clear");

      mesh.draw(&mut eng, &uniforms);
      mesh2.draw(&mut eng, &uniforms);
      mesh3.draw(&mut eng, &uniforms);
      mesh4.draw(&mut eng, &uniforms);
      mesh5.draw(&mut eng, &uniforms);
      mesh6.draw(&mut eng, &uniforms);
      mesh7.draw(&mut eng, &uniforms);
      mesh8.draw(&mut eng, &uniforms);
      mesh9.draw(&mut eng, &uniforms);
      mesh10.draw(&mut eng, &uniforms);
      mesh11.draw(&mut eng, &uniforms);
      anim.play(&eng, &mut mesh12);
      mesh12.draw(&mut eng, &uniforms);
      reshnquad.draw(&mut eng, &uniforms);

      eng.begin_main("clear", "clear");

      mesh.draw(&mut eng, &uniforms);
      mesh2.draw(&mut eng, &uniforms);
      mesh3.draw(&mut eng, &uniforms);
      mesh4.draw(&mut eng, &uniforms);
      mesh5.draw(&mut eng, &uniforms);
      mesh6.draw(&mut eng, &uniforms);
      mesh7.draw(&mut eng, &uniforms);
      mesh8.draw(&mut eng, &uniforms);
      mesh9.draw(&mut eng, &uniforms);
      mesh10.draw(&mut eng, &uniforms);
      mesh11.draw(&mut eng, &uniforms);
      mesh12.draw(&mut eng, &uniforms);
      reshnquad.draw(&mut eng, &uniforms);
      skybox.draw(&mut eng, &uniforms);
      
      eng.begin_post("clear", "clear");

      renquad.draw(&mut eng, &uniforms);

      eng.end();
    };
    engine::render::render::drawloopexec(drawloop)
}