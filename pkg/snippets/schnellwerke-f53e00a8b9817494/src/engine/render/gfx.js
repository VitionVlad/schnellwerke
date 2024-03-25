if (!navigator.gpu) {
    throw new Error("WebGPU not supported on this browser.");
}

const adapter = await navigator.gpu.requestAdapter();
if (!adapter) {
  throw new Error("No appropriate GPUAdapter found.");
}

const device = await adapter.requestDevice();

export class Gfxrender{
    constructor(canvasid, renderscale, shadowmapres){
        this.rscale = renderscale;
        this.shadowr = shadowmapres;
        this.canvas = document.getElementById(canvasid);
        this.canvas.width = this.canvas.offsetWidth;
        this.canvas.height = this.canvas.offsetHeight; 
        this.context = this.canvas.getContext("webgpu");
        this.canvasFormat = navigator.gpu.getPreferredCanvasFormat();
        this.context.configure({
          device: device,
          format: this.canvasFormat,
        });
        console.log("Gfxrender: canvas resolution is: " + this.canvas.width + " " + this.canvas.height);
        this.depthTexture = [
            device.createTexture({
                label: "depth1",
                format: "depth24plus",
                size: [this.canvas.width, this.canvas.height],
                usage: GPUTextureUsage.RENDER_ATTACHMENT,
            }),
            device.createTexture({
                label: "depth2",
                format: "depth24plus",
                size: [this.canvas.width, this.canvas.height],
                usage: GPUTextureUsage.RENDER_ATTACHMENT,
            })
        ];
        this.oldresx = this.canvas.width;
        this.oldresy = this.canvas.height;
        this.mainPassTexture = [
            device.createTexture({
                label: "main1",
                format: "rgba16float",
                size: [this.canvas.width*this.rscale, this.canvas.height*this.rscale],
                usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.RENDER_ATTACHMENT,
            }),
            device.createTexture({
                label: "main2",
                format: "rgba16float",
                size: [this.canvas.width*this.rscale, this.canvas.height*this.rscale],
                usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.RENDER_ATTACHMENT,
            })
        ];
        this.mainPassDepthTexture = [
            device.createTexture({
                label: "maindepth1",
                format: "depth24plus",
                size: [Number(this.canvas.width*this.rscale), Number(this.canvas.height*this.rscale)],
                usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.RENDER_ATTACHMENT,
            }),
            device.createTexture({
                label: "maindepth2",
                format: "depth24plus",
                size: [Number(this.canvas.width*this.rscale), Number(this.canvas.height*this.rscale)],
                usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.RENDER_ATTACHMENT,
            })
        ];
        this.shadowTexture = [
            device.createTexture({
                label: "shadow1",
                format: "depth32float",
                size: [this.shadowr, this.shadowr],
                usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.RENDER_ATTACHMENT,
            }),
            device.createTexture({
                label: "shadow2",
                format: "depth32float",
                size: [this.shadowr, this.shadowr],
                usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.RENDER_ATTACHMENT,
            })
        ];
        this.encoder = device.createCommandEncoder();
        this.isshadowpass = false;
        this.inpost = false;
        this.passbeg = false;
        this.currentworkingbuffers = false;
        this.currentworkingbufferssh = false;
        this.change = false;
        this.changesh = false;
        this.rb = false;
        this.exect = performance.now();
        this.fexect = performance.now() - this.exect;
    }
    gfxgetcanvassizex(){
        return this.canvas.width;
    }
    gfxgetcanvassizey(){
        return this.canvas.height;
    }
    gfxsetrenderscale(renderscale){
        this.rscale = renderscale;
        this.change = true;
    }
    gfxsetshadowmapres(shadowmapres){
        this.shadowr = shadowmapres;
        this.shadowTexture[Number(!this.currentworkingbufferssh)] = device.createTexture({
            format: "depth32float",
            size: [this.shadowr, this.shadowr],
            usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.RENDER_ATTACHMENT,
        });
        this.changesh = true;
    }
    gfxbeginpass(lop, dlop){
        if(this.rb === false){
            this.rb = true;
            this.fexect = performance.now() - this.exect;
            this.exect = performance.now();
        }
        this.passbeg = true;
        this.inpost = true;
        this.isshadowpass = false;
        this.pass = this.encoder.beginRenderPass({
            colorAttachments: [{
               view: this.context.getCurrentTexture().createView(),
               clearValue: { r: 0, g: 0, b: 0, a: 1 },
               loadOp: lop,
               storeOp: "store",
            }],
            depthStencilAttachment: {
                view: this.depthTexture[Number(this.currentworkingbuffers)].createView(),
                depthClearValue: 1.0,
                depthLoadOp: dlop,
                depthStoreOp: "store",
            }
        });
    }
    gfxbeginmainpass(lop, dlop){
        if(this.rb === false){
            this.rb = true;
            this.fexect = performance.now() - this.exect;
            this.exect = performance.now();
        }
        this.passbeg = true;
        this.inpost = false;
        this.isshadowpass = false;
        this.pass = this.encoder.beginRenderPass({
            colorAttachments: [{
               view: this.mainPassTexture[Number(this.currentworkingbuffers)].createView(),
               clearValue: { r: 0, g: 0, b: 0, a: 1 },
               loadOp: lop,
               storeOp: "store",
            }],
            depthStencilAttachment: {
                view: this.mainPassDepthTexture[Number(this.currentworkingbuffers)].createView(),
                depthClearValue: 1.0,
                depthLoadOp: dlop,
                depthStoreOp: "store",
            }
        });
    }
    gfxbeginshadowpass(dlop){
        if(this.rb === false){
            this.rb = true;
            this.fexect = performance.now() - this.exect;
            this.exect = performance.now();
        }
        this.passbeg = true;
        this.inpost = false;
        this.isshadowpass = true;
        this.pass = this.encoder.beginRenderPass({
            colorAttachments: [],
            depthStencilAttachment: {
                view: this.shadowTexture[Number(this.currentworkingbufferssh)].createView(),
                depthClearValue: 1.0,
                depthLoadOp: dlop,
                depthStoreOp: "store",
            }
        });
    }
    gfxendpass(){
        if(this.passbeg){
            this.pass.end();
            this.passbeg = false;
        }
    }
    gfxfinishrender(){
        device.queue.submit([this.encoder.finish()]);
        this.encoder = device.createCommandEncoder();
        this.canvas.width = this.canvas.offsetWidth;
        this.canvas.height = this.canvas.offsetHeight;

        if(this.oldresx != this.canvas.width || this.oldresy != this.canvas.height || this.change){
            console.log("Gfxrender: changing working buffers from " + Number(this.currentworkingbuffers) + " to " + Number(!this.currentworkingbuffers));
            this.depthTexture[Number(!this.currentworkingbuffers)].destroy();
            this.depthTexture[Number(!this.currentworkingbuffers)] = device.createTexture({
                label: "d",
                format: "depth24plus",
                size: [this.canvas.width, this.canvas.height],
                usage: GPUTextureUsage.RENDER_ATTACHMENT,
            });
            this.mainPassTexture[Number(this.currentworkingbuffers)].destroy();
            this.mainPassTexture[Number(!this.currentworkingbuffers)] = device.createTexture({
                label: "m",
                format: "rgba16float",
                size: [this.canvas.width*this.rscale, this.canvas.height*this.rscale],
                usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.RENDER_ATTACHMENT,
            });
            this.mainPassDepthTexture[Number(!this.currentworkingbuffers)].destroy();
            this.mainPassDepthTexture[Number(!this.currentworkingbuffers)] = device.createTexture({
                label: "md",
                format: "depth24plus",
                size: [this.canvas.width*this.rscale, this.canvas.height*this.rscale],
                usage:  GPUTextureUsage.TEXTURE_BINDING |  GPUTextureUsage.RENDER_ATTACHMENT,
            });
            this.currentworkingbuffers = !this.currentworkingbuffers;
            console.log("Gfxrender: canvas resized from: x="+this.oldresx+" to x="+this.canvas.width+" from y="+this.oldresy+" to y="+this.canvas.height);
            this.oldresx = this.canvas.width;
            this.oldresy = this.canvas.height;
            this.change = false
        }
        if(this.changesh){
            this.currentworkingbufferssh = !this.currentworkingbufferssh;
            this.changesh = false;
        }
        this.rb = false;
    }
    gfxgetexectime(){
        return this.fexect;
    }
}

export class Gfxmesh{
    getPixels(id) {
        var canvas = document.createElement('canvas');
        var context = canvas.getContext('2d');
        canvas.width = document.getElementById(id).width;
        canvas.height = document.getElementById(id).height;
        context.drawImage(document.getElementById(id), 0, 0);
        return context.getImageData(0, 0, document.getElementById(id).width, document.getElementById(id).height).data;
    }
    genMips(id){
        this.mippsres = [
            [document.getElementById(id).width, document.getElementById(id).height],
        ];
        this.mipimages = [
            new Uint8Array(this.getPixels(id)),
        ];
        for(var i = 1; this.mippsres[i-1][0] != 1 || this.mippsres[i-1][1] != 1; i+=1){
            this.mippsres.push(
                [Math.floor(this.mippsres[i-1][0]/2), Math.floor(this.mippsres[i-1][1]/2)],
            );
            this.mipimages.push(new Uint8Array(this.mippsres[i][0]*this.mippsres[i][1]*4));
            for(var y = 0; y != this.mippsres[i][1]; y+=1){
                for(var x = 0; x != this.mippsres[i][0]*4; x+=4){
                    this.mipimages[i][y*this.mippsres[i][0]*4+x] =   (this.mipimages[i-1][(y*2)*this.mippsres[i-1][0]*4+(x*2)] + this.mipimages[i-1][(y*2)*this.mippsres[i-1][0]*4+(x*2+4)] + this.mipimages[i-1][(y*2+1)*this.mippsres[i-1][0]*4+(x*2)] + this.mipimages[i-1][(y*2+1)*this.mippsres[i-1][0]*4+(x*2+4)])/4;
                    this.mipimages[i][y*this.mippsres[i][0]*4+x+1] = (this.mipimages[i-1][(y*2)*this.mippsres[i-1][0]*4+(x*2+1)] + this.mipimages[i-1][(y*2)*this.mippsres[i-1][0]*4+(x*2+5)] + this.mipimages[i-1][(y*2+1)*this.mippsres[i-1][0]*4+(x*2+1)] + this.mipimages[i-1][(y*2+1)*this.mippsres[i-1][0]*4+(x*2+5)])/4;
                    this.mipimages[i][y*this.mippsres[i][0]*4+x+2] = (this.mipimages[i-1][(y*2)*this.mippsres[i-1][0]*4+(x*2+2)] + this.mipimages[i-1][(y*2)*this.mippsres[i-1][0]*4+(x*2+6)] + this.mipimages[i-1][(y*2+1)*this.mippsres[i-1][0]*4+(x*2+2)] + this.mipimages[i-1][(y*2+1)*this.mippsres[i-1][0]*4+(x*2+6)])/4;
                    this.mipimages[i][y*this.mippsres[i][0]*4+x+3] = (this.mipimages[i-1][(y*2)*this.mippsres[i-1][0]*4+(x*2+3)] + this.mipimages[i-1][(y*2)*this.mippsres[i-1][0]*4+(x*2+7)] + this.mipimages[i-1][(y*2+1)*this.mippsres[i-1][0]*4+(x*2+3)] + this.mipimages[i-1][(y*2+1)*this.mippsres[i-1][0]*4+(x*2+7)])/4;
                }
            }
        }
    }
    preparesh(shadowvertexcode, cullmode){
        this.vertexshadercode = device.createShaderModule({
            code: shadowvertexcode
        });
        const shadowbindGroupLayout = device.createBindGroupLayout({
            entries: [
              {
                binding: 0,
                visibility: GPUShaderStage.VERTEX | GPUShaderStage.FRAGMENT,
                buffer: {},
              },
            ],
        });
        this.shadowpipeline = device.createRenderPipeline({
            layout: device.createPipelineLayout({
                bindGroupLayouts: [shadowbindGroupLayout],
            }),
            vertex: {
              module: this.vertexshadercode,
              entryPoint: "vertexMain",
              buffers: [
                this.vertexBufferLayout,
            ]
            },
            depthStencil: {
                depthWriteEnabled: true,
                depthCompare: 'less-equal',
                format: 'depth32float',
            },
            primitive: {
                cullMode: cullmode
            },
        });
        this.sbindGroup = device.createBindGroup({
            layout: this.shadowpipeline.getBindGroupLayout(0),
            entries: [
                { 
                    binding: 0, 
                    resource: { 
                        buffer: this.uniformBuffer 
                    }
                },
            ],
        });
    }
    preparemainrender(vertexcode, fragmentcode, texid, cubeid, gfx, magfilter, minfilter, cullmode, repeatmode){
        this.vertexcode = device.createShaderModule({
            code: vertexcode
        });
        this.fragmentcode = device.createShaderModule({
            code: fragmentcode
        });
        const bindGroupLayout = device.createBindGroupLayout({
            label: "mainGroupLayout",
            entries: [
              {
                binding: 0,
                visibility: GPUShaderStage.VERTEX | GPUShaderStage.FRAGMENT,
                buffer: {},
              },
              {
                binding: 1,
                visibility: GPUShaderStage.FRAGMENT,
                sampler: {},
              },
              {
                binding: 2,
                visibility: GPUShaderStage.FRAGMENT,
                texture: {
                    viewDimension: "2d-array",
                },
              },
              {
                binding: 3,
                visibility: GPUShaderStage.FRAGMENT,
                texture: {
                    sampleType: 'depth',
                  },
              },
              {
                binding: 4,
                visibility: GPUShaderStage.FRAGMENT,
                texture: {
                    viewDimension: "cube",
                },
              },
              {
                binding: 5,
                visibility: GPUShaderStage.FRAGMENT,
                sampler: {
                    type: 'comparison',
                },
              },
            ],
          });
        this.pipeline = device.createRenderPipeline({
            label: "mainPipeline",
            layout: device.createPipelineLayout({
                bindGroupLayouts: [bindGroupLayout],
            }),
            vertex: {
              module: this.vertexcode,
              entryPoint: "vertexMain",
              buffers: [
                this.vertexBufferLayout,
                this.uvBufferLayout,
                this.nBufferLayout,
                this.tBufferLayout,
            ]
            },
            fragment: {
              module: this.fragmentcode,
              entryPoint: "fragmentMain",
              targets: [{
                format: "rgba16float",
                blend: {
                    color: {
                      srcFactor: 'one',
                      dstFactor: 'one-minus-src-alpha'
                    },
                    alpha: {
                      srcFactor: 'one',
                      dstFactor: 'one-minus-src-alpha'
                    },
                },
              }]
            },
            depthStencil: {
                depthWriteEnabled: true,
                depthCompare: 'less-equal',
                format: 'depth24plus',
            },
            primitive: {
                cullMode: cullmode
            },
        });

        this.sampler = device.createSampler({
            magFilter: magfilter,
            minFilter: minfilter,
            mipmapFilter: magfilter,
            addressModeU: repeatmode,
            addressModeV: repeatmode,
            addressModeW: repeatmode,
        });

        const ids = texid.split(";");
        if(ids.length <= 1 && ids[0].length === 0){
            this.colortex = device.createTexture({
                label: "colorTex",
                size: [2, 2, 2],
                dimension: "2d",
                format: 'rgba8unorm',
                usage:
                  GPUTextureUsage.TEXTURE_BINDING |
                  GPUTextureUsage.COPY_DST |
                  GPUTextureUsage.RENDER_ATTACHMENT,
            });
            const textureData = new Uint8Array([
                160, 32, 240, 256,
                0, 0, 0, 256,
                0, 0, 0, 256,
                160, 32, 240, 256
              ].flat());
            for(let i = 0; i < 2; i++){
                device.queue.writeTexture(
                    {
                        origin: [0, 0, i],
                        texture: this.colortex,
                    },
                    textureData,
                    { bytesPerRow: 8 },
                    { width: 2, height: 2 },
                );
            }
        }else{
            this.genMips(ids[0]);
            this.colortex = device.createTexture({
                label: "colorTex",
                size: [document.getElementById(ids[0]).width, document.getElementById(ids[0]).height, ids.length+1],
                mipLevelCount: this.mippsres.length,
                dimension: "2d",
                format: 'rgba8unorm',
                usage:
                  GPUTextureUsage.TEXTURE_BINDING |
                  GPUTextureUsage.COPY_DST |
                  GPUTextureUsage.RENDER_ATTACHMENT,
            });
            for(let i = 0; i < ids.length; i++){
                this.genMips(ids[i]);
                for(var m = 0; m < this.mippsres.length; m+=1){
                    device.queue.writeTexture(
                        {
                            origin: [0, 0, i],
                            texture: this.colortex,
                            mipLevel: m,
                        },
                        this.mipimages[m],
                        { bytesPerRow: this.mippsres[m][0]*4 },
                        { width: this.mippsres[m][0], height: this.mippsres[m][1] },
                    );
                }
                this.mipimages = [];
                this.mippsres = [];
            }
        }

        const cds = cubeid.split(";");
        if(cds.length != 6){
            this.cubemap = device.createTexture({
                label: "cubeMap",
                size: [2, 2, 6],
                dimension: "2d",
                format: 'rgba8unorm',
                usage:
                  GPUTextureUsage.TEXTURE_BINDING |
                  GPUTextureUsage.COPY_DST |
                  GPUTextureUsage.RENDER_ATTACHMENT,
            });
            const textureData = new Uint8Array([
                160, 32, 240, 256,
                0, 0, 0, 256,
                0, 0, 0, 256,
                160, 32, 240, 256
              ].flat());
            for(let i = 0; i < 6; i++){
                device.queue.writeTexture(
                    {
                        origin: [0, 0, i],
                        texture: this.cubemap,
                    },
                    textureData,
                    { bytesPerRow: 8 },
                    { width: 2, height: 2 },
                );
            }
        }else{
            this.cubemap = device.createTexture({
                label: "cubeMap",
                size: [document.getElementById(cds[0]).width, document.getElementById(cds[0]).height, cds.length],
                dimension: "2d",
                format: 'rgba8unorm',
                usage:
                  GPUTextureUsage.TEXTURE_BINDING |
                  GPUTextureUsage.COPY_DST |
                  GPUTextureUsage.RENDER_ATTACHMENT,
            });
            for(let i = 0; i < cds.length; i++){
                device.queue.copyExternalImageToTexture(
                    { source: document.getElementById(cds[i]) },
                    { 
                        texture: this.cubemap,
                        origin: [0, 0, i]
                    },
                    [document.getElementById(cds[i]).width, document.getElementById(cds[i]).height]
                );
            }
        }

        this.bindGroup = device.createBindGroup({
            label: "mainBindGroup",
            layout: this.pipeline.getBindGroupLayout(0),
            entries: [
                { 
                    binding: 0, 
                    resource: { 
                        buffer: this.uniformBuffer 
                    }},
                {
                    binding: 1,
                    resource: this.sampler
                },
                {
                    binding: 2,
                    resource: this.colortex.createView()
                },
                {
                    binding: 3,
                    resource: gfx.shadowTexture[Number(gfx.currentworkingbufferssh)].createView()
                },
                {
                    binding: 4,
                    resource: this.cubemap.createView({
                        dimension: 'cube',
                    })
                },
                {
                    binding: 5,
                    resource: device.createSampler({
                      compare: 'less',
                    }),
                },
            ],
        });
    }
    preparpostrender(vertexcode, fragmentcode, texid, gfx, magfilter, minfilter, repeatmode){
        this.vertexcode = device.createShaderModule({
            code: vertexcode
        });
        this.fragmentcode = device.createShaderModule({
            code: fragmentcode
        });
        const bindpostGroupLayout = device.createBindGroupLayout({
            entries: [
              {
                binding: 0,
                visibility: GPUShaderStage.VERTEX | GPUShaderStage.FRAGMENT,
                buffer: {},
              },
              {
                binding: 1,
                visibility: GPUShaderStage.FRAGMENT,
                sampler: {},
              },
              {
                binding: 2,
                visibility: GPUShaderStage.FRAGMENT,
                texture: {
                    viewDimension: "2d-array",
                },
              },
              {
                binding: 3,
                visibility: GPUShaderStage.FRAGMENT,
                texture: {
                    sampleType: 'depth',
                  },
              },
              {
                binding: 4,
                visibility: GPUShaderStage.FRAGMENT,
                texture: {},
              },
              {
                binding: 5,
                visibility: GPUShaderStage.FRAGMENT,
                texture: {
                    sampleType: 'depth',
                  },
              },
              {
                binding: 6,
                visibility: GPUShaderStage.FRAGMENT,
                sampler: {
                    type: 'comparison',
                },
              },
            ],
        });

        this.sampler = device.createSampler({
            magFilter: magfilter,
            minFilter: minfilter,
            addressModeU: repeatmode,
            addressModeV: repeatmode,
            addressModeW: repeatmode,
        });

          const ids = texid.split(";");
          if(ids.length <= 1 && ids[0].length === 0){
              this.colortex = device.createTexture({
                  label: "colorTex",
                  size: [2, 2, 2],
                  dimension: "2d",
                  format: 'rgba8unorm',
                  usage:
                    GPUTextureUsage.TEXTURE_BINDING |
                    GPUTextureUsage.COPY_DST |
                    GPUTextureUsage.RENDER_ATTACHMENT,
              });
              const textureData = new Uint8Array([
                  160, 32, 240, 256,
                  0, 0, 0, 256,
                  0, 0, 0, 256,
                  160, 32, 240, 256
                ].flat());
              for(let i = 0; i < 2; i++){
                  device.queue.writeTexture(
                      {
                          origin: [0, 0, i],
                          texture: this.colortex,
                      },
                      textureData,
                      { bytesPerRow: 8 },
                      { width: 2, height: 2 },
                  );
              }
          }else{
              this.colortex = device.createTexture({
                  label: "colorTex",
                  size: [document.getElementById(ids[0]).width, document.getElementById(ids[0]).height, ids.length+1],
                  dimension: "2d",
                  format: 'rgba8unorm',
                  usage:
                    GPUTextureUsage.TEXTURE_BINDING |
                    GPUTextureUsage.COPY_DST |
                    GPUTextureUsage.RENDER_ATTACHMENT,
              });
              for(let i = 0; i < ids.length; i++){
                  device.queue.copyExternalImageToTexture(
                      { source: document.getElementById(ids[i]) },
                      { 
                          texture: this.colortex,
                          origin: [0, 0, i]
                      },
                      [document.getElementById(ids[i]).width, document.getElementById(ids[i]).height]
                  );
              }
          }

        this.postpipeline = device.createRenderPipeline({
            layout: device.createPipelineLayout({
                bindGroupLayouts: [bindpostGroupLayout],
            }),
            vertex: {
              module: this.vertexcode,
              entryPoint: "vertexMain",
              buffers: [
                this.vertexBufferLayout,
                this.uvBufferLayout,
                this.nBufferLayout,
                this.tBufferLayout,
            ]
            },
            fragment: {
              module: this.fragmentcode,
              entryPoint: "fragmentMain",
              targets: [{
                format: gfx.canvasFormat
              }]
            },
            depthStencil: {
                depthWriteEnabled: true,
                depthCompare: 'less-equal',
                format: 'depth24plus',
            },
        });
        this.postbindGroup = device.createBindGroup({
            layout: this.postpipeline.getBindGroupLayout(0),
            entries: [
                { 
                    binding: 0, 
                    resource: { 
                        buffer: this.uniformBuffer 
                    }},
                {
                    binding: 1,
                    resource: this.sampler
                },
                {
                    binding: 2,
                    resource: this.colortex.createView()
                },
                {
                    binding: 3,
                    resource: gfx.shadowTexture[Number(gfx.currentworkingbufferssh)].createView()
                },
                {
                    binding: 4,
                    resource: gfx.mainPassTexture[Number(gfx.currentworkingbuffers)].createView()
                },
                {
                    binding: 5,
                    resource: gfx.mainPassDepthTexture[Number(gfx.currentworkingbuffers)].createView()
                },
                {
                    binding: 6,
                    resource: device.createSampler({
                      compare: 'less',
                    }),
                },
            ],
        });
    }
    constructor(gfx, vertices, uv, normals, tang, lenght, vertexcode, shadowvertexcode, fragmentcode, ubol, texid, cubeid, magfilter, minfilter, cullMode, shcullMode, repeatmode, forpost){
        this.forpost = forpost;
        this.lenght = lenght;
        this.ubol = ubol;
        this.uniformBuffer = device.createBuffer({
            size: ubol,
            usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
        });
        this.vertexBuffer = device.createBuffer({
            size: 16*lenght,
            usage: GPUBufferUsage.VERTEX | GPUBufferUsage.COPY_DST,
        });
        this.uvBuffer = device.createBuffer({
            size: 8*lenght,
            usage: GPUBufferUsage.VERTEX | GPUBufferUsage.COPY_DST,
        });
        this.nBuffer = device.createBuffer({
            size: 12*lenght,
            usage: GPUBufferUsage.VERTEX | GPUBufferUsage.COPY_DST,
        });
        this.tBuffer = device.createBuffer({
            size: 12*lenght,
            usage: GPUBufferUsage.VERTEX | GPUBufferUsage.COPY_DST,
        });
        device.queue.writeBuffer(this.vertexBuffer, 0, vertices);
        device.queue.writeBuffer(this.uvBuffer, 0, uv);
        device.queue.writeBuffer(this.nBuffer, 0, normals);
        device.queue.writeBuffer(this.tBuffer, 0, tang);
        this.vertexBufferLayout = {
            arrayStride: 16,
            attributes: [{
              format: "float32x4",
              offset: 0,
              shaderLocation: 0,
            }],
        };
        this.uvBufferLayout = {
            arrayStride: 8,
            attributes: [{
              format: "float32x2",
              offset: 0,
              shaderLocation: 1,
            }],
        };
        this.nBufferLayout = {
            arrayStride: 12,
            attributes: [{
              format: "float32x3",
              offset: 0,
              shaderLocation: 2,
            }],
        };
        this.tBufferLayout = {
            arrayStride: 12,
            attributes: [{
              format: "float32x3",
              offset: 0,
              shaderLocation: 3,
            }],
        };
        if(!forpost){
            this.preparemainrender(vertexcode, fragmentcode, texid, cubeid, gfx, magfilter, minfilter, cullMode, repeatmode);
        }else{
            this.preparpostrender(vertexcode, fragmentcode, texid, gfx, magfilter, minfilter, repeatmode);
        }
        this.preparesh(shadowvertexcode, shcullMode);
    }
    recpostg(gfx){
        this.postbindGroup = device.createBindGroup({
            layout: this.postpipeline.getBindGroupLayout(0),
            entries: [
                { 
                    binding: 0, 
                    resource: { 
                        buffer: this.uniformBuffer 
                    }},
                {
                    binding: 1,
                    resource: this.sampler
                },
                {
                    binding: 2,
                    resource: this.colortex.createView()
                },
                {
                    binding: 3,
                    resource: gfx.shadowTexture[Number(gfx.currentworkingbufferssh)].createView()
                },
                {
                    binding: 4,
                    resource: gfx.mainPassTexture[Number(gfx.currentworkingbuffers)].createView()
                },
                {
                    binding: 5,
                    resource: gfx.mainPassDepthTexture[Number(gfx.currentworkingbuffers)].createView()
                },
                {
                    binding: 6,
                    resource: device.createSampler({
                      compare: 'less',
                    }),
                },
            ],
        });
    }
    recg(gfx){
        this.bindGroup = device.createBindGroup({
            label: "mainBindGroup",
            layout: this.pipeline.getBindGroupLayout(0),
            entries: [
                { 
                    binding: 0, 
                    resource: { 
                        buffer: this.uniformBuffer 
                    }},
                {
                    binding: 1,
                    resource: this.sampler
                },
                {
                    binding: 2,
                    resource: this.colortex.createView()
                },
                {
                    binding: 3,
                    resource: gfx.shadowTexture[Number(gfx.currentworkingbufferssh)].createView()
                },
                {
                    binding: 4,
                    resource:this.cubemap.createView({
                        dimension: 'cube',
                    })
                },
                {
                    binding: 5,
                    resource: device.createSampler({
                      compare: 'less',
                    }),
                },
            ],
        });
    }
    writenewvertices(vertices){
        device.queue.writeBuffer(this.vertexBuffer, 0, vertices);
    }
    draw(gfx, uniformValues){
        device.queue.writeBuffer(this.uniformBuffer, 0, uniformValues);
        if(gfx.isshadowpass){
            gfx.pass.setPipeline(this.shadowpipeline);
            gfx.pass.setBindGroup(0, this.sbindGroup);
            gfx.pass.setVertexBuffer(0, this.vertexBuffer);
        }else{
            if(gfx.inpost && this.forpost){
                this.recpostg(gfx);
                gfx.pass.setPipeline(this.postpipeline);
                gfx.pass.setBindGroup(0, this.postbindGroup);
                gfx.pass.setVertexBuffer(0, this.vertexBuffer);
                gfx.pass.setVertexBuffer(1, this.uvBuffer);
                gfx.pass.setVertexBuffer(2, this.nBuffer);
                gfx.pass.setVertexBuffer(3, this.tBuffer);
            }
            if (!gfx.inpost && !this.forpost){
                this.recg(gfx);
                gfx.pass.setPipeline(this.pipeline);
                gfx.pass.setBindGroup(0, this.bindGroup);
                gfx.pass.setVertexBuffer(0, this.vertexBuffer);
                gfx.pass.setVertexBuffer(1, this.uvBuffer);
                gfx.pass.setVertexBuffer(2, this.nBuffer);
                gfx.pass.setVertexBuffer(3, this.tBuffer);
            }
        }
        gfx.pass.draw(this.lenght);
    }
}

export class Gpucompute{
    constructor(ibs, obs, code){
        this.is = ibs*4;
        this.os = obs*4;
        const module = device.createShaderModule({
            code: code
        });

        const bindGroupLayout = device.createBindGroupLayout({
            label: "compute group layout",
            entries: [{
              binding: 0,
              visibility: GPUShaderStage.COMPUTE,
              buffer: { type: "read-only-storage"}
            }, {
              binding: 1,
              visibility: GPUShaderStage.COMPUTE,
              buffer: { type: "storage"}
            }]
          });

        this.pipeline = device.createComputePipeline({
            label: 'compute pipeline',
            layout: device.createPipelineLayout({
                bindGroupLayouts: [bindGroupLayout],
            }),
            compute: {
              module: module,
              entryPoint: 'computeMain',
            },
        });

        this.inbuf = device.createBuffer({
            size: ibs*4, 
            usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST
        });

        this.outbuf = device.createBuffer({
            size: obs*4, 
            usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_SRC
        });

        this.outbufcpu = device.createBuffer({
            size: obs*4, 
            mappedAtCreation: false,
            usage: GPUBufferUsage.MAP_READ | GPUBufferUsage.COPY_DST
        });

        this.bindGroup = device.createBindGroup({
            layout: this.pipeline.getBindGroupLayout(0),
            entries: [{ 
                binding: 0, 
                resource: { 
                    buffer: this.inbuf 
                }
            },{ 
                binding: 1, 
                resource: { 
                    buffer: this.outbuf 
                }
            },
            ],
        });
        this.rsoutbuf = new Float32Array(this.obs);
        this.ended = true;
    }
    async execute(inbuf, workgroupcount){
        this.ended = false;
        device.queue.writeBuffer(this.inbuf, 0, inbuf);
        const encoder = device.createCommandEncoder();
        const computePass = encoder.beginComputePass();

        computePass.setPipeline(this.pipeline);
        computePass.setBindGroup(0, this.bindGroup);
        computePass.dispatchWorkgroups(workgroupcount);
        computePass.end();

        encoder.copyBufferToBuffer(this.outbuf, 0, this.outbufcpu, 0, this.os);
        const commandBuffer = encoder.finish();
        device.queue.submit([commandBuffer]);

        await this.outbufcpu.mapAsync(GPUMapMode.READ);

        this.rsoutbuf = new Float32Array(this.outbufcpu.getMappedRange(0, this.os).slice());
        this.outbufcpu.unmap();
        this.ended = true;
    }
    getstate(){
        return this.ended;
    }
    getresult(){
        return this.rsoutbuf;
    }
}