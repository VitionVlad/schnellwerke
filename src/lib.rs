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
mod engine;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn main() {
    const SPEED: f32 = 0.1f32;
    let mut eng: Engine = Engine::new("render", 1f32, 8000);

    let vertices: [f32; 24] = [
        -1.0, -1.0, 1.0, 1.0,
        -1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0, 1.0,

        -1.0, -1.0, 1.0, 1.0,
        1.0, 1.0, 1.0, 1.0,
        1.0, -1.0, 1.0, 1.0
    ];

    let uv: [f32; 12] = [
        0.0, 1.0,
        0.0, 0.0,
        1.0, 0.0,

        0.0, 1.0,
        1.0, 0.0,
        1.0, 1.0,
    ];

    let normals: [f32; 18] = [0f32; 18];

    let fragc: &str = "
    struct UBO {
      mvp_proj: mat4x4<f32>,
      mvp_mod: mat4x4<f32>,
      smvp: mat4x4<f32>,
      lightpos: vec4f,
      lightcolor: vec4f,
      playerpos: vec4f,
    };

    @group(0) @binding(0) var<uniform> ubo: UBO;

    @group(0) @binding(1) var mySampler: sampler;

    @group(0) @binding(2) var myTexture: texture_2d_array<f32>;

    @group(0) @binding(3) var shadowMap: texture_depth_2d;

    @group(0) @binding(5) var shadowSampler: sampler_comparison;

    struct OUT{
      @location(0) uv: vec2f,
      @location(1) smv: vec4f,
      @location(2) norm: vec3f,
      @location(3) tangent: vec3f,
      @location(4) bitangent: vec3f,
      @location(5) vertex: vec4f,
    }

    fn shadowmapping(smv: vec4f) -> f32{
      let proj = vec3f((smv.x / smv.w)*0.5+0.5, (smv.y / smv.w)*-0.5+0.5, smv.z / smv.w);
      return 1.0-textureSampleCompare(shadowMap, shadowSampler, proj.xy, proj.z-0.001);
    }

    fn DirectionalLight(in: OUT) -> vec3f{
      let TBN = mat3x3<f32>(
        normalize(in.tangent),
        normalize(in.bitangent),
          normalize(in.norm),
      );
      let albedo = textureSample(myTexture, mySampler, in.uv, 0).rgb;
      let specularpower = textureSample(myTexture, mySampler, in.uv, 1).r;
      let normal = normalize(TBN * (textureSample(myTexture, mySampler, in.uv, 2).rgb * 2.0 - 1.0));
      let ambient = ubo.lightcolor.a * ubo.lightcolor.rgb;

      let lightdir = normalize(-ubo.lightpos.xyz);
      let diff = max(dot(normal, lightdir), 0.0);
      let diffuse = diff * ubo.lightcolor.rgb;

      let viewDir = normalize(-ubo.playerpos.xyz - in.vertex.xyz);
      let reflectDir = reflect(-lightdir, normal); 
      let spec = pow(max(dot(viewDir, reflectDir), 0.0), 32);
      let specular = specularpower * spec * albedo;  

      return (ambient + (1.0 - shadowmapping(in.smv)) * (diffuse + specular)) * albedo;
    }

    @fragment
    fn fragmentMain(in: OUT) -> @location(0) vec4f {
      return vec4f(DirectionalLight(in), 1);
    }";

    let mut uniforms: Vec<Uniformstruct> = vec![];
    uniforms.push(createmvpmat("mvp"));
    uniforms.push(createsmvpmat("smvp"));
    uniforms.push(createvec4(Vec4::newdefined(0.8f32, -1.0f32, -0.8f32, 0.0f32), "lightpos"));
    uniforms.push(createvec4(Vec4::newdefined(1f32, 1f32, 1f32, 0.2f32), "lightcolor"));
    uniforms.push(createvec4(Vec4::newdefined(0f32, 0f32, 0f32, 0.0f32), "playerpos"));

    let mut shaders = ShaderBuilder::new(&uniforms);

    let mut mesh: Object = Object::new_from_obj(&eng, "md1", &shaders.vertex_code, &shaders.shadow_vertex_code, fragc, &uniforms, "tex;stex;ntex", "", "linear", "linear", "none", "none", false);
    mesh.scale = Vec3::newdefined(0.025, 0.025, 0.025);

    let mut mesh2: Object = Object::new_from_obj(&eng, "md2", &shaders.vertex_code, &shaders.shadow_vertex_code, fragc, &uniforms, "tex2;stex2;ntex2", "", "linear", "linear", "none", "none", false);
    mesh2.scale = Vec3::newdefined(0.025, 0.025, 0.025);
    let mut mesh3: Object = Object::new_from_obj(&eng, "md3", &shaders.vertex_code, &shaders.shadow_vertex_code, fragc, &uniforms, "tex3;stex3;ntex3", "", "linear", "linear", "none", "none", false);
    mesh3.scale = Vec3::newdefined(0.025, 0.025, 0.025); 
    mesh3.collision_detect = false;

    let mut mesh4: Object = Object::new_from_obj(&eng, "md4", &shaders.vertex_code, &shaders.shadow_vertex_code, fragc, &uniforms, "tex4;stex4;ntex4", "", "linear", "linear", "none", "none", false);
    mesh4.scale = Vec3::newdefined(0.025, 0.025, 0.025);
    mesh4.collision_detect = false;

    let mut mesh5: Object = Object::new_from_obj(&eng, "md5", &shaders.vertex_code, &shaders.shadow_vertex_code, fragc, &uniforms, "tex5;stex5;ntex5", "", "linear", "linear", "none", "none", false);
    mesh5.scale = Vec3::newdefined(0.025, 0.025, 0.025);
    
    let mut mesh6: Object = Object::new_from_obj(&eng, "md6", &shaders.vertex_code, &shaders.shadow_vertex_code, fragc, &uniforms, "tex6;stex6;ntex6", "", "linear", "linear", "none", "none", false);
    mesh6.scale = Vec3::newdefined(0.025, 0.025, 0.025);
    mesh6.collision_detect = false;

    let mut mesh7: Object = Object::new_from_obj(&eng, "md7", &shaders.vertex_code, &shaders.shadow_vertex_code, fragc, &uniforms, "tex7;stex7;ntex7", "", "linear", "linear", "none", "none", false);
    mesh7.scale = Vec3::newdefined(0.025, 0.025, 0.025);
    mesh7.collision_detect = false;

    let mut mesh8: Object = Object::new_from_obj(&eng, "md8", &shaders.vertex_code, &shaders.shadow_vertex_code, fragc, &uniforms, "tex8;stex8;ntex8", "", "linear", "linear", "none", "none", false);
    mesh8.scale = Vec3::newdefined(0.025, 0.025, 0.025);

    let mut mesh9: Object = Object::new_from_obj(&eng, "md9", &shaders.vertex_code, &shaders.shadow_vertex_code, fragc, &uniforms, "tex9;stex9;ntex9", "", "linear", "linear", "none", "none", false);
    mesh9.scale = Vec3::newdefined(0.025, 0.025, 0.025);
    mesh9.collision_detect = false;

    let mut mesh10: Object = Object::new_from_obj(&eng, "md10", &shaders.vertex_code, &shaders.shadow_vertex_code, fragc, &uniforms, "tex10;stex10;ntex10", "", "linear", "linear", "none", "none", false);
    mesh10.scale = Vec3::newdefined(0.025, 0.025, 0.025);

    let mut mesh11: Object = Object::new_from_obj(&eng, "md11", &shaders.vertex_code, &shaders.shadow_vertex_code, fragc, &uniforms, "tex11;stex11;ntex11", "", "linear", "linear", "none", "none", false);
    mesh11.scale = Vec3::newdefined(0.025, 0.025, 0.025);
    mesh11.collision_detect = false;

    shaders = ShaderBuilder::new_skybox(&uniforms);
    shaders.new_fragment_shader();
    shaders.fragment_begin_main();
    shaders.fragment_code += "
      return vec4f(textureSample(mycube, mySampler, in.vertex.xyz).rgb+1.0, 1);
    ";
    shaders.fragment_end_main();

    let mut skybox: Object = Object::new_from_obj(&eng, "cube", &shaders.vertex_code, &shaders.shadow_vertex_code, &shaders.fragment_code, &uniforms, "", "right;left;top;bottom;front;back", "linear", "linear", "front", "back", false);
    skybox.collision_detect = false;
    skybox.scale = Vec3::newdefined(1000f32, 1000f32, 1000f32);

    shaders = ShaderBuilder::new_post_procces(&uniforms);
    shaders.new_fragment_shader();
    shaders.fragment_code += "
        fn separateh(uv: vec2f) -> vec3f{
          var tor: vec3f = vec3f(0.0, 0.0, 0.0);
          let alb = textureSample(mainMap, mySampler, uv).rgb;
          if alb.r >= 1.0 || alb.g >= 1.0 || alb.b >= 1.0 {
              tor = alb-1.0;
          }
          return tor;
        }
        fn bloom(uv: vec2f, off: f32) -> vec3f{
          let offset = 1.0 / off;
          let offsets = array<vec2f, 9>( 
            vec2f(-offset,  offset),
            vec2f( 0.0f,    offset),
            vec2f( offset,  offset),
            vec2f(-offset,  0.0f),  
            vec2f( 0.0f,    0.0f),  
            vec2f( offset,  0.0f),  
            vec2f(-offset, -offset),
            vec2f( 0.0f,   -offset),
            vec2f( offset, -offset) 
          );
          let kernel = array<f32, 9>( 
            1.0 / 16, 2.0 / 16, 1.0 / 16,
            2.0 / 16, 4.0 / 16, 2.0 / 16,
            1.0 / 16, 2.0 / 16, 1.0 / 16  
          );
          var col = vec3f(0.0, 0.0, 0.0);
          for(var i = 0; i < 9; i+=1){
            col += separateh(uv + offsets[i]) * kernel[i];
          }
          return col;
        }
    ";
    shaders.fragment_begin_main();
    shaders.fragment_code += "
      return vec4f(textureSample(mainMap, mySampler, in.uv).rgb + bloom(in.uv, 50.0), 1);
    ";
    shaders.fragment_end_main();

    let mut renquad: Object = Object::new(&eng, &vertices, &uv, &normals, 6, &shaders.vertex_code, &shaders.shadow_vertex_code, &shaders.fragment_code, &uniforms, "", "", "nearest", "nearest", "none", "none", true);
    renquad.collision_detect = false;
    let mut rd = 1.0f32;

    eng.pos.y = -20f32;

    let mut as1 = Audiosource3d::new("assets/sample.mp3", Vec3::newdefined(0f32, -4f32, 0f32), 10f32);

    eng.shadowpos = Vec3::newdefined(80f32, -142f32, -35f32);
    eng.shadoworthographic = true;
    eng.shadowfov = 50f32;
    eng.shadow_z_far = 220f32;
    eng.shadowrot = Vec2::newdefined(1.05f32, 1.05f32);
    let drawloop = move || {
      eng.speed.y = 0.1;
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
          if rd > 0.1f32{
            rd-=0.1;
          }
          eng.ren.change_render_scale(rd);
        }
        if is_key_pressed(76){
          rd+=0.1;
          eng.ren.change_render_scale(rd);
        }
        set_touch_index(0);
        if get_is_touching(){
          eng.rot.y += ((get_touch_x() as f32/eng.ren.get_canvas_size_x()as f32)*2.0f32 - 1.0f32) / 100f32;
        }
      }
      uniforms[4].vec4.x = eng.pos.x;
      uniforms[4].vec4.y = eng.pos.y;
      uniforms[4].vec4.z = eng.pos.z;

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

      skybox.draw(&mut eng, &uniforms);

      eng.begin_post("clear", "clear");

      renquad.draw(&mut eng, &uniforms);

      eng.end();
    };

    engine::render::render::drawloopexec(drawloop)
}