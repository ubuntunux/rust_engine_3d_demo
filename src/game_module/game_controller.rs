use nalgebra::{Vector2, Vector3};
use winit::event::VirtualKeyCode;

use rust_engine_3d::application::application::TimeData;
use rust_engine_3d::application::input::{KeyboardInputData, MouseMoveData, MouseInputData};
use rust_engine_3d::application::scene_manager::ProjectSceneManagerBase;
use rust_engine_3d::renderer::camera::CameraObjectData;
use rust_engine_3d::utilities::math;
use rust_engine_3d::utilities::system::{ptr_as_ref, ptr_as_mut};
use crate::application::project_scene_manager::ProjectSceneManager;
use crate::game_module::actors::actor::ActorController;
use crate::game_module::game_constants::{
    CAMERA_DISTANCE_MIN,
    CAMERA_DISTANCE_MAX,
    CAMERA_DISTANCE_SPEED,
    CAMERA_EDGE_SCROLL_SPEED,
    CAMERA_EDGE_SCROLL_SPEED_BY_MOUSE,
    MOUSE_PITCH_MIN,
    MOUSE_PITCH_MAX,
    MOUSE_ROTATION_SPEED
};
use crate::game_module::game_client::GameClient;
use crate::game_module::game_ui::GameUIManager;


#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GameViewMode {
    TopViewMode,
    FpsViewMode,
    Count
}

pub struct GameController {
    pub _game_client: *const GameClient,
    pub _game_ui_manager: *const GameUIManager,
    pub _camera_distance: f32,
    pub _camera_goal_distance: f32,
    pub _target_position: Vector3<f32>,
    pub _target_direction: Vector3<f32>,
    pub _relative_target_position: Vector3<f32>,
    pub _game_view_mode: GameViewMode,
}

impl GameController {
    pub fn create_game_controller() -> Box<GameController> {
        let default_camera_distance = (CAMERA_DISTANCE_MIN + CAMERA_DISTANCE_MAX) * 0.5;
        Box::new(GameController {
            _game_client: std::ptr::null(),
            _game_ui_manager: std::ptr::null(),
            _camera_distance: default_camera_distance,
            _camera_goal_distance: default_camera_distance,
            _target_position: Vector3::zeros(),
            _target_direction: Vector3::zeros(),
            _relative_target_position: Vector3::zeros(),
            _game_view_mode: GameViewMode::TopViewMode,
        })
    }

    pub fn initialize_game_controller(&mut self, game_client: &GameClient) {
        self._game_client = game_client;
        self._game_ui_manager = game_client._game_ui_manager.as_ref();
        self.change_view_mode(GameViewMode::TopViewMode);
    }
    pub fn get_game_client(&self) -> &GameClient { ptr_as_ref(self._game_client) }
    pub fn get_game_client_mut(&self) -> &mut GameClient { ptr_as_mut(self._game_client) }
    pub fn get_game_ui_manager(&self) -> &GameUIManager { ptr_as_ref(self._game_ui_manager) }
    pub fn get_game_ui_manager_mut(&self) -> &mut GameUIManager { ptr_as_mut(self._game_ui_manager) }
    pub fn get_main_camera(&self) -> &CameraObjectData {
        self.get_game_client().get_project_scene_manager().get_main_camera()
    }
    pub fn get_main_camera_mut(&self) -> &mut CameraObjectData {
        self.get_game_client().get_project_scene_manager().get_main_camera_mut()
    }
    pub fn is_view_mode(&self, target_view_mode: GameViewMode) -> bool {
        if target_view_mode == self._game_view_mode { true } else { false }
    }
    pub fn change_view_mode(&mut self, view_mode: GameViewMode) {
        self.get_game_ui_manager_mut().show_selection_area(GameViewMode::TopViewMode == view_mode);
        self.get_game_ui_manager_mut().set_crosshair_tracking_mouse(GameViewMode::TopViewMode == view_mode);
        self._game_view_mode = view_mode;
    }
    pub fn toggle_view_mode(&mut self) {
        let next_view_mode = (self._game_view_mode as i32 + 1) % GameViewMode::Count as i32;
        self.change_view_mode(unsafe { std::mem::transmute(next_view_mode) });
    }
    pub fn get_camera_distance_ratio(&self) -> f32 {
        (self._camera_distance - CAMERA_DISTANCE_MIN) / (CAMERA_DISTANCE_MAX - CAMERA_DISTANCE_MIN)
    }
    pub fn update_camera_distance(&mut self, distance: f32) {
        self._camera_goal_distance += distance;
        if self._camera_goal_distance < CAMERA_DISTANCE_MIN {
            self._camera_goal_distance = CAMERA_DISTANCE_MIN;
        } else if CAMERA_DISTANCE_MAX < self._camera_goal_distance {
            self._camera_goal_distance = CAMERA_DISTANCE_MAX;
        }
    }
    pub fn update_target_position(&mut self, project_scene_manager: &ProjectSceneManager, main_camera: &CameraObjectData, mouse_pos: &Vector2<i32>) {
        let relative_pos = main_camera.convert_screen_to_relative_world(mouse_pos);
        if project_scene_manager.get_height_map_collision_point(main_camera._transform_object.get_position(), &relative_pos.normalize(), -1.0, &mut self._target_position) {
            self._target_direction = self._target_position.normalize();
            self._relative_target_position = self._target_position - main_camera._transform_object.get_position();
        }
    }
    pub fn update_event_for_top_view_mode(
        &mut self,
        time_data: &TimeData,
        keyboard_input_data: &KeyboardInputData,
        mouse_move_data: &MouseMoveData,
        mouse_input_data: &MouseInputData,
        mouse_delta: &Vector2<f32>,
        main_camera: &mut CameraObjectData,
        player_actor: &mut ActorController
    ) {
        let _btn_left: bool = mouse_input_data._btn_l_pressed;
        let btn_right: bool = mouse_input_data._btn_r_pressed;
        let btn_right_hold: bool = mouse_input_data._btn_r_hold;
        let pressed_key_a = keyboard_input_data.get_key_hold(VirtualKeyCode::A);
        let pressed_key_d = keyboard_input_data.get_key_hold(VirtualKeyCode::D);
        let pressed_key_w = keyboard_input_data.get_key_hold(VirtualKeyCode::W);
        let pressed_key_s = keyboard_input_data.get_key_hold(VirtualKeyCode::S);
        let modifier_keys_shift = keyboard_input_data.get_key_hold(VirtualKeyCode::LShift);
        let modifier_keys_ctrl = keyboard_input_data.get_key_hold(VirtualKeyCode::LControl);

        let mut front_xz: Vector3<f32> = main_camera._transform_object.get_front().clone_owned();
        front_xz.y = 0.0;
        front_xz.try_normalize_mut(0.0);

        let mut left_xz: Vector3<f32> = main_camera._transform_object.get_left().clone_owned();
        left_xz.y = 0.0;
        left_xz.try_normalize_mut(0.0);

        // camera move
        if 0 == mouse_move_data._mouse_pos.x && mouse_move_data._mouse_pos_delta.x < 0 ||
            0 == mouse_move_data._mouse_pos.y && mouse_move_data._mouse_pos_delta.y < 0 ||
            (main_camera._window_size.x - 1) == mouse_move_data._mouse_pos.x && 0 < mouse_move_data._mouse_pos_delta.x ||
            (main_camera._window_size.y - 1) == mouse_move_data._mouse_pos.y && 0 < mouse_move_data._mouse_pos_delta.y {
            let move_delta: Vector3<f32> = (front_xz * mouse_move_data._mouse_pos_delta.y as f32 + left_xz * mouse_move_data._mouse_pos_delta.x as f32) * CAMERA_EDGE_SCROLL_SPEED_BY_MOUSE;
            main_camera._transform_object.move_position(&move_delta);
        } else {
            let camera_move_speed_multiplier = if modifier_keys_shift { 2.0 } else { 1.0 };
            let camera_move_speed: f32 = CAMERA_EDGE_SCROLL_SPEED * camera_move_speed_multiplier * time_data._delta_time as f32;
            if pressed_key_w {
                let move_delta = front_xz * -camera_move_speed;
                main_camera._transform_object.move_position(&move_delta);
            }
            else if pressed_key_s {
                let move_delta = front_xz * camera_move_speed;
                main_camera._transform_object.move_position(&move_delta);
            }

            if pressed_key_a {
                let move_delta = left_xz * -camera_move_speed;
                main_camera._transform_object.move_position(&move_delta);
            }
            else if pressed_key_d {
                let move_delta = left_xz * camera_move_speed;
                main_camera._transform_object.move_position(&move_delta);
            }
        }

        // camera yaw
        if btn_right_hold && 0.0 != mouse_delta.x {
            let yaw = main_camera._transform_object.get_yaw() - mouse_delta.x * MOUSE_ROTATION_SPEED;
            main_camera._transform_object.set_yaw(yaw);
        }

        // update cross hair
        self.get_game_ui_manager_mut().set_crosshair_pos(&mouse_move_data._mouse_pos);

        // player controll
        if btn_right {
            if modifier_keys_ctrl {
                player_actor.set_command_actor_attack(&self._target_position);
            } else {
                player_actor.set_command_actor_move(&self._target_position);
            }
        }

        // TestCode : Trace
        // for target in ptr_as_ref(self.get_game_client()).get_actor_manager()._actors.values() {
        //     let actor = target.as_ref();
        //     if false == actor.is_player_actor() {
        //         let target_position = actor.get_transform().get_position().clone_owned();
        //         player_actor.set_command_actor_move(&target_position);
        //         break;
        //     }
        // }

        if modifier_keys_shift {
            player_actor.get_ship_mut().get_controller_mut().boost_on();
        }

        // player ship project to height map
        // let relative_pos = main_camera.convert_screen_to_relative_world(&mouse_move_data._mouse_pos);
        // let mut actor_pos = main_camera._transform_object.get_position() + relative_pos.normalize() * self._camera_distance;
        // let player_ship_controller = player_actor.get_ship_mut().get_controller_mut();
        // let time_instance = time::Instant::now();
        // let current_time = time_instance.elapsed().as_secs_f64();
        // if project_application.get_project_scene_manager().get_height_map_collision_point(main_camera._transform_object.get_position(), &relative_pos.normalize(), -1.0, &mut actor_pos) {
        //     actor_pos.y += 5.0;
        //     player_ship_controller.set_position(&actor_pos);
        // }
        // let current_time2 = time_instance.elapsed().as_secs_f64();
        // log::info!("time: {:.3}ms", (current_time2 - current_time) * 1000.0);
    }

    pub fn update_event_for_fps_view_mode(
        &mut self,
        time_data: &TimeData,
        keyboard_input_data: &KeyboardInputData,
        _mouse_move_data: &MouseMoveData,
        mouse_input_data: &MouseInputData,
        mouse_delta: &Vector2<f32>,
        main_camera: &mut CameraObjectData,
        player_actor: &mut ActorController
    ) {
        let btn_left: bool = mouse_input_data._btn_l_pressed;
        let hold_key_a = keyboard_input_data.get_key_hold(VirtualKeyCode::A);
        let hold_key_d = keyboard_input_data.get_key_hold(VirtualKeyCode::D);
        let hold_key_w = keyboard_input_data.get_key_hold(VirtualKeyCode::W);
        let hold_key_s = keyboard_input_data.get_key_hold(VirtualKeyCode::S);
        let hold_key_q = keyboard_input_data.get_key_hold(VirtualKeyCode::Q);
        let hold_key_e = keyboard_input_data.get_key_hold(VirtualKeyCode::E);
        let modifier_keys_shift = keyboard_input_data.get_key_hold(VirtualKeyCode::LShift);

        // fire
        if btn_left {
            player_actor.manual_actor_attack(self.get_game_client());
        }

        let can_controll = player_actor.can_manual_controll();

        // set yaw
        let player_ship_controller = player_actor.get_ship_mut().get_controller_mut();
        if 0.0 != mouse_delta.x {
            if can_controll {
                player_ship_controller.set_velocity_yaw(-mouse_delta.x * 0.1);
            } else {
                main_camera._transform_object.rotation_yaw(-mouse_delta.x * 0.1 * time_data._delta_time as f32);
            }
        }

        // set pitch
        if 0.0 != mouse_delta.y {
            let pitch = MOUSE_PITCH_MIN.max(MOUSE_PITCH_MAX.min(main_camera._transform_object.get_pitch() - mouse_delta.y * MOUSE_ROTATION_SPEED));
            main_camera._transform_object.set_pitch(pitch);
        }

        // player move
        if modifier_keys_shift {
            player_ship_controller.boost_on();
        }

        let mut cancle_move = false;
        if hold_key_w {
            player_ship_controller.acceleration_forward(1.0);
            cancle_move = true;
        }
        else if hold_key_s {
            player_ship_controller.acceleration_forward(-1.0);
            cancle_move = true;
        }

        if hold_key_a {
            player_ship_controller.acceleration_side(1.0);
            cancle_move = true;
        }
        else if hold_key_d {
            player_ship_controller.acceleration_side(-1.0);
            cancle_move = true;
        }

        if hold_key_q {
            player_ship_controller.acceleration_vertical(-1.0);
            cancle_move = true;
        }
        else if hold_key_e {
            player_ship_controller.acceleration_vertical(1.0);
            cancle_move = true;
        }

        if cancle_move {
            player_actor.clear_command_of_actor();
        }
    }

    pub fn update_camera(&mut self, delta_time: f32) {
        if self._camera_goal_distance != self._camera_distance {
            self._camera_distance = math::lerp(self._camera_distance, self._camera_goal_distance, 1.0f32.min(delta_time * CAMERA_DISTANCE_SPEED));
        }

        let project_scene_manager = self.get_game_client().get_project_scene_manager();
        let player_actor = self.get_game_client().get_actor_manager().get_player_actor();
        let main_camera = self.get_main_camera_mut();
        let player_transform = player_actor.get_transform();

        if GameViewMode::TopViewMode == self._game_view_mode {
            // camera pitch
            let dist_ratio = self.get_camera_distance_ratio();
            let pitch: f32 = math::degree_to_radian(math::lerp(-25.0, -75.0, dist_ratio));
            main_camera._transform_object.set_pitch(pitch);
            main_camera._transform_object.update_transform_object();

            // camera postion
            let mut camera_pos = main_camera._transform_object.get_position().clone_owned();
            camera_pos.y = project_scene_manager.get_height_bilinear(&camera_pos, 0) + self._camera_distance;
            main_camera._transform_object.set_position(&camera_pos);

        } else if GameViewMode::FpsViewMode == self._game_view_mode {
            let can_controll = player_actor.can_manual_controll();

            // camera yaw
            if can_controll {
                let yaw = player_transform.get_yaw() + std::f32::consts::PI;
                main_camera._transform_object.set_yaw(yaw);
            }
            main_camera._transform_object.update_transform_object();

            // camera offset
            let mut cockpit_offset = main_camera._transform_object.get_front().clone();
            {
                cockpit_offset.y = 0.0;
                cockpit_offset.normalize_mut();
                if main_camera._transform_object.get_up().y < 0.0 {
                    cockpit_offset = -cockpit_offset;
                }

                let bound_box = &player_actor.get_bound_box();
                const BOUND_BOX_MIN: f32 = 2.0;
                cockpit_offset = cockpit_offset * -BOUND_BOX_MIN.max(bound_box._size.z * 0.5);
                cockpit_offset.y = BOUND_BOX_MIN.max(bound_box._size.y * 0.5);
            }

            // camera postion
            let mut camera_pos = player_transform.get_position() + main_camera._transform_object.get_front() * self._camera_distance + cockpit_offset;
            let floating_height = project_scene_manager.get_height_bilinear(&camera_pos, 0) + 1.0;
            if camera_pos.y < floating_height {
                camera_pos.y = floating_height;
            }
            main_camera._transform_object.set_position(&camera_pos);
        } else {
            assert!(false, "Not implemented.");
        }
    }

    pub fn update_game_controller(&mut self, delta_time: f32) {
        self.update_camera(delta_time);
    }
}