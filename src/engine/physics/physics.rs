#[allow(dead_code)]
pub static PHYSICS_GPU: &str = "
@group(0) @binding(0) var<storage> in: array<f32>;
@group(0) @binding(1) var<storage, read_write> out: array<f32>;

@compute @workgroup_size(1) fn computeMain() {
    let mat = mat4x4f(
        in[0], in[1], in[2], in[3],
        in[4], in[5], in[6], in[7],
        in[8], in[9], in[10], in[11],
        in[12], in[13], in[14], in[15],
    );
    let pos = vec3f(in[16], in[17], in[18]);
    let aabb = vec3f(in[19], in[20], in[21]);
    let speed = vec3f(in[22], in[23], in[24]);
    var outval = 0f;
    for(var i = 26u; i < u32(in[25]);i+=4){
        var v = mat * vec4f(in[i], in[i+1], in[i+2], in[i+3]);
        if (pos.x + aabb.x >= -v.x) && (pos.x - aabb.x <= -v.x) &&
        (pos.z + aabb.z >= -v.z) && (pos.z - aabb.z <= -v.z) &&
        (pos.y + aabb.y >= -v.y) && (pos.y <= -v.y) {
            outval = 1.0f;
            if (pos.y + aabb.y/2 >= -v.y) && (distance(pos+speed, vec3f(v.x, v.y, v.z)) >= distance(pos, vec3f(v.x, v.y, v.z))) {
                outval = 2.0f;
            }
        }
    }
    out[0] = outval;
    out[1] = pos.x;
    out[2] = pos.y;
    out[3] = pos.z;
}";