# <p align="center"> <img width="460" height="300" src="![logo](https://github.com/VitionVlad/schnellwerke/assets/48290199/a17cd58f-2648-415e-b6a1-b1a6d0b966eb)"> </p>
Writing games or other 3D apps is quite an interesting process, until you realize some aspects, like porting or bad performance. You can write a web game, but it might perform poorly. Alternatively, you can write a standalone game, but porting can be time-consuming. I decided to combine incompatible things. My game engine is designed to be completely programmable from Rust. Most of the engine components are written in Rust, which allows for much higher performance compared to alternatives in JavaScript/TypeScript. At the same time, it is an engine for the web, requiring nothing special to work on other platforms (currently more of a future prospect due to WebGPU API). It works on PC Chromium, tested on Windows and Linux, and works on Android with a specific flag enabled. Additionally, for many programmers, programming in Rust can be simply more comfortable. Currently, I am working on implementing:  
1. A simple physics engine, mostly based on WebGPU compute shaders.  
2. A parser for OBJ models.  
3. Cubemaps.  
4. Shadow mapping (partially implemented).  
5. Advanced lighting: Physically Based Rendering (PBR).  
