if (!navigator.gpu) {
    throw new Error("WebGPU not supported on this browser.");
}

const adapter = await navigator.gpu.requestAdapter();
if (!adapter) {
  throw new Error("No appropriate GPUAdapter found.");
}

var use_16bit_depth = true;

var deffformat = "rgba32float";

var samplertype = "unfilterable-float";

var samplertextype = "non-filtering";

var mcabps = 64;

if(use_16bit_depth){
    deffformat = "rgba16float";
    samplertype = "float";
    samplertextype = "filtering";
    mcabps = 32;
}

const device = await adapter.requestDevice({
    requiredLimits: {
        maxColorAttachmentBytesPerSample: mcabps,
    },
});

class Gauss{
    constructor(){
        this.handle = Array(0);
        this.models = Array(0);
        this.shaders = Array(0);
        this.meshes = Array(0);
        this.textures = Array(0);
    }
}

var gs = new Gauss();

var pressedk = new Array(100).fill(false);

var mpos = [0, 0];

function key_to_code(key){
    switch(key){
        case "F1":
            return 0;
        case "F2":
            return 1;
        case "F3":
            return 2;
        case "F4":
            return 3;
        case "F5":
            return 4;
        case "F6":
            return 5;
        case "F7":
            return 6;
        case "F8":
            return 7;
        case "F8":
            return 8;
        case "F9":
            return 9;
        case "F10":
            return 10;
        case "F11":
            return 11;
        case "0":
            return 12;
        case "1":
            return 13;
        case "2":
            return 14;
        case "3":
            return 15;
        case "4":
            return 16;
        case "5":
            return 17;
        case "6":
            return 18;
        case "7":
            return 19;
        case "8":
            return 20;
        case "9":
            return 21;
        case "a":
            return 22;
        case "b":
            return 23;
        case "c":
            return 24;
        case "d":
            return 25;
        case "e":
            return 26;
        case "f":
            return 27;
        case "g":
            return 28;
        case "h":
            return 29;
        case "i":
            return 30;
        case "j":
            return 31;
        case "k":
            return 32;
        case "l":
            return 33;
        case "m":
            return 34;
        case "n":
            return 35;
        case "o":
            return 36;
        case "p":
            return 37;
        case "q":
            return 38;
        case "r":
            return 39;
        case "s":
            return 40;
        case "t":
            return 41;
        case "u":
            return 42;
        case "v":
            return 43;
        case "w":
            return 44;
        case "x":
            return 45;
        case "y":
            return 46;
        case "z":
            return 47;
        case "A":
            return 22;
        case "B":
            return 23;
        case "C":
            return 24;
        case "D":
            return 25;
        case "E":
            return 26;
        case "F":
            return 27;
        case "G":
            return 28;
        case "H":
            return 29;
        case "I":
            return 30;
        case "K":
            return 31;
        case "J":
            return 32;
        case "L":
            return 33;
        case "M":
            return 34;
        case "N":
            return 35;
        case "O":
            return 36;
        case "P":
            return 37;
        case "Q":
            return 38;
        case "R":
            return 39;
        case "S":
            return 40;
        case "T":
            return 41;
        case "U":
            return 42;
        case "V":
            return 43;
        case "W":
            return 44;
        case "X":
            return 45;
        case "Y":
            return 46;
        case "Z":
            return 47;
        case "Space":
            return 48;
        case "Escape":
            return 49;
        case "ShiftLeft":
            return 50;
        case "ShiftRight":
            return 50;
        case "ControlLeft":
            return 51;
        case "ControlRight":
            return 51;
        case "ArrowUp":
            return 52;
        case "ArrowLeft":
            return 53;
        case "ArrowDown":
            return 54;
        case "ArrowRight":
            return 55;
        case "Enter":
            return 56;
        case "Backspace":
            return 57;
    }
}

addEventListener("keypress", (event) => { 
    pressedk[key_to_code(event.key)] = true;
})

addEventListener("keyup", (event) => { 
    pressedk[key_to_code(event.key)] = false;
})

document.addEventListener('mousemove', function(event) {
    if(document.pointerLockElement != null){
        mpos[0] += event.movementX;
        mpos[1] += event.movementY;
    }else{
        mpos[0] = event.clientX;
        mpos[1] = event.clientY;
    }
});

document.addEventListener('touchmove', function(e) {
  e.preventDefault(); 
  var touch = e.touches[0]; 
  mpos[0] = touch.clientX;
  mpos[1] = touch.clientY;
});

var msb = [false, false, false];

document.addEventListener("mousedown", (e) => {
  switch (e.button) {
    case 0:
      msb[0] = true;
      break;
    case 1:
      msb[1] = true;
      break;
    case 2:
      msb[2] = true;
      break;
    default:
  }
});

document.addEventListener("touchstart", (e) => {
  msb[0] = true;
});

document.addEventListener("touchend", (e) => {
  msb[0] = false;
});

document.addEventListener("mouseup", (e) => {
  switch (e.button) {
    case 0:
      msb[0] = false;
      break;
    case 1:
      msb[1] = false;
      break;
    case 2:
      msb[2] = false;
      break;
    default:
  }
});

class Gaussh{
    constructor(canvasid){
        this.shadowmapresolution = 1000;
        this.oshadowmapresolution = 1000;
        this.shadowmapcnt = 1;
        this.oshadowmapcnt = 1;

        this.resolutionScale = 1.0;
        this.oresolutionScale = 1.0;
        this.defferedcnt = 1;
        this.odefferedcnt = 1;

        this.canvas = document.getElementById(canvasid);
        if(this.canvas === null){
            alert("Null canvas!");
        }
        this.canvas.width = this.canvas.offsetWidth;
        this.canvas.height = this.canvas.offsetHeight; 
        this.resx = this.canvas.offsetWidth;
        this.resy = this.canvas.offsetHeight; 
        this.oresx = this.canvas.offsetWidth;
        this.oresy = this.canvas.offsetHeight; 
        this.context = this.canvas.getContext("webgpu");
        this.maindepth = device.createTexture({
            label: "depth",
            format: "depth32float",
            size: [this.canvas.width, this.canvas.height],
            usage: GPUTextureUsage.RENDER_ATTACHMENT,
        });

        this.shadowdepth = device.createTexture({
            label: "depth",
            format: "depth32float",
            size: [this.oshadowmapresolution, this.oshadowmapresolution, 2],
            usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.RENDER_ATTACHMENT,
        });

        this.rres = [Math.floor(this.canvas.width*this.resolutionScale), Math.floor(this.canvas.width*this.resolutionScale)];

        this.deffdepth = device.createTexture({
            label: "depth",
            format: "depth32float",
            size: [this.rres[0], this.rres[1], 2],
            usage: GPUTextureUsage.TEXTURE_BINDING |  GPUTextureUsage.RENDER_ATTACHMENT,
        });

        this.deffered = device.createTexture({
            label: "deff",
            format: deffformat,
            size: [this.rres[0], this.rres[1], this.defferedcnt*4],
            usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.RENDER_ATTACHMENT,
        });

        this.canvasFormat = navigator.gpu.getPreferredCanvasFormat();
        this.context.configure({
          device: device,
          format: this.canvasFormat,
        });

        this.deffbuf = [];
        this.shbuf = [];

        for(var i = 0; i < 10; i+=1){
            this.deffbuf.push(device.createBuffer({
              size: 16*4,
              usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
            }));
        }

        this.mdeffbuf = device.createBuffer({
          size: 240*4,
          usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
        });

        for(var i = 0; i < 100; i+=1){
            this.shbuf.push(device.createBuffer({
              size: 16*4,
              usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
            }));
        }

        this.mshbuf = device.createBuffer({
          size: 2400*4,
          usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
        });

        this.deffbf = new Float32Array(240);
        this.shbf = new Float32Array(2400);
        this.frametime = performance.now();
        this.lastt = performance.now();
        this.totalFrames = 0;

        this.reggr = 0;
        this.sreggr = 0;
    }
    startrender(){
        this.frametime = performance.now() - this.lastt;
        this.lastt = performance.now();
        this.totalFrames += 1;
        this.canvas.width = this.canvas.offsetWidth;
        this.canvas.height = this.canvas.offsetHeight; 
        this.resx = this.canvas.offsetWidth;
        this.resy = this.canvas.offsetHeight;
        if(this.oresx !== this.resx || this.oresy !== this.resy || this.resolutionScale !== this.oresolutionScale || this.defferedcnt !== this.odefferedcnt){
            this.oresx = this.canvas.offsetWidth;
            this.oresy = this.canvas.offsetHeight;

            this.oresolutionScale = this.resolutionScale;
            this.odefferedcnt = this.defferedcnt;

            this.maindepth.destroy();
            this.maindepth = device.createTexture({
                label: "depth",
                format: "depth32float",
                size: [this.resx, this.resy],
                usage: GPUTextureUsage.RENDER_ATTACHMENT,
            });

            this.rres = [Math.max(Math.floor(this.canvas.width*this.resolutionScale), 1.0), Math.max(Math.floor(this.canvas.width*this.resolutionScale), 1.0)];

            this.deffdepth.destroy();
            this.deffdepth = device.createTexture({
                label: "deffdepth",
                format: "depth32float",
                size: [this.rres[0], this.rres[1], Math.max(this.defferedcnt, 2)],
                usage: GPUTextureUsage.TEXTURE_BINDING |  GPUTextureUsage.RENDER_ATTACHMENT,
            });

            this.deffered.destroy();
            this.deffered = device.createTexture({
                label: "deff",
                format: deffformat,
                size: [this.rres[0], this.rres[1], this.defferedcnt*4],
                usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.RENDER_ATTACHMENT,
            });
            this.reggr += 1;
        }

        if(this.shadowmapresolution !== this.oshadowmapresolution || this.shadowmapcnt !== this.oshadowmapcnt){
            this.oshadowmapresolution = this.shadowmapresolution;
            this.oshadowmapcnt = this.shadowmapcnt;

            this.shadowdepth.destroy();
            this.shadowdepth = device.createTexture({
                label: "sh",
                format: "depth32float",
                size: [this.oshadowmapresolution, this.oshadowmapresolution, Math.max(this.oshadowmapcnt, 2)],
                usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.RENDER_ATTACHMENT,
            });

            this.sreggr += 1;
        }

        for(var i = 0; i < 10; i+=1){
            device.queue.writeBuffer(this.deffbuf[i], 0, this.deffbf, i*16, 16);
        }

        for(var i = 0; i < 100; i+=1){
            device.queue.writeBuffer(this.shbuf[i], 0, this.shbf, i*16, 16);
        }

        device.queue.writeBuffer(this.mdeffbuf, 0, this.deffbf);

        device.queue.writeBuffer(this.mshbuf, 0, this.shbf);

        this.encoder = device.createCommandEncoder();
    }
    startmainrenderpass(){
        this.pass = this.encoder.beginRenderPass({
          colorAttachments: [{
             view: this.context.getCurrentTexture().createView(),
             loadOp: "clear",
             clearValue: { r: 0, g: 0, b: 0, a: 1 },
             storeOp: "store",
          }],
          depthStencilAttachment: {
                view: this.maindepth.createView({
                    dimension: "2d",
                    baseArrayLayer: 0,
                }),
                depthClearValue: 1.0,
                depthLoadOp: "clear",
                depthStoreOp: "store",
            },
        });
    }
    startshadowpass(i){
        this.pass = this.encoder.beginRenderPass({
          colorAttachments: [],
          depthStencilAttachment: {
                view: this.shadowdepth.createView({
                    dimension: "2d",
                    baseArrayLayer: i,
                }),
                depthClearValue: 1.0,
                depthLoadOp: "clear",
                depthStoreOp: "store",
            },
        });
    }
    startdeffpass(i){
        this.pass = this.encoder.beginRenderPass({
          colorAttachments: [{
             view: this.deffered.createView({
                    dimension: "2d",
                    baseArrayLayer: i*4,
                }),
             loadOp: "clear",
             clearValue: { r: 0, g: 0, b: 0, a: 1 },
             storeOp: "store",
          },
          {
             view: this.deffered.createView({
                    dimension: "2d",
                    baseArrayLayer: i*4+1,
                }),
             loadOp: "clear",
             clearValue: { r: 0, g: 0, b: 0, a: 1 },
             storeOp: "store",
          },
          {
             view: this.deffered.createView({
                    dimension: "2d",
                    baseArrayLayer: i*4+2,
                }),
             loadOp: "clear",
             clearValue: { r: 0, g: 0, b: 0, a: 1 },
             storeOp: "store",
          },
          {
             view: this.deffered.createView({
                    dimension: "2d",
                    baseArrayLayer: i*4+3,
                }),
             loadOp: "clear",
             clearValue: { r: 0, g: 0, b: 0, a: 1 },
             storeOp: "store",
          }],
          depthStencilAttachment: {
                view: this.deffdepth.createView({
                    dimension: "2d",
                    baseArrayLayer: i,
                }),
                depthClearValue: 1.0,
                depthLoadOp: "clear",
                depthStoreOp: "store",
            },
        });
    }
    endrenderpass(){
        this.pass.end();
    }
    endrender(){
        if(!(this.oresx !== this.resx || this.oresy !== this.resy || this.resolutionScale !== this.oresolutionScale || this.defferedcnt !== this.odefferedcnt || this.shadowmapresolution !== this.oshadowmapresolution || this.shadowmapcnt !== this.oshadowmapcnt)){
            const commandBuffer = this.encoder.finish();
            device.queue.submit([commandBuffer]);
        }
    }
}

class Gausshader{
    constructor(vert, frag, shadow, cullmode, shcullmode){
        this.cullmode = cullmode;
        this.shcullmode = shcullmode;
        this.vertshadermodule = device.createShaderModule({
          code: vert,
        });
        this.fragshadermodule = device.createShaderModule({
          code: frag,
        });
        this.shadowshadermodule = device.createShaderModule({
          code: shadow,
        });
    }
}

class Gaussmodel{
    constructor(vertices, uv, normal, tan, ctag){
        this.vcnt = vertices.length/3;
        this.vertexBuffer = device.createBuffer({
          label: "Vertices",
          size: vertices.byteLength,
          usage: GPUBufferUsage.VERTEX | GPUBufferUsage.COPY_DST,
        });
        this.uvBuffer = device.createBuffer({
          label: "UV",
          size: uv.byteLength,
          usage: GPUBufferUsage.VERTEX | GPUBufferUsage.COPY_DST,
        });
        this.normalBuffer = device.createBuffer({
          label: "Normals",
          size: normal.byteLength,
          usage: GPUBufferUsage.VERTEX | GPUBufferUsage.COPY_DST,
        });
        this.tanBuffer = device.createBuffer({
          label: "Tan",
          size: tan.byteLength,
          usage: GPUBufferUsage.VERTEX | GPUBufferUsage.COPY_DST,
        });
        this.cotanBuffer = device.createBuffer({
          label: "CTan",
          size: ctag.byteLength,
          usage: GPUBufferUsage.VERTEX | GPUBufferUsage.COPY_DST,
        });
        device.queue.writeBuffer(this.vertexBuffer, 0, vertices);
        device.queue.writeBuffer(this.uvBuffer, 0, uv);
        device.queue.writeBuffer(this.normalBuffer, 0, normal);
        device.queue.writeBuffer(this.tanBuffer, 0, tan);
        device.queue.writeBuffer(this.cotanBuffer, 0, ctag);
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
        this.normalBufferLayout = {
            arrayStride: 12,
            attributes: [{
                format: "float32x3",
                offset: 0,
                shaderLocation: 2,
            }],
        };
        this.tanBufferLayout = {
            arrayStride: 12,
            attributes: [{
                format: "float32x3",
                offset: 0,
                shaderLocation: 3,
            }],
        };
        this.ctanBufferLayout = {
            arrayStride: 12,
            attributes: [{
                format: "float32x3",
                offset: 0,
                shaderLocation: 4,
            }],
        };
    }
}

class Gausstex{
    genMips(pixels, xs, ys, zs){
        var or = [];
        for(var z = 0; z < zs; z+=1){
            var oi = new Uint8Array(xs*ys*4);
            for(var i = 0; i < ys*xs*4; i+=1){
                oi[i] = pixels[xs*ys*4*z+i];
            }
            or.push(oi);
        }

        for(var z = 0; z < zs; z+=1){
            this.mippsres = [
                [xs, ys],
            ];
            var lmipimages = [or[z]];
            for(var i = 1; this.mippsres[i-1][0] != 1 || this.mippsres[i-1][1] != 1; i+=1){
                this.mippsres[i] = [Math.max(Math.floor(this.mippsres[i-1][0]/2), 1.0), Math.max(Math.floor(this.mippsres[i-1][1]/2), 1.0)];
                lmipimages.push(new Uint8Array(this.mippsres[i][0]*this.mippsres[i][1]*4));
                for(var y = 0; y != this.mippsres[i][1]; y+=1){
                    for(var x = 0; x != this.mippsres[i][0]*4; x+=4){
                        lmipimages[i][y*this.mippsres[i][0]*4+x] =   (lmipimages[i-1][(y*2)*this.mippsres[i-1][0]*4+(x*2)]   + lmipimages[i-1][(y*2)*this.mippsres[i-1][0]*4+(x*2+4)] + lmipimages[i-1][(y*2+1)*this.mippsres[i-1][0]*4+(x*2)]   + lmipimages[i-1][(y*2+1)*this.mippsres[i-1][0]*4+(x*2+4)])/4;
                        lmipimages[i][y*this.mippsres[i][0]*4+x+1] = (lmipimages[i-1][(y*2)*this.mippsres[i-1][0]*4+(x*2+1)] + lmipimages[i-1][(y*2)*this.mippsres[i-1][0]*4+(x*2+5)] + lmipimages[i-1][(y*2+1)*this.mippsres[i-1][0]*4+(x*2+1)] + lmipimages[i-1][(y*2+1)*this.mippsres[i-1][0]*4+(x*2+5)])/4;
                        lmipimages[i][y*this.mippsres[i][0]*4+x+2] = (lmipimages[i-1][(y*2)*this.mippsres[i-1][0]*4+(x*2+2)] + lmipimages[i-1][(y*2)*this.mippsres[i-1][0]*4+(x*2+6)] + lmipimages[i-1][(y*2+1)*this.mippsres[i-1][0]*4+(x*2+2)] + lmipimages[i-1][(y*2+1)*this.mippsres[i-1][0]*4+(x*2+6)])/4;
                        lmipimages[i][y*this.mippsres[i][0]*4+x+3] = (lmipimages[i-1][(y*2)*this.mippsres[i-1][0]*4+(x*2+3)] + lmipimages[i-1][(y*2)*this.mippsres[i-1][0]*4+(x*2+7)] + lmipimages[i-1][(y*2+1)*this.mippsres[i-1][0]*4+(x*2+3)] + lmipimages[i-1][(y*2+1)*this.mippsres[i-1][0]*4+(x*2+7)])/4;
                    }
                }
            }
            this.mipimages.push(lmipimages);
        }
    }
    constructor(pixels, xs, ys, zs){
        this.mipimages = [];
        this.mippsres = [];
        this.genMips(pixels, xs, ys, zs);
        this.tex = device.createTexture({
            label: "texture",
            size: [xs, ys, Math.max(zs, 2)],
            mipLevelCount: this.mippsres.length,
            dimension: "2d",
            format: 'rgba8unorm',
            usage:
              GPUTextureUsage.TEXTURE_BINDING |
              GPUTextureUsage.COPY_DST |
              GPUTextureUsage.RENDER_ATTACHMENT,
        });

        for(var z = 0; z < zs; z+=1){
            for(var m = 0; m < this.mippsres.length; m+=1){
                device.queue.writeTexture(
                    {
                        origin: [0, 0, z],
                        texture: this.tex,
                        mipLevel: m,
                    },
                    this.mipimages[z][m],
                    { bytesPerRow: this.mippsres[m][0]*4 },
                    { width: this.mippsres[m][0], height: this.mippsres[m][1] },
                );
            }
        }
    }
}

class Gausmesh{
    createmp(eh, em, es){
        this.pipeline = device.createRenderPipeline({
          label: "pipeline",
          layout: device.createPipelineLayout({
                    bindGroupLayouts: [this.bindGroupLayout],
                }),
          vertex: {
            module: gs.shaders[es].vertshadermodule,
            entryPoint: "main",
            buffers: [ 
                gs.models[em].vertexBufferLayout,
                gs.models[em].uvBufferLayout,
                gs.models[em].normalBufferLayout,
                gs.models[em].tanBufferLayout,
                gs.models[em].ctanBufferLayout,
            ]
          },
          fragment: {
            module: gs.shaders[es].fragshadermodule,
            entryPoint: "main",
            targets: [{
              format: gs.handle[eh].canvasFormat,
            }]
          },
          depthStencil: {
            depthWriteEnabled: true, 
            depthCompare: 'less-equal',
            format: 'depth32float',
          },
          primitive: {
            cullMode: gs.shaders[es].cullmode,
          }
        });
    }
    createdp(em, es){
        this.deffpipeline = device.createRenderPipeline({
          label: "deffpipeline",
          layout: device.createPipelineLayout({
                    bindGroupLayouts: [this.bindGroupLayout],
                }),
          vertex: {
            module: gs.shaders[es].vertshadermodule,
            entryPoint: "main",
            buffers: [ 
                gs.models[em].vertexBufferLayout,
                gs.models[em].uvBufferLayout,
                gs.models[em].normalBufferLayout,
                gs.models[em].tanBufferLayout,
                gs.models[em].ctanBufferLayout,
            ]
          },
          fragment: {
            module: gs.shaders[es].fragshadermodule,
            entryPoint: "main",
            targets: [{
              format: deffformat,
            },
            {
              format: deffformat,
            },
            {
              format: deffformat,
            },
            {
              format: deffformat,
            }]
          },
          depthStencil: {
            depthWriteEnabled: true, 
            depthCompare: 'less-equal',
            format: 'depth32float',
          },
          primitive: {
            cullMode: gs.shaders[es].cullmode,
          }
        });
    }
    createsh(em, es){
        this.shpipeline = device.createRenderPipeline({
          label: "shpipeline",
          layout: device.createPipelineLayout({
                    bindGroupLayouts: [this.bindGroupLayout],
                }),
          vertex: {
            module: gs.shaders[es].shadowshadermodule,
            entryPoint: "main",
            buffers: [ 
                gs.models[em].vertexBufferLayout,
                gs.models[em].uvBufferLayout,
                gs.models[em].normalBufferLayout,
                gs.models[em].tanBufferLayout,
                gs.models[em].ctanBufferLayout,
            ]
          },
          depthStencil: {
            depthWriteEnabled: true, 
            depthCompare: 'less-equal',
            format: 'depth32float',
          },
          primitive: {
            cullMode: gs.shaders[es].shcullmode,
          }
        });
    }
    constructor(eh, em, es, te, usage){
        this.pipeline = null;
        this.deffpipeline = null;
        this.shpipeline = null;
        this.te = te;
        this.usage = usage;
        this.eh = eh;
        this.draw = 1;
        this.rendercam = -1;
        this.em = em;
        this.es = es;
        this.ubo = new Float32Array(28);
        this.deffbindGroup = [];
        this.shbindGroup = [];
        this.reggr = 0;
        this.sreggr = 0;

        this.uniformBuffer = device.createBuffer({
          size: 28*4,
          usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
        });

        if(usage === 1 || usage === 2 || usage === 3){
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
                  visibility: GPUShaderStage.VERTEX | GPUShaderStage.FRAGMENT,
                  buffer: {},
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
                  sampler: {},
                },
              ],
            });
            if(usage === 1 || usage === 3){
                this.createdp(em, es);
                for(var i = 0; i < 10; i += 1){
                    this.deffbindGroup.push(device.createBindGroup({
                        label: "deffBindGroup",
                        layout: this.deffpipeline.getBindGroupLayout(0),
                        entries: [
                            { 
                                binding: 0, 
                                resource: { 
                                    buffer: gs.handle[eh].deffbuf[i],
                                }},
                            {
                                binding: 1,
                                resource: { 
                                    buffer: this.uniformBuffer
                                },
                            },
                            {
                                binding: 2,
                                resource: gs.textures[te].tex.createView(),
                            },
                            {
                                binding: 3,
                                resource: device.createSampler({
                                    magFilter: "linear",
                                    minFilter: "linear",
                                    mipmapFilter: "linear",
                                    addressModeU: "repeat",
                                    addressModeV: "repeat",
                                    addressModeW: "repeat",
                                }),
                            },
                        ],
                    }));
                }
            }
            if(usage === 2 || usage === 3){
                this.createsh(em, es);
                for(var i = 0; i < 100; i += 1){
                    this.shbindGroup.push(device.createBindGroup({
                        label: "shBindGroup",
                        layout: this.shpipeline.getBindGroupLayout(0),
                        entries: [
                            { 
                                binding: 0, 
                                resource: { 
                                    buffer: gs.handle[eh].shbuf[i],
                                }},
                            {
                                binding: 1,
                                resource: { 
                                    buffer: this.uniformBuffer
                                },
                            },
                            {
                                binding: 2,
                                resource: gs.textures[te].tex.createView(),
                            },
                            {
                                binding: 3,
                                resource: device.createSampler({
                                    magFilter: "linear",
                                    minFilter: "linear",
                                    mipmapFilter: "linear",
                                    addressModeU: "repeat",
                                    addressModeV: "repeat",
                                    addressModeW: "repeat",
                                }),
                            },
                        ],
                    }));
                }
            }
        }else{
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
                  visibility: GPUShaderStage.VERTEX | GPUShaderStage.FRAGMENT,
                  buffer: {},
                },
                {
                  binding: 2,
                  visibility: GPUShaderStage.VERTEX | GPUShaderStage.FRAGMENT,
                  buffer: {},
                },
                {
                  binding: 3,
                  visibility: GPUShaderStage.FRAGMENT,
                  texture: {
                      viewDimension: "2d-array",
                  },
                },
                {
                  binding: 4,
                  visibility: GPUShaderStage.FRAGMENT,
                  texture: {
                      viewDimension: "2d-array",
                      sampleType: samplertype,
                  },
                },
                {
                  binding: 5,
                  visibility: GPUShaderStage.FRAGMENT,
                  texture: {
                      viewDimension: "2d-array",
                      sampleType: 'depth',
                    },
                },
                {
                  binding: 6,
                  visibility: GPUShaderStage.FRAGMENT,
                  texture: {
                      viewDimension: "2d-array",
                      sampleType: 'depth',
                    },
                },
                {
                  binding: 7,
                  visibility: GPUShaderStage.FRAGMENT,
                  sampler: {},
                },
                {
                  binding: 8,
                  visibility: GPUShaderStage.FRAGMENT,
                  sampler: {
                      type: 'comparison',
                  },
                },
                {
                  binding: 9,
                  visibility: GPUShaderStage.FRAGMENT,
                  sampler: {
                    type: samplertextype,
                  },
                },
              ],
            });
            this.createmp(eh, em, es);
            this.bindGroup = device.createBindGroup({
                label: "mainBindGroup",
                layout: this.pipeline.getBindGroupLayout(0),
                entries: [
                    { 
                        binding: 0, 
                        resource: { 
                            buffer: this.uniformBuffer,
                        }},
                    {
                        binding: 1,
                        resource: { 
                            buffer: gs.handle[eh].mshbuf
                        },
                    },
                    {
                        binding: 2,
                        resource: { 
                            buffer: gs.handle[eh].mdeffbuf
                        },
                    },
                    {
                        binding: 3,
                        resource: gs.textures[te].tex.createView(),
                    },
                    {
                        binding: 4,
                        resource: gs.handle[eh].deffered.createView(),
                    },
                    {
                        binding: 5,
                        resource: gs.handle[eh].deffdepth.createView(),
                    },
                    {
                        binding: 6,
                        resource: gs.handle[eh].shadowdepth.createView(),
                    },
                    {
                        binding: 7,
                        resource: device.createSampler({
                            magFilter: "linear",
                            minFilter: "linear",
                            mipmapFilter: "linear",
                            addressModeU: "repeat",
                            addressModeV: "repeat",
                            addressModeW: "repeat",
                        }),
                    },
                    {
                        binding: 8,
                        resource: device.createSampler({
                            addressModeU: "clamp-to-edge",
                            addressModeV: "clamp-to-edge",
                            addressModeW: "clamp-to-edge",
                            magFilter: "linear",
                            minFilter: "linear",
                            compare: 'less',
                        }),
                    },
                    {
                        binding: 9,
                        resource: device.createSampler({
                            magFilter: "nearest",
                            minFilter: "nearest",
                            addressModeU: "repeat",
                            addressModeV: "repeat",
                            addressModeW: "repeat",
                        }),
                    },
                ],
            });
        }
    }
    drawsh(eh, i){
        this.ubo[0] = gs.handle[eh].resx;
        this.ubo[1] = gs.handle[eh].resy;
        this.ubo[2] = gs.handle[eh].shadowmapresolution;
        this.ubo[3] = gs.handle[eh].totalFrames;
        this.ubo[4] = gs.handle[eh].shadowmapcnt;
        this.ubo[5] = gs.handle[eh].resx*gs.handle[eh].resolutionScale;
        this.ubo[6] = gs.handle[eh].resy*gs.handle[eh].resolutionScale;
        device.queue.writeBuffer(this.uniformBuffer, 0, this.ubo);
        gs.handle[eh].pass.setPipeline(this.shpipeline);
        gs.handle[eh].pass.setBindGroup(0, this.shbindGroup[i]);
        gs.handle[eh].pass.setVertexBuffer(0, gs.models[this.em].vertexBuffer);
        gs.handle[eh].pass.setVertexBuffer(1, gs.models[this.em].uvBuffer);
        gs.handle[eh].pass.setVertexBuffer(2, gs.models[this.em].normalBuffer);
        gs.handle[eh].pass.setVertexBuffer(3, gs.models[this.em].tanBuffer);
        gs.handle[eh].pass.setVertexBuffer(4, gs.models[this.em].cotanBuffer);
        gs.handle[eh].pass.draw(gs.models[this.em].vcnt);
    }
    drawdeff(eh, i){
        this.ubo[0] = gs.handle[eh].resx;
        this.ubo[1] = gs.handle[eh].resy;
        this.ubo[2] = gs.handle[eh].shadowmapresolution;
        this.ubo[3] = gs.handle[eh].totalFrames;
        this.ubo[4] = gs.handle[eh].shadowmapcnt;
        this.ubo[5] = gs.handle[eh].resx*gs.handle[eh].resolutionScale;
        this.ubo[6] = gs.handle[eh].resy*gs.handle[eh].resolutionScale;
        device.queue.writeBuffer(this.uniformBuffer, 0, this.ubo);
        gs.handle[eh].pass.setPipeline(this.deffpipeline);
        gs.handle[eh].pass.setBindGroup(0, this.deffbindGroup[i]);
        gs.handle[eh].pass.setVertexBuffer(0, gs.models[this.em].vertexBuffer);
        gs.handle[eh].pass.setVertexBuffer(1, gs.models[this.em].uvBuffer);
        gs.handle[eh].pass.setVertexBuffer(2, gs.models[this.em].normalBuffer);
        gs.handle[eh].pass.setVertexBuffer(3, gs.models[this.em].tanBuffer);
        gs.handle[eh].pass.setVertexBuffer(4, gs.models[this.em].cotanBuffer);
        gs.handle[eh].pass.draw(gs.models[this.em].vcnt);
    }
    drawmain(eh){
        if(gs.handle[eh].reggr !== this.reggr || gs.handle[eh].sreggr !== this.sreggr){
            this.bindGroup = device.createBindGroup({
                label: "mainBindGroup",
                layout: this.pipeline.getBindGroupLayout(0),
                entries: [
                    { 
                        binding: 0, 
                        resource: { 
                            buffer: this.uniformBuffer,
                        }},
                    {
                        binding: 1,
                        resource: { 
                            buffer: gs.handle[eh].mshbuf
                        },
                    },
                    {
                        binding: 2,
                        resource: { 
                            buffer: gs.handle[eh].mdeffbuf
                        },
                    },
                    {
                        binding: 3,
                        resource: gs.textures[this.te].tex.createView(),
                    },
                    {
                        binding: 4,
                        resource: gs.handle[eh].deffered.createView(),
                    },
                    {
                        binding: 5,
                        resource: gs.handle[eh].deffdepth.createView(),
                    },
                    {
                        binding: 6,
                        resource: gs.handle[eh].shadowdepth.createView(),
                    },
                    {
                        binding: 7,
                        resource: device.createSampler({
                            magFilter: "linear",
                            minFilter: "linear",
                            mipmapFilter: "linear",
                            addressModeU: "repeat",
                            addressModeV: "repeat",
                            addressModeW: "repeat",
                        }),
                    },
                    {
                        binding: 8,
                        resource: device.createSampler({
                            addressModeU: "clamp-to-edge",
                            addressModeV: "clamp-to-edge",
                            addressModeW: "clamp-to-edge",
                            magFilter: "linear",
                            minFilter: "linear",
                            compare: 'less',
                        }),
                    },
                    {
                        binding: 9,
                        resource: device.createSampler({
                            magFilter: "nearest",
                            minFilter: "nearest",
                            mipmapFilter: "nearest",
                            addressModeU: "repeat",
                            addressModeV: "repeat",
                            addressModeW: "repeat",
                        }),
                    },
                ],
            });
            this.reggr = gs.handle[eh].reggr;
            this.sreggr = gs.handle[eh].sreggr;
        }

        this.ubo[0] = gs.handle[eh].resx;
        this.ubo[1] = gs.handle[eh].resy;
        this.ubo[2] = gs.handle[eh].shadowmapresolution;
        this.ubo[3] = gs.handle[eh].totalFrames;
        this.ubo[4] = gs.handle[eh].shadowmapcnt;
        this.ubo[5] = gs.handle[eh].resolutionScale;
        this.ubo[6] = gs.handle[eh].defferedcnt;
        device.queue.writeBuffer(this.uniformBuffer, 0, this.ubo);
        gs.handle[eh].pass.setPipeline(this.pipeline);
        gs.handle[eh].pass.setBindGroup(0, this.bindGroup);
        gs.handle[eh].pass.setVertexBuffer(0, gs.models[this.em].vertexBuffer);
        gs.handle[eh].pass.setVertexBuffer(1, gs.models[this.em].uvBuffer);
        gs.handle[eh].pass.setVertexBuffer(2, gs.models[this.em].normalBuffer);
        gs.handle[eh].pass.setVertexBuffer(3, gs.models[this.em].tanBuffer);
        gs.handle[eh].pass.setVertexBuffer(4, gs.models[this.em].cotanBuffer);
        gs.handle[eh].pass.draw(gs.models[this.em].vcnt);
    }
}

export function get_frametime(eh){
    return gs.handle[eh].frametime;
}
export function get_resx(eh){
    return gs.handle[eh].resx;
}
export function get_resy(eh){
    return gs.handle[eh].resy;
}
export function setresolution(eh, xs, ys){
}
export function seticon(eh, xs, ys, pixels){
}
export function settitle(title){
    document.title = title;
}
export function setfullscreen(eh){
    gs.handle[eh].canvas.requestFullscreen();
}
export function quitfullscreen(eh){
    document.exitFullscreen();
}
export function getKeyPressed(index){
    return pressedk[index];
}
export function getmr(){
    return msb[2];
}
export function getml(){
    return msb[0];
}
export function getmm(){
    return msb[1];
}
export function get_mouse_posx() {
    return mpos[0];
}
export function get_mouse_posy() {
    return mpos[1];
}
export function get_mouse_stat() {
    return document.pointerLockElement != null;
}
export function req_mouse_lock(eh){
    gs.handle[eh].canvas.requestPointerLock();
}
export function req_mouse_unlock(eh){
    gs.handle[eh].canvas.exitPointerLock();
}
export function modifyshadowdata(eh, ncnt, nres){
    gs.handle[eh].shadowmapcnt = ncnt;
    gs.handle[eh].shadowmapresolution = nres;
}
export function modifydeffereddata(eh, ncnt, nres){
    gs.handle[eh].defferedcnt = ncnt;
    gs.handle[eh].resolutionScale = nres;
}
export function modifyshadowuniform(eh, pos, value){ 
    gs.handle[eh].shbf[pos] = value;
}
export function modifydeffereduniform(eh, pos, value){
    gs.handle[eh].deffbf[pos] = value;
}
export function neweng(canvasid){
    let eh = gs.handle.length;
    gs.handle.push(new Gaussh(canvasid));

    return eh;
}
export function destroy(eh){
}
export function newmaterial(vert, frag, shadow, cullmode, scullmode){
    var clm = "none";
    var sclm = "none";
    switch(cullmode){
        case 1:
            clm = "front";
            break;
        case 2:
            clm = "back";
            break;
    }
    switch(scullmode){
        case 1:
            sclm = "front";
            break;
        case 2:
            sclm = "back";
            break;
    }
    let es = gs.shaders.length;
    gs.shaders.push(new Gausshader(vert, frag, shadow, clm, sclm));

    return es;
}
export function newmodel(vertices, uv, normal, tan, ctag){
    let em = gs.models.length;
    gs.models.push(new Gaussmodel(vertices, uv, normal, tan, ctag));

    return em;
}
export function setrendercamera(eme, val){
    gs.meshes[eme].rendercam = val;
}
export function setmeshbuf(eme, i, val){
    gs.meshes[eme].ubo[i+8] = val;
}
export function setdrawable(eme, val){
    gs.meshes[eme].draw = val;
}
export function newmesh(eh, es, em, te, usage){
    let eme = gs.meshes.length;
    gs.meshes.push(new Gausmesh(eh, em, es, te, usage));

    return eme;
}
export function newtexture(xsize, ysize, zsize, pixels){
    let te = gs.textures.length;
    gs.textures.push(new Gausstex(pixels, xsize, ysize, zsize));

    return te;
}
export function rn(eh){
    gs.handle[eh].startrender();
    for(var i = 0; i < gs.handle[eh].oshadowmapcnt; i+=1){
        gs.handle[eh].startshadowpass(i);
        for(var j = 0; j < gs.meshes.length; j+=1){
            if(gs.meshes[j].eh === eh && (gs.meshes[j].draw === 1 || gs.meshes[j].draw === 2) && (gs.meshes[j].usage === 2 || gs.meshes[j].usage === 3)){
                gs.meshes[j].drawsh(eh, i);
            }
        }
        gs.handle[eh].endrenderpass();
    }
    for(var i = 0; i < gs.handle[eh].odefferedcnt; i+=1){
        gs.handle[eh].startdeffpass(i);
        for(var j = 0; j < gs.meshes.length; j+=1){
            if(gs.meshes[j].eh === eh && (gs.meshes[j].draw == 1 || gs.meshes[j].draw == 3) && (gs.meshes[j].usage == 1 || gs.meshes[j].usage == 3) && (gs.meshes[j].rendercam == -1 || gs.meshes[j].rendercam == i || (gs.meshes[j].rendercam - 10 != i && gs.meshes[j].rendercam >= 10))){
                gs.meshes[j].drawdeff(eh, i);
            }
        }
        gs.handle[eh].endrenderpass();
    }
    gs.handle[eh].startmainrenderpass();
    for(var i = 0; i < gs.meshes.length; i+=1){
        if(gs.meshes[i].eh === eh && gs.meshes[i].draw !== 0 && gs.meshes[i].usage === 0){
            gs.meshes[i].drawmain(eh);
        }
    }
    gs.handle[eh].endrenderpass();
    gs.handle[eh].endrender();
}
export function renderloop(func){
    function draw(){
        func();
        requestAnimationFrame(draw);
    }
    draw();
}

/*
export function main(){
    let rni = neweng('render');

    const vertices = new Float32Array([
      -0.8, -0.8, 0.0,
       0.8, -0.8, 0.0,
       0.8,  0.8, 0.0,
    ]);

    const uv = new Float32Array([
       0.0,  0.0,
       1.0,  0.0,
       1.0,  1.0,
    ]);

    const frag = `
    @fragment
    fn fragmentMain() -> @location(0) vec4f {
      return vec4f(1, 0, 0, 1);
    }
    `;

    const frag2 = `
    struct GBufferOutput {
      @location(0) albedo : vec4f,
      @location(1) material : vec4f,
      @location(2) normal : vec4f,
      @location(3) position : vec4f,
    }

    @fragment
    fn fragmentMain() -> GBufferOutput {
      var output: GBufferOutput;
      output.albedo = vec4f(1, 1, 0, 1);
      output.material = vec4f(1, 1, 1, 1);
      output.normal = vec4f(0, 1, 1, 1);
      output.position = vec4f(0, 0, 0, 1);
      return output;
    }
    `;

    const vert = `
    @vertex
    fn vertexMain(@location(0) pos: vec3f, @location(1) uv: vec2f, @location(2) normal: vec3f, @location(3) tan: vec3f, @location(4) ctan: vec3f) ->
      @builtin(position) vec4f {
      return vec4f(pos.xy, 0, 1);
    }
    `;

    let mat = newmaterial(vert, frag, vert, 0, 0);
    let mat2 = newmaterial(vert, frag2, vert, 0, 0);
    let model = newmodel(vertices, uv, vertices, vertices, vertices);
    
    const texdt =  new Uint8Array([
      255, 0, 0, 255,
      255, 255, 0, 255,
      255, 0, 255, 255,
      255, 255, 255, 255,

      0, 0, 0, 255,
      0, 255, 0, 255,
      0, 0, 255, 255,
      0, 255, 255, 255,
    ]);

    let tex = newtexture(2, 2, 2, texdt);
    let mesh = newmesh(rni, mat, model, tex, 0);
    let mesh2 = newmesh(rni, mat2, model, tex, 3);

    function f(){
    }

    loopcont(rni, f);
}
*/