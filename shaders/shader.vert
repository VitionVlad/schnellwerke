struct MeshInput {
    resolutions: vec4f,
    lightinfo: vec4f,
    model: mat4x4<f32>,
    addinfo: vec4f
}
@group(0) @binding(0) var<uniform> mi: MeshInput;

struct ShadowMatricesInput {
    shadowViews: array<mat4x4<f32>, 100>,
    lightpos: array<vec4f, 100>,
    lightcol: array<vec4f, 100>
}
@group(0) @binding(1) var<uniform> smi: ShadowMatricesInput;

struct DefferedMatricesInput {
    defferedViews: array<mat4x4<f32>, 10>,
    campos: array<vec4f, 10>,
    camrot: array<vec4f, 10>
}
@group(0) @binding(2) var<uniform> dmi: DefferedMatricesInput;

struct OUT{
  @builtin(position) position: vec4f,
  @location(0) fuv: vec2f,
}

@vertex
fn main(
    @location(0) pos: vec3f,
    @location(1) uv: vec2f,
    @location(2) normal: vec3f,
    @location(3) tg: vec3f,
    @location(4) ctg: vec3f
) -> OUT {
    var ot: OUT;
    ot.fuv = uv;

    let fpos: vec4f = mi.model * vec4f(pos, 1.0);
    ot.position = vec4f(
        (fpos.x / mi.resolutions.x) * 2.0 - 1.0,
        -((fpos.y / mi.resolutions.y) * 2.0 - 1.0),
        fpos.z,
        fpos.w
    );
    return ot;
}