use nalgebra::Vector2;
use rust_engine_3d::application::application::EngineApplication;
use rust_engine_3d::effect::effect_manager::EffectManager;
use rust_engine_3d::renderer::renderer_context::RendererContext;
use rust_engine_3d::renderer::renderer_data::RendererData;
use rust_engine_3d::resource::resource::EngineResources;
use rust_engine_3d::scene::scene_manager::{ProjectSceneManagerBase, SceneManager};
use rust_engine_3d::utilities::system::{ptr_as_mut, ptr_as_ref};

pub struct ProjectSceneManager {
    pub _scene_manager: *const SceneManager,
    pub _engine_resources: *const EngineResources,
    pub _renderer_data: *const RendererData,
    pub _effect_manager: *const EffectManager,
}

impl ProjectSceneManagerBase for ProjectSceneManager {}

impl ProjectSceneManager {
    pub fn get_scene_manager(&self) -> &SceneManager {
        ptr_as_ref(self._scene_manager)
    }
    pub fn get_scene_manager_mut(&self) -> &mut SceneManager {
        ptr_as_mut(self._scene_manager)
    }

    pub fn create_project_scene_manager() -> Box<ProjectSceneManager> {
        Box::new(ProjectSceneManager {
            _scene_manager: std::ptr::null(),
            _engine_resources: std::ptr::null(),
            _renderer_data: std::ptr::null(),
            _effect_manager: std::ptr::null(),
        })
    }

    pub fn initialize_project_scene_manager(
        &mut self,
        scene_manager: &mut SceneManager,
        renderer_context: &RendererContext,
        effect_manager: &EffectManager,
        engine_resources: &EngineResources,
        window_size: &Vector2<i32>,
    ) {
        self._scene_manager = scene_manager;
        self._renderer_data = renderer_context.get_renderer_data();
        self._effect_manager = effect_manager;
        self._engine_resources = engine_resources;

        scene_manager.initialize_scene_manager(
            self as *const dyn ProjectSceneManagerBase,
            renderer_context,
            effect_manager,
            engine_resources,
            window_size,
        )
    }

    pub fn open_scene_data(&mut self, scene_data_name: &str) {
        self.get_scene_manager_mut()
            .open_scene_data(scene_data_name);
    }

    pub fn close_scene_data(&mut self) {
        self.get_scene_manager_mut().close_scene_data();
    }

    pub fn destroy_project_scene_manager(&mut self) {
        self.get_scene_manager_mut().destroy_scene_manager();
    }

    pub fn update_project_scene_manager(
        &mut self,
        engine_application: &EngineApplication,
        delta_time: f64,
    ) {
        self.get_scene_manager_mut()
            .update_scene_manager(engine_application, delta_time);
    }
}
