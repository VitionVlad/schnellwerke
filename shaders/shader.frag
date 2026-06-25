struct MeshInput {
    resolutions: vec4<f32>,
    lightinfo: vec4<f32>,
    t: mat4x4<f32>,
    r: mat4x4<f32>,
    s: mat4x4<f32>,
    addinfo: vec4<f32>,
};

struct ShadowMatricesInput {
    shadowViews: array<mat4x4<f32>, 100>,
    lightpos: array<vec4<f32>, 100>,
    lightcol: array<vec4<f32>, 100>,
};

struct DefferedMatricesInput {
    defferedMVP: array<mat4x4<f32>, 10>,
    defferedMVPInverse: array<mat4x4<f32>, 10>,
    deffpos: array<vec4<f32>, 10>,
    deffrot: array<vec4<f32>, 10>,
};

@group(0) @binding(0) var<uniform> mi: MeshInput;
@group(0) @binding(1) var<uniform> smi: ShadowMatricesInput;
@group(0) @binding(2) var<uniform> dmi: DefferedMatricesInput;

@group(0) @binding(3) var texTexture: texture_2d_array<f32>;
@group(0) @binding(4) var defferedTexture: texture_2d_array<f32>;
@group(0) @binding(5) var defferedDepthTexture: texture_depth_2d_array;
@group(0) @binding(6) var shadowTexture: texture_depth_2d_array;

@group(0) @binding(7) var imageSampler: sampler;
@group(0) @binding(8) var attachmentSampler: sampler;
@group(0) @binding(9) var shadowComparisonSampler: sampler_comparison;

const PI: f32 = 3.14159265359;
const near: f32 = 0.1;
const far: f32 = 100.0;

fn DistributionGGX(N: vec3<f32>, H: vec3<f32>, roughness: f32) -> f32 {
    let a = roughness * roughness;
    let a2 = a * a;
    let NdotH = max(dot(N, H), 0.0);
    let NdotH2 = NdotH * NdotH;
    let num = a2;
    var denom = (NdotH2 * (a2 - 1.0) + 1.0);
    denom = PI * denom * denom;
    return num / denom;
}

fn GeometrySchlickGGX(NdotV: f32, roughness: f32) -> f32 {
    let r = (roughness + 1.0);
    let k = (r * r) / 8.0;
    let num = NdotV;
    let denom = NdotV * (1.0 - k) + k;
    return num / denom;
}

fn GeometrySmith(N: vec3<f32>, V: vec3<f32>, L: vec3<f32>, roughness: f32) -> f32 {
    let NdotV = max(dot(N, V), 0.0);
    let NdotL = max(dot(N, L), 0.0);
    let ggx2 = GeometrySchlickGGX(NdotV, roughness);
    let ggx1 = GeometrySchlickGGX(NdotL, roughness);
    return ggx1 * ggx2;
}

fn fresnelSchlick(cosTheta: f32, F0: vec3<f32>) -> vec3<f32> {
    return F0 + (1.0 - F0) * pow(clamp(1.0 - cosTheta, 0.0, 1.0), 5.0);
}

fn shcalcpl(WorldPos: vec3<f32>, bias: f32, i: i32) -> f32 {
    var visibility: f32 = 0.0;
    let smv = smi.shadowViews[i] * vec4<f32>(WorldPos, 1.0);
    let proj = vec3<f32>(
        (smv.x / smv.w) * 0.5 + 0.5,
        (smv.y / smv.w) * -0.5 + 0.5,
        smv.z / smv.w
    );
    let oneOverShadowDepthTextureSize = 1.0 / mi.resolutions.z;
    for (var y: i32 = -1; y <= 1; y++) {
        for (var x: i32 = -1; x <= 1; x++) {
            let offset = vec2<f32>(f32(x), f32(y)) * oneOverShadowDepthTextureSize;
            var lv: f32 = 0.0;
            let depth = textureSampleCompare(
                shadowTexture,
                shadowComparisonSampler,
                vec2<f32>(proj.x + offset.x, proj.y + offset.y),
                i,
                proj.z - bias
            );
            if (depth > 0.0) {
                lv = 1.0;
            }
            if (!(proj.x > 0.99 || proj.x < 0.001 || proj.y > 0.99 || proj.y < 0.001 || proj.z > 1.0 || proj.z < -1.0)) {
                visibility += lv;
            }
        }
    }
    return visibility / 9.0;
}

fn shcalcpld(WorldPos: vec3<f32>, bias: f32, i: i32) -> f32 {
    var visibility: f32 = 0.0;
    let smv = smi.shadowViews[i] * vec4<f32>(WorldPos, 1.0);
    let proj = vec3<f32>(
        (smv.x / smv.w) * 0.5 + 0.5, 
        (smv.y / smv.w) * -0.5 + 0.5, 
        smv.z / smv.w
    );
    let oneOverShadowDepthTextureSize = 1.0 / mi.resolutions.z;
    for (var y: i32 = -1; y <= 1; y = y + 1) {  
        for (var x: i32 = -1; x <= 1; x = x + 1) {
            let offset = vec2<f32>(f32(x), f32(y)) * oneOverShadowDepthTextureSize;
            var lv: f32 = 0.0;  
            let depth = textureSampleCompare(
                shadowTexture,
                shadowComparisonSampler,
                vec2<f32>(proj.x + offset.x, proj.y + offset.y),
                i,
                proj.z - bias
            );
            if (depth > 0.0) {
                lv = 1.0;
            }
            visibility = visibility + lv;
        }
    }
    return visibility / 9.0;
}

fn PBR(norm: vec3<f32>, albedo: vec3<f32>, metallic: f32, roughness: f32, ao: f32, WorldPos: vec3<f32>) -> vec3<f32> {
    let N = normalize(norm);
    var V = normalize(dmi.deffpos[0].xyz - WorldPos - vec3<f32>(1.0));
    var F0 = vec3<f32>(0.04);
    F0 = mix(F0, albedo, metallic);
    var Lo = vec3<f32>(0.0);
    var scs: i32 = 0;
    let lightCount = i32(mi.lightinfo.w);
    for (var i: i32 = 0; i < lightCount; i = i + 1) {
        var L = normalize(-smi.lightpos[i].xyz);
        var H = normalize(V + L);
        var distance: f32 = 1.0;
        if (smi.lightpos[i].w == 1.0) {
            V = normalize(dmi.deffpos[0].xyz - WorldPos);
            L = normalize(smi.lightpos[i].xyz - WorldPos);
            H = normalize(V + L);
            distance = length(smi.lightpos[i].xyz - WorldPos);
        }
        let attenuation = 1.0 / (distance * distance);
        let radiance = smi.lightcol[i].xyz * attenuation;
        let NDF = DistributionGGX(N, H, roughness);
        let G = GeometrySmith(N, V, L, roughness);
        let F = fresnelSchlick(max(dot(H, V), 0.0), F0);
        let kS = F;
        var kD = vec3<f32>(1.0) - kS;
        kD = kD * (1.0 - metallic);
        let numerator = NDF * G * F;
        let denominator = 4.0 * max(dot(N, V), 0.0) * max(dot(N, L), 0.0) + 0.0001;
        let specular = numerator / denominator;
        let NdotL = max(dot(N, L), 0.0);
        var shadow: f32 = 1.0;
        if (smi.lightcol[i].w != 0.0) {
            if (smi.lightpos[i].w == 1.0) {
                shadow = max(shcalcpl(WorldPos, 0.0, scs), 0.001);
            } else {
                shadow = max(shcalcpld(WorldPos, 0.0, scs), 0.001);
            }
            scs = scs + 1;
        }
        Lo = Lo + (kD * albedo / PI + specular) * radiance * NdotL * max(shadow, 0.001);
    }
    let ambient = vec3<f32>(0.001) * albedo * ao;
    var color = ambient + Lo;
    color = color / (color + vec3<f32>(1.0));
    color = pow(color, vec3<f32>(1.0 / 2.2));
    return color;
}

fn WorldPosFromDepth(depth: f32, uv: vec2<f32>, inversemat: mat4x4<f32>) -> vec3<f32> {
    let clipSpacePosition = vec4<f32>(uv.x * 2.0 - 1.0, (1.0 - uv.y) * 2.0 - 1.0, depth, 1.0);
    var viewSpacePosition = inversemat * clipSpacePosition;
    viewSpacePosition = viewSpacePosition / viewSpacePosition.w;
    return viewSpacePosition.xyz;
}

fn LinearizeDepth(d: f32) -> f32 {
    return near * far / (far + d * (far - near));
}

struct FragmentInput {
    @location(0) fuv: vec2<f32>,
};

@fragment
fn main(in: FragmentInput) -> @location(0) vec4<f32> {
    let uv = vec2f(in.fuv.x, 1.0 - in.fuv.y);
    var d = textureSample(defferedDepthTexture, attachmentSampler, uv, 0);
    //d = (d + 1.0) / 2.0;
    // let d2 = LinearizeDepth(d);

    let albedo = textureSample(defferedTexture, attachmentSampler, uv, 0).rgb;

    let rma = textureSample(defferedTexture, attachmentSampler, uv, 1).rgb;
    let normal = textureSample(defferedTexture, attachmentSampler, uv, 2).rgb;
    let wrldpos = WorldPosFromDepth(d, uv, dmi.defferedMVPInverse[0]);

    var op = vec4<f32>(PBR(normal, albedo, rma.x, rma.y, 1.0, wrldpos), 1.0);

    let dst = smoothstep(0.0, 30.0, distance(mi.addinfo.yz, wrldpos.xz));

    op = mix(vec4<f32>(smi.lightcol[0].xyz, 1.0), op, 1.0 - max(min(dst, 1.0), 0.0));

    if (mi.addinfo.w < 1.0) {
        op = vec4<f32>(op.rg * max(min(mi.addinfo.w, 1.0), 0.25), op.ba);
    } else if (mi.addinfo.w >= 5.0) {
        op = vec4<f32>(mix(op.rgb, vec3<f32>(0.0), max(min(mi.addinfo.w - 10.0, 1.0), 0.0)), op.a);
    } else {
        op = vec4<f32>(mix(op.rgb, vec3<f32>(1.0), max(min(mi.addinfo.w - 1.0, 1.0), 0.0)), op.a);
    }

    // op = mix(op, vec4<f32>(0.5, 0.5, 1.0, 1.0), max(min(mi.addinfo.w, 0.5), 0.0));
    // op = vec4<f32>(abs(normal), 1.0);
    // if (rma.y <= 0.1 && rma.x <= 0.1) { op = vec4<f32>(albedo, 1.0); }
    // let mxpw = smoothstep(10.0, 20.0, distance(mi.addinfo.yz, wrldpos.xz));
    // op = mix(op, vec4<f32>(smi.lightcol[0].xyz, 1.0), mxpw);
    // op = mix(op, vec4<f32>(0.0, 0.0, 0.0, 1.0), mi.addinfo.x);

    return op*1.75;

    // return vec4<f32>(textureSample(defferedTexture, attachmentSampler, uv, 0).rgb, 1.0);
    // return vec4<f32>(vec3<f32>(textureSample(shadowTexture, attachmentSampler, uv, 0)), 1.0);
}
