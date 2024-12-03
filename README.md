# <p align="center"> <img src="https://github.com/VitionVlad/schnellwerke/blob/main/assets/logo_long.png"> </p>
Writing advanced 3D or 2D applications can be an exciting challenge, but it often becomes complex when dealing with the details of porting or ensuring the application works seamlessly on the web. To address these challenges, I developed my own 3D graphics engine, Schnellwerke, designed specifically for the web.  
The core idea behind Schnellwerke is to maximize performance by leveraging WebAssembly and WebGPU technologies while optimizing the engine’s internal architecture for efficiency. It takes into account various crucial details to ensure high performance and smooth functionality across platforms.  
⚠️ The engine requires WebGPU and WebAssembly support to function!  
![image](https://github.com/user-attachments/assets/b304d9b0-a353-4ba1-b987-e47910f605a0)
# <p align="center"> Internal structure </p>  
Since the first versions, I’ve made several changes to the internal structure. The engine now operates using two asynchronous loops: one managed by requestAnimationFrame and the other by a timer. This approach achieves two key goals:  
1. It decouples app logic and physics from the renderer, ensuring independence between them.  
2. Theoretically, it allows the browser to run these two tasks on different threads.  

Additionally, the entire rendering process is handled in JavaScript, which is faster since it avoids Rust-to-JavaScript calls. This is important because rendering cannot be done directly from Rust.  
![image](https://github.com/user-attachments/assets/f95a01ae-7d87-44e3-ba83-228db0d2b574)  
# <p align="center"> Render </p>  
Since its initial versions, the renderer has undergone several changes. Firstly, it now supports multiple cameras and shadow maps, made possible by layered framebuffers. The only limits to the number of light sources in a scene are your imagination and available memory.  
The rendering method has also evolved, transitioning from the older forward rendering approach to a modern deferred renderer. This switch enables the use of significantly more lights and improves overall performance. As a result, the engine can achieve relatively good performance even on older hardware, such as Haswell iGPUs.  
Additionally, you can adjust the render resolution, shadow map resolution, and the number of cameras and shadow maps in real time. The engine automatically manages the generation of uniform buffers, processes them across all cameras, and handles other tasks seamlessly. 
<p align="center"> <img src="https://github.com/user-attachments/assets/cf1c680b-a74a-4fe0-af21-f5172281333f"> </p>  
<p align="center"> <img src="https://github.com/user-attachments/assets/2d19e8f7-dcfe-45db-9133-91c943194405"> </p>  

