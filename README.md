# <p align="center"> <img src="https://github.com/VitionVlad/schnellwerke/blob/main/logo.png"> </p>
Writing games or other 3D apps is quite an interesting process, until you realize some aspects, like porting or bad performance. You can write a web game, but it might perform poorly. Alternatively, you can write a standalone game, but porting can be time-consuming. I decided to combine incompatible things. My game engine is designed to be completely programmable from Rust. Most of the engine components are written in Rust, which allows for much higher performance compared to alternatives in JavaScript/TypeScript. At the same time, it is an engine for the web, requiring nothing special to work on other platforms (currently more of a future prospect due to WebGPU API). It works on PC Chromium, tested on Windows and Linux, and works on Android with a specific flag enabled. Additionally, for many programmers, programming in Rust can be simply more comfortable. Currently, I am working on implementing:  
1. A simple physics engine, mostly based on WebGPU compute shaders (compute shaders implemented).  
2. Cubemaps.  
3. Shadow mapping (partially implemented).  
4. Advanced lighting: Physically Based Rendering (PBR).  
# <p align="center"> Model Loader </p>  
now, to load a model you just need, an element which contains obj file (only obj files supported, vertices+uv+normals), you need an html element like this:  
```  
  <iframe src="assets/m.txt" type="text/plain" id="md1" style="display: none;"></iframe>
```
and then use this element to create a Objreader class, it will conatin everything necessery
```
let md = Objreader::new("md1");
let mut mesh: Object = Object::new(&eng, &md.vert.as_slice(), &md.uv.as_slice(), &md.norm.as_slice(), md.size, vertc, vertsc, fragc, &uniforms, "tex;spec", "linear", "linear", false);
```
and you are done!  
![image](https://github.com/VitionVlad/schnellwerke/assets/48290199/3de30dca-cb6a-4b36-828a-87f1dea01fe8)  
this model parsing takes less than a second, its size is 12 mb, i tried a 60 mb file, it is parsing in about 3 seconds, most of time is being spended on browser to load resource. by the way, before loading any data make sure you page with resources is completly loaded:
```
import init, { main } from "./pkg/schnellwerke.js";
      window.addEventListener("load", function (event) {
        init().then(() => {
          main();
        });
      });
```
by the way, here is demo of its working on mobile:  
![Screenshot_20240301-195530](https://github.com/VitionVlad/schnellwerke/assets/48290199/d53c8fe9-b48d-472d-85b6-7dfd5e2edc64)
