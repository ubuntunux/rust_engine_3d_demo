extern crate rust_engine_3d;
extern crate nalgebra;
extern crate ash;

pub mod application_constants;
pub mod application;
pub mod game_module;
pub mod render_pass;
pub mod renderer;
pub mod resource;

use crate::application::project_application::run_project_application;

#[cfg_attr(target_os = "android", ndk_glue::main(backtrace = "on"))]
pub fn main() {
    run_project_application();
}