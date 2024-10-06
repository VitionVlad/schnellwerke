use super::math::{mat4::Mat4, vec3::Vec3, vec4::Vec4};

#[allow(dead_code)]
pub fn check_for_intersection(x1: f32, x2: f32, y1: f32, y2: f32) -> bool{
    return x1 <= y2 && y1 <= x2;
}

#[allow(dead_code)]
pub fn getpoints(v: Vec<f32>) -> Vec<Vec3>{
    let mut v1 = Vec3::newdefined(v[0], v[1], v[2]);
    let mut v2 = Vec3::newdefined(v[0], v[1], v[2]);
    for i in (0..v.len()).step_by(3){
        if v[i] > v1.x {
            v1.x = v[i];
        }
        if v[i+1] > v1.y {
            v1.y = v[i+1];
        }
        if v[i+2] > v1.z {
            v1.z = v[i+2];
        }
        if v[i] < v2.x {
            v2.x = v[i];
        }
        if v[i+1] < v2.y {
            v2.y = v[i+1];
        }
        if v[i+2] < v2.z {
            v2.z = v[i+2];
        }
    }
    return vec![v1, v2];
}

#[allow(dead_code)]
#[derive(Clone)]
#[derive(Copy)]
pub struct PhysicsObject{
    pub acceleration: Vec3,
    pub speed: Vec3,
    pub v1: Vec3,
    pub v2: Vec3,
    pub is_static: bool,
    pub is_interacting: bool,
    pub elasticity: f32,
    pub gravity: bool,
    pub air_friction: f32,
    pub pos: Vec3,
    pub rot: Vec3,
    pub scale: Vec3,
    pub mat: Mat4,
    pub solid: bool,
    pub mass: f32,
    retur: bool,
    oldpos: Vec3,
    oldrot: Vec3,
    oldscale: Vec3,
}

impl PhysicsObject{
    #[allow(dead_code)]
    pub fn new(v: Vec<Vec3>, is_static: bool) -> PhysicsObject{
        PhysicsObject{
            acceleration: Vec3::new(),
            speed: Vec3::new(),
            v1: v[0],
            v2: v[1],
            is_static: is_static,
            is_interacting: false,
            elasticity: 0.5f32,
            gravity: true,
            air_friction: 0.9f32,
            pos: Vec3::new(),
            rot: Vec3::new(),
            scale: Vec3::newdefined(1f32, 1f32, 1f32),
            mat: Mat4::new(),
            solid: true,
            mass: 0.01f32,
            retur: false,
            oldpos: Vec3::new(),
            oldrot: Vec3::new(),
            oldscale: Vec3::newdefined(1f32, 1f32, 1f32),
        }
    }
    #[allow(dead_code)]
    pub fn exec(&mut self){
        if !self.is_static{
            if self.retur{
                self.retur = !self.retur;
                self.pos = self.oldpos;
                self.rot = self.oldrot;
                self.scale = self.oldscale;
            }
            self.oldpos = self.pos;
            self.oldrot = self.rot;
            self.oldscale = self.scale;
            self.speed.x *= self.air_friction;
            self.speed.y *= self.air_friction;
            self.speed.z *= self.air_friction;
            self.speed.sum(self.acceleration);
            self.pos.sum(self.speed);
            if self.gravity{
                self.acceleration.y = -self.mass;
            }
        }
        let mut mmat = Mat4::new();
        mmat.trans(self.pos);
        let mut t: Mat4 = Mat4::new();
        //t.yrot(self.rot.y);
        //mmat.mul(&t);
        //t = Mat4::new();
        //t.xrot(self.rot.x);
        //mmat.mul(&t);
        //t = Mat4::new();
        //t.zrot(self.rot.z);
        //mmat.mul(&t);
        //t = Mat4::new();
        t.scale(self.scale);
        mmat.mul(&t);
        self.mat = mmat;
    }
    #[allow(dead_code)]
    pub fn reset_states(&mut self){
        self.is_interacting = false;
    }
    #[allow(dead_code)]
    pub fn interact_with_other_object(&mut self, ph2: PhysicsObject){
        let p1fv1 = self.mat.vec4mul(Vec4::newdefined(self.v1.x, self.v1.y, self.v1.z, 1.0));
        let p1fv2 = self.mat.vec4mul(Vec4::newdefined(self.v2.x, self.v2.y, self.v2.z, 1.0));
        let p2fv1 = ph2.mat.vec4mul(Vec4::newdefined(ph2.v1.x, ph2.v1.y, ph2.v1.z, 1.0));
        let p2fv2 = ph2.mat.vec4mul(Vec4::newdefined(ph2.v2.x, ph2.v2.y, ph2.v2.z, 1.0));

        if check_for_intersection(p2fv2.y, p2fv1.y, p1fv2.y, p1fv1.y) && 
            check_for_intersection(p2fv2.x, p2fv1.x, p1fv2.x, p1fv1.x) && 
            check_for_intersection(p2fv2.z, p2fv1.z, p1fv2.z, p1fv1.z) {
            self.is_interacting = true;
            if self.solid && ph2.solid{
                self.acceleration.y = 0f32;
                self.speed.y = -self.speed.y * self.elasticity;
                if p1fv2.y + 1f32 <= p2fv1.y{
                    self.acceleration.x = 0f32;
                    self.speed.x = -self.speed.x * self.elasticity;
                    self.acceleration.z = 0f32;
                    self.speed.z = -self.speed.z * self.elasticity;
                    self.retur = true;
                }
            }
        }
    }
}