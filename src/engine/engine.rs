use self::mat4::Mat4;
use self::vec3::Vec3;

use super::render::render::Render;
use super::math::{self, *};

#[allow(dead_code)]

pub struct Engine{
    pub ren: Render,
    pub projection: math::mat4::Mat4,
    pub pos: math::vec3::Vec3,
    pub size: math::vec3::Vec3,
    pub speed: math::vec3::Vec3,
    pub rot: math::vec3::Vec3,
    pub shadowprojection: math::mat4::Mat4,
    pub shadowpos: math::vec3::Vec3,
    pub shadowrot: math::vec3::Vec3,
    pub orthographic: bool,
    pub shadoworthographic: bool,
    pub fov: f32,
    pub shadowfov: f32,
    pub z_near: f32,
    pub z_far: f32,
    pub shadow_z_near: f32,
    pub shadow_z_far: f32,
    pub inshadow: bool,
    pub volume: f32,
    pub frametime: f64,
    pub fps: i32,
    pub norm: u8,
    pub renderscale: f32,
    pub shadowmapres: i32,
    oldrenderscale: f32,
    oldshadowmapres: i32,
    fr: i32,
    totdur: f64,
    pub usemaxy: bool,
    pub maxy: f32,
    pub use_resolution_scale: bool,
    pub min_scale: f32,
    pub max_scale: f32,
    pub scale_modifier: f32,
    pub prefered_fps: i32,
    pub allowed_diff: i32,
    pub bias: f32,
}

impl Engine{
    #[allow(dead_code)]
    pub fn new(canvasid: &str, renderscale: f32, shadowmapres: i32) -> Engine{      
        Engine{
            ren: Render::init(canvasid, renderscale, shadowmapres),
            projection: Mat4::new(),
            pos: Vec3::new(),
            size: Vec3::newdefined(0.5f32, 4f32, 0.5f32),
            speed: Vec3::new(),
            rot: Vec3::new(),
            orthographic: false,
            fov: 90.0f32,
            shadowprojection: Mat4::new(),
            shadowpos: Vec3::new(),
            shadowrot: Vec3::new(),
            shadoworthographic: false,
            shadowfov: 90.0f32,
            z_near: 0.1f32,
            z_far: 100f32,
            shadow_z_near: 0.1f32,
            shadow_z_far: 100f32,
            inshadow: false,
            volume: 1.0f32,
            frametime: 0.0f64,
            fps: 0,
            fr: 0,
            totdur: 0.0f64,
            norm: 0,
            renderscale: renderscale,
            shadowmapres: shadowmapres,
            oldrenderscale: renderscale,
            oldshadowmapres: shadowmapres,
            usemaxy: false,
            maxy: 0.0f32,
            use_resolution_scale: false,
            min_scale: 0.5f32,
            max_scale: 1.0f32,
            scale_modifier: 0.1,
            prefered_fps: 60,
            allowed_diff: 5,
            bias: 0.0015f32,
        }
    }
    #[allow(dead_code)]
    pub fn calculate_projection(&mut self){
        self.projection = Mat4::new();
        if !self.orthographic{
            self.projection.perspective(self.fov, self.z_far, self.z_near, self.ren.get_canvas_size_x() as f32/self.ren.get_canvas_size_y() as f32);
        }else{
            self.projection.orthographic(self.fov, -self.fov, self.fov, -self.fov, self.z_near, self.z_far);
        }
        let mut t: Mat4 = Mat4::new();
        t.xrot(self.rot.x);
        self.projection.mul(&t);

        t = Mat4::new();
        t.yrot(self.rot.y);
        self.projection.mul(&t);

        t = Mat4::new();
        t.zrot(self.rot.z);
        self.projection.mul(&t);

        t = Mat4::new();
        t.trans(Vec3::newdefined(self.pos.x, self.pos.y, self.pos.z));
        self.projection.mul(&t);
        self.projection.transpose();
    }
    #[allow(dead_code)]
    pub fn calculate_shadow_projection(&mut self){
        self.shadowprojection = Mat4::new();
        if !self.shadoworthographic{
            self.shadowprojection.perspective(self.shadowfov, self.shadow_z_far, self.shadow_z_near, 1f32);
        }else{
            self.shadowprojection.orthographic(self.shadowfov, -self.shadowfov, self.shadowfov, -self.shadowfov, self.shadow_z_near, self.shadow_z_far);
        }

        let mut t: Mat4 = Mat4::new();
        t.xrot(self.shadowrot.x);
        self.shadowprojection.mul(&t);

        t = Mat4::new();
        t.yrot(self.shadowrot.y);
        self.shadowprojection.mul(&t);

        t = Mat4::new();
        t.zrot(self.shadowrot.z);
        self.shadowprojection.mul(&t);

        t = Mat4::new();
        t.trans(Vec3::newdefined(self.shadowpos.x, self.shadowpos.y, self.shadowpos.z));
        self.shadowprojection.mul(&t);
        self.shadowprojection.transpose();
    }
    #[allow(dead_code)]
    pub fn begin_shadow(&mut self, loadop: &str){
        self.ren.begin_shadow_pass(loadop);
        self.calculate_shadow_projection();
        self.inshadow = true;
    }
    #[allow(dead_code)]
    pub fn begin_main(&mut self, loadop: &str, depthloadop: &str){
        self.ren.begin_main_pass(loadop, depthloadop);
        self.calculate_projection();
        self.inshadow = false;
    }
    #[allow(dead_code)]
    pub fn begin_post(&mut self, loadop: &str, depthloadop: &str){
        self.ren.begin_post_pass(loadop, depthloadop);
        self.inshadow = false;
    }
    #[allow(dead_code)]
    pub fn end(&mut self){
        if self.renderscale != self.oldrenderscale {
            self.ren.change_render_scale(self.renderscale);
            self.oldrenderscale = self.renderscale;
        }
        if self.shadowmapres != self.oldshadowmapres {
            self.ren.change_shadow_map_resolution(self.shadowmapres);
            self.oldshadowmapres = self.shadowmapres;
        }
        self.ren.end_render();
        if self.norm > 1{
            self.speed.x /= self.fps as f32;
            self.speed.y /= self.fps as f32;
            if self.usemaxy{
                self.pos.y = -self.maxy - self.size.y;
                self.usemaxy = false;
            }
            if self.speed.y >= 1f32 && self.fps <= 5{
                self.speed.y *= self.fps as f32;
                self.speed.y /= self.prefered_fps as f32;
            }
            self.speed.z /= self.fps as f32;
            self.pos.sum(self.speed);
        }
        self.speed = Vec3::new();
        self.inshadow = false;
        self.totdur += self.ren.jsren.gfxgetexectime();
        self.fr += 1;
        if self.totdur >= 1000.0f64 {
            self.fps = self.fr;
            self.frametime = self.totdur / self.fr as f64;
            self.fr = 0;
            self.totdur = 0.0f64;
            if self.norm < 2{
                self.norm += 1;
            }
            if self.fps < self.prefered_fps - self.allowed_diff && self.use_resolution_scale{
                if self.renderscale > self.min_scale{
                    self.renderscale -= self.scale_modifier;
                }
            }
            if self.fps >= self.prefered_fps && self.use_resolution_scale{
                if self.renderscale < self.max_scale{
                    self.renderscale += self.scale_modifier;
                }
            }
        }
    }
}