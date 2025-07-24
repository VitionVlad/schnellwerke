struct MeshInput {
    resolutions: vec4<f32>,
    lightinfo: vec4<f32>,
    model: mat4x4<f32>,
    addinfo: vec4<f32>,
}

struct ShadowMatricesInput {
    shadowViews: array<mat4x4<f32>, 100>,
    lightpos: array<vec4<f32>, 100>,
    lightcol: array<vec4<f32>, 100>,
}

struct DefferedMatricesInput {
    defferedViews: array<mat4x4<f32>, 10>,
    lightpos: array<vec4<f32>, 10>,
    lightcol: array<vec4<f32>, 10>,
}

@group(0) @binding(0) var<uniform> mi: MeshInput;
@group(0) @binding(1) var<uniform> smi: ShadowMatricesInput;
@group(0) @binding(2) var<uniform> dmi: DefferedMatricesInput;
@group(0) @binding(3) var texSampler: texture_2d_array<f32>;
@group(0) @binding(4) var defferedSampler: texture_2d_array<f32>;
@group(0) @binding(5) var defferedDepthSampler: texture_depth_2d_array;
@group(0) @binding(6) var shadowSampler: texture_depth_2d_array;
@group(0) @binding(7) var colorSampler: sampler;
@group(0) @binding(8) var shadowComparisonSampler: sampler_comparison;

@fragment
fn main(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    var outColor: vec4<f32>;
    if (mi.addinfo.z > 0.0) {
        outColor = vec4<f32>(
            1.0 - textureSample(texSampler, colorSampler, vec2<f32>(uv.x, -uv.y), 0).rgb,
            1.0
        );
    } else {
        outColor = vec4<f32>(
            textureSample(texSampler, colorSampler, vec2<f32>(uv.x, -uv.y), 0).rgb,
            1.0
        );
    }
    return outColor;
}