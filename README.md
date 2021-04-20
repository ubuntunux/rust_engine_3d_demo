# rust_engine_3d_demo
RustEngine3D Demo

# install
git clone https://github.com/ubuntunux/rust_engine_3d_demo --recursive

# update
git submodule update --recursive

# run on window or llinux
cargo run --release

# run on android
- run first linux or windows cause generate resources.txt and shader cahces
./run_android.sh

# dependency
now use ndk-glue 0.2 but ndk-glue 0.3 for audio.
problem is ndk-glue 0.3 is not compatibility with winit make freeze.
