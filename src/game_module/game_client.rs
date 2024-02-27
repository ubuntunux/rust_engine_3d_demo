use nalgebra::Vector2;
use rust_engine_3d::core::engine_core::EngineCore;
use rust_engine_3d::utilities::system::{ptr_as_mut, ptr_as_ref};

use crate::application::application::Application;
use crate::game_module::character::character_manager::CharacterManager;
use crate::game_module::game_controller::GameController;
use crate::game_module::game_resource::GameResources;
use crate::game_module::game_scene_manager::GameSceneManager;
use crate::game_module::game_ui_manager::GameUIManager;

pub struct GameClient {
    pub _engine_core: *const EngineCore,
    pub _application: *const Application,
    pub _character_manager: *const CharacterManager,
    pub _game_scene_manager: *const GameSceneManager,
    pub _game_resources: *const GameResources,
    pub _game_controller: *const GameController,
    pub _game_ui_manager: *const GameUIManager,
}

impl GameClient {
    pub fn create_game_client() -> Box<GameClient> {
        Box::new(GameClient {
            _engine_core: std::ptr::null(),
            _application: std::ptr::null(),
            _character_manager: std::ptr::null(),
            _game_scene_manager: std::ptr::null(),
            _game_resources: std::ptr::null(),
            _game_controller: std::ptr::null(),
            _game_ui_manager: std::ptr::null(),
        })
    }

    pub fn initialize_game_client(&mut self, engine_core: *const EngineCore, application: &Application) {
        log::info!("initialize_game_client");
        self._engine_core = engine_core;
        self._application = application;
        self._game_controller = application.get_game_controller();
        self._character_manager = application.get_character_manager();
        self._game_scene_manager = application.get_game_scene_manager();
        self._game_resources = application.get_game_resources();
        self._game_ui_manager = application.get_game_ui_manager();
    }
    pub fn destroy_game_client(&mut self) {
        ptr_as_mut(self._game_ui_manager).destroy_game_ui_manager();
    }
    pub fn get_engine_core(&self) -> &EngineCore {
        ptr_as_ref(self._engine_core)
    }
    pub fn get_engine_core_mut(&self) -> &EngineCore {
        ptr_as_mut(self._engine_core)
    }
    pub fn get_application(&self) -> &Application {
        ptr_as_ref(self._application)
    }
    pub fn get_application_mut(&self) -> &mut Application {
        ptr_as_mut(self._application)
    }
    pub fn get_character_manager(&self) -> &CharacterManager { ptr_as_ref(self._character_manager) }
    pub fn get_character_manager_mut(&self) -> &mut CharacterManager { ptr_as_mut(self._character_manager) }
    pub fn get_game_scene_manager(&self) -> &GameSceneManager { ptr_as_ref(self._game_scene_manager) }
    pub fn get_game_scene_manager_mut(&self) -> &mut GameSceneManager { ptr_as_mut(self._game_scene_manager)}
    pub fn get_game_resources(&self) -> &GameResources { ptr_as_ref(self._game_resources) }
    pub fn get_game_resources_mut(&self) -> &mut GameResources {
        ptr_as_mut(self._game_resources)
    }
    pub fn get_game_controller(&self) -> &GameController { ptr_as_ref(self._game_controller) }
    pub fn get_game_controller_mut(&self) -> &mut GameController { ptr_as_mut(self._game_controller) }
    pub fn get_game_ui_manager(&self) -> &GameUIManager { ptr_as_ref(self._game_ui_manager) }
    pub fn get_game_ui_manager_mut(&self) -> &mut GameUIManager { ptr_as_mut(self._game_ui_manager) }
    pub fn start_game(&mut self) {
        log::info!("start_game");
        self.get_game_scene_manager_mut().open_game_scene_data("intro_stage");
    }

    pub fn set_game_mode(&mut self, _is_game_mode: bool) {
        //self.get_game_ui_manager_mut().show_ui(is_game_mode);
    }

    pub fn update_game_mode(&mut self, _delta_time: f64) {
        let engine_core = self.get_engine_core();
        let game_scene_manager = self.get_game_scene_manager();
        let scene_manager = game_scene_manager.get_scene_manager();
        let time_data = &engine_core._time_data;
        let mouse_move_data = &engine_core._mouse_move_data;
        let mouse_input_data = &engine_core._mouse_input_data;
        let keyboard_input_data = &engine_core._keyboard_input_data;
        let mouse_speed_ratio = 1.0;
        let mouse_delta: Vector2<f32> = Vector2::new(
            mouse_move_data._mouse_pos_delta.x as f32 / mouse_speed_ratio,
            mouse_move_data._mouse_pos_delta.y as f32 / mouse_speed_ratio,
        );
        let player = self.get_character_manager().get_player();
        let main_camera = scene_manager.get_main_camera_mut();
        if false == self._game_controller.is_null() {
            let game_controller = ptr_as_mut(self._game_controller);
            game_controller.update_game_controller(
                time_data,
                &keyboard_input_data,
                &mouse_move_data,
                &mouse_input_data,
                &mouse_delta,
                main_camera,
                player,
            );
        }
    }
}
