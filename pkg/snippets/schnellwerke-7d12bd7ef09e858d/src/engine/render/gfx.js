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
        this.mainpasslayers = 2;
        this.mainPassTexture = [
            device.createTexture({
                label: "main1",
                format: "rgba16float",
                size: [this.canvas.width*this.rscale, this.canvas.height*this.rscale, this.mainpasslayers],
                usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.RENDER_ATTACHMENT | GPUTextureUsage.COPY_SRC,
            }),
            device.createTexture({
                label: "main2",
                format: "rgba16float",
                size: [this.canvas.width*this.rscale, this.canvas.height*this.rscale, this.mainpasslayers],
                usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.RENDER_ATTACHMENT | GPUTextureUsage.COPY_SRC,
            })
        ];

        this.matPassTexture = [
            device.createTexture({
                label: "material2",
                format: "rgba16float",
                size: [this.canvas.width*this.rscale, this.canvas.height*this.rscale, this.mainpasslayers],
                usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.RENDER_ATTACHMENT | GPUTextureUsage.COPY_SRC,
            }),
            device.createTexture({
                label: "material2",
                format: "rgba16float",
                size: [this.canvas.width*this.rscale, this.canvas.height*this.rscale, this.mainpasslayers],
                usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.RENDER_ATTACHMENT | GPUTextureUsage.COPY_SRC,
            })
        ];
        this.normalPassTexture = [
            device.createTexture({
                label: "normal1",
                format: "rgba16float",
                size: [this.canvas.width*this.rscale, this.canvas.height*this.rscale, this.mainpasslayers],
                usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.RENDER_ATTACHMENT | GPUTextureUsage.COPY_SRC,
            }),
            device.createTexture({
                label: "normal2",
                format: "rgba16float",
                size: [this.canvas.width*this.rscale, this.canvas.height*this.rscale, this.mainpasslayers],
                usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.RENDER_ATTACHMENT | GPUTextureUsage.COPY_SRC,
            })
        ];
        this.positionPassTexture = [
            device.createTexture({
                label: "position1",
                format: "rgba16float",
                size: [this.canvas.width*this.rscale, this.canvas.height*this.rscale, this.mainpasslayers],
                usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.RENDER_ATTACHMENT | GPUTextureUsage.COPY_SRC,
            }),
            device.createTexture({
                label: "position2",
                format: "rgba16float",
                size: [this.canvas.width*this.rscale, this.canvas.height*this.rscale, this.mainpasslayers],
                usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.RENDER_ATTACHMENT | GPUTextureUsage.COPY_SRC,
            })
        ];

        this.mainPassDepthTexture = [
            device.createTexture({
                label: "maindepth1",
                format: "depth24plus",
                size: [Number(this.canvas.width*this.rscale), Number(this.canvas.height*this.rscale), this.mainpasslayers],
                usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.RENDER_ATTACHMENT,
            }),
            device.createTexture({
                label: "maindepth2",
                format: "depth24plus",
                size: [Number(this.canvas.width*this.rscale), Number(this.canvas.height*this.rscale), this.mainpasslayers],
                usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.RENDER_ATTACHMENT,
            })
        ];
        this.shadowcount = 2;
        this.shadowTexture = [
            device.createTexture({
                label: "shadow1",
                format: "depth32float",
                size: [this.shadowr, this.shadowr, this.shadowcount],
                usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.RENDER_ATTACHMENT,
            }),
            device.createTexture({
                label: "shadow2",
                format: "depth32float",
                size: [this.shadowr, this.shadowr, this.shadowcount],
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
        this.renderlayers = 1;
        this.rendershadows = 1;
        this.ldtex = [];
        this.ldctex = [];
    }
    gfxgetcanvassizex(){
        return this.canvas.width;
    }
    gfxgetcanvassizey(){
        return this.canvas.height;
    }
    gfxsetrenderscale(renderscale, mainpasslayers){
        if(renderscale !== this.renderscale || this.renderlayers !== mainpasslayers){
            this.mainpasslayers = mainpasslayers;
            this.renderlayers = mainpasslayers;
            if(this.mainpasslayers < 2){
                this.mainpasslayers = 2;
            }
            this.rscale = renderscale;
            this.change = true;
        }
    }
    gfxsetshadowmapres(shadowmapres, shadowmapcnt){
        if(shadowmapres !== this.shadowmapres || this.rendershadows !== shadowmapcnt){
            this.shadowcount = shadowmapcnt;
            this.shadowr = shadowmapres;
            this.rendershadows = shadowmapcnt;
            if(this.shadowcount < 2){
                this.shadowcount = 2;
            }
            this.shadowTexture[Number(!this.currentworkingbufferssh)] = device.createTexture({
                format: "depth32float",
                size: [this.shadowr, this.shadowr, this.shadowcount],
                usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.RENDER_ATTACHMENT,
            });
            this.changesh = true;
        }
    }
    gfxbeginpass(lop, dlop){
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
    gfxbeginmainpass(lop, dlop, layer){
        this.passbeg = true;
        this.inpost = false;
        this.isshadowpass = false;
        this.pass = this.encoder.beginRenderPass({
            colorAttachments: [{
               view: this.mainPassTexture[Number(this.currentworkingbuffers)].createView({
                    dimension: "2d",
                    baseArrayLayer: layer,
               }),
               clearValue: { r: 0, g: 0, b: 0, a: 1 },
               loadOp: lop,
               storeOp: "store",
            },{
                view: this.matPassTexture[Number(this.currentworkingbuffers)].createView({
                     dimension: "2d",
                     baseArrayLayer: layer,
                }),
                clearValue: { r: 0, g: 0, b: 0, a: 1 },
                loadOp: lop,
                storeOp: "store",
            },{
                view: this.normalPassTexture[Number(this.currentworkingbuffers)].createView({
                     dimension: "2d",
                     baseArrayLayer: layer,
                }),
                clearValue: { r: 0, g: 0, b: 0, a: 1 },
                loadOp: lop,
                storeOp: "store",
            },{
                view: this.positionPassTexture[Number(this.currentworkingbuffers)].createView({
                     dimension: "2d",
                     baseArrayLayer: layer,
                }),
                clearValue: { r: 0, g: 0, b: 0, a: 1 },
                loadOp: lop,
                storeOp: "store",
            }],
            depthStencilAttachment: {
                view: this.mainPassDepthTexture[Number(this.currentworkingbuffers)].createView({
                    dimension: "2d",
                    baseArrayLayer: layer,
                }),
                depthClearValue: 1.0,
                depthLoadOp: dlop,
                depthStoreOp: "store",
            }
        });
    }
    gfxbeginshadowpass(dlop, layer){
        this.passbeg = true;
        this.inpost = false;
        this.isshadowpass = true;
        this.pass = this.encoder.beginRenderPass({
            colorAttachments: [],
            depthStencilAttachment: {
                view: this.shadowTexture[Number(this.currentworkingbufferssh)].createView({
                    dimension: "2d",
                    baseArrayLayer: layer,
                }),
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
    gfxcheckchange(){
        if(this.canvas.offsetWidth !== this.canvas.width || this.canvas.offsetHeight !== this.canvas.height || this.change){
            console.log("Gfxrender: changing working buffers from " + Number(this.currentworkingbuffers) + " to " + Number(!this.currentworkingbuffers));
            this.depthTexture[Number(!this.currentworkingbuffers)].destroy();
            this.depthTexture[Number(!this.currentworkingbuffers)] = device.createTexture({
                label: "d",
                format: "depth24plus",
                size: [this.canvas.offsetWidth, this.canvas.offsetHeight],
                usage: GPUTextureUsage.RENDER_ATTACHMENT,
            });
            this.mainPassTexture[Number(!this.currentworkingbuffers)].destroy();
            this.mainPassTexture[Number(!this.currentworkingbuffers)] = device.createTexture({
                label: "m",
                format: "rgba16float",
                size: [this.canvas.offsetWidth*this.rscale, this.canvas.offsetHeight*this.rscale, this.mainpasslayers],
                usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.RENDER_ATTACHMENT | GPUTextureUsage.COPY_SRC,
            });
            this.matPassTexture[Number(!this.currentworkingbuffers)].destroy();
            this.matPassTexture[Number(!this.currentworkingbuffers)] = device.createTexture({
                label: "mat",
                format: "rgba16float",
                size: [this.canvas.offsetWidth*this.rscale, this.canvas.offsetHeight*this.rscale, this.mainpasslayers],
                usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.RENDER_ATTACHMENT | GPUTextureUsage.COPY_SRC,
            });
            this.normalPassTexture[Number(!this.currentworkingbuffers)].destroy();
            this.normalPassTexture[Number(!this.currentworkingbuffers)] = device.createTexture({
                label: "normal",
                format: "rgba16float",
                size: [this.canvas.offsetWidth*this.rscale, this.canvas.offsetHeight*this.rscale, this.mainpasslayers],
                usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.RENDER_ATTACHMENT | GPUTextureUsage.COPY_SRC,
            });
            this.positionPassTexture[Number(!this.currentworkingbuffers)].destroy();
            this.positionPassTexture[Number(!this.currentworkingbuffers)] = device.createTexture({
                label: "position",
                format: "rgba16float",
                size: [this.canvas.offsetWidth*this.rscale, this.canvas.offsetHeight*this.rscale, this.mainpasslayers],
                usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.RENDER_ATTACHMENT | GPUTextureUsage.COPY_SRC,
            });
            this.mainPassDepthTexture[Number(!this.currentworkingbuffers)].destroy();
            this.mainPassDepthTexture[Number(!this.currentworkingbuffers)] = device.createTexture({
                label: "md",
                format: "depth24plus",
                size: [this.canvas.offsetWidth*this.rscale, this.canvas.offsetHeight*this.rscale, this.mainpasslayers],
                usage:  GPUTextureUsage.TEXTURE_BINDING |  GPUTextureUsage.RENDER_ATTACHMENT,
            });
            this.currentworkingbuffers = !this.currentworkingbuffers;
            console.log("Gfxrender: canvas resized from: x="+this.canvas.width+" to x="+this.canvas.offsetWidth+", from y="+this.canvas.height+" to y="+this.canvas.offsetHeight);
            this.canvas.width = this.canvas.offsetWidth;
            this.canvas.height = this.canvas.offsetHeight;
            this.change = false
        }
        if(this.changesh){
            this.currentworkingbufferssh = !this.currentworkingbufferssh;
            this.changesh = false;
        }
    }
    gfxfinishrender(){
        device.queue.submit([this.encoder.finish()]);
        this.encoder = device.createCommandEncoder();
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
                [Math.max(Math.floor(this.mippsres[i-1][0]/2), 1.0), Math.max(Math.floor(this.mippsres[i-1][1]/2), 1.0)],
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
    preparesbg(i){
        this.sbindGroup[i] = device.createBindGroup({
            layout: this.shadowpipeline.getBindGroupLayout(0),
            entries: [
                { 
                    binding: 0, 
                    resource: { 
                        buffer: this.uniformBuffers[i][this.currentubo] 
                    }
                },
            ],
        });
    }
    createshpipeline(shadowvertexcode, cullmode){
        this.vertexshadercode = device.createShaderModule({
            code: shadowvertexcode
        });
        this.shadowpipeline = device.createRenderPipeline({
            layout: device.createPipelineLayout({
                bindGroupLayouts: [this.shadowbindGroupLayout],
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
        this.sbindGroup = [device.createBindGroup({
            layout: this.shadowpipeline.getBindGroupLayout(0),
            entries: [
                { 
                    binding: 0, 
                    resource: { 
                        buffer: this.uniformBuffers[0][this.currentubo] 
                    }
                },
            ],
        })];
    }
    preparesh(shadowvertexcode, cullmode){
        this.shadowbindGroupLayout = device.createBindGroupLayout({
            entries: [
              {
                binding: 0,
                visibility: GPUShaderStage.VERTEX | GPUShaderStage.FRAGMENT,
                buffer: {},
              },
            ],
        });
        this.createshpipeline(shadowvertexcode, cullmode);
    }
    queuepipeline(svertexcode, vertexcode, fragmentcode, cullmode, shcullmode){
        this.newsvc = svertexcode;
        this.newvc = vertexcode;
        this.newfc = fragmentcode;
        this.cullmq = cullmode;
        this.shcullmq = shcullmode;
        this.reqpl = true;
    }
    createpipeline(gfx, vertexcode, fragmentcode, cullmode){
        this.vertexcode = device.createShaderModule({
            code: vertexcode
        });
        this.fragmentcode = device.createShaderModule({
            code: fragmentcode
        });
        if(this.usage === 4){
            this.postpipeline = device.createRenderPipeline({
                layout: device.createPipelineLayout({
                    bindGroupLayouts: [this.bindGroupLayout],
                }),
                vertex: {
                  module: this.vertexcode,
                  entryPoint: "vertexMain",
                  buffers: [
                    this.vertexBufferLayout,
                    this.uvBufferLayout,
                    this.nBufferLayout,
                    this.tBufferLayout,
                    this.btBufferLayout,
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
        }else{
            this.pipeline = device.createRenderPipeline({
                label: "mainPipeline",
                layout: device.createPipelineLayout({
                    bindGroupLayouts: [this.bindGroupLayout],
                }),
                vertex: {
                  module: this.vertexcode,
                  entryPoint: "vertexMain",
                  buffers: [
                    this.vertexBufferLayout,
                    this.uvBufferLayout,
                    this.nBufferLayout,
                    this.tBufferLayout,
                    this.btBufferLayout,
                ]
                },
                fragment: {
                  module: this.fragmentcode,
                  entryPoint: "fragmentMain",
                  targets: [{ 
                    format: "rgba16float",
                    //blend: {
                    //    color: {
                    //      srcFactor: 'one',
                    //      dstFactor: 'one-minus-src-alpha'
                    //    },
                    //    alpha: {
                    //      srcFactor: 'one',
                    //      dstFactor: 'one-minus-src-alpha'
                    //    },
                    //},
                  },{
                    format: "rgba16float",
                  },{
                    format: "rgba16float",
                  },{
                    format: "rgba16float",
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
        }
    }
    preparemainrender(vertexcode, fragmentcode, texid, cubeid, gfx, magfilter, minfilter, cullmode, repeatmode){
        this.bindGroupLayout = device.createBindGroupLayout({
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
                    viewDimension: "2d-array",
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
        this.createpipeline(gfx, vertexcode, fragmentcode, cullmode);
        this.sampler = device.createSampler({
            magFilter: magfilter,
            minFilter: minfilter,
            mipmapFilter: magfilter,
            addressModeU: repeatmode,
            addressModeV: repeatmode,
            addressModeW: repeatmode,
        });

        this.colorid = -1;
        for(var i = 0; i != gfx.ldtex.length; i+=1){
            if(gfx.ldtex[i].label == texid){
                this.colorid = i;
                break;
            }
        }
        if(this.colorid == -1){
            this.colorid = gfx.ldtex.length;
            const ids = texid.split(";");
            if(ids.length <= 1 && ids[0].length === 0){
                gfx.ldtex.push(device.createTexture({
                    label: texid,
                    size: [2, 2, 2],
                    dimension: "2d",
                    format: 'rgba8unorm',
                    usage:
                      GPUTextureUsage.TEXTURE_BINDING |
                      GPUTextureUsage.COPY_DST |
                      GPUTextureUsage.RENDER_ATTACHMENT,
                }));
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
                            texture: gfx.ldtex[this.colorid],
                        },
                        textureData,
                        { bytesPerRow: 8 },
                        { width: 2, height: 2 },
                    );
                }
            }else{
                this.genMips(ids[0]);
                gfx.ldtex.push(device.createTexture({
                    label: texid,
                    size: [document.getElementById(ids[0]).width, document.getElementById(ids[0]).height, ids.length+1],
                    mipLevelCount: this.mippsres.length,
                    dimension: "2d",
                    format: 'rgba8unorm',
                    usage:
                      GPUTextureUsage.TEXTURE_BINDING |
                      GPUTextureUsage.COPY_DST |
                      GPUTextureUsage.RENDER_ATTACHMENT,
                }));
                for(let i = 0; i < ids.length; i++){
                    this.genMips(ids[i]);
                    for(var m = 0; m < this.mippsres.length; m+=1){
                        device.queue.writeTexture(
                            {
                                origin: [0, 0, i],
                                texture: gfx.ldtex[this.colorid],
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
        }

        this.cubeid = -1;
        for(var i = 0; i != gfx.ldctex.length; i+=1){
            if(gfx.ldctex[i].label == cubeid){
                this.cubeid = i;
                break;
            }
        }
        if(this.cubeid == -1){
            const cds = cubeid.split(";");
            this.cubeid = gfx.ldctex.length;
            if(cds.length != 6){
                gfx.ldctex.push(device.createTexture({
                    label: cubeid,
                    size: [2, 2, 6],
                    dimension: "2d",
                    format: 'rgba8unorm',
                    usage:
                      GPUTextureUsage.TEXTURE_BINDING |
                      GPUTextureUsage.COPY_DST |
                      GPUTextureUsage.RENDER_ATTACHMENT,
                }));
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
                            texture: gfx.ldctex[this.cubeid],
                        },
                        textureData,
                        { bytesPerRow: 8 },
                        { width: 2, height: 2 },
                    );
                }
            }else{
                gfx.ldctex.push(device.createTexture({
                    label: cubeid,
                    size: [document.getElementById(cds[0]).width, document.getElementById(cds[0]).height, cds.length],
                    dimension: "2d",
                    format: 'rgba8unorm',
                    usage:
                      GPUTextureUsage.TEXTURE_BINDING |
                      GPUTextureUsage.COPY_DST |
                      GPUTextureUsage.RENDER_ATTACHMENT,
                }));
                for(let i = 0; i < cds.length; i++){
                    device.queue.writeTexture(
                        {
                            origin: [0, 0, i],
                            texture: gfx.ldctex[this.cubeid],
                        },
                        this.getPixels(cds[i]),
                        { bytesPerRow: 4*document.getElementById(cds[0]).width },
                        { width: document.getElementById(cds[0]).width, height: document.getElementById(cds[0]).height },
                    );
                }
            }
        }

        this.bindGroup = [device.createBindGroup({
            label: "mainBindGroup",
            layout: this.pipeline.getBindGroupLayout(0),
            entries: [
                { 
                    binding: 0, 
                    resource: { 
                        buffer: this.uniformBuffers[0][this.currentubo] 
                    }},
                {
                    binding: 1,
                    resource: this.sampler
                },
                {
                    binding: 2,
                    resource: gfx.ldtex[this.colorid].createView()
                },
                {
                    binding: 3,
                    resource: gfx.shadowTexture[Number(gfx.currentworkingbufferssh)].createView()
                },
                {
                    binding: 4,
                    resource: gfx.ldctex[this.cubeid].createView({
                        dimension: 'cube',
                    })
                },
                {
                    binding: 5,
                    resource: device.createSampler({
                        addressModeU: "clamp-to-edge",
                        addressModeV: "clamp-to-edge",
                        addressModeW: "clamp-to-edge",
                        magFilter: "linear",
                        minFilter: "linear",
                        compare: 'less',
                    }),
                },
            ],
        })];
    }
    preparpostrender(vertexcode, fragmentcode, texid, gfx, magfilter, minfilter, repeatmode){
        this.bindGroupLayout = device.createBindGroupLayout({
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
                    viewDimension: "2d-array",
                    sampleType: 'depth',
                  },
              },
              {
                binding: 4,
                visibility: GPUShaderStage.FRAGMENT,
                texture: {
                    viewDimension: "2d-array",
                },
              },
              {
                binding: 5,
                visibility: GPUShaderStage.FRAGMENT,
                texture: {
                    viewDimension: "2d-array",
                },
              },
              {
                binding: 6,
                visibility: GPUShaderStage.FRAGMENT,
                texture: {
                    viewDimension: "2d-array",
                },
              },
              {
                binding: 7,
                visibility: GPUShaderStage.FRAGMENT,
                texture: {
                    viewDimension: "2d-array",
                },
              },
              {
                binding: 8,
                visibility: GPUShaderStage.FRAGMENT,
                texture: {
                    viewDimension: "2d-array",
                    sampleType: 'depth',
                  },
              },
              {
                binding: 9,
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

        this.colorid = -1;
        this.cubeid = -1;
        for(var i = 0; i != gfx.ldtex.length; i+=1){
            if(gfx.ldtex[i].label == texid){
                this.colorid = i;
                break;
            }
        }
        if(this.colorid == -1){
            this.colorid = gfx.ldtex.length;
            const ids = texid.split(";");
            if(ids.length <= 1 && ids[0].length === 0){
                gfx.ldtex.push(device.createTexture({
                    label: texid,
                    size: [2, 2, 2],
                    dimension: "2d",
                    format: 'rgba8unorm',
                    usage:
                      GPUTextureUsage.TEXTURE_BINDING |
                      GPUTextureUsage.COPY_DST |
                      GPUTextureUsage.RENDER_ATTACHMENT,
                }));
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
                            texture: gfx.ldtex[this.colorid],
                        },
                        textureData,
                        { bytesPerRow: 8 },
                        { width: 2, height: 2 },
                    );
                }
            }else{
                this.genMips(ids[0]);
                gfx.ldtex.push(device.createTexture({
                    label: texid,
                    size: [document.getElementById(ids[0]).width, document.getElementById(ids[0]).height, ids.length+1],
                    mipLevelCount: this.mippsres.length,
                    dimension: "2d",
                    format: 'rgba8unorm',
                    usage:
                      GPUTextureUsage.TEXTURE_BINDING |
                      GPUTextureUsage.COPY_DST |
                      GPUTextureUsage.RENDER_ATTACHMENT,
                }));
                for(let i = 0; i < ids.length; i++){
                    this.genMips(ids[i]);
                    for(var m = 0; m < this.mippsres.length; m+=1){
                        device.queue.writeTexture(
                            {
                                origin: [0, 0, i],
                                texture: gfx.ldtex[this.colorid],
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
        }

        this.createpipeline(gfx, vertexcode, fragmentcode, "none");
        this.postbindGroup = device.createBindGroup({
            layout: this.postpipeline.getBindGroupLayout(0),
            entries: [
                { 
                    binding: 0, 
                    resource: { 
                        buffer: this.uniformBuffers[0][this.currentubo] 
                    }},
                {
                    binding: 1,
                    resource: this.sampler
                },
                {
                    binding: 2,
                    resource: gfx.ldtex[this.colorid].createView()
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
                    resource: gfx.matPassTexture[Number(gfx.currentworkingbuffers)].createView()
                },
                {
                    binding: 6,
                    resource: gfx.normalPassTexture[Number(gfx.currentworkingbuffers)].createView()
                },
                {
                    binding: 7,
                    resource: gfx.positionPassTexture[Number(gfx.currentworkingbuffers)].createView()
                },
                {
                    binding: 8,
                    resource: gfx.mainPassDepthTexture[Number(gfx.currentworkingbuffers)].createView()
                },
                {
                    binding: 9,
                    resource: device.createSampler({
                        addressModeU: "clamp-to-edge",
                        addressModeV: "clamp-to-edge",
                        addressModeW: "clamp-to-edge",
                        magFilter: "linear",
                        minFilter: "linear",
                        compare: 'less',
                    }),
                },
            ],
        });
    }
    createub(ubol){
        this.currentubo = Number(!this.currentubo);
        for(var i = 0; i != this.uniformBuffers.length; i+=1){
            this.uniformBuffers[i][this.currentubo].destroy();
            this.uniformBuffers[i][this.currentubo] = device.createBuffer({
                size: ubol*4,
                usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
                label: "bf" + this.currentubo,
            });
        }
    }
    constructor(gfx, vertices, uv, normals, tang, bitang, lenght, vertexcode, shadowvertexcode, fragmentcode, ubol, texid, cubeid, magfilter, minfilter, cullMode, shcullMode, repeatmode, usage){
        this.usage = usage;
        this.lenght = lenght;
        this.ubol = ubol;
        this.ubo = new Float32Array(ubol);
        this.currentubo = 0;
        this.currentpipe = 0;
        this.uniformBuffers = [[device.createBuffer({
            size: ubol*4,
            usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
            label: "bf0",
        }), device.createBuffer({
            size: ubol*4,
            usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
            label: "bf1",
        })]];
        this.vertexBuffer = device.createBuffer({
            size: 12*lenght,
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
        this.btBuffer = device.createBuffer({
            size: 12*lenght,
            usage: GPUBufferUsage.VERTEX | GPUBufferUsage.COPY_DST,
        });
        device.queue.writeBuffer(this.vertexBuffer, 0, vertices);
        device.queue.writeBuffer(this.uvBuffer, 0, uv);
        device.queue.writeBuffer(this.nBuffer, 0, normals);
        device.queue.writeBuffer(this.tBuffer, 0, tang);
        device.queue.writeBuffer(this.btBuffer, 0, bitang);
        this.vertexBufferLayout = {
            arrayStride: 12,
            attributes: [{
              format: "float32x3",
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
        this.btBufferLayout = {
            arrayStride: 12,
            attributes: [{
              format: "float32x3",
              offset: 0,
              shaderLocation: 4,
            }],
        };
        if(usage === 1 || usage === 2){
            this.preparemainrender(vertexcode, fragmentcode, texid, cubeid, gfx, magfilter, minfilter, cullMode, repeatmode);
        }
        if(usage === 4){
            this.preparpostrender(vertexcode, fragmentcode, texid, gfx, magfilter, minfilter, repeatmode);
        }
        this.preparesh(shadowvertexcode, shcullMode);
        this.recbuf = false;
        this.newsvc = "";
        this.newvc = "";
        this.newfc = "";
        this.cullmq = "";
        this.shcullmq = "";
        this.reqpl = false;
        this.render = true;
        console.log("Gfxmesh: colorid="+this.colorid+" cubeid="+this.cubeid);
    }
    will_render(render){
        this.render = render;
    }
    recpostg(gfx){
        this.postbindGroup = device.createBindGroup({
            layout: this.postpipeline.getBindGroupLayout(0),
            entries: [
                { 
                    binding: 0, 
                    resource: { 
                        buffer: this.uniformBuffers[0][this.currentubo]  
                    }},
                {
                    binding: 1,
                    resource: this.sampler
                },
                {
                    binding: 2,
                    resource: gfx.ldtex[this.colorid].createView()
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
                    resource: gfx.matPassTexture[Number(gfx.currentworkingbuffers)].createView()
                },
                {
                    binding: 6,
                    resource: gfx.normalPassTexture[Number(gfx.currentworkingbuffers)].createView()
                },
                {
                    binding: 7,
                    resource: gfx.positionPassTexture[Number(gfx.currentworkingbuffers)].createView()
                },
                {
                    binding: 8,
                    resource: gfx.mainPassDepthTexture[Number(gfx.currentworkingbuffers)].createView()
                },
                {
                    binding: 9,
                    resource: device.createSampler({
                      addressModeU: "clamp-to-edge",
                      addressModeV: "clamp-to-edge",
                      addressModeW: "clamp-to-edge",
                      magFilter: "linear",
                      minFilter: "linear",
                      compare: 'less',
                    }),
                },
            ],
        });
    }
    recg(gfx, i){
        this.bindGroup[i] = device.createBindGroup({
            label: "mainBindGroup",
            layout: this.pipeline.getBindGroupLayout(0),
            entries: [
                { 
                    binding: 0, 
                    resource: { 
                        buffer: this.uniformBuffers[i][this.currentubo] 
                    }},
                {
                    binding: 1,
                    resource: this.sampler
                },
                {
                    binding: 2,
                    resource: gfx.ldtex[this.colorid].createView()
                },
                {
                    binding: 3,
                    resource: gfx.shadowTexture[Number(gfx.currentworkingbufferssh)].createView()
                },
                {
                    binding: 4,
                    resource: gfx.ldctex[this.cubeid].createView({
                        dimension: 'cube',
                    })
                },
                {
                    binding: 5,
                    resource: device.createSampler({
                        addressModeU: "clamp-to-edge",
                        addressModeV: "clamp-to-edge",
                        addressModeW: "clamp-to-edge",
                        magFilter: "linear",
                        minFilter: "linear",
                        compare: 'less',
                    }),
                },
            ],
        });
    }
    set_ubo(uniformValues){
        if(uniformValues.length !== this.ubo.length){
            this.recbuf = true;
        }
        this.ubo = uniformValues;
    }
    draw(gfx, i){
        if(this.render){
            if(i+1 > this.uniformBuffers.length){
                this.uniformBuffers.push([device.createBuffer({
                    size: this.ubo.length*4,
                    usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
                    label: "bf0",
                }), device.createBuffer({
                    size: this.ubo.length*4,
                    usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
                    label: "bf1",
                })]);
            }
            this.ubo[0] = Math.floor(gfx.canvas.width*gfx.rscale);
            this.ubo[1] = Math.floor(gfx.canvas.height*gfx.rscale);
            this.ubo[2] = gfx.shadowmapres;
            this.ubo[3] = i;
            device.queue.writeBuffer(this.uniformBuffers[i][this.currentubo], 0, this.ubo);
            if(gfx.isshadowpass){
                if(this.usage === 1 || this.usage === 3){
                    this.preparesbg(i);
                    gfx.pass.setPipeline(this.shadowpipeline);
                    gfx.pass.setBindGroup(0, this.sbindGroup[i]);
                    gfx.pass.setVertexBuffer(0, this.vertexBuffer);
                    gfx.pass.draw(this.lenght);
                }
            }else{
                if(gfx.inpost && this.usage == 4){
                    this.recpostg(gfx);
                    gfx.pass.setPipeline(this.postpipeline);
                    gfx.pass.setBindGroup(0, this.postbindGroup);
                    gfx.pass.setVertexBuffer(0, this.vertexBuffer);
                    gfx.pass.setVertexBuffer(1, this.uvBuffer);
                    gfx.pass.setVertexBuffer(2, this.nBuffer);
                    gfx.pass.setVertexBuffer(3, this.tBuffer);
                    gfx.pass.setVertexBuffer(4, this.btBuffer);
                    gfx.pass.draw(this.lenght);
                }
                if (!gfx.inpost && (this.usage == 1 || this.usage == 2)){
                    this.recg(gfx, i);
                    gfx.pass.setPipeline(this.pipeline);
                    gfx.pass.setBindGroup(0, this.bindGroup[i]);
                    gfx.pass.setVertexBuffer(0, this.vertexBuffer);
                    gfx.pass.setVertexBuffer(1, this.uvBuffer);
                    gfx.pass.setVertexBuffer(2, this.nBuffer);
                    gfx.pass.setVertexBuffer(3, this.tBuffer);
                    gfx.pass.setVertexBuffer(4, this.btBuffer);
                    gfx.pass.draw(this.lenght);
                }
            }
        }
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

export class Jsloop{
    constructor(gfxr){
        this.gfxr = gfxr;
        this.gfxms = [];
    }
    drawloop(){
        let self = this;
        function fr(){
            for(var i = 0; i != self.gfxms.length; i+=1){
                if(self.gfxms[i].recbuf){
                    self.gfxms[i].createub(self.gfxms[i].ubo.length);
                    self.gfxms[i].recbuf = false;
                }
            }
            for(var i = 0; i != self.gfxms.length; i+=1){
                if(self.gfxms[i].reqpl){
                    self.gfxms[i].createpipeline(self.gfxr, self.gfxms[i].newvc, self.gfxms[i].newfc, self.gfxms[i].cullmq);
                    self.gfxms[i].createshpipeline(self.gfxms[i].newsvc, self.gfxms[i].shcullmq);
                    self.gfxms[i].reqpl = false;
                }
            }
            self.gfxr.gfxcheckchange();
            for(var i = 0; i !== self.gfxr.rendershadows; i += 1){
                self.gfxr.gfxbeginshadowpass("clear", i);
                for(var b = 0; b != self.gfxms.length; b+=1){
                    self.gfxms[b].draw(self.gfxr, i);
                }
                self.gfxr.gfxendpass();
            }
            for(var i = 0; i !== self.gfxr.renderlayers; i += 1){
                self.gfxr.gfxbeginmainpass("clear", "clear", i);
                for(var b = 0; b != self.gfxms.length; b+=1){
                    self.gfxms[b].draw(self.gfxr, i);
                }
                self.gfxr.gfxendpass();
            }
            self.gfxr.gfxbeginpass("clear", "clear");
            for(var i = 0; i != self.gfxms.length; i+=1){
                self.gfxms[i].draw(self.gfxr, 0);
            }
            self.gfxr.gfxendpass();
            self.gfxr.gfxfinishrender();
            requestAnimationFrame(fr);
        }
        requestAnimationFrame(fr);
    }
    push_mesh(mesh, index){
        this.gfxms[index] = mesh;
    }
    set_render(ren){
        this.gfxr = ren;
    }
}

export function snlll(fun, to){
    function ll(){
        fun();
        setTimeout(ll, to);
    }
    ll();
}