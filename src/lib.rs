use crate::application::project_application::run_project_application;

pub mod application;
pub mod application_constants;
pub mod game_module;
pub mod render_pass;
pub mod scene;
pub mod resource;

#[cfg_attr(target_os = "android", ndk_glue::main(backtrace = "on"))]
pub fn main() {
    run_project_application();
}
