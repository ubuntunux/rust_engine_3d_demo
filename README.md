## Welcome to RustEngine3D Demo
* An open source Rust Vulkan renderer using the ash API.
* RustEngine3D: https://github.com/ubuntunux/RustEngine3D
* ash: https://github.com/ash-rs/ash
* Video: https://www.youtube.com/watch?v=lAMA23NmRTI

# install
```
git clone https://github.com/ubuntunux/rust_engine_3d_demo --recursive
```

# update
```
git submodule update --recursive
```

# run on window or llinux
```
cargo run --release
```

# run on android
- run first linux or windows cause generate resources.txt and shader cahces
```
./run_android.sh
```

# dependency
now use ndk-glue 0.2 but ndk-glue 0.3 for audio.
problem is ndk-glue 0.3 is not compatibility with winit make freeze.

## Features
* Features : https://www.youtube.com/watch?v=lAMA23NmRTI
* Import
    - Mesh 
        - [x] .obj, .dae ( colada )
        - [ ] .fbx 
        - [ ] gltf, blender
    - Texture
        - [x] .png, .tga, .bmp etc 
        - [ ] Compressed Texture (ETC, DDS)
* Light
    - [x] Directional light & Shadow mapping    
    - [ ] Spot light
    - [ ] Area light
    - [x] Point light
        - [ ] shadow map using dual paraboloid mapping
* Particle System
    - [x] GPU Based Particle
    - [ ] Vector Field
    - [ ] Spawn particle on mesh
* Object
    - [x] Skeleton Mesh
    - [x] Static Mesh
    - [ ] Tree, Foliage, Grass
    - [x] Terrain
    - [x] Atmoshpere & Sky
    - [x] FFT Ocean
* Rendering
    - [ ] Culling
        - [ ] occlusion culling
        - [ ] distance culling
        - [x] view frustum culling
    - [ ] Calculate the animation in gpu
    - [x] Distance Field Font 
    - [x] Real time light probe 
    - [x] PBR
    - [x] Temporal AA
    - [ ] FSR
    - [x] Screen Space Relfection
    - [x] Screen Space Ambient Occlusion    
    - [ ] Depth Of Field
    - [ ] Bloom
    - [x] Tone mapping
    - [ ] Glare
    - [ ] Light Shaft
    - [x] Motion Blur

## References
- https://wickedengine.net/2017/11/07/gpu-based-particle-simulation
- http://kode80.com/blog/2015/03/11/screen-space-reflections-in-unity-5/index.html
- http://proland.inrialpes.fr/publications.html
- http://casual-effects.com/data/index.html
- https://github.com/TheRealMJP/MSAAFilter
- https://learnopengl.com/
- http://www.songho.ca/index.html
