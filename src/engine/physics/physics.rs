#[allow(dead_code)]
pub static PHYSICS_GPU: &str = "
@group(0) @binding(0) var<storage> in: array<f32>;
@group(0) @binding(1) var<storage, read_write> out: array<f32>;

fn bg3(a: f32, b: f32, c: f32) -> f32{
    var max_value: f32;
    if (a >= b) {
        max_value = a;
    } else {
        max_value = b;
    }
    if (max_value < c) {
        max_value = c;
    }
    return max_value;
}

fn sm3(a: f32, b: f32, c: f32) -> f32{
    var min_value: f32;
    if (a <= b) {
        min_value = a;
    } else {
        min_value = b;
    }
    if (min_value > c) {
        min_value = c;
    }
    return min_value;
}

@compute @workgroup_size(1) fn computeMain() {
    let mat = mat4x4f(
        in[0], in[1], in[2], in[3],
        in[4], in[5], in[6], in[7],
        in[8], in[9], in[10], in[11],
        in[12], in[13], in[14], in[15],
    );
    var pos = vec3f(-in[16], -in[17], -in[18]);
    let aabb = vec3f(in[19], in[20], in[21]);
    let speed = vec3f(-in[22], -in[23], -in[24]);
    pos += normalize(speed)*aabb;
    pos.y = -in[17];
    var outval = 0f;
    for(var i = 26u; i < u32(in[25]);i+=12){
        var v1 = mat * vec4f(in[i], in[i+1], in[i+2], in[i+3]);
        var v2 = mat * vec4f(in[i+4], in[i+5], in[i+6], in[i+7]);
        var v3 = mat * vec4f(in[i+8], in[i+9], in[i+10], in[i+11]);

        var bb = vec3f(
            bg3(v1.x, v2.x, v3.x),
            bg3(v1.y, v2.y, v3.y),
            bg3(v1.z, v2.z, v3.z)
        );

        var sb = vec3f(
            sm3(v1.x, v2.x, v3.x),
            sm3(v1.y, v2.y, v3.y),
            sm3(v1.z, v2.z, v3.z)
        ); 

        var v = (v1+v2+v3)/3;

        if (pos.x >= sb.x-aabb.x) && (pos.x <= bb.x+aabb.x) &&
            (pos.z >= sb.z-aabb.z) && (pos.z <= bb.z+aabb.z) &&
            (pos.y - aabb.y <= bb.y){
            outval = 1.0f;
            if ((pos.y - aabb.y*0.8 <= sb.y) || (pos.y - aabb.y*0.8 <= bb.y)) && (pos.y >= sb.y){
                outval = 2.0f;
                break;
            }
        }
    }
    out[0] = outval;
    out[1] = pos.x;
    out[2] = pos.y;
    out[3] = pos.z;
}";