extern crate rust_engine_3d;
extern crate ash;
extern crate nalgebra;
extern crate winit;
extern crate log;
extern crate cgmath;
extern crate rust_engine_3d_demo;

pub mod application;
pub mod application_constants;
pub mod game_module;
pub mod render_pass;
pub mod renderer;
pub mod resource;

use crate::application::project_application::run_project_application;

pub fn main() {
    run_project_application();
}