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
and then use loaded and indexed model in mesh creation(still working on)  
```
  let reslod = Jsrelod::new("md1");

  let v = reslod.getvert().to_vec();
  let u = reslod.getuv().to_vec();
  let n = reslod.getnorm().to_vec();

  let mut mesh: Object = Object::new(&eng, &v.as_slice(), &u.as_slice(), &n.as_slice(), reslod.getlen() as i32, vertc, vertsc, fragc, &uniforms, "tex;spec", "linear", "linear", false);
```
and you are done!  
![image](https://github.com/VitionVlad/schnellwerke/assets/48290199/fdceb6be-8eb1-4d9b-8990-f7143341f397)
