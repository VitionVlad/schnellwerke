use crate::createvec4_with_usage;

use super::{audiosource3d::Audiosource3d, math::{uniformstruct::{Uniformstruct, InShaderUsage}, vec4::Vec4}, object::Object, shader_builder::ShaderBuilder};

#[allow(dead_code)]
struct SceneObject{
    pub object: Object,
    pub is_static: bool,
}

impl SceneObject {
    #[allow(dead_code)]
    pub fn new(obj: Object, is_static: bool) -> SceneObject{
        SceneObject{
            object: obj,
            is_static: is_static,
        }
    }
}

#[allow(dead_code)]
struct LightSource{
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
struct Scene{
    pub shaders: ShaderBuilder,
    pub uniformbuffer: Vec<Uniformstruct>,
    pub objects: Vec<SceneObject>,
    pub audiosources: Vec<Audiosource3d>,
    pub lightsources: Vec<LightSource>,
}

impl Scene {
    #[allow(dead_code)]
    pub fn new_custom_uniform_buffer_custom_shaders(uniform_buffer: Vec<Uniformstruct>, shaders: ShaderBuilder) -> Scene {
        Scene{
            shaders: shaders,
            uniformbuffer: uniform_buffer,
            objects: vec![],
            audiosources: vec![],
            lightsources: vec![],
        }
    }
    #[allow(dead_code)]
    pub fn new_custom_uniform_buffer(uniform_buffer: Vec<Uniformstruct>, use_shadows: bool) -> Scene {
        let mut lss: Vec<LightSource> = Vec::new();
        for i in 0..uniform_buffer.len() {
            if uniform_buffer[i].scene_usage == InShaderUsage::LightPosition && uniform_buffer[i+1].scene_usage == InShaderUsage::LightColor{
                lss.push(LightSource::new(Vec4::newdefined(uniform_buffer[i].vec4.x, uniform_buffer[i].vec4.y, uniform_buffer[i].vec4.z, uniform_buffer[i].vec4.w), Vec4::newdefined(uniform_buffer[i+1].vec4.x, uniform_buffer[i+1].vec4.y, uniform_buffer[i+1].vec4.z, uniform_buffer[i+1].vec4.w), i as u32));
            }
        }
        let mut sh = ShaderBuilder::new(&uniform_buffer);
        sh.new_fragment_shader();
        sh.fragment_begin_main();
        for i in 0..lss.len(){
            sh.fragment_add_light(use_shadows, &uniform_buffer[lss[i].index as usize].label, &uniform_buffer[(lss[i].index+1) as usize].label);
        }
        sh.fragment_end_main();
        Scene{
            shaders: sh,
            uniformbuffer: uniform_buffer,
            objects: vec![],
            audiosources: vec![],
            lightsources: lss,
        }
    }
    #[allow(dead_code)]
    pub fn new(number_of_lights: u32, use_shadows: bool) -> Scene{
        let mut uniforms: Vec<Uniformstruct> = vec![];
        for i in 0..number_of_lights {
            uniforms.push(createvec4_with_usage(Vec4::newdefined(0f32, 0f32, 0f32, 0f32), &("lightPos".to_string() + &i.to_string()), InShaderUsage::LightPosition));
            uniforms.push(createvec4_with_usage(Vec4::newdefined(0f32, 0f32, 0f32, 0f32), &("lightCol".to_string() + &i.to_string()), InShaderUsage::LightColor));
        }
        Scene::new_custom_uniform_buffer(uniforms, use_shadows)
    }
    #[allow(dead_code)]
    pub fn execute() {
    }
}