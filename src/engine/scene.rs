use crate::createvec4_with_usage;

use super::{audiosource3d::Audiosource3d, engine::Engine, math::{uniformstruct::{InShaderUsage, Uniformstruct}, vec2::Vec2, vec3::Vec3, vec4::Vec4}, object::Object, shader_builder::ShaderBuilder};

#[allow(dead_code)]
pub struct LightSource{
    pub pos: Vec4,
    pub color: Vec4,
    pub index: u32,
}

impl LightSource {
    #[allow(dead_code)]
    pub fn new(pos: Vec4, color: Vec4, index: u32) -> LightSource{
        LightSource{
            pos: pos,
            color: color,
            index: index,
        }
    }
}

#[allow(dead_code)]
pub struct Scene{
    pub shaders: ShaderBuilder,
    pub uniformbuffer: Vec<Uniformstruct>,
    pub objects: Vec<Object>,
    pub audiosources: Vec<Audiosource3d>,
    pub lightsources: Vec<LightSource>,
    pub render_shadows: bool,
    pub light_shadow_source_pos: Vec3,
    pub light_shadow_source_rot: Vec3,
    pub light_shadow_source_clip: Vec2,
    pub light_shadow_source_ortho: bool,
    pub light_shadow_source_fov: f32,
    pub min_filter: String,
    pub mag_filter: String,
    pub culling: String,
    pub sh_culling: String,
    pub repeat_mode: String,
    pub shadow_loadop: String,
    index: usize,
}

impl Scene {
    #[allow(dead_code)]
    pub fn new_custom_uniform_buffer_custom_shaders(uniform_buffer: Vec<Uniformstruct>, shaders: ShaderBuilder, use_shadows: bool) -> Scene {
        Scene{
            shaders: shaders,
            uniformbuffer: uniform_buffer,
            objects: vec![],
            audiosources: vec![],
            lightsources: vec![],
            render_shadows: use_shadows,
            light_shadow_source_pos: Vec3::new(),
            light_shadow_source_rot: Vec3::new(),
            min_filter: "linear".to_string(),
            mag_filter: "linear".to_string(),
            culling: "none".to_string(),
            sh_culling: "none".to_string(),
            repeat_mode: "repeat".to_string(),
            shadow_loadop: "clear".to_string(),
            index: 0,
            light_shadow_source_clip: Vec2::newdefined(0.1f32, 100f32),
            light_shadow_source_ortho: false,
            light_shadow_source_fov: 90f32,
        }
    }
    #[allow(dead_code)]
    pub fn new_custom_uniform_buffer(uniform_buffer: Vec<Uniformstruct>, use_shadows: bool) -> Scene {
        let mut lss: Vec<LightSource> = Vec::new();
        for i in (0..uniform_buffer.len()).step_by(2) {
            if uniform_buffer[i].scene_usage == InShaderUsage::LightPosition && uniform_buffer[i+1].scene_usage == InShaderUsage::LightColor{
                lss.push(LightSource::new(Vec4::newdefined(uniform_buffer[i].vec4.x, uniform_buffer[i].vec4.y, uniform_buffer[i].vec4.z, uniform_buffer[i].vec4.w), Vec4::newdefined(uniform_buffer[i+1].vec4.x, uniform_buffer[i+1].vec4.y, uniform_buffer[i+1].vec4.z, uniform_buffer[i+1].vec4.w), i as u32));
            }
        }
        let mut sh = ShaderBuilder::new(&uniform_buffer);
        sh.new_fragment_shader();
        sh.fragment_begin_main();
        for i in 0..lss.len(){
            sh.fragment_add_light(use_shadows, &uniform_buffer[(lss[i].index+1) as usize].label, &uniform_buffer[lss[i].index as usize].label);
        }
        sh.fragment_end_main();
        Scene{
            shaders: sh,
            uniformbuffer: uniform_buffer,
            objects: vec![],
            audiosources: vec![],
            lightsources: lss,
            render_shadows: use_shadows,
            light_shadow_source_pos: Vec3::new(),
            light_shadow_source_rot: Vec3::new(),
            min_filter: "linear".to_string(),
            mag_filter: "linear".to_string(),
            culling: "none".to_string(),
            sh_culling: "none".to_string(),
            repeat_mode: "repeat".to_string(),
            shadow_loadop: "clear".to_string(),
            index: 0,
            light_shadow_source_clip: Vec2::newdefined(0.1f32, 100f32),
            light_shadow_source_ortho: false,
            light_shadow_source_fov: 90f32,
        }
    }
    #[allow(dead_code)]
    pub fn new(number_of_lights: u32, use_shadows: bool) -> Scene{
        let mut uniforms: Vec<Uniformstruct> = vec![];
        for i in 0..number_of_lights {
            uniforms.push(createvec4_with_usage(Vec4::newdefined(0f32, 0.0f32, 0.0f32, 0.0f32), &("lightPos".to_string() + &i.to_string()), InShaderUsage::LightPosition));
            uniforms.push(createvec4_with_usage(Vec4::newdefined(0f32, 0f32, 0f32, 0.0f32), &("lightCol".to_string() + &i.to_string()), InShaderUsage::LightColor));
        }
    
        Scene::new_custom_uniform_buffer(uniforms, use_shadows)
    }
    #[allow(dead_code)]
    pub fn push_custom_object(&mut self, object: Object) {
        self.objects.push(object);
        self.index+=1;
    }
    #[allow(dead_code)]
    pub fn push_audio_source(&mut self, audio_source: Audiosource3d) {
        self.audiosources.push(audio_source);
    }
    #[allow(dead_code)]
    pub fn push_object(&mut self, eng: &Engine, modid: &str, texid: &str, cubeid: &str, pos: Vec3, rot: Vec3, scale: Vec3) {
        self.push_custom_object(Object::new_from_obj(eng, modid, &self.shaders.vertex_code, &self.shaders.shadow_vertex_code, &self.shaders.fragment_code, &self.uniformbuffer, texid, cubeid, &self.mag_filter, &self.min_filter, &self.culling, &self.sh_culling, &self.repeat_mode, false));
        self.objects[self.index-1].pos = pos;
        self.objects[self.index-1].rot = rot;
        self.objects[self.index-1].scale = scale;
    }
    #[allow(dead_code)]
    pub fn push_object_vertices(&mut self, eng: &Engine, vertices: &[f32], uv: &[f32], normals: &[f32], lenght: i32, texid: &str, cubeid: &str, pos: Vec3, rot: Vec3, scale: Vec3) {
        self.push_custom_object(Object::new(eng, vertices, uv, normals, lenght, &self.shaders.vertex_code, &self.shaders.shadow_vertex_code, &self.shaders.fragment_code, &self.uniformbuffer, texid, cubeid, &self.mag_filter, &self.min_filter, &self.culling, &self.sh_culling, &self.repeat_mode, false));
        self.objects[self.index-1].pos = pos;
        self.objects[self.index-1].rot = rot;
        self.objects[self.index-1].scale = scale;
    }
    #[allow(dead_code)]
    pub fn push_plane(&mut self, eng: &Engine, texid: &str, cubeid: &str, pos: Vec3, rot: Vec3, scale: Vec3) {
        self.push_custom_object(Object::new_plane(eng, &self.shaders.vertex_code, &self.shaders.shadow_vertex_code, &self.shaders.fragment_code, &self.uniformbuffer, texid, cubeid, &self.mag_filter, &self.min_filter, &self.culling, &self.sh_culling, &self.repeat_mode, false));
        self.objects[self.index-1].pos = pos;
        self.objects[self.index-1].rot = rot;
        self.objects[self.index-1].scale = scale;
    }
    #[allow(dead_code)]
    pub fn push_cube(&mut self, eng: &Engine, texid: &str, cubeid: &str, pos: Vec3, rot: Vec3, scale: Vec3) {
        self.push_custom_object(Object::new_cube(eng, &self.shaders.vertex_code, &self.shaders.shadow_vertex_code, &self.shaders.fragment_code, &self.uniformbuffer, texid, cubeid, &self.mag_filter, &self.min_filter, &self.culling, &self.sh_culling, &self.repeat_mode, false));
        self.objects[self.index-1].pos = pos;
        self.objects[self.index-1].rot = rot;
        self.objects[self.index-1].scale = scale;
    }
    #[allow(dead_code)]
    pub fn push_cube_planeuv(&mut self, eng: &Engine, texid: &str, cubeid: &str, pos: Vec3, rot: Vec3, scale: Vec3) {
        self.push_custom_object(Object::new_cube_planeuv(eng, &self.shaders.vertex_code, &self.shaders.shadow_vertex_code, &self.shaders.fragment_code, &self.uniformbuffer, texid, cubeid, &self.mag_filter, &self.min_filter, &self.culling, &self.sh_culling, &self.repeat_mode, false));
        self.objects[self.index-1].pos = pos;
        self.objects[self.index-1].rot = rot;
        self.objects[self.index-1].scale = scale;
    }
    #[allow(dead_code)]
    pub fn draw(&mut self, eng: &mut Engine) {
        for i in 0..self.lightsources.len(){
            self.uniformbuffer[self.lightsources[i].index as usize].vec4 = self.lightsources[i].pos;
            self.uniformbuffer[(self.lightsources[i].index + 1) as usize].vec4 = self.lightsources[i].color;
        }
        for i in 0..self.objects.len() {
            self.objects[i].draw(eng, &self.uniformbuffer);
        }
    }
    #[allow(dead_code)]
    pub fn draw_shadow(&mut self, eng: &mut Engine) {
        eng.shadowpos = self.light_shadow_source_pos;
        eng.shadowrot = self.light_shadow_source_rot;
        eng.shadoworthographic = self.light_shadow_source_ortho;
        eng.shadow_z_near = self.light_shadow_source_clip.x;
        eng.shadow_z_far = self.light_shadow_source_clip.y;
        eng.shadowfov = self.light_shadow_source_fov;
        eng.begin_shadow(&self.shadow_loadop);
        for i in 0..self.objects.len() {
            self.objects[i].draw(eng, &self.uniformbuffer);
        }
    }
}