use engine::engine::Engine;
use engine::math::uniformstruct::{createmvpmat, createsmvpmat, createvec4, Uniformstruct};
use engine::math::vec4::Vec4;
use engine::object::Object;
use engine::input::keyboard::is_key_pressed;
use engine::input::mouse::{get_mouse_x, get_mouse_y};
use engine::input::touch::*;
use wasm_bindgen::prelude::*;
use crate::engine::audiosource3d::Audiosource3d;
use crate::engine::math::vec3::Vec3;
use crate::engine::math::vec2::Vec2;
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
        -1.0, -1.0, -0.5, 1.0,
        -1.0, 1.0, -0.5, 1.0,
        1.0, 1.0, -0.5, 1.0,

        -1.0, -1.0, -0.8, 1.0,
        1.0, 1.0, -0.8, 1.0,
        1.0, -1.0, -0.8, 1.0
    ];

    let uv: [f32; 12] = [
        0.0, 1.0,
        0.0, 0.0,
        1.0, 0.0,

        0.0, 1.0,
        1.0, 0.0,
        1.0, 1.0,
    ];

    let normals: [f32; 18] = [
        -1.0, -1.0, -1.0,
        -1.0, 1.0, -1.0,
        1.0, 1.0, -1.0,

        -1.0, -1.0, -1.0,
        -1.0, 1.0, -1.0,
        1.0, 1.0, -1.0
    ];

    let vertc: &str = "
    struct OurStruct {
      mvp: mat4x4<f32>,
      smvp: mat4x4<f32>,
      lightpos: vec4f,
      lightcolor: vec4f,
      playerpos: vec4f,
    };

    @group(0) @binding(0) var<uniform> ourStruct: OurStruct;

    struct OUT{
      @builtin(position) position: vec4f,
      @location(0) uv: vec2f,
      @location(1) norm: vec3f,
      @location(2) t: vec3f,
      @location(3) smv: vec4f,
    }

    @vertex
    fn vertexMain(@location(0) pos: vec4f, @location(1) uv: vec2f, @location(2) n: vec3f, @location(3) t: vec3f) -> OUT {
      var out: OUT;
      out.position = ourStruct.mvp * vec4f(pos.xyz, 1);
      out.uv = vec2f(uv.x, 1.0-uv.y);
      out.norm = n;
      out.t = t;
      out.smv = ourStruct.smvp * vec4f(pos.xyz, 1);
      return out;
    }";

    let vertsk: &str = "
    struct OurStruct {
      mvp: mat4x4<f32>,
      smvp: mat4x4<f32>,
      lightpos: vec4f,
      lightcolor: vec4f,
      playerpos: vec4f,
    };

    @group(0) @binding(0) var<uniform> ourStruct: OurStruct;

    struct OUT{
      @builtin(position) position: vec4f,
      @location(0) p: vec3f,
    }

    @vertex
    fn vertexMain(@location(0) pos: vec4f, @location(1) uv: vec2f, @location(2) n: vec3f, @location(3) t: vec3f) -> OUT {
      var out: OUT;
      out.position = ourStruct.mvp * vec4f(pos.xyz, 1);
      out.position.z = out.position.w;
      out.p = pos.xyz;
      return out;
    }";

    let pvertc: &str = "
    struct OurStruct {
      mvp: mat4x4<f32>,
      smvp: mat4x4<f32>,
      lightpos: vec4f,
      lightcolor: vec4f,
      playerpos: vec4f,
    };

    @group(0) @binding(0) var<uniform> ourStruct: OurStruct;

    struct OUT{
      @builtin(position) position: vec4f,
      @location(0) uv: vec2f,
      @location(1) norm: vec3f,
      @location(2) t: vec3f,
    }

    @vertex
    fn vertexMain(@location(0) pos: vec4f, @location(1) uv: vec2f, @location(2) n: vec3f, @location(3) t: vec3f) -> OUT {
      var out: OUT;
      out.position = vec4f(pos.xy, 0.5, 1);
      out.uv = uv;
      out.norm = n;
      out.t = t;
      return out;
    }";

    let vertsc: &str = "
    struct OurStruct {
      mvp: mat4x4<f32>,
      smvp: mat4x4<f32>,
      lightpos: vec4f,
      lightcolor: vec4f,
      playerpos: vec4f,
    };

    @group(0) @binding(0) var<uniform> in: OurStruct;

    @vertex
    fn vertexMain(@location(0) pos: vec4f) -> @builtin(position) vec4f {
      return in.smvp * vec4f(pos.xyz, 1);
    }
    ";

    let fragc: &str = "
    struct UBO {
      mvp: mat4x4<f32>,
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
      @builtin(position) position: vec4f,
      @location(0) uv: vec2f,
      @location(1) norm: vec3f,
      @location(2) t: vec3f,
      @location(3) smv: vec4f,
    }

    fn shadowmapping(smv: vec4f) -> f32{
      let proj = vec3f((smv.x / smv.w)*0.5+0.5, (smv.y / smv.w)*-0.5+0.5, smv.z / smv.w);
      return textureSampleCompare(shadowMap, shadowSampler, proj.xy, proj.z-0.001);
    }

    @fragment
    fn fragmentMain(in: OUT) -> @location(0) vec4f {
      return vec4f(textureSample(myTexture, mySampler, in.uv, 0).rgb-(1.0-shadowmapping(in.smv))/2, 1);
    }";

    let fragsk: &str = "
    struct UBO {
      mvp: mat4x4<f32>,
      smvp: mat4x4<f32>,
      lightpos: vec4f,
      lightcolor: vec4f,
      playerpos: vec4f,
    };

    @group(0) @binding(0) var<uniform> ubo: UBO;

    @group(0) @binding(1) var mySampler: sampler;

    @group(0) @binding(4) var mycube: texture_cube<f32>;

    struct OUT{
      @builtin(position) position: vec4f,
      @location(0) p: vec3f,
    }

    @fragment
    fn fragmentMain(in: OUT) -> @location(0) vec4f {
      return vec4f(textureSample(mycube, mySampler, in.p).rgb, 1);
    }";

    let pfragc: &str = "
    struct UBO {
      mvp: mat4x4<f32>,
      smvp: mat4x4<f32>,
      lightpos: vec4f,
      lightcolor: vec4f,
      playerpos: vec4f,
    };

    @group(0) @binding(0) var<uniform> ubo: UBO;

    @group(0) @binding(1) var mySampler: sampler;

    @group(0) @binding(2) var myTexture: texture_2d_array<f32>;

    @group(0) @binding(3) var shadowMap: texture_depth_2d;

    @group(0) @binding(4) var mainMap: texture_2d<f32>;

    @group(0) @binding(5) var mainDepthMap: texture_depth_2d;

    struct OUT{
      @builtin(position) position: vec4f,
      @location(0) uv: vec2f,
      @location(1) norm: vec3f,
      @location(2) t: vec3f,
    }

    @fragment
    fn fragmentMain(in: OUT) -> @location(0) vec4f {
      return vec4f(textureSample(mainMap, mySampler, in.uv).rgb, 1);
    }";

    let mut uniforms: Vec<Uniformstruct> = vec![];
    uniforms.push(createmvpmat());
    uniforms.push(createsmvpmat());
    uniforms.push(createvec4(Vec4::new()));
    uniforms.push(createvec4(Vec4::new()));
    uniforms.push(createvec4(Vec4::new()));



    let mut mesh: Object = Object::new_from_obj(&eng, "md1", vertc, vertsc, fragc, &uniforms, "tex", "", "linear", "linear", "none", "none", false);
    mesh.scale = Vec3::newdefined(0.025, 0.025, 0.025);

    let mut mesh2: Object = Object::new_from_obj(&eng, "md2", vertc, vertsc, fragc, &uniforms, "tex2", "", "linear", "linear", "none", "none", false);
    mesh2.scale = Vec3::newdefined(0.025, 0.025, 0.025);

    let mut mesh3: Object = Object::new_from_obj(&eng, "md3", vertc, vertsc, fragc, &uniforms, "tex3", "", "linear", "linear", "none", "none", false);
    mesh3.scale = Vec3::newdefined(0.025, 0.025, 0.025);
    mesh3.collision_detect = false;

    let mut mesh4: Object = Object::new_from_obj(&eng, "md4", vertc, vertsc, fragc, &uniforms, "tex4", "", "linear", "linear", "none", "none", false);
    mesh4.scale = Vec3::newdefined(0.025, 0.025, 0.025);
    mesh4.collision_detect = false;

    let mut mesh5: Object = Object::new_from_obj(&eng, "md5", vertc, vertsc, fragc, &uniforms, "tex5", "", "linear", "linear", "none", "none", false);
    mesh5.scale = Vec3::newdefined(0.025, 0.025, 0.025);

    let mut mesh6: Object = Object::new_from_obj(&eng, "md6", vertc, vertsc, fragc, &uniforms, "tex6", "", "linear", "linear", "none", "none", false);
    mesh6.scale = Vec3::newdefined(0.025, 0.025, 0.025);
    mesh6.collision_detect = false;

    let mut mesh7: Object = Object::new_from_obj(&eng, "md7", vertc, vertsc, fragc, &uniforms, "tex7", "", "linear", "linear", "none", "none", false);
    mesh7.scale = Vec3::newdefined(0.025, 0.025, 0.025);
    mesh7.collision_detect = false;

    let mut mesh8: Object = Object::new_from_obj(&eng, "md8", vertc, vertsc, fragc, &uniforms, "tex8", "", "linear", "linear", "none", "none", false);
    mesh8.scale = Vec3::newdefined(0.025, 0.025, 0.025);

    let mut mesh9: Object = Object::new_from_obj(&eng, "md9", vertc, vertsc, fragc, &uniforms, "tex9", "", "linear", "linear", "none", "none", false);
    mesh9.scale = Vec3::newdefined(0.025, 0.025, 0.025);
    mesh9.collision_detect = false;

    let mut mesh10: Object = Object::new_from_obj(&eng, "md10", vertc, vertsc, fragc, &uniforms, "tex10", "", "linear", "linear", "none", "none", false);
    mesh10.scale = Vec3::newdefined(0.025, 0.025, 0.025);

    let mut mesh11: Object = Object::new_from_obj(&eng, "md11", vertc, vertsc, fragc, &uniforms, "tex11", "", "linear", "linear", "none", "none", false);
    mesh11.scale = Vec3::newdefined(0.025, 0.025, 0.025);
    mesh11.collision_detect = false;



    let mut skybox: Object = Object::new_from_obj(&eng, "cube", vertsk, vertsc, fragsk, &uniforms, "", "right;left;top;bottom;front;back", "linear", "linear", "front", "back", false);
    skybox.collision_detect = false;
    skybox.scale = Vec3::newdefined(1000f32, 1000f32, 1000f32);

    let mut renquad: Object = Object::new(&eng, &vertices, &uv, &normals, 6, pvertc, vertsc, pfragc, &uniforms, "", "", "nearest", "nearest", "none", "none", true);
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