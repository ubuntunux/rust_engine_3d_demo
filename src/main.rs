pub mod application;
pub mod application_constants;
pub mod render_pass;
pub mod renderer;
pub mod resource;

use crate::application::project_application::run_application;

pub fn main() {
    run_application();
}