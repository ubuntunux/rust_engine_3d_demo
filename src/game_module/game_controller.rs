use nalgebra::Vector2;
use rust_engine_3d::core::engine_core::TimeData;
use rust_engine_3d::core::input::{KeyboardInputData, MouseInputData, MouseMoveData};
use rust_engine_3d::scene::camera::CameraObjectData;
use rust_engine_3d::utilities::system::{ptr_as_mut, ptr_as_ref, RcRefCell};
use winit::event::VirtualKeyCode;

use crate::application::application::Application;
use crate::game_module::character::character::Character;
use crate::game_module::game_client::GameClient;
use crate::game_module::game_constants::*;
use crate::game_module::game_ui_manager::GameUIManager;

pub struct GameController {
    pub _game_client: *const GameClient,
    pub _game_ui_manager: *const GameUIManager,
    pub _camera_distance: f32,
    pub _camera_goal_distance: f32
}

impl GameController {
    pub fn create_game_controller() -> Box<GameController> {
        Box::new(GameController {
            _game_client: std::ptr::null(),
            _game_ui_manager: std::ptr::null(),
            _camera_goal_distance: CAMERA_DISTANCE_MAX,
            _camera_distance: 0.0,
        })
    }

    pub fn initialize_game_controller(&mut self, application: &Application) {
        log::info!("initialize_game_controller");
        self._game_client = application.get_game_client();
        self._game_ui_manager = application.get_game_ui_manager();
    }
    pub fn get_game_client(&self) -> &GameClient {
        ptr_as_ref(self._game_client)
    }
    pub fn get_game_client_mut(&self) -> &mut GameClient {
        ptr_as_mut(self._game_client)
    }
    pub fn get_game_ui_manager(&self) -> &GameUIManager {
        ptr_as_ref(self._game_ui_manager)
    }
    pub fn get_game_ui_manager_mut(&self) -> &mut GameUIManager {
        ptr_as_mut(self._game_ui_manager)
    }
    pub fn get_main_camera(&self) -> &CameraObjectData {
        self.get_game_client()
            .get_game_scene_manager()
            .get_scene_manager()
            .get_main_camera()
    }
    pub fn get_main_camera_mut(&self) -> &mut CameraObjectData {
        self.get_game_client()
            .get_game_scene_manager()
            .get_scene_manager()
            .get_main_camera_mut()
    }
    pub fn update_game_controller(
        &mut self,
        time_data: &TimeData,
        keyboard_input_data: &KeyboardInputData,
        mouse_move_data: &MouseMoveData,
        mouse_input_data: &MouseInputData,
        _mouse_delta: &Vector2<f32>,
        main_camera: &mut CameraObjectData,
        player: &RcRefCell<Character>
    ) {
        let btn_left: bool = mouse_input_data._btn_l_pressed;
        let _btn_right: bool = mouse_input_data._btn_r_pressed;
        let _btn_right_hold: bool = mouse_input_data._btn_r_hold;


        let is_left = keyboard_input_data.get_key_hold(VirtualKeyCode::Left) | keyboard_input_data.get_key_hold(VirtualKeyCode::A);
        let is_right = keyboard_input_data.get_key_hold(VirtualKeyCode::Right) | keyboard_input_data.get_key_hold(VirtualKeyCode::D);
        let is_jump = keyboard_input_data.get_key_hold(VirtualKeyCode::Up) | keyboard_input_data.get_key_hold(VirtualKeyCode::W) | keyboard_input_data.get_key_hold(VirtualKeyCode::Space);
        let _modifier_keys_ctrl = keyboard_input_data.get_key_hold(VirtualKeyCode::LControl);
        let mut player_mut = player.borrow_mut();

        // update player control
        if is_left || is_right {
            player_mut.set_move_walk(is_left);
        }

        if is_jump {
            player_mut.set_move_jump();
        }

        if btn_left {
            player_mut.set_action_attack();
        }

        // update camera
        self._camera_goal_distance -= mouse_move_data._scroll_delta.y as f32;
        self._camera_goal_distance = CAMERA_DISTANCE_MIN.max(CAMERA_DISTANCE_MAX.min(self._camera_goal_distance));
        if self._camera_goal_distance != self._camera_distance {
            let diff = (self._camera_goal_distance - self._camera_distance) * CAMERA_ZOOM_SPEED;
            let sign = diff.signum();
            let delta =  diff * time_data._delta_time as f32;
            self._camera_distance += delta;
            if sign != (self._camera_goal_distance - self._camera_distance).signum() {
                self._camera_distance = self._camera_goal_distance;
            }

            let mut camera_position = player_mut.get_position() - main_camera._transform_object.get_front() * self._camera_distance;
            camera_position.y += CAMERA_PITCH;
            main_camera._transform_object.set_position(&camera_position);
            main_camera._transform_object.set_pitch(CAMERA_PITCH);
        }
    }
}
