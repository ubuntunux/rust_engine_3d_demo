use nalgebra::Vector2;
use winit::event::VirtualKeyCode;

use rust_engine_3d::application::audio_manager::AudioManager;
use rust_engine_3d::effect::effect_manager::EffectManager;
use rust_engine_3d::utilities::system::{ptr_as_ref, ptr_as_mut};
use crate::application::project_application::ProjectApplication;
use crate::application::project_scene_manager::ProjectSceneManager;
use crate::game_module::actor_manager::ActorManager;
use crate::game_module::game_constants::SCROLL_DELTA_TO_CAMERA_DISTANCE_SPEED;
use crate::game_module::game_controller::{GameViewMode, GameController};
use crate::game_module::game_ui::GameUIManager;
use crate::game_module::weapon_manager::WeaponManager;
use crate::resource::project_resource::ProjectResources;
use crate::renderer::project_ui::ProjectUIManager;
use rust_engine_3d::application::scene_manager::ProjectSceneManagerBase;


pub struct GameClient {
    pub _project_application: *const ProjectApplication,
    pub _project_scene_manager: *const ProjectSceneManager,
    pub _project_resources: *const ProjectResources,
    pub _project_ui_manager: *const ProjectUIManager,
    pub _audio_manager: *const AudioManager,
    pub _effect_manager: *const EffectManager,
    pub _actor_manager: Box<ActorManager>,
    pub _game_controller: Box<GameController>,
    pub _game_ui_manager: Box<GameUIManager>,
    pub _weapon_manager: Box<WeaponManager>
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
            _actor_manager: ActorManager::create_actor_manager(),
            _game_controller: GameController::create_game_controller(),
            _game_ui_manager: GameUIManager::create_game_ui_manager(),
            _weapon_manager: WeaponManager::create_weapon_manager(),
        })
    }

    pub fn initialize_game_client(&mut self, project_application: &ProjectApplication) {
        // initialize game clients
        let game_client = project_application.get_game_client();
        self._project_application = project_application;
        self._project_scene_manager = project_application.get_project_scene_manager();
        self._project_resources = project_application.get_project_resources();
        self._project_ui_manager = project_application.get_project_ui_manager();
        self._audio_manager = project_application.get_audio_manager();
        self._effect_manager = project_application.get_effect_manager();
        self._game_ui_manager.initialize_game_ui_manager(game_client);
        self._game_controller.initialize_game_controller(game_client);
        self._actor_manager.initialize_actor_manager(game_client);
        self._weapon_manager.initialize_weapon_manager(game_client);
    }

    pub fn destroy_game_client(&mut self) {
        self._weapon_manager.destroy_weapon_manager();
        self._actor_manager.destroy_actor_manager();
        self._game_ui_manager.destroy_game_ui_manager();
    }

    pub fn get_project_application(&self) -> &ProjectApplication { ptr_as_ref(self._project_application) }
    pub fn get_project_application_mut(&self) -> &mut ProjectApplication { ptr_as_mut(self._project_application) }
    pub fn get_project_scene_manager(&self) -> &ProjectSceneManager { ptr_as_ref(self._project_scene_manager) }
    pub fn get_project_scene_manager_mut(&self) -> &mut ProjectSceneManager { ptr_as_mut(self._project_scene_manager) }
    pub fn get_project_resources(&self) -> &ProjectResources { ptr_as_ref(self._project_resources) }
    pub fn get_project_resources_mut(&self) -> &mut ProjectResources { ptr_as_mut(self._project_resources) }
    pub fn get_project_ui_manager(&self) -> &ProjectUIManager { ptr_as_ref(self._project_ui_manager) }
    pub fn get_project_ui_manager_mut(&self) -> &mut ProjectUIManager { ptr_as_mut(self._project_ui_manager) }
    pub fn get_audio_manager(&self) -> &AudioManager { ptr_as_ref(self._audio_manager) }
    pub fn get_audio_manager_mut(&self) -> &mut AudioManager { ptr_as_mut(self._audio_manager) }
    pub fn get_effect_manager(&self) -> &EffectManager { ptr_as_ref(self._effect_manager) }
    pub fn get_effect_manager_mut(&self) -> &mut EffectManager { ptr_as_mut(self._effect_manager) }
    pub fn get_actor_manager(&self) -> &ActorManager { ptr_as_ref(self._actor_manager.as_ref()) }
    pub fn get_actor_manager_mut(&self) -> &mut ActorManager { ptr_as_mut(self._actor_manager.as_ref()) }
    pub fn get_game_controller(&self) -> &GameController { ptr_as_ref(self._game_controller.as_ref()) }
    pub fn get_game_controller_mut(&self) -> &mut GameController { ptr_as_mut(self._game_controller.as_ref()) }
    pub fn get_game_ui_manager(&self) -> &GameUIManager { ptr_as_ref(self._game_ui_manager.as_ref()) }
    pub fn get_game_ui_manager_mut(&self) -> &mut GameUIManager { ptr_as_mut(self._game_ui_manager.as_ref()) }
    pub fn get_weapon_manager(&self) -> &WeaponManager { ptr_as_ref(self._weapon_manager.as_ref()) }
    pub fn get_weapon_manager_mut(&self) -> &mut WeaponManager { ptr_as_mut(self._weapon_manager.as_ref()) }

    pub fn start_game(&mut self) {
        self.get_project_scene_manager_mut().open_scene_data("default");
        self.get_actor_manager_mut().spawn_actors();
    }

    pub fn update_event(&mut self) {
        let project_application = ptr_as_ref(self._project_application);
        let engine_application = project_application.get_engine_application();
        let project_scene_manager = ptr_as_ref(self._project_scene_manager);
        let time_data = &engine_application._time_data;
        let mouse_move_data = &engine_application._mouse_move_data;
        let mouse_input_data = &engine_application._mouse_input_data;
        let keyboard_input_data = &engine_application._keyboard_input_data;
        let mouse_speed_ratio = engine_application._window_size.y as f32 / 1080.0;
        let mouse_delta: Vector2<f32> = Vector2::new(mouse_move_data._mouse_pos_delta.x as f32 / mouse_speed_ratio, mouse_move_data._mouse_pos_delta.y as f32 / mouse_speed_ratio);
        let scroll_delta = &mouse_move_data._scroll_delta;
        let pressed_key_c = keyboard_input_data.get_key_pressed(VirtualKeyCode::C);

        let main_camera = project_scene_manager.get_main_camera_mut();
        let player_actor = ptr_as_mut(self.get_actor_manager().get_player_actor());

        if 0 != mouse_move_data._mouse_pos_delta.x || 0 != mouse_move_data._mouse_pos_delta.y || 0 != scroll_delta.y || keyboard_input_data.is_any_key_hold() {
            self._game_controller.update_target_position(project_scene_manager, main_camera, &mouse_move_data._mouse_pos);
        }

        if 0 != scroll_delta.y {
            self._game_controller.update_camera_distance(-scroll_delta.y as f32 * SCROLL_DELTA_TO_CAMERA_DISTANCE_SPEED);
        }

        if pressed_key_c {
            self._game_controller.toggle_view_mode();
        }

        match self._game_controller._game_view_mode {
            GameViewMode::TopViewMode => self._game_controller.update_event_for_top_view_mode(
                time_data,
                &keyboard_input_data,
                &mouse_move_data,
                &mouse_input_data,
                &mouse_delta,
                main_camera,
                player_actor
            ),
            GameViewMode::FpsViewMode => self._game_controller.update_event_for_fps_view_mode(
                time_data,
                &keyboard_input_data,
                &mouse_move_data,
                &mouse_input_data,
                &mouse_delta,
                main_camera,
                player_actor
            ),
            _ => assert!(false, "Not implemented."),
        };
    }

    pub fn update_game_client(&mut self) {
        let delta_time = self.get_project_application().get_engine_application()._time_data._delta_time as f32;
        self._game_controller.update_game_controller(delta_time);
        self._actor_manager.update_actor_manager(delta_time);
        self._weapon_manager.update_weapon_manager(delta_time);
        self._game_ui_manager.update_game_ui(delta_time);
    }
}