use super::math::{mat4::Mat4, vec3::Vec3};

#[allow(dead_code)]
pub struct Camera{
    pub pos: Vec3,
    pub rot: Vec3,
    pub fov: f32,
    pub znear: f32,
    pub zfar: f32,
    pub is_orthographic: bool,
}

impl Camera{
    #[allow(dead_code)]
    pub fn get_projection(&self, aspect: f32) -> Mat4{
        let mut ubm = Mat4::new();
        if !self.is_orthographic{
            ubm.perspective(self.fov, self.zfar, self.znear, aspect);
        }else{
            ubm.orthographic(self.fov, -self.fov, self.fov, -self.fov, self.znear, self.zfar);
        }
        let mut t: Mat4 = Mat4::new();
        t.xrot(self.rot.x);
        ubm.mul(&t);
        t = Mat4::new();
        t.yrot(self.rot.y);
        ubm.mul(&t);
        t = Mat4::new();
        t.zrot(self.rot.z);
        ubm.mul(&t);
        t = Mat4::new();
        t.trans(Vec3::newdefined(-self.pos.x, -self.pos.y, -self.pos.z));
        ubm.mul(&t);
        ubm.transpose();
        return ubm;
    }
}