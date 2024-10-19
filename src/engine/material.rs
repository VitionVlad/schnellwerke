use super::math::uniformstruct::Uniformstruct;

#[allow(dead_code)]
#[derive(Clone)]
pub struct Material{
    pub tex_ids: String,
    pub cube_ids: String,
    pub vertex_shader: String,
    pub fragment_shader: String,
    pub uniend: String,
    pub magfilter: String,
    pub minfilter: String,
    pub culling_mode: String,
    pub culling_mode_shadow: String,
    pub repeat_mode: String,
    pub ubo_size: i32,
}

#[allow(dead_code)]
pub struct MaterialGenerator{
    pub vertex_shader: String,
    pub fragment_shader: String,
    pub magfilter: String,
    pub minfilter: String,
    pub culling_mode: String,
    pub culling_mode_shadow: String,
    pub repeat_mode: String,
    shaderbeg: String,
    pub ubo_size: i32,
}

impl MaterialGenerator{
    #[allow(dead_code)]
    pub fn new(uniforms: Vec<Uniformstruct>) -> MaterialGenerator{
        let mut shaderbeg = "".to_string();
        let mut plus = 0;
        for i in 0..uniforms.len(){
            shaderbeg += &uniforms[i].label;
            shaderbeg += &": ";
            match uniforms[i].usage as i32 {
                0 => {shaderbeg += &"float,"; plus+=1},
                1 => {shaderbeg += &"vec2f,"; plus+=2},
                2 => {shaderbeg += &"vec3f,"; plus+=3},
                3 => {shaderbeg += &"vec4f,"; plus+=4},
                4 => {shaderbeg += &"mat4x4<f32>,"; plus+=16},
                _ => {},
            }
        }
        shaderbeg += &"}";
        let mut vertex_code = shaderbeg.to_string();
        vertex_code += "
        @group(0) @binding(0) var<uniform> ubo: uniforms;
        struct OUT{
          @builtin(position) position: vec4f,
          @location(0) uv: vec2f,
          @location(1) vp: vec4f,
          @location(2) smv: vec4f,
          @location(3) norm: vec3f,
          @location(4) tangent: vec3f,
          @location(5) bitangent: vec3f,
          @location(6) rp: vec4f,
        }
        @vertex
        fn vertexMain(@location(0) pos: vec3f, @location(1) uv: vec2f, @location(2) n: vec3f, @location(3) t: vec3f, @location(4) bt: vec3f) -> OUT {
          var out: OUT;
          out.position = ubo.mvp[ubo.eng.a] * ubo.model * vec4f(pos, 1.0);
          out.uv = vec2f(uv.x, 1.0-uv.y);
          out.vp = ubo.model * vec4f(pos, 1.0);
          out.norm = n;
          out.tangent = t;
          out.bitangent = bt;
          return out;
        }
        ";
        let mut fragment_code = shaderbeg.to_string();
        fragment_code += "
        @group(0) @binding(0) var<uniform> ubo: uniforms;

        @group(0) @binding(1) var mySampler: sampler;

        @group(0) @binding(2) var myTexture: texture_2d_array<f32>;

        @group(0) @binding(3) var shadowMap: texture_depth_2d_array;

        @group(0) @binding(4) var mycube: texture_cube<f32>;

        @group(0) @binding(5) var shadowSampler: sampler_comparison;

        struct OUT{
          @builtin(position) position: vec4f,
          @location(0) uv: vec2f,
          @location(1) vp: vec4f,
          @location(2) smv: vec4f,
          @location(3) norm: vec3f,
          @location(4) tangent: vec3f,
          @location(5) bitangent: vec3f,
          @location(6) rp: vec4f,
        }

        struct GBufferOutput {
          @location(0) albedo : vec4f,
          @location(1) material : vec4f,
          @location(2) normal : vec4f,
          @location(3) position : vec4f,
        }

        @fragment
        fn fragmentMain(in: OUT) -> GBufferOutput {
          var output: GBufferOutput;
          output.albedo = textureSample(myTexture, mySampler, in.uv, 0).rgba;
          output.material.r = textureSample(myTexture, mySampler, in.uv, 1).r;
          output.material.g = textureSample(myTexture, mySampler, in.uv, 2).r;
          output.material.b = textureSample(myTexture, mySampler, in.uv, 3).r;
          output.normal = vec4f(in.norm, 1.0);
          output.position = in.vp;
          return output;
        }
        ";
        MaterialGenerator{
            vertex_shader: vertex_code,
            fragment_shader: fragment_code.to_string(),
            magfilter: "linear".to_string(),
            minfilter: "linear".to_string(),
            culling_mode: "none".to_string(),
            culling_mode_shadow: "none".to_string(),
            repeat_mode: "repeat".to_string(),
            shaderbeg: shaderbeg,
            ubo_size: plus,
        }
    }
    #[allow(dead_code)]
    pub fn gen_post_vertex(&mut self){
      self.vertex_shader = self.shaderbeg.to_string();
      self.vertex_shader += "
      @group(0) @binding(0) var<uniform> ubo: uniforms;

      struct OUT{
        @builtin(position) position: vec4f,
        @location(0) uv: vec2f,
      }

      @vertex
      fn vertexMain(@location(0) pos: vec3f, @location(1) uv: vec2f, @location(2) n: vec3f, @location(3) t: vec3f, @location(4) bt: vec3f) -> OUT {
        var out: OUT;
        out.position = vec4f(pos.xyz, 1);
        out.uv = uv;
        return out;
      }";
    }
    #[allow(dead_code)]
    pub fn gen_vertex(&mut self){
      self.vertex_shader = self.shaderbeg.to_string();
      self.vertex_shader += "
      @group(0) @binding(0) var<uniform> ubo: uniforms;
      struct OUT{
        @builtin(position) position: vec4f,
        @location(0) uv: vec2f,
        @location(1) vp: vec4f,
        @location(2) smv: vec4f,
        @location(3) norm: vec3f,
        @location(4) tangent: vec3f,
        @location(5) bitangent: vec3f,
        @location(6) rp: vec4f,
      }
      @vertex
      fn vertexMain(@location(0) pos: vec3f, @location(1) uv: vec2f, @location(2) n: vec3f, @location(3) t: vec3f, @location(4) bt: vec3f) -> OUT {
        var out: OUT;
        out.position = ubo.mvp[i32(ubo.eng.a)] * ubo.model * vec4f(pos, 1.0);
        out.uv = vec2f(uv.x, 1.0-uv.y);
        out.vp = ubo.model * vec4f(pos, 1.0);
        out.norm = n;
        out.tangent = t;
        out.bitangent = bt;
        out.rp = vec4f(pos, 1.0);
        return out;
      }";
    }
    #[allow(dead_code)]
    pub fn gen_vertex_beg(&mut self){
      self.vertex_shader = self.shaderbeg.to_string();
      self.vertex_shader += "
      @group(0) @binding(0) var<uniform> ubo: uniforms;
      struct OUT{
        @builtin(position) position: vec4f,
        @location(0) uv: vec2f,
        @location(1) vp: vec4f,
        @location(2) smv: vec4f,
        @location(3) norm: vec3f,
        @location(4) tangent: vec3f,
        @location(5) bitangent: vec3f,
        @location(6) rp: vec4f,
      }
      @vertex
      fn vertexMain(@location(0) pos: vec3f, @location(1) uv: vec2f, @location(2) n: vec3f, @location(3) t: vec3f, @location(4) bt: vec3f) -> OUT {
        var out: OUT;";
    }
    #[allow(dead_code)]
    pub fn gen_fragpost_beg(&mut self){
      self.fragment_shader = self.shaderbeg.to_string();
      self.fragment_shader += "
      @group(0) @binding(0) var<uniform> ubo: uniforms;

      @group(0) @binding(1) var mySampler: sampler;

      @group(0) @binding(2) var myTexture: texture_2d_array<f32>;

      @group(0) @binding(3) var shadowMap: texture_depth_2d_array;

      @group(0) @binding(4) var mainMap: texture_2d_array<f32>;

      @group(0) @binding(5) var matMap: texture_2d_array<f32>;

      @group(0) @binding(6) var normalMap: texture_2d_array<f32>;

      @group(0) @binding(7) var positionMap: texture_2d_array<f32>;

      @group(0) @binding(8) var mainDepthMap: texture_depth_2d_array;

      @group(0) @binding(9) var shadowSampler: sampler_comparison;

      struct OUT{
        @location(0) uv: vec2f,
      }

      const PI = 3.14159265359;

      fn DistributionGGX(N: vec3f, H: vec3f, roughness: f32) -> f32{
        let a      = roughness*roughness;
        let a2     = a*a;
        let NdotH  = max(dot(N, H), 0.0);
        let NdotH2 = NdotH*NdotH;
        let num   = a2;
        var denom = (NdotH2 * (a2 - 1.0) + 1.0);
        denom = PI * denom * denom;
        return num / denom;
      }

      fn GeometrySchlickGGX(NdotV: f32, roughness: f32) -> f32{
        let r = (roughness + 1.0);
        let k = (r*r) / 8.0;
        let num   = NdotV;
        let denom = NdotV * (1.0 - k) + k;
        return num / denom;
      }

      fn GeometrySmith(N: vec3f, V: vec3f, L: vec3f, roughness: f32) -> f32{
        let NdotV = max(dot(N, V), 0.0);
        let NdotL = max(dot(N, L), 0.0);
        let ggx2  = GeometrySchlickGGX(NdotV, roughness);
        let ggx1  = GeometrySchlickGGX(NdotL, roughness);
        return ggx1 * ggx2;
      }

      fn fresnelSchlick(cosTheta: f32, F0: vec3f) -> vec3f{
        return F0 + (1.0 - F0) * pow(clamp(1.0 - cosTheta, 0.0, 1.0), 5.0);
      }

      fn shcalc(WorldPos: vec3f, bias: f32) -> f32{
        var visibility = 0.0;
        for (var i = 0; i < LIGHTMN; i++) {
          let smv = ubo.smvp[i] * vec4f(WorldPos, 1.0);
          let proj = vec3f((smv.x / smv.w)*0.5+0.5, (smv.y / smv.w)*-0.5+0.5, smv.z / smv.w);
          let oneOverShadowDepthTextureSize = 1.0 / 1000.0;
          for (var y = -1; y <= 1; y++) {
            for (var x = -1; x <= 1; x++) {
              let offset = vec2f(vec2(x, y)) * oneOverShadowDepthTextureSize;
              visibility += textureSampleCompare(
                shadowMap, shadowSampler,
                proj.xy + offset, i, proj.z - bias
              );
            }
          }
        }
        return visibility / 9.0;
      }

      fn PBR(norm: vec3f, albedo: vec3f, shadow: f32, metallic: f32, roughness: f32, ao: f32, WorldPos: vec3f) -> vec3f{
        let N = normalize(norm);
        let V = normalize(ubo.pos[0].xyz - WorldPos);

        var F0 = vec3f(0.04); 
        F0 = mix(F0, albedo, metallic);

        var Lo = vec3f(0.0);

        for(var i = 0; i < LIGHTN; i++) {
          let L = normalize(ubo.lpos[i].xyz - WorldPos);
          let H = normalize(V + L);
          let distance    = length(ubo.lpos[i].xyz - WorldPos);
          let attenuation = 1.0 / (distance * distance);
          let radiance     = (ubo.lcolor[i].xyz) * attenuation;        

          let NDF = DistributionGGX(N, H, roughness);        
          let G   = GeometrySmith(N, V, L, roughness);      
          let F   = fresnelSchlick(max(dot(H, V), 0.0), F0);       

          let kS = F;
          var kD = vec3(1.0) - kS;
          kD *= 1.0 - metallic;	  

          let numerator    = NDF * G * F;
          let denominator = 4.0 * max(dot(N, V), 0.0) * max(dot(N, L), 0.0) + 0.0001;
          let specular     = numerator / denominator;  

          let NdotL = max(dot(N, L), 0.0);                
          Lo += (kD * albedo / PI + specular) * radiance * NdotL; 
        }

        let ambient = vec3(0.001) * albedo * ao;
        var color = ambient + shadow * Lo;

        color = color / (color + vec3(1.0));
        color = pow(color, vec3(1.0/2.2));  
        return color;
      }

      @fragment
      fn fragmentMain(in: OUT) -> @location(0) vec4f {";
    }
    #[allow(dead_code)]
    pub fn gen_frag_beg(&mut self){
      self.fragment_shader = self.shaderbeg.to_string();
      self.fragment_shader += "
      @group(0) @binding(0) var<uniform> ubo: uniforms;

      @group(0) @binding(1) var mySampler: sampler;

      @group(0) @binding(2) var myTexture: texture_2d_array<f32>;

      @group(0) @binding(3) var shadowMap: texture_depth_2d_array;

      @group(0) @binding(4) var mycube: texture_cube<f32>;

      @group(0) @binding(5) var shadowSampler: sampler_comparison;

      struct OUT{
        @builtin(position) position: vec4f,
        @location(0) uv: vec2f,
        @location(1) vp: vec4f,
        @location(2) smv: vec4f,
        @location(3) norm: vec3f,
        @location(4) tangent: vec3f,
        @location(5) bitangent: vec3f,
        @location(6) rp: vec4f,
      }

      struct GBufferOutput {
        @location(0) albedo : vec4f,
        @location(1) material : vec4f,
        @location(2) normal : vec4f,
        @location(3) position : vec4f,
      }

      @fragment
      fn fragmentMain(in: OUT) -> GBufferOutput {
        var output: GBufferOutput;";
    }
    #[allow(dead_code)]
    pub fn gen_frag_end(&mut self){
      self.fragment_shader += "}";
    }
    #[allow(dead_code)]
    pub fn gen_vert_end(&mut self){
      self.vertex_shader += "}";
    }
    #[allow(dead_code)]
    pub fn generate_material(&mut self, tex_ids: String, cube_ids: String) -> Material{
        Material{
            tex_ids: tex_ids,
            cube_ids: cube_ids,
            vertex_shader: self.vertex_shader.to_string(),
            fragment_shader: self.fragment_shader.to_string(),
            uniend: self.shaderbeg.to_owned(),
            magfilter: self.magfilter.to_string(),
            minfilter: self.minfilter.to_string(),
            culling_mode: self.culling_mode.to_string(),
            culling_mode_shadow: self.culling_mode_shadow.to_string(),
            repeat_mode: self.repeat_mode.to_string(),
            ubo_size: self.ubo_size,
        }
    }
}