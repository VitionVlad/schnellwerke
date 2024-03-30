use super::math::uniformstruct::{Uniformstruct, Usages};

#[allow(dead_code)]
pub struct ShaderBuilder{
    pub vertex_code: String,
    pub shadow_vertex_code: String,
    pub fragment_code: String,
    mvpl: String,
    smvpl: String,
    instr: String,
    inpostuse: bool,
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
                    inst += "_proj: mat4x4<f32>,";
                    inst += &uniformbuffer[i].label.to_string();
                    inst += "_mod: mat4x4<f32>,";
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
        @group(0) @binding(0) var<uniform> ubo: uniforms;";
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
              return ubo.".to_string();
            vertex_shadow_code += &smvl.to_string();
            vertex_shadow_code += &" * ubo.";
            vertex_shadow_code += &mvl.to_string();
            vertex_shadow_code += "_mod * pos;
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
            out.position = ubo.".to_string();
            vertex_code += &mvl.to_string();
            vertex_code += &"_proj * ubo.";
            vertex_code += &mvl.to_string();
            vertex_code += "_mod * pos;
            out.uv = vec2f(uv.x, 1.0-uv.y);
            out.norm = n;
            out.tangent = t;
            out.bitangent = cross(n, t);
            out.vertex = ubo.";
            vertex_code += &mvl.to_string();
            vertex_code += "_mod * pos;
            out.smv = ubo.";
            vertex_code += &smvl.to_string();
            vertex_code += &" * ubo.";
            vertex_code += &mvl.to_string();
            vertex_code += "_mod * pos;
            return out;
            }";
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
            out.position = ubo.".to_string();
            vertex_code += &mvl.to_string();
            vertex_code += &"_proj * ubo.";
            vertex_code += &mvl.to_string();
            vertex_code += "_mod * vec4f(pos.xyz, 1);
            out.uv = vec2f(uv.x, 1.0-uv.y);
            out.norm = n;
            out.tangent = t;
            out.bitangent = cross(n, t);
            out.smv = vec4f(pos.xyz, 1);
            out.vertex = ubo.";
            vertex_code += &mvl.to_string();
            vertex_code += "_mod * pos;
            return out;
            }";
        }

        ShaderBuilder { 
            vertex_code: vertex_code.to_string(), 
            shadow_vertex_code: vertex_shadow_code.clone(), 
            fragment_code: "".to_string(),
            mvpl: mvl,
            smvpl: smvl,
            instr: inst,
            inpostuse: false,
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
                    inst += "_proj: mat4x4<f32>,";
                    inst += &uniformbuffer[i].label.to_string();
                    inst += "_mod: mat4x4<f32>,";
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
        @group(0) @binding(0) var<uniform> ubo: uniforms;";

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
            out.position = ubo.".to_string();
            vertex_code += &mvl.to_string();
            vertex_code += &"_proj * ubo.";
            vertex_code += &mvl.to_string();
            vertex_code += "_mod * vec4f(pos.xyz, 1);
            out.position.z = out.position.w;
            out.uv = vec2f(uv.x, 1.0-uv.y);
            out.norm = n;
            out.tangent = t;
            out.bitangent = cross(n, t);
            out.smv = vec4f(pos.xyz, 1);
            out.vertex = ubo.";
            vertex_code += &mvl.to_string();
            vertex_code += "_mod * pos;
            return out;
            }";
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
            inpostuse: false,
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
                    inst += "_proj: mat4x4<f32>,";
                    inst += &uniformbuffer[i].label.to_string();
                    inst += "_mod: mat4x4<f32>,";
                },
                Usages::Smvpmat => {
                    inst += &uniformbuffer[i].label.to_string();
                    inst += ": mat4x4<f32>,";
                },
            }
        }
        inst += " }; 
        @group(0) @binding(0) var<uniform> ubo: uniforms;";

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
            inpostuse: true,
        }
    }
    #[allow(dead_code)]
    pub fn new_fragment_shader(&mut self){
        self.fragment_code = String::new();
        self.fragment_code += &self.instr.to_string();
        if self.inpostuse {
            self.fragment_code += &"
            @group(0) @binding(1) var mySampler: sampler;

            @group(0) @binding(2) var myTexture: texture_2d_array<f32>;

            @group(0) @binding(3) var shadowMap: texture_depth_2d;

            @group(0) @binding(4) var mainMap: texture_2d<f32>;

            @group(0) @binding(5) var mainDepthMap: texture_depth_2d;

            struct OUT{
              @location(0) uv: vec2f,
              @location(1) vertex: vec4f,
            }
            
            fn separateh(uv: vec2f) -> vec3f{
                var tor: vec3f = vec3f(0.0, 0.0, 0.0);
                let alb = textureSample(mainMap, mySampler, uv).rgb;
                if alb.r >= 1.0 || alb.g >= 1.0 || alb.b >= 1.0 {
                    tor = alb-1.0;
                }
                return tor;
              }
              fn bloom(uv: vec2f, off: f32) -> vec3f{
                let offset = 1.0 / off;
                let offsets = array<vec2f, 9>( 
                  vec2f(-offset,  offset),
                  vec2f( 0.0f,    offset),
                  vec2f( offset,  offset),
                  vec2f(-offset,  0.0f),  
                  vec2f( 0.0f,    0.0f),  
                  vec2f( offset,  0.0f),  
                  vec2f(-offset, -offset),
                  vec2f( 0.0f,   -offset),
                  vec2f( offset, -offset) 
                );
                let kernel = array<f32, 9>( 
                  1.0 / 16, 2.0 / 16, 1.0 / 16,
                  2.0 / 16, 4.0 / 16, 2.0 / 16,
                  1.0 / 16, 2.0 / 16, 1.0 / 16  
                );
                var col = vec3f(0.0, 0.0, 0.0);
                for(var i = 0; i < 9; i+=1){
                  col += separateh(uv + offsets[i]) * kernel[i];
                }
                return col;
              }
              fn kbao(uv: vec2f, off: f32) -> vec3f{
                let offset = 1.0 / off;
                let offsets = array<vec2f, 9>( 
                  vec2f(-offset,  offset),
                  vec2f( 0.0f,    offset),
                  vec2f( offset,  offset),
                  vec2f(-offset,  0.0f),  
                  vec2f( 0.0f,    0.0f),  
                  vec2f( offset,  0.0f),  
                  vec2f(-offset, -offset),
                  vec2f( 0.0f,   -offset),
                  vec2f( offset, -offset) 
                );
                let kernel1 = array<f32, 9>( 
                  0.0, -2.5, 0.0,
                  -2.5, 10.0, -2.5,
                  0.0, -2.5, 0.0  
                );
                var col = vec3f(0.0, 0.0, 0.0);
                for(var i = 0; i < 9; i+=1){
                  col += vec3f(textureSample(mainDepthMap, mySampler, uv + offsets[i]) * kernel1[i]);
                }
                col *= 100.0;
                if col.x > 0.005 && col.x < 0.3 {
                  col = vec3f(1.0);
                }else{
                  col = vec3f(0.0);
                }
                return col;
              }
            ";
        }else{
            self.fragment_code += &"
            @group(0) @binding(1) var mySampler: sampler;

            @group(0) @binding(2) var myTexture: texture_2d_array<f32>;

            @group(0) @binding(3) var shadowMap: texture_depth_2d;

            @group(0) @binding(4) var mycube: texture_cube<f32>;

            @group(0) @binding(5) var shadowSampler: sampler_comparison;

            struct OUT{
              @location(0) uv: vec2f,
              @location(1) smv: vec4f,
              @location(2) norm: vec3f,
              @location(3) tangent: vec3f,
              @location(4) bitangent: vec3f,
              @location(5) vertex: vec4f,
            }
            
            fn shadowmapping(smv: vec4f) -> f32{
                let proj = vec3f((smv.x / smv.w)*0.5+0.5, (smv.y / smv.w)*-0.5+0.5, smv.z / smv.w);
                return 1.0-textureSampleCompare(shadowMap, shadowSampler, proj.xy, proj.z-0.001);
              }
          
              fn light(in: OUT, useshadows: bool, lightcolor: vec4f, lightpos: vec4f, playerpos: vec3f) -> vec4f{
                let TBN = mat3x3<f32>(
                  normalize(in.tangent),
                  normalize(in.bitangent),
                    normalize(in.norm),
                );
                let albedo = textureSample(myTexture, mySampler, in.uv, 0).rgb;
                let specularpower = textureSample(myTexture, mySampler, in.uv, 1).r;
                let normal = normalize(TBN * (textureSample(myTexture, mySampler, in.uv, 2).rgb * 2.0 - 1.0));
                var ambient = lightcolor.a * lightcolor.rgb;
          
                if lightpos.w >= 1.0{
                  const constant = 1.0f;
                  const linear = 0.09f;
                  const quadratic = 0.032f; 
                  let dist = length(lightpos.xyz - in.vertex.xyz);
                  let attenuation = 1.0 / (constant + linear * dist + quadratic * (dist * dist));    

                  let lightdir = normalize(lightpos.xyz - in.vertex.xyz);
                  let diff = max(dot(normal, lightdir), 0.0);
                  let diffuse = diff * lightcolor.rgb * attenuation;
          
                  let viewDir = normalize(-playerpos.xyz - in.vertex.xyz);
                  let reflectDir = reflect(-lightdir, normal); 
                  let spec = pow(max(dot(viewDir, reflectDir), 0.0), 32);
                  let specular = specularpower * spec * albedo * attenuation;  

                  ambient *= attenuation;
                  if !useshadows{
                    return vec4f((ambient + diffuse + specular) * albedo, textureSample(myTexture, mySampler, in.uv, 0).a);
                  }
                  return vec4f((ambient + (1.0 - shadowmapping(in.smv)) * (diffuse + specular)) * albedo, textureSample(myTexture, mySampler, in.uv, 0).a);
                }
          
                let lightdir = normalize(-lightpos.xyz);
                let diff = max(dot(normal, lightdir), 0.0);
                let diffuse = diff * lightcolor.rgb;
          
                let viewDir = normalize(-playerpos.xyz - in.vertex.xyz);
                let reflectDir = reflect(-lightdir, normal); 
                let spec = pow(max(dot(viewDir, reflectDir), 0.0), 32);
                let specular = specularpower * spec * albedo;  
          
                if !useshadows{
                  return vec4f((ambient + diffuse + specular) * albedo, textureSample(myTexture, mySampler, in.uv, 0).a);
                }
                return vec4f((ambient + (1.0 - shadowmapping(in.smv)) * (diffuse + specular)) * albedo, textureSample(myTexture, mySampler, in.uv, 0).a);
            }";
        }
    }
    #[allow(dead_code)]
    pub fn fragment_begin_main(&mut self){
        self.fragment_code += &"
        @fragment
        fn fragmentMain(in: OUT) -> @location(0) vec4f {
            var col = vec4f(0.0);
        ".to_string();
    }
    #[allow(dead_code)]
    pub fn fragment_end_main(&mut self){
        self.fragment_code += &"
          return col;
        }
        ".to_string();
    }
    #[allow(dead_code)]
    pub fn fragment_add_light(&mut self, useshadows: bool, lightcolorlabel: &str, lightposlabel: &str, playerposlabel: &str){
        self.fragment_code += &"
          col += light(in, ";
          if useshadows {
            self.fragment_code += "true, ubo.";
          }else{
            self.fragment_code += "false, ubo.";
          }
          self.fragment_code += lightcolorlabel;
          self.fragment_code += ", ubo.";
          self.fragment_code += lightposlabel;
          self.fragment_code += ", ubo.";
          self.fragment_code += playerposlabel;
          self.fragment_code += ".xyz);";
    }
    #[allow(dead_code)]
    pub fn fragment_add_bloom(&mut self){
        self.fragment_code += &"
          col += vec4f(bloom(in.uv, 50.0), 0);
        ".to_string();
    }
    #[allow(dead_code)]
    pub fn fragment_add_kbao(&mut self){
        self.fragment_code += &"
          col -= vec4f(kbao(in.uv, 500.0)/20, 0);
        ".to_string();
    }
    #[allow(dead_code)]
    pub fn fragment_add_mainframebuffer(&mut self){
        self.fragment_code += &"
          col += vec4f(textureSample(mainMap, mySampler, in.uv).rgb, 1);
        ".to_string();
    }
    #[allow(dead_code)]
    pub fn fragment_add_texure(&mut self, layer: u32){
        self.fragment_code += &"
        col += vec4f(textureSample(myTexture, mySampler, in.uv,";
        self.fragment_code += &layer.to_string();
        self.fragment_code += ").rgb, 1);
        ";
    }
    #[allow(dead_code)]
    pub fn fragment_add_cubemap(&mut self){
        self.fragment_code += &"
          col += vec4f(textureSample(mycube, mySampler, in.vertex.xyz).rgb, 1);
        ".to_string();
    }
}