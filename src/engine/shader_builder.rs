use super::math::uniformstruct::{Uniformstruct, Usages};

#[allow(dead_code)]
pub struct ShaderBuilder{
    pub vertex_code: String,
    pub shadow_vertex_code: String,
    pub fragment_code: String,
    mvpl: String,
    smvpl: String,
    instr: String,
}

impl ShaderBuilder {
    #[allow(dead_code)]
    pub fn new(uniformbuffer: &Vec<Uniformstruct>) -> ShaderBuilder{
        let mut mvl: String = "".to_string();
        let mut smvl: String = "".to_string();
        let mut mve: bool = false;
        let mut smve: bool = false;
        let mut inst: String = "
            struct uniforms {
        ".to_string();
        for i in 0..uniformbuffer.len() {
            match uniformbuffer[i].usage{
                Usages::Float => {
                    inst += &uniformbuffer[i].label.to_string();
                    inst += ": f32,";
                },
                Usages::Vec2 => {
                    inst += &uniformbuffer[i].label.to_string();
                    inst += ": vec2f,";
                },
                Usages::Vec3 => {
                    inst += &uniformbuffer[i].label.to_string();
                    inst += ": vec3f,";
                },
                Usages::Vec4 => {
                    inst += &uniformbuffer[i].label.to_string();
                    inst += ": vec4f,";
                },
                Usages::Mat => {
                    inst += &uniformbuffer[i].label.to_string();
                    inst += ": mat4x4<f32>,";
                },
                Usages::Mvpmat => {
                    inst += &uniformbuffer[i].label.to_string();
                    inst += ": mat4x4<f32>,";
                    mvl = uniformbuffer[i].label.clone();
                    mve = true;
                },
                Usages::Smvpmat => {
                    inst += &uniformbuffer[i].label.to_string();
                    inst += ": mat4x4<f32>,";
                    smvl = uniformbuffer[i].label.clone();
                    smve = true;
                },
            }
        }
        inst += " }; 
        @group(0) @binding(0) var<uniform> in: uniforms;";
        let mut vertex_shadow_code: String = "
        @vertex
        fn vertexMain(@location(0) pos: vec4f) -> @builtin(position) vec4f {
          return vec4f(pos.xyz, 1);
        }
        ".to_string();
        if smve == true {
            vertex_shadow_code = String::new();
            vertex_shadow_code += &inst.to_string();
            vertex_shadow_code += &"
            @vertex
            fn vertexMain(@location(0) pos: vec4f) -> @builtin(position) vec4f {
              return in.".to_string();
            vertex_shadow_code += &smvl.to_string();
            vertex_shadow_code += &" * vec4f(pos.xyz, 1);
            }
            ";
        }

        let mut vertex_code: String = "
        @vertex
        fn vertexMain(@location(0) pos: vec4f) -> @builtin(position) vec4f {
          return vec4f(pos.xyz, 1);
        }
        ".to_string();
        if mve == true && smve == true {
            vertex_code = String::new();
            vertex_code += &inst.to_string();
            vertex_code += &"
            struct OUT{
              @builtin(position) position: vec4f,
              @location(0) uv: vec2f,
              @location(1) smv: vec4f,
              @location(2) norm: vec3f,
              @location(3) tangent: vec3f,
              @location(4) bitangent: vec3f,
              @location(5) vertex: vec4f,
            }

            @vertex
            fn vertexMain(@location(0) pos: vec4f, @location(1) uv: vec2f, @location(2) n: vec3f, @location(3) t: vec3f) -> OUT {
            var out: OUT;
            out.position = in.".to_string();
            vertex_code += &mvl.to_string();
            vertex_code += &" * vec4f(pos.xyz, 1);
            out.uv = vec2f(uv.x, 1.0-uv.y);
            out.norm = n;
            out.tangent = t;
            out.bitangent = cross(n, t);
            out.vertex = pos;
            out.smv = in.".to_string();
            vertex_code += &smvl.to_string();
            vertex_code += &" * vec4f(pos.xyz, 1);
            return out;
            }".to_string();
        }

        if mve == true && smve == false {
            vertex_code = String::new();
            vertex_code += &inst.to_string();
            vertex_code += &"
            @group(0) @binding(0) var<uniform> in: uniforms;

            struct OUT{
                @builtin(position) position: vec4f,
                @location(0) uv: vec2f,
                @location(1) smv: vec4f,
                @location(2) norm: vec3f,
                @location(3) tangent: vec3f,
                @location(4) bitangent: vec3f,
                @location(5) vertex: vec4f,
            }

            @vertex
            fn vertexMain(@location(0) pos: vec4f, @location(1) uv: vec2f, @location(2) n: vec3f, @location(3) t: vec3f) -> OUT {
            var out: OUT;
            out.position = in.".to_string();
            vertex_code += &mvl.to_string();
            vertex_code += &" * vec4f(pos.xyz, 1);
            out.uv = vec2f(uv.x, 1.0-uv.y);
            out.norm = n;
            out.tangent = t;
            out.bitangent = cross(n, t);
            out.smv = vec4f(pos.xyz, 1);
            out.vertex = pos;
            return out;
            }".to_string();
        }

        ShaderBuilder { 
            vertex_code: vertex_code.to_string(), 
            shadow_vertex_code: vertex_shadow_code.clone(), 
            fragment_code: "".to_string(),
            mvpl: mvl,
            smvpl: smvl,
            instr: inst,
        }
    }
    #[allow(dead_code)]
    pub fn new_skybox(uniformbuffer: &Vec<Uniformstruct>) -> ShaderBuilder{
        let mut mvl: String = "".to_string();
        let mut mve: bool = false;
        let mut inst: String = "
            struct uniforms {
        ".to_string();
        for i in 0..uniformbuffer.len() {
            match uniformbuffer[i].usage{
                Usages::Float => {
                    inst += &uniformbuffer[i].label.to_string();
                    inst += ": f32,";
                },
                Usages::Vec2 => {
                    inst += &uniformbuffer[i].label.to_string();
                    inst += ": vec2f,";
                },
                Usages::Vec3 => {
                    inst += &uniformbuffer[i].label.to_string();
                    inst += ": vec3f,";
                },
                Usages::Vec4 => {
                    inst += &uniformbuffer[i].label.to_string();
                    inst += ": vec4f,";
                },
                Usages::Mat => {
                    inst += &uniformbuffer[i].label.to_string();
                    inst += ": mat4x4<f32>,";
                },
                Usages::Mvpmat => {
                    inst += &uniformbuffer[i].label.to_string();
                    inst += ": mat4x4<f32>,";
                    mvl = uniformbuffer[i].label.clone();
                    mve = true;
                },
                Usages::Smvpmat => {
                    inst += &uniformbuffer[i].label.to_string();
                    inst += ": mat4x4<f32>,";
                },
            }
        }
        inst += " }; 
        @group(0) @binding(0) var<uniform> in: uniforms;";

        let mut vertex_code: String = "
        @vertex
        fn vertexMain(@location(0) pos: vec4f) -> @builtin(position) vec4f {
          return vec4f(pos.xy, 1, 1);
        }
        ".to_string();
        if mve == true {
            vertex_code = String::new();
            vertex_code += &inst.to_string();
            vertex_code += &"
            struct OUT{
              @builtin(position) position: vec4f,
              @location(0) uv: vec2f,
              @location(1) smv: vec4f,
              @location(2) norm: vec3f,
              @location(3) tangent: vec3f,
              @location(4) bitangent: vec3f,
              @location(5) vertex: vec4f,
            }

            @vertex
            fn vertexMain(@location(0) pos: vec4f, @location(1) uv: vec2f, @location(2) n: vec3f, @location(3) t: vec3f) -> OUT {
            var out: OUT;
            out.position = in.".to_string();
            vertex_code += &mvl.to_string();
            vertex_code += &" * vec4f(pos.xyz, 1);
            out.position.z = out.position.w;
            out.uv = vec2f(uv.x, 1.0-uv.y);
            out.norm = n;
            out.tangent = t;
            out.bitangent = cross(n, t);
            out.smv = vec4f(pos.xyz, 1);
            out.vertex = pos;
            return out;
            }".to_string();
        }

        ShaderBuilder { 
            vertex_code: vertex_code.to_string(), 
            shadow_vertex_code: "
            @vertex
            fn vertexMain(@location(0) pos: vec4f) -> @builtin(position) vec4f {
              return vec4f(pos.xyz, 1);
            }
            ".to_string(), 
            fragment_code: "
            @group(0) @binding(1) var mySampler: sampler;

            @group(0) @binding(4) var mycube: texture_cube<f32>;

            struct OUT{
              @location(0) uv: vec2f,
              @location(1) smv: vec4f,
              @location(2) norm: vec3f,
              @location(3) tangent: vec3f,
              @location(4) bitangent: vec3f,
              @location(5) vertex: vec4f,
            }
        
            @fragment
            fn fragmentMain(in: OUT) -> @location(0) vec4f {
              return vec4f(textureSample(mycube, mySampler, in.vertex.xyz).rgb, 1);
            }".to_string(),
            mvpl: mvl,
            smvpl: "".to_string(),
            instr: inst,
        }
    }
    #[allow(dead_code)]
    pub fn new_post_procces(uniformbuffer: &Vec<Uniformstruct>) -> ShaderBuilder{
        let mut inst: String = "
            struct uniforms {
        ".to_string();
        for i in 0..uniformbuffer.len() {
            match uniformbuffer[i].usage{
                Usages::Float => {
                    inst += &uniformbuffer[i].label.to_string();
                    inst += ": f32,";
                },
                Usages::Vec2 => {
                    inst += &uniformbuffer[i].label.to_string();
                    inst += ": vec2f,";
                },
                Usages::Vec3 => {
                    inst += &uniformbuffer[i].label.to_string();
                    inst += ": vec3f,";
                },
                Usages::Vec4 => {
                    inst += &uniformbuffer[i].label.to_string();
                    inst += ": vec4f,";
                },
                Usages::Mat => {
                    inst += &uniformbuffer[i].label.to_string();
                    inst += ": mat4x4<f32>,";
                },
                Usages::Mvpmat => {
                    inst += &uniformbuffer[i].label.to_string();
                    inst += ": mat4x4<f32>,";
                },
                Usages::Smvpmat => {
                    inst += &uniformbuffer[i].label.to_string();
                    inst += ": mat4x4<f32>,";
                },
            }
        }
        inst += " }; 
        @group(0) @binding(0) var<uniform> in: uniforms;";

        let mut vertex_code = String::new();
        vertex_code += &inst.to_string();
        vertex_code += &"
        struct OUT{
            @builtin(position) position: vec4f,
            @location(0) uv: vec2f,
            @location(1) vertex: vec4f,
          }
      
          @vertex
          fn vertexMain(@location(0) pos: vec4f, @location(1) uv: vec2f, @location(2) n: vec3f, @location(3) t: vec3f) -> OUT {
            var out: OUT;
            out.position = vec4f(pos.xyz, 1);
            out.uv = uv;
            out.vertex = pos;
            return out;
          }".to_string();

        ShaderBuilder { 
            vertex_code: vertex_code.to_string(), 
            shadow_vertex_code: "
            @vertex
            fn vertexMain(@location(0) pos: vec4f) -> @builtin(position) vec4f {
              return vec4f(pos.xyz, 1);
            }
            ".to_string(), 
            fragment_code: "
            @group(0) @binding(1) var mySampler: sampler;

            @group(0) @binding(2) var myTexture: texture_2d_array<f32>;

            @group(0) @binding(3) var shadowMap: texture_depth_2d;

            @group(0) @binding(4) var mainMap: texture_2d<f32>;

            @group(0) @binding(5) var mainDepthMap: texture_depth_2d;

            struct OUT{
              @location(0) uv: vec2f,
              @location(1) vertex: vec4f,
            }
        
            @fragment
            fn fragmentMain(in: OUT) -> @location(0) vec4f {
              return vec4f(textureSample(mainMap, mySampler, in.uv).rgb, 1);
            }".to_string(),
            mvpl: "".to_string(),
            smvpl: "".to_string(),
            instr: inst,
        }
    }
}