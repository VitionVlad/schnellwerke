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
    deffpos: array<vec4<f32>, 10>,
    deffrot: array<vec4<f32>, 10>,
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

const PI: f32 = 3.14159265359;

fn DistributionGGX(N: vec3<f32>, H: vec3<f32>, roughness: f32) -> f32 {
    let a = roughness * roughness;
    let a2 = a * a;
    let NdotH = max(dot(N, H), 0.0);
    let NdotH2 = NdotH * NdotH;
    let num = a2;
    let denom = (NdotH2 * (a2 - 1.0) + 1.0);
    return num / (PI * denom * denom);
}

fn GeometrySchlickGGX(NdotV: f32, roughness: f32) -> f32 {
    let r = roughness + 1.0;
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
    return F0 + (vec3<f32>(1.0) - F0) * pow(clamp(1.0 - cosTheta, 0.0, 1.0), 5.0);
}

fn shcalc(WorldPos: vec3<f32>, bias: f32) -> f32 {
    var visibility: f32 = 0.0;
    for (var i: i32 = 0; i < i32(mi.lightinfo.x); i++) {
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
                    shadowSampler,
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
    }
    return visibility / 9.0;
}

fn PBR(norm: vec3<f32>, albedo: vec3<f32>, shadow: f32, metallic: f32, roughness: f32, ao: f32, WorldPos: vec3<f32>) -> vec3<f32> {
    let N = normalize(norm);
    let V = normalize(dmi.deffpos[0].xyz - WorldPos);
    var F0 = vec3<f32>(0.04);
    F0 = mix(F0, albedo, metallic);
    var Lo = vec3<f32>(0.0);
    for (var i: i32 = 0; i < i32(mi.lightinfo.x); i++) {
        var L = smi.lightpos[i].xyz;
        var H = normalize(V + L);
        if (smi.lightpos[i].w != 0.0) {
            L = normalize(smi.lightpos[i].xyz - WorldPos);
            H = normalize(V + L);
        }
        let distance = length(smi.lightpos[i].xyz - WorldPos);
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
        Lo += (kD * albedo / PI + specular) * radiance * NdotL;
    }
    let ambient = vec3<f32>(0.0025) * albedo * ao;
    var color = ambient + shadow * Lo;
    color = color / (color + vec3<f32>(1.0));
    color = pow(color, vec3<f32>(1.0 / 2.2));
    return color;
}

fn getCameraBasis(eulerAngles: vec3<f32>, forward: ptr<function, vec3<f32>>, right: ptr<function, vec3<f32>>, up: ptr<function, vec3<f32>>) {
    let pitch = -eulerAngles.x;
    let yaw = -eulerAngles.y;
    let roll = eulerAngles.z;
    *forward = normalize(vec3<f32>(
        cos(pitch) * sin(yaw),
        sin(pitch),
        cos(pitch) * cos(yaw)
    ));
    *right = normalize(vec3<f32>(
        sin(yaw - 1.5708),
        0.0,
        cos(yaw - 1.5708)
    ));
    *up = normalize(cross(*right, *forward));
}

fn nightSkyFog(uv: vec2<f32>, cameraPos: vec3<f32>, cameraEuler: vec3<f32>, time: f32, rng: bool) -> vec3<f32> {
    var forward: vec3<f32>;
    var right: vec3<f32>;
    var up: vec3<f32>;
    getCameraBasis(cameraEuler, &forward, &right, &up);
    let ndc = uv * 2.0 - 1.0;
    let fovScale = 1.0;
    let rayDir = normalize(
        forward +
        ndc.x * fovScale * right +
        ndc.y * fovScale * up
    );
    let samplePos = cameraPos + rayDir * 20.0;
    let fogDriftSpeed = -20.2;
    let drift = (cameraPos.z + time * fogDriftSpeed) * 0.05;
    var noise = sin(dot(samplePos.xz, vec2<f32>(0.05, 0.05)) + drift);
    noise = noise * 0.5 + 0.5;
    let heightFog = smoothstep(50.0, 0.0, samplePos.y);
    let distFog = smoothstep(5.0, 30.0, length(samplePos - cameraPos));
    let fogAmount = noise * heightFog * distFog * 0.5;
    let colmx1 = vec3<f32>(0.002, 0.002, 0.005);
    let colmx2 = vec3<f32>(0.005, 0.005, 0.01);
    var fogColor = mix(colmx1, colmx2, noise);

    let tGround = -(cameraPos.y) / rayDir.y;
    let hitGround = (rayDir.y < -0.001) && (tGround > 0.0);
    if (hitGround && rng) {
        let groundPos = vec3<f32>(0.0, cameraPos.y, cameraPos.z) + rayDir * tGround;
        let groundDist = length(groundPos - vec3<f32>(0.0, cameraPos.y, cameraPos.z));
        if (groundDist <= 15.0) {
            var groundUV = groundPos.xz * 0.2;
            groundUV.y -= time * 2.0;
            var groundPattern = cos(groundUV.y) * 0.5 + 0.5;
            groundPattern = pow(groundPattern, 3.0);
            let groundColor = mix(
                vec3<f32>(0.01, 0.03, 0.01),
                vec3<f32>(0.035, 0.025, 0.02),
                min(max(groundPattern, 0.0), 1.0)
            );
            let groundFogFactor = smoothstep(10.0, 0.0, groundDist);
            fogColor = mix(fogColor, groundColor, groundFogFactor);
        }
    }

    return fogColor * fogAmount;
}

@fragment
fn main(@location(0) fuv: vec2<f32>) -> @location(0) vec4<f32> {
    var uv = vec2f(fuv.x, -fuv.y);
    let time = mi.addinfo.y;
    let vibration = vec2<f32>(0.0, cos(time * 15.0)) * 0.0015;
    uv += vibration;

    let albedo = pow(textureSample(defferedSampler, colorSampler, uv, 0).rgb, vec3<f32>(2.2));
    let rma = textureSample(defferedSampler, colorSampler, uv, 1).rgb;
    let normal = textureSample(defferedSampler, colorSampler, uv, 2).rgb;
    let wrldpos = textureSample(defferedSampler, colorSampler, uv, 3).rgb;
    let lt = textureSample(defferedSampler, colorSampler, uv, 6).rgb;
    let glps = textureSample(defferedSampler, colorSampler, uv, 7).rgb;

    let shd = shcalc(wrldpos, 0.0);
    let shdgl = shcalc(glps, 0.0);

    let fogSkyColor = nightSkyFog(uv, dmi.deffpos[0].xyz, dmi.deffrot[0].xyz, time, rma.b == 0.0);

    var op = vec4<f32>(PBR(normal, albedo, shd, rma.y, rma.x, 1.0, wrldpos), 1.0);

    //let depth0 = textureSampleCompare(defferedDepthSampler, shadowComparisonSampler, uv, 0, 0.999);
    //if depth0 > 0.0{
    //    op = vec4<f32>(fogSkyColor, 1.0);
    //}

    let mxpw = smoothstep(10.0, 5.0, distance(dmi.deffpos[0].xyz, wrldpos));
    op = mix(vec4<f32>(fogSkyColor, 1.0), op, mxpw);

    if albedo.r == 0.0 && albedo.g == 0.0 && albedo.b == 0.0 {
        let gf = vec4<f32>(PBR(lt, vec3<f32>(0.1), shdgl, 0.1, 0.1, 1.0, glps), 1.0);
        let glmxpw = smoothstep(10.0, 5.0, distance(dmi.deffpos[0].xyz, glps));
        let fgf = mix(vec4<f32>(fogSkyColor, 1.0), gf, glmxpw);
        op = mix(op, fgf, gf.r);
    }

    if (uv.x > 1.0 || uv.x < 0.0 || uv.y < -1.0 || uv.y > 0.0) {
        op = vec4<f32>(0.0, 0.0, 0.0, 1.0);
    }

    op = mix(op, vec4<f32>(0.0, 0.0, 0.0, 1.0), distance(vec2<f32>(0.5, -0.5), uv) / 0.7);
    op = mix(op, vec4<f32>(0.0, 0.0, 0.0, 1.0), mi.addinfo.x);

    return op;
}