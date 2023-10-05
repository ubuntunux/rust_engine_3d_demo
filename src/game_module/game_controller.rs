use nalgebra::{Vector2, Vector3};
use winit::event::VirtualKeyCode;

use crate::game_module::game_client::GameClient;
use crate::game_module::game_constants::{
    CAMERA_EDGE_SCROLL_SPEED, MOUSE_PITCH_MAX, MOUSE_PITCH_MIN, MOUSE_ROTATION_SPEED,
};
use crate::game_module::game_ui::GameUIManager;
use rust_engine_3d::application::application::TimeData;
use rust_engine_3d::application::input::{KeyboardInputData, MouseInputData, MouseMoveData};
use rust_engine_3d::scene::camera::CameraObjectData;
use rust_engine_3d::utilities::system::{ptr_as_mut, ptr_as_ref};

pub struct GameController {
    pub _game_client: *const GameClient,
    pub _game_ui_manager: *const GameUIManager,
}

impl GameController {
    pub fn create_game_controller() -> Box<GameController> {
        Box::new(GameController {
            _game_client: std::ptr::null(),
            _game_ui_manager: std::ptr::null(),
        })
    }

    pub fn initialize_game_controller(&mut self, game_client: &GameClient) {
        self._game_client = game_client;
        self._game_ui_manager = game_client._game_ui_manager.as_ref();
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
            .get_project_scene_manager()
            .get_scene_manager()
            .get_main_camera()
    }
    pub fn get_main_camera_mut(&self) -> &mut CameraObjectData {
        self.get_game_client()
            .get_project_scene_manager()
            .get_scene_manager()
            .get_main_camera_mut()
    }
    pub fn update_game_event(
        &mut self,
        time_data: &TimeData,
        keyboard_input_data: &KeyboardInputData,
        mouse_move_data: &MouseMoveData,
        mouse_input_data: &MouseInputData,
        mouse_delta: &Vector2<f32>,
        main_camera: &mut CameraObjectData,
    ) {
        let _btn_left: bool = mouse_input_data._btn_l_pressed;
        let _btn_right: bool = mouse_input_data._btn_r_pressed;
        let _btn_right_hold: bool = mouse_input_data._btn_r_hold;
        let pressed_key_a = keyboard_input_data.get_key_hold(VirtualKeyCode::A);
        let pressed_key_d = keyboard_input_data.get_key_hold(VirtualKeyCode::D);
        let pressed_key_w = keyboard_input_data.get_key_hold(VirtualKeyCode::W);
        let pressed_key_s = keyboard_input_data.get_key_hold(VirtualKeyCode::S);
        let modifier_keys_shift = keyboard_input_data.get_key_hold(VirtualKeyCode::LShift);
        let _modifier_keys_ctrl = keyboard_input_data.get_key_hold(VirtualKeyCode::LControl);

        let mut front_xz: Vector3<f32> = main_camera._transform_object.get_front().clone_owned();
        front_xz.y = 0.0;
        front_xz.try_normalize_mut(0.0);

        let mut left_xz: Vector3<f32> = main_camera._transform_object.get_right().clone_owned();
        left_xz.y = 0.0;
        left_xz.try_normalize_mut(0.0);

        // camera move
        let camera_move_speed_multiplier = if modifier_keys_shift { 2.0 } else { 1.0 };
        let camera_move_speed: f32 =
            CAMERA_EDGE_SCROLL_SPEED * camera_move_speed_multiplier * time_data._delta_time as f32;
        if pressed_key_w {
            let move_delta = front_xz * camera_move_speed;
            main_camera._transform_object.move_position(&move_delta);
        } else if pressed_key_s {
            let move_delta = front_xz * -camera_move_speed;
            main_camera._transform_object.move_position(&move_delta);
        }

        if pressed_key_a {
            let move_delta = left_xz * -camera_move_speed;
            main_camera._transform_object.move_position(&move_delta);
        } else if pressed_key_d {
            let move_delta = left_xz * camera_move_speed;
            main_camera._transform_object.move_position(&move_delta);
        }

        // set yaw
        if 0.0 != mouse_delta.x {
            main_camera
                ._transform_object
                .rotation_yaw(mouse_delta.x * 0.5 * time_data._delta_time as f32);
        }

        // set pitch
        if 0.0 != mouse_delta.y {
            let pitch = MOUSE_PITCH_MIN.max(MOUSE_PITCH_MAX.min(
                main_camera._transform_object.get_pitch() + mouse_delta.y * MOUSE_ROTATION_SPEED,
            ));
            main_camera._transform_object.set_pitch(pitch);
        }

        // update cross hair
        self.get_game_ui_manager_mut()
            .set_crosshair_pos(&mouse_move_data._mouse_pos);
    }

    pub fn update_game_controller(&mut self, _delta_time: f32) {}
}
