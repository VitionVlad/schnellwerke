struct DefferedMatricesInput {
    defferedView: mat4x4<f32>
}
@group(0) @binding(0) var<uniform> dmi: DefferedMatricesInput;

struct MeshInput {
    resolutions: vec4f,
    lightinfo: vec4f,
    model: mat4x4<f32>,
    addinfo: vec4f
}
@group(0) @binding(1) var<uniform> mi: MeshInput;

struct VertexOutput {
    @builtin(position) position: vec4f,
    @location(0) fuv: vec2f,
    @location(1) fpos: vec4f,
    @location(2) ftg: vec3f,
    @location(3) fctg: vec3f,
    @location(4) fnormal: vec3f
}

@vertex
fn main(
    @location(0) pos: vec3f,
    @location(1) uv: vec2f,
    @location(2) normal: vec3f,
    @location(3) tg: vec3f,
    @location(4) ctg: vec3f
) -> VertexOutput {
    var output: VertexOutput;
    output.fuv = uv;
    output.fpos = mi.model * vec4f(pos.x, pos.y, pos.z, 1.0);
    output.ftg = tg;
    output.fctg = ctg;
    output.fnormal = normal;
    output.position = dmi.defferedView * mi.model * vec4f(pos.x, pos.y, pos.z, 1.0);
    return output;
}