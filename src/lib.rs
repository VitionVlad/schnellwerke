use engine::engine::Engine;
use engine::object::Object;
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
  let mut eng: Engine = Engine::new("render");
  let res: Objreader = Objreader::new("cube");

  let vertex_code = "
  struct uniforms {
    eng: vec4f,
    mvp: mat4x4<f32>
  }
  @group(0) @binding(0) var<uniform> ubo: uniforms;
  struct OUT{
    @builtin(position) position: vec4f,
    @location(0) uv: vec2f,
    @location(1) smv: vec4f,
    @location(2) norm: vec3f,
    @location(3) tangent: vec3f,
    @location(4) bitangent: vec3f,
  }
  @vertex
  fn vertexMain(@location(0) pos: vec3f, @location(1) uv: vec2f, @location(2) n: vec3f, @location(3) t: vec3f) -> OUT {
    var out: OUT;
    out.position = ubo.mvp * vec4f(pos, 1.0);
    out.uv = vec2f(uv.x, 1.0-uv.y);
    out.norm = n;
    out.tangent = t;
    out.bitangent = cross(n, t);
    return out;
  }
  ";
  let fragment_code = "
  @group(0) @binding(1) var mySampler: sampler;

  @group(0) @binding(2) var myTexture: texture_2d_array<f32>;

  @group(0) @binding(3) var shadowMap: texture_depth_2d_array;

  @group(0) @binding(4) var mycube: texture_cube<f32>;

  @group(0) @binding(5) var mainMap: texture_2d_array<f32>;

  @group(0) @binding(6) var shadowSampler: sampler_comparison;

  struct OUT{
    @builtin(position) position: vec4f,
    @location(0) uv: vec2f,
    @location(1) smv: vec4f,
    @location(2) norm: vec3f,
    @location(3) tangent: vec3f,
    @location(4) bitangent: vec3f,
  }

  @fragment
  fn fragmentMain(in: OUT) -> @location(0) vec4f {
    return textureSample(myTexture, mySampler, in.uv, 0).rgba;
  }
  ";


  let mesh1: Object = Object::new(&eng, res.arr, vertex_code, fragment_code, 20, "tex", "", false);

  let postvertex_code = "
  struct uniforms {
    eng: vec4f,
    mvp: mat4x4<f32>
  }
  struct OUT{
    @builtin(position) position: vec4f,
    @location(0) uv: vec2f,
  }
      
  @vertex
  fn vertexMain(@location(0) pos: vec3f, @location(1) uv: vec2f, @location(2) n: vec3f, @location(3) t: vec3f) -> OUT {
    var out: OUT;
    out.position = vec4f(pos.xyz, 1);
    out.uv = uv;
    return out;
    }
  ";

  let postfragment_code = "
  @group(0) @binding(1) var mySampler: sampler;

  @group(0) @binding(2) var myTexture: texture_2d_array<f32>;

  @group(0) @binding(3) var shadowMap: texture_depth_2d_array;

  @group(0) @binding(4) var mainMap: texture_2d_array<f32>;

  @group(0) @binding(5) var mainDepthMap: texture_depth_2d_array;

  struct OUT{
    @location(0) uv: vec2f,
  }
        
  @fragment
  fn fragmentMain(in: OUT) -> @location(0) vec4f {
    return vec4f(textureSample(mainMap, mySampler, in.uv, 0).rgb, 1);
  }
  ";

  let plane: [f32; 48] = [
    -1.0, -1.0, 1.0,
    -1.0, 1.0, 1.0, 
    1.0, 1.0, 1.0,

    -1.0, -1.0, 1.0,
    1.0, 1.0, 1.0,
    1.0, -1.0, 1.0, 

    0.0, 1.0,
    0.0, 0.0,
    1.0, 0.0,

    0.0, 1.0,
    1.0, 0.0,
    1.0, 1.0,

    0.0, 0.0, 1.0,
    0.0, 0.0, 1.0,
    0.0, 0.0, 1.0,

    0.0, 0.0, 1.0,
    0.0, 0.0, 1.0,
    0.0, 0.0, 1.0
  ];

  let mesh2: Object = Object::new(&eng, plane.to_vec(), postvertex_code, postfragment_code, 20, "tex", "", true);

  eng.cameras[0].pos = Vec3::newdefined(-2f32, 0f32, 4f32);
  eng.cameras[0].rot = Vec3::newdefined(0f32, 0.5f32, 0f32);
  eng.object_to_draw = vec![mesh1, mesh2];
  eng.start();
}