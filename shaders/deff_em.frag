struct DefferedMatricesInput {
    defferedViews: mat4x4<f32>
}
@group(0) @binding(0) var<uniform> dmi: DefferedMatricesInput;

struct MeshInput {
    resolutions: vec4f,
    lightinfo: vec4f,
    model: mat4x4<f32>,
    addinfo: vec4f
}
@group(0) @binding(1) var<uniform> mi: MeshInput;

struct FragmentOutput {
    @location(0) outColor: vec4f,
    @location(1) outMaterial: vec4f,
    @location(2) outNormal: vec4f,
    @location(3) outPos: vec4f
}

@fragment
fn main(
    @location(0) uv: vec2f,
    @location(1) pos: vec4f,
    @location(2) ftg: vec3f,
    @location(3) fctg: vec3f,
    @location(4) fnormal: vec3f
) -> FragmentOutput {
    var output: FragmentOutput;
    output.outColor = vec4f(1.0, 1.0, 0.5, 1.0);
    output.outMaterial = vec4f(0.1, 0.1, 0.1, 1.0);
    output.outNormal = vec4f(fnormal.x, -fnormal.y, fnormal.z, 1.0);
    output.outPos = pos;
    return output;
}