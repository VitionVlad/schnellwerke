struct DefferedMatricesInput {
    defferedViews: mat4x4<f32>,
}

struct MeshInput {
    resolutions: vec4<f32>,
    lightinfo: vec4<f32>,
    model: mat4x4<f32>,
    addinfo: vec4<f32>,
}

@group(0) @binding(0) var<uniform> dmi: DefferedMatricesInput;
@group(0) @binding(1) var<uniform> mi: MeshInput;
@group(0) @binding(2) var texSampler: texture_2d_array<f32>;
@group(0) @binding(3) var colorSampler: sampler;

struct FragmentInput {
    @location(0) uv: vec2<f32>,
    @location(1) pos: vec4<f32>,
    @location(2) ftg: vec3<f32>,
    @location(3) fctg: vec3<f32>,
    @location(4) fnormal: vec3<f32>,
}

struct FragmentOutput {
    @location(0) outColor: vec4<f32>,
    @location(1) outMaterial: vec4<f32>,
    @location(2) outNormal: vec4<f32>,
    @location(3) outPos: vec4<f32>,
}

@fragment
fn main(input: FragmentInput) -> FragmentOutput {
    var output: FragmentOutput;
    
    output.outColor = vec4<f32>(textureSample(texSampler, colorSampler, input.uv, 0).rgb, 1.0);
    output.outMaterial.r = textureSample(texSampler, colorSampler, input.uv, 1).r;
    output.outMaterial.g = textureSample(texSampler, colorSampler, input.uv, 2).r;
    output.outMaterial.b = 1.0;
    let TBN = mat3x3<f32>(input.ftg, input.fctg, input.fnormal);
    let n = textureSample(texSampler, colorSampler, input.uv, 3).rgb * 2.0 - 1.0;
    output.outNormal = vec4<f32>(TBN * n, 1.0);
    output.outPos = input.pos;
    
    return output;
}