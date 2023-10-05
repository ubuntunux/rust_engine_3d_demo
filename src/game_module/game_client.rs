use nalgebra::Vector2;
use rust_engine_3d::audio::audio_manager::AudioManager;
use rust_engine_3d::effect::effect_manager::EffectManager;
use rust_engine_3d::utilities::system::{ptr_as_mut, ptr_as_ref};

use crate::application::project_application::ProjectApplication;
use crate::application::project_scene_manager::ProjectSceneManager;
use crate::game_module::game_controller::GameController;
use crate::game_module::game_ui::GameUIManager;
use crate::resource::project_resource::ProjectResources;
use crate::scene::project_ui::ProjectUIManager;

pub struct GameClient {
    pub _project_application: *const ProjectApplication,
    pub _project_scene_manager: *const ProjectSceneManager,
    pub _project_resources: *const ProjectResources,
    pub _project_ui_manager: *const ProjectUIManager,
    pub _audio_manager: *const AudioManager,
    pub _effect_manager: *const EffectManager,
    pub _game_controller: Box<GameController>,
    pub _game_ui_manager: Box<GameUIManager>,
}

impl GameClient {
    pub fn create_game_client() -> Box<GameClient> {
        Box::new(GameClient {
            _project_application: std::ptr::null(),
            _project_scene_manager: std::ptr::null(),
            _project_resources: std::ptr::null(),
            _project_ui_manager: std::ptr::null(),
            _audio_manager: std::ptr::null(),
            _effect_manager: std::ptr::null(),
            _game_controller: GameController::create_game_controller(),
            _game_ui_manager: GameUIManager::create_game_ui_manager(),
        })
    }

    pub fn initialize_game_client(&mut self, project_application: &ProjectApplication) {
        let game_client = project_application.get_game_client();
        self._project_application = project_application;
        self._project_scene_manager = project_application.get_project_scene_manager();
        self._project_resources = project_application.get_project_resources();
        self._project_ui_manager = project_application.get_project_ui_manager();
        self._audio_manager = project_application.get_audio_manager();
        self._effect_manager = project_application.get_effect_manager();
        self._game_ui_manager
            .initialize_game_ui_manager(game_client);
        self._game_controller
            .initialize_game_controller(game_client);
    }

    pub fn destroy_game_client(&mut self) {
        self._game_ui_manager.destroy_game_ui_manager();
    }

    pub fn get_project_application(&self) -> &ProjectApplication {
        ptr_as_ref(self._project_application)
    }
    pub fn get_project_application_mut(&self) -> &mut ProjectApplication {
        ptr_as_mut(self._project_application)
    }
    pub fn get_project_scene_manager(&self) -> &ProjectSceneManager {
        ptr_as_ref(self._project_scene_manager)
    }
    pub fn get_project_scene_manager_mut(&self) -> &mut ProjectSceneManager {
        ptr_as_mut(self._project_scene_manager)
    }
    pub fn get_project_resources(&self) -> &ProjectResources {
        ptr_as_ref(self._project_resources)
    }
    pub fn get_project_resources_mut(&self) -> &mut ProjectResources {
        ptr_as_mut(self._project_resources)
    }
    pub fn get_project_ui_manager(&self) -> &ProjectUIManager {
        ptr_as_ref(self._project_ui_manager)
    }
    pub fn get_project_ui_manager_mut(&self) -> &mut ProjectUIManager {
        ptr_as_mut(self._project_ui_manager)
    }
    pub fn get_audio_manager(&self) -> &AudioManager {
        ptr_as_ref(self._audio_manager)
    }
    pub fn get_audio_manager_mut(&self) -> &mut AudioManager {
        ptr_as_mut(self._audio_manager)
    }
    pub fn get_effect_manager(&self) -> &EffectManager {
        ptr_as_ref(self._effect_manager)
    }
    pub fn get_effect_manager_mut(&self) -> &mut EffectManager {
        ptr_as_mut(self._effect_manager)
    }
    pub fn get_game_controller(&self) -> &GameController {
        ptr_as_ref(self._game_controller.as_ref())
    }
    pub fn get_game_controller_mut(&self) -> &mut GameController {
        ptr_as_mut(self._game_controller.as_ref())
    }
    pub fn get_game_ui_manager(&self) -> &GameUIManager {
        ptr_as_ref(self._game_ui_manager.as_ref())
    }
    pub fn get_game_ui_manager_mut(&self) -> &mut GameUIManager {
        ptr_as_mut(self._game_ui_manager.as_ref())
    }

    pub fn start_game(&mut self) {
        self.get_project_scene_manager_mut()
            .open_scene_data("sponza");
    }

    pub fn set_game_mode(&mut self, is_game_mode: bool) {
        let game_ui_layout_mut = ptr_as_mut(self.get_project_ui_manager().game_ui_layout());
        game_ui_layout_mut
            .get_ui_component_mut()
            .set_visible(is_game_mode);
    }

    pub fn update_game_event(&mut self) {
        let project_application = ptr_as_ref(self._project_application);
        let engine_application = project_application.get_engine_application();
        let project_scene_manager = project_application.get_project_scene_manager();
        let scene_manager = project_scene_manager.get_scene_manager();
        let time_data = &engine_application._time_data;
        let mouse_move_data = &engine_application._mouse_move_data;
        let mouse_input_data = &engine_application._mouse_input_data;
        let keyboard_input_data = &engine_application._keyboard_input_data;
        let mouse_speed_ratio = engine_application._window_size.y as f32 / 1080.0;
        let mouse_delta: Vector2<f32> = Vector2::new(
            mouse_move_data._mouse_pos_delta.x as f32 / mouse_speed_ratio,
            mouse_move_data._mouse_pos_delta.y as f32 / mouse_speed_ratio,
        );
        let _scroll_delta = &mouse_move_data._scroll_delta;

        let main_camera = scene_manager.get_main_camera_mut();

        self._game_controller.update_game_event(
            time_data,
            &keyboard_input_data,
            &mouse_move_data,
            &mouse_input_data,
            &mouse_delta,
            main_camera,
        );
    }

    pub fn update_game_client(&mut self, delta_time: f32) {
        self._game_controller.update_game_controller(delta_time);
        self._game_ui_manager.update_game_ui(delta_time);
    }
}
