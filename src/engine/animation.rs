use super::math::vec3::Vec3;

#[allow(dead_code)]
struct Keytiming{
    pub tim: i64,
    pub fpos: Vec3,
    pub frot: Vec3,
    pub fscale: Vec3,
}

impl Keytiming {
    #[allow(dead_code)]
    pub fn new(tim: i64, fpos: Vec3, frot: Vec3, fscale: Vec3) -> Keytiming{
        Keytiming{
            tim: tim,
            fpos: fpos,
            frot: frot,
            fscale: fscale
        }
    }
}