struct UniformBufferObject {
    view: mat4x4<f32>
}
@group(0) @binding(0) var<uniform> ubo: UniformBufferObject;

struct Model {
    resolutions: vec4f,
    lightinfo: vec4f,
    model: mat4x4<f32>
}
@group(0) @binding(1) var<uniform> modelbuf: Model;

@vertex
fn main(
    @location(0) pos: vec3f,
    @location(1) uv: vec2f,
    @location(2) normal: vec3f,
    @location(3) tg: vec3f,
    @location(4) ctg: vec3f
) -> @builtin(position) vec4f {
    return ubo.view * modelbuf.model * vec4f(pos.x, pos.y, pos.z, 1.0);
}