use engine::engine::Engine;
use js_sys::Float32Array;
use wasm_bindgen::prelude::*;
use engine::render::render::*;
use engine::render::mesh::Mesh;
use engine::resourceloader::resourceloader::Objreader;
use engine::math::vec3::Vec3;
use engine::math::vec2::Vec2;
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

  let mut vcnt: u32 = 0;
  let jst = js_sys::Float32Array::new_with_length((res.size*3) as u32);
  for i in (0..res.vert.length()).step_by(9){
      let v0 = Vec3::newdefined(res.vert.get_index(i), res.vert.get_index(i+1), res.vert.get_index(i+2));
      let v1 = Vec3::newdefined(res.vert.get_index(i+3), res.vert.get_index(i+4), res.vert.get_index(i+5));
      let v2 = Vec3::newdefined(res.vert.get_index(i+6), res.vert.get_index(i+7), res.vert.get_index(i+8));
      let uv0 = Vec2::newdefined(res.uv.get_index(vcnt), res.uv.get_index(vcnt+1)+1.0);
      let uv1 = Vec2::newdefined(res.uv.get_index(vcnt+2), res.uv.get_index(vcnt+3)+1.0);
      let uv2 = Vec2::newdefined(res.uv.get_index(vcnt+4), res.uv.get_index(vcnt+5)+1.0);
      let deltapos1 = Vec3::newdefined(v1.x-v0.x, v1.y-v0.y, v1.z-v0.z);
      let deltapos2 = Vec3::newdefined(v2.x-v0.x, v2.y-v0.y, v2.z-v0.z);
      let delta_uv1 = Vec2::newdefined(uv1.x-uv0.x, uv1.y-uv0.y);
      let delta_uv2 = Vec2::newdefined(uv2.x-uv0.x, uv2.y-uv0.y);
      let r = 1.0f32 / (delta_uv1.x * delta_uv2.y - delta_uv1.y * delta_uv2.x);
      jst.set_index(i, (deltapos1.x * delta_uv2.y - deltapos2.x * delta_uv1.y)*r);
      jst.set_index(i+1, (deltapos1.y * delta_uv2.y - deltapos2.y * delta_uv1.y)*r);
      jst.set_index(i+2, (deltapos1.z * delta_uv2.y - deltapos2.z * delta_uv1.y)*r);
      jst.set_index(i+3, (deltapos1.x * delta_uv2.y - deltapos2.x * delta_uv1.y)*r);
      jst.set_index(i+4, (deltapos1.y * delta_uv2.y - deltapos2.y * delta_uv1.y)*r);
      jst.set_index(i+5, (deltapos1.z * delta_uv2.y - deltapos2.z * delta_uv1.y)*r);
      jst.set_index(i+6, (deltapos1.x * delta_uv2.y - deltapos2.x * delta_uv1.y)*r);
      jst.set_index(i+7, (deltapos1.y * delta_uv2.y - deltapos2.y * delta_uv1.y)*r);
      jst.set_index(i+8, (deltapos1.z * delta_uv2.y - deltapos2.z * delta_uv1.y)*r);
      vcnt+=6
  }

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
  let svertex_code = "
  struct uniforms {
    eng: vec4f,
    mvp: mat4x4<f32>
  }
  @group(0) @binding(0) var<uniform> ubo: uniforms;
  @vertex
  fn vertexMain(@location(0) pos: vec3f) -> @builtin(position) vec4f {
    return ubo.mvp * vec4f(pos, 1.0);
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


  let mesh1: Mesh = Mesh::create(&eng.render, &res.vert, &res.uv, &res.norm, &jst, res.size, vertex_code, svertex_code, fragment_code, 20, "tex", "", "linear", "linear", "none", "none", "repeat", false);
  push_mesh(&mesh1.jsmesh);

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

  let vertices: [f32; 18] = [
    -1.0, -1.0, 1.0,
    -1.0, 1.0, 1.0, 
    1.0, 1.0, 1.0,

    -1.0, -1.0, 1.0,
    1.0, 1.0, 1.0,
    1.0, -1.0, 1.0, 
  ];
  let vuv: [f32; 12] = [
      0.0, 1.0,
      0.0, 0.0,
      1.0, 0.0,

      0.0, 1.0,
      1.0, 0.0,
      1.0, 1.0,
  ];
  let normals: [f32; 18] = [
      0.0, 0.0, 1.0,
      0.0, 0.0, 1.0,
      0.0, 0.0, 1.0,

      0.0, 0.0, 1.0,
      0.0, 0.0, 1.0,
      0.0, 0.0, 1.0
  ];

  let v = Float32Array::new_with_length(18);
  v.copy_from(&vertices);

  let uv = Float32Array::new_with_length(12);
  uv.copy_from(&vuv);

  let vn = Float32Array::new_with_length(18);
  vn.copy_from(&normals);

  let tn = Float32Array::new_with_length(18);

  let mesh2: Mesh = Mesh::create(&eng.render, &v, &uv, &vn, &tn, 6, postvertex_code, svertex_code, postfragment_code, 20, "tex", "", "linear", "linear", "none", "none", "repeat", true);
  push_mesh(&mesh2.jsmesh);

  eng.cameras[0].pos = Vec3::newdefined(-2f32, 0f32, 4f32);
  eng.cameras[0].rot = Vec3::newdefined(0f32, 0.5f32, 0f32);
  eng.mesh_to_draw = vec![mesh1, mesh2];
  eng.start();
}