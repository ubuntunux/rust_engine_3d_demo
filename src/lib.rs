pub mod application;
pub mod game_module;
pub mod render_pass;

#[cfg_attr(target_os = "android", ndk_glue::main(backtrace = "on"))]
pub fn main() {
    application::application::run_application();
}
