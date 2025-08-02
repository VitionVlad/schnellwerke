# <p align="center"> <img src="https://github.com/VitionVlad/schnellwerke/blob/main/assets/logo_long.png"> </p>
Writing advanced 3D or 2D applications can be an exciting challenge, but it often becomes complex when dealing with the details of porting or ensuring the application works seamlessly on the web. To address these challenges, I developed my own 3D graphics engine, Schnellwerke, designed specifically for the web, and later for native apps, Schnellwerke native.  
The core idea behind Schnellwerke is to maximize performance by leveraging WebAssembly and WebGPU technologies while optimizing the engine’s internal architecture for efficiency. It takes into account various crucial details to ensure high performance and smooth functionality across platforms.  
⚠️ The engine requires WebGPU and WebAssembly support to function!  
<img width="1600" height="900" alt="image" src="https://github.com/user-attachments/assets/0b09f2d3-6be1-4af0-a8e5-b9d0c9047d9e" />
# <p align="center"> Structure  </p>
Version 3.0 introduced a restructured system focused on more efficient resource usage. Now, textures, shaders, and models are no longer bound to individual objects—they can be shared across multiple objects.  
This is especially useful for textures, as it eliminates unnecessary duplication. Instead of loading the same texture for each object (which leads to increased memory consumption), a single shared texture is used, significantly reducing memory usage.  
For example, the demo game ZUG runs with a scene containing over 100 objects—many of which reuse the same materials — and it consumes only about 300 MB of RAM.  
The engine and object structure hasn’t changed much, remaining mostly the same. The only notable difference is the controls, which are now part of the engine structure, as shown in this diagram.  
<p align="center"><img width="602" height="622" alt="Diagramă fără titlu-Pagină-1 drawio" src="https://github.com/user-attachments/assets/3e6b3ecf-67af-4a25-8325-e272f89e98dd" /> </p>
and the strucutre itself:  

```rust
pub struct Engine{
    pub render: Render,
    pub audio: AudioEngine,
    pub control: Control,
    pub cameras: [Camera; 10],
    pub used_camera_count: u32,
    pub lights: [Light; 100],
    pub used_light_count: u32,
    pub physics_tick: u32,
    pub times_to_calculate_physics: u32,
    pub obj_ph: Vec<PhysicsObject>,
    pub fps: u32,
    pub primary_camera: usize,
}
```

Engine handle creation and render loop handling is also quiet simple:  

```rust
let mut eng = Engine::new();

...

render_loop(Closure::new(move || {
eng.work();

...

}));
```

Objects as was earlier mentioned didnt change that much, as shown in this diagram:  
<p align="center"><img width="621" height="382" alt="Diagramă fără titlu-Pagină-2 drawio" src="https://github.com/user-attachments/assets/3104494d-1da7-40af-9cfe-adc1047312b2" /></p>
and objects structure itself:  

```rust
pub struct Object{
    pub mesh: Mesh,
    pub physic_object: PhysicsObject,
    pub is_looking_at: bool,
    pub draw: bool,
    pub draw_shadow: bool,
    pub draw_distance: f32,
    pub view_reaction_distance: f32,
    pub render_in_behind: bool,
}
```

But now creation an object is much more tricky, as it requires to load in a separate structure model, images, and shaders:  

```rust
let dvert = fs::read("shaders/vdeffered").unwrap();
let dfragem = fs::read("shaders/fdeffem").unwrap();
let shadow = fs::read("shaders/shadow").unwrap();
let mat4 = Material::new(&eng, dvert, dfragem, shadow, [engine::render::render::CullMode::CullModeBackBit, engine::render::render::CullMode::CullModeFrontBit]);
let image = Image::new_color(&eng, [i8::MAX, i8::MAX, i8::MAX, i8::MAX]);//can also be loaded from file
let mut vrt1 = ModelAsset::load_obj("assets/train_em.obj");
let md1 = Model::new(&mut eng, vrt1.vertices[0].clone());
let mut trainem = Object::new(&mut eng, md1, mat4, image, engine::render::render::MeshUsage::DefferedPass, true);

render_loop(Closure::new(move || {
eng.work();
...
//but executing them is still simple
trainem.exec(&mut eng);
...
}));
```

There also are some special objects, which are UItext and UIplane, that are obviosly used for ui, here an example:  

```rust
let mut viewport = UIplane::new(&mut eng, mat, image);
let mut text: [UItext; 5] = [
  UItext::new(&mut eng, matt, ti, "aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ0123456789,.;:'+-<>_"),
  ...
];
```

they also can be intractive if you want, this will requiere to change a flag, speaking about this, here is their structure:  

```rust
pub struct UIplane{
    pub object: Object,
    pub clickzone: Clickzone,
    pub signal: bool,
    pub allow_when_mouse_locked: bool,
}

pub struct UItext{
    pub font: Image,
    pub symbols: Vec<u8>,
    pub planes: Vec<Object>,
    pub symbol_number: u32,
    pub material: Material,
    pub size: Vec2,
    pub pos: Vec3,
    pub clickzone: Clickzone,
    pub signal: bool,
    pub per_symbol: bool,
    pub allow_when_mouse_locked: bool,
    pub draw: bool,
    pub symbol_pressed: u8,
    pub symbol_index: usize,
}
```

Also, Objects can be loaded from file, via scene, in this case, engine will parse also material library, only needed thing along side textures and model itself are shaders, so an example:  

```rust
let mat2 = Material::new(&eng, dvert.clone(), dfrag, shadow.clone(), [engine::render::render::CullMode::CullModeBackBit, engine::render::render::CullMode::CullModeFrontBit]);
...
let mut train = Scene::load_from_obj(&mut eng, "assets/train.obj", mat2);
```

here is also scene structure:  

```rust
pub struct Scene{
    pub objects: Vec<Object>,
    pub use_global_values: bool,
    pub pos: Vec3,
    pub scale: Vec3,
    pub rot: Vec3,
    pub render_all_cameras: bool,
    pub exclude_selected_camera: bool,
    pub camera_number: i8,
}
```

# <p align="center"> Render </p>
Rendering in Schnellwerke 3 is based on the Gauss component, which handles all interaction with the WebGPU API, as well as managing input and other operations.  
The main rendering approach is deferred rendering, although you're free to rewrite the shaders yourself and use traditional forward rendering instead.  
Below is a diagram that represents the entire rendering process.  
<p align="center"><img width="531" height="1791" alt="Diagramă fără titlu-Pagină-3 drawio" src="https://github.com/user-attachments/assets/6b755df8-e7dc-43e9-949f-8d3db038684d" /> </p>
This demo also showcase the rendering of transparent objects, which is significantly more difficult in a deferred rendering approach.  

# <p align="center"> Physics </p>
All physics calculations are not directly exposed to the programmer. They are mostly executed at the start of each new frame, as I chose to use a tick-based approach for physics simulation. This means the physics engine runs at a different tick rate than the game itself—it can be higher or lower. This approach ensures frame rate–independent physics.  
Currently, physics is calculated only between the player and objects. However, in the future, I may consider enabling physics objects to interact with each other—allowing them to fall, collide, and behave more dynamically. This will largely depend on performance and how efficiently it can be implemented.  

# <p align="center"> Audio </p>
The native version of the engine uses Audio API for audio handling.  
It supports all popular audio formats and provides simple yet sufficient functionality—such as setting pan and volume.  
To create an audio source, the engine uses a Speaker structure, which looks like this:  

```rust
pub struct Speaker{
    pub pos: Vec3,
    pub play: bool,
    pub power: f32,
    pub use_pan: bool,
    pub pos_dependency: bool,
    pub volume: f32,
}
```

and working with this structure looks like this:  

```rust
let mut trains = Speaker::new(&mut eng, "assets/audio/train.mp3");
...
render_loop(Closure::new(move || {
eng.work();
...
trains.exec(&mut eng);
...
}));
```

# <p align="center"> <img width="1599" height="262" alt="image" src="https://github.com/user-attachments/assets/6ba28016-4bf8-437f-9bba-26fa2047faad" /> </p>

ZUG is a demo game created using my new graphics engine. The engine was developed both for debugging and integrating new features, but also as an experiment to create a first-person puzzle experience.  
  
So, what is this game about?  
ZUG is a first-person puzzle game set during the time of the Great War. It aims to recreate the atmosphere of that era and perhaps show or tell something about the war itself—such as the use of ciphers or historical events.  
  
The game is quite short, but instead of focusing on length, I put effort into making everything feel realistic. I also used it as a playground to experiment with the engine’s capabilities, shaders, and more.  
  
Taken from this repository, the game can be compiled without any code modifications for both Windows and Linux.  
  
Controls are standard:
WASD for movement, mouse for looking around, and LMB for interacting with objects. Also touch controls are avaible, game automatically switches to mobile controls.    
  
I can’t give specific system requirements, but the main one is a WebGPU GPU, and a browser which supports it. In my case, the game ran at maximum graphics settings on a Ryzen 5 5600G with integrated Vega 7 graphics and 16 GB of RAM—which was more than enough.  
Demo is also playeble on mobile devices, for example here is a exemple of it running on my Google Pixel 7:  

<img width="1240" height="558" alt="image" src="https://github.com/user-attachments/assets/8e01f911-88f0-42c4-9c37-ccb703928d0c" />

<img width="1600" height="900" alt="Screenshot 2025-07-12 102033" src="https://github.com/user-attachments/assets/06a76a3d-a8c7-4d7d-afc7-3319dc1917c3" />  

<img width="1600" height="900" alt="Screenshot 2025-07-12 102051" src="https://github.com/user-attachments/assets/ebad32b1-5b90-44eb-a7ee-e4532f7e07ed" />

<img width="1600" height="900" alt="Screenshot 2025-07-12 102116" src="https://github.com/user-attachments/assets/ed48a51a-6710-4860-8dfe-2c7eb36c401b" />

<img width="1600" height="900" alt="Screenshot 2025-07-12 102137" src="https://github.com/user-attachments/assets/e8a656ad-0ad5-4030-8793-f95b30062134" />
