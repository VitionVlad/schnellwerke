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
use engine::scene::Scene;
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
    eng.use_resolution_scale = true;

    let mut scene: Scene = Scene::new(1, true);
    scene.light_shadow_source_pos = Vec3::newdefined(80f32, -142f32, -35f32);
    scene.light_shadow_source_rot = Vec2::newdefined(1.05f32, 1.05f32);

    scene.push_object(&eng, "md1", "tex;stex;ntex", "", Vec3::new(), Vec3::new(), Vec3::newdefined(0.025f32, 0.025f32, 0.025f32));
    scene.push_object(&eng, "md2", "tex2;stex2;ntex2", "", Vec3::new(), Vec3::new(), Vec3::newdefined(0.025f32, 0.025f32, 0.025f32));
    scene.push_object(&eng, "md3", "tex3;stex3;ntex3", "", Vec3::new(), Vec3::new(), Vec3::newdefined(0.025f32, 0.025f32, 0.025f32));
    scene.push_object(&eng, "md4", "tex4;stex4;ntex4", "", Vec3::new(), Vec3::new(), Vec3::newdefined(0.025f32, 0.025f32, 0.025f32));
    scene.push_object(&eng, "md5", "tex5;stex5;ntex5", "", Vec3::new(), Vec3::new(), Vec3::newdefined(0.025f32, 0.025f32, 0.025f32));
    scene.push_object(&eng, "md6", "tex6;stex6;ntex6", "", Vec3::new(), Vec3::new(), Vec3::newdefined(0.025f32, 0.025f32, 0.025f32));
    scene.push_object(&eng, "md7", "tex7;stex7;ntex7", "", Vec3::new(), Vec3::new(), Vec3::newdefined(0.025f32, 0.025f32, 0.025f32));
    scene.push_object(&eng, "md8", "tex8;stex8;ntex8", "", Vec3::new(), Vec3::new(), Vec3::newdefined(0.025f32, 0.025f32, 0.025f32));
    scene.push_object(&eng, "md9", "tex9;stex9;ntex9", "", Vec3::new(), Vec3::new(), Vec3::newdefined(0.025f32, 0.025f32, 0.025f32));
    scene.push_object(&eng, "md10", "tex10;stex10;ntex10", "", Vec3::new(), Vec3::new(), Vec3::newdefined(0.025f32, 0.025f32, 0.025f32));
    scene.push_object(&eng, "md11", "tex11;stex11;ntex11", "", Vec3::new(), Vec3::new(), Vec3::newdefined(0.025f32, 0.025f32, 0.025f32));

    scene.lightsources[0].pos = Vec4::newdefined(0.8f32, -1.0f32, -0.8f32, 0.0f32);
    scene.lightsources[0].color = Vec4::newdefined(1f32, 1f32, 1f32, 0.2f32);

    let mut uniforms: Vec<Uniformstruct> = vec![];
    uniforms.push(createvec4_with_usage(Vec4::newdefined(0.8f32, -1.0f32, -0.8f32, 0.0f32), "lightpos", InShaderUsage::LightPosition));
    uniforms.push(createvec4_with_usage(Vec4::newdefined(1f32, 1f32, 1f32, 0.2f32), "lightcolor", InShaderUsage::LightColor));

    let mut shaders = ShaderBuilder::new(&uniforms);
    shaders.new_fragment_shader();
    shaders.fragment_begin_main();
    shaders.fragment_add_light(true, "lightcolor", "lightpos");
    shaders.fragment_end_main();

    let mut mesh12: Object = Object::new_cube(&eng, &shaders.vertex_code, &shaders.shadow_vertex_code, &shaders.fragment_code, &uniforms, "tex11;stex11;ntex11", "", "linear", "linear", "none", "none", "repeat", false);

    let mut anim = Keytiming::new(10000, &mesh12, Vec3::newdefined(0f32, 8f32, -5f32), Vec3::newdefined(10f32, 5f32, 2.5f32), Vec3::newdefined(1f32, 1.5f32, 1f32));

    shaders.new_fragment_shader();
    shaders.fragment_begin_main();
    shaders.fragment_code += "
      let luv = vec2f(in.position.x/(ubo.ress.x*ubo.ress.z), in.position.y/(ubo.ress.y*ubo.ress.z));
      col += vec4(1.0 - textureSample(mainMap, mySampler, luv).rgb, 1.0);
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

    scene.light_shadow_source_ortho = true;
    scene.light_shadow_source_clip.y = 220f32;
    scene.light_shadow_source_fov = 50f32;

    let drawloop = move || {
      eng.speed.y = SPEED;
      {
        eng.rot.x += get_mouse_y() as f32/eng.ren.get_canvas_size_y()as f32;
        eng.rot.y += get_mouse_x() as f32/eng.ren.get_canvas_size_x()as f32;
        if eng.rot.x > 1.5f32{
          eng.rot.x = 1.5f32;
        }
        if eng.rot.x < -1.5f32{
          eng.rot.x = -1.5f32;
        }
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
        set_touch_index(0);
        if get_is_touching(){
          eng.rot.y += ((get_touch_x() as f32/eng.ren.get_canvas_size_x()as f32)*2.0f32 - 1.0f32) / 100f32;
        }
      }

      scene.draw_shadow(&mut eng);
      anim.play(&eng, &mut mesh12);
      mesh12.draw(&mut eng, &uniforms);

      eng.begin_main("clear", "clear");

      scene.draw(&mut eng);
      mesh12.draw(&mut eng, &uniforms);
      skybox.draw(&mut eng, &uniforms);

      eng.begin_main("load", "load");
      reshnquad.draw(&mut eng, &uniforms);

      eng.begin_post("clear", "clear");
      renquad.draw(&mut eng, &uniforms);

      eng.end();
    };
    engine::render::render::drawloopexec(drawloop)
}