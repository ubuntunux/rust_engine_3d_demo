use std::rc::Rc;
use nalgebra::Vector3;

use rust_engine_3d::application::scene_manager::ProjectSceneManagerBase;
use rust_engine_3d::renderer::render_object::{RenderObjectData};
use rust_engine_3d::renderer::transform_object::TransformObjectData;
use rust_engine_3d::utilities::bounding_box::BoundingBox;
use rust_engine_3d::utilities::math;
use rust_engine_3d::utilities::system::{RcRefCell, ptr_as_mut, ptr_as_ref};
use crate::application::project_scene_manager::ProjectSceneManager;
use crate::game_module::game_client::GameClient;
use crate::game_module::game_constants::{CHECK_TARGET_DISTANCE_MAX};
use crate::game_module::ship::ship::{ShipInstance, ShipData};
use crate::game_module::ship::ship_controller::{ ShipController };

pub struct ActorData {
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ActorControllerState {
    None,
    Attack,
    Move,
    Patrol,
    Trace,
}

// ActorController
pub struct ActorController {
    pub _id: u64,
    pub _actor_data: ActorData,
    pub _ship: ShipInstance,
    pub _actor_controller_state: ActorControllerState,
    pub _target_position: Vector3<f32>,
    pub _is_player_actor: bool,
    pub _command_move: bool,
    pub _command_rotate: bool,
    pub _command_attack: bool
}

impl ActorController {
    pub fn create_actor_controller(
        id: u64,
        ship_data: &RcRefCell<ShipData>,
        render_object: &RcRefCell<RenderObjectData>,
        is_player_actor: bool
    ) -> Rc<ActorController> {
        Rc::new(ActorController {
            _id: id,
            _actor_data: ActorData {},
            _ship: ShipInstance::create_ship_instance(ship_data, render_object),
            _actor_controller_state: ActorControllerState::None,
            _target_position: Vector3::zeros(),
            _is_player_actor: is_player_actor,
            _command_move: false,
            _command_rotate: false,
            _command_attack: false,
        })
    }

    pub fn initialize_actor(&mut self, project_scene_manager: &mut ProjectSceneManager) {
        self._ship.initialize_ship_instance(self, project_scene_manager);
    }
    pub fn remove_actor(&mut self, project_scene_manager: &mut ProjectSceneManager) {
        self._ship.remove_ship_instance(project_scene_manager);
    }
    pub fn get_actor_id(&self) -> u64 {
        self._id
    }
    pub fn is_player_actor(&self) -> bool {
        self._is_player_actor
    }
    pub fn get_actor_data(&self) -> &ActorData {
        &self._actor_data
    }
    pub fn get_actor_data_mut(&mut self) -> &mut ActorData {
        &mut self._actor_data
    }
    pub fn get_ship(&self) -> &ShipInstance { &self._ship }
    pub fn get_ship_mut(&mut self) -> &mut ShipInstance {
        &mut self._ship
    }
    pub fn get_controller(&self) -> &ShipController {
        &self._ship._controller
    }
    pub fn get_controller_mut(&mut self) -> &mut ShipController {
        &mut self._ship._controller
    }
    pub fn get_bound_box(&self) -> &BoundingBox { self._ship.get_bound_box() }
    pub fn get_transform(&self) -> &TransformObjectData { self._ship.get_transform() }
    pub fn get_transform_mut(&self) -> &mut TransformObjectData {
        self._ship.get_transform_mut()
    }
    pub fn get_velocity(&self) -> &Vector3<f32> { self.get_controller().get_velocity() }
    pub fn can_manual_controll(&self) -> bool {
        ActorControllerState::None == self._actor_controller_state
    }

    pub fn manual_actor_attack(&mut self, game_client: &GameClient) {
        let project_scene_manager = game_client.get_project_scene_manager();
        let main_camera = project_scene_manager.get_main_camera();
        let fire_start = main_camera.get_camera_position();
        let fire_dir = -main_camera.get_camera_front() as Vector3<f32>;
        let mut target_position: Vector3<f32> = fire_start + &fire_dir * CHECK_TARGET_DISTANCE_MAX;
        project_scene_manager.get_height_map_collision_point(fire_start, &fire_dir, CHECK_TARGET_DISTANCE_MAX, &mut target_position);

        self._ship.ship_fire(game_client, &fire_start, &fire_dir, &target_position);
    }

    pub fn set_command_actor_attack(&mut self, target_position: &Vector3<f32>) {
        self.clear_command_of_actor();
        self._actor_controller_state = ActorControllerState::Attack;
        self._command_attack = true;
        self._command_rotate = true;
        self._target_position.clone_from(target_position);
    }

    pub fn set_command_actor_move(&mut self, target_position: &Vector3<f32>) {
        self.clear_command_of_actor();
        self._actor_controller_state = ActorControllerState::Move;
        self._command_move = true;
        self._command_rotate = true;
        self._target_position.clone_from(target_position);
    }

    pub fn clear_command_of_actor(&mut self) {
        self._actor_controller_state = ActorControllerState::None;
        self._command_attack = false;
        self._command_move = false;
        self._command_rotate = false;
    }

    fn roate_to_target(ship_controller: &mut ShipController, to_target_dir: &Vector3<f32>, actor_left: &Vector3<f32>, actor_front: &Vector3<f32>, delta_time: f32) -> bool {
        let front_dot_target = actor_front.dot(&to_target_dir);
        let velocity_yaw = ship_controller.get_velocity_yaw().abs();
        let yaw_delta = velocity_yaw * delta_time;
        let yaw_diff = (0.5 - front_dot_target * 0.5) * std::f32::consts::PI;
        let breaking_time = velocity_yaw / ship_controller._controller_data.borrow()._rotation_acceleration;
        let breaking_distance = velocity_yaw * breaking_time;
        if yaw_diff <= yaw_delta {
            let goal_yaw: f32 = to_target_dir.x.atan2(to_target_dir.z);
            ship_controller.set_yaw(goal_yaw);
            ship_controller.set_velocity_yaw(0.0);
            return true;
        }

        if breaking_distance < yaw_diff {
            let accel_yaw = if 0.0 <= actor_left.dot(&to_target_dir) { 1.0 } else { -1.0 };
            ship_controller.acceleration_yaw(accel_yaw);
        }

        false
    }

    fn move_to_target(
        ship_controller: &mut ShipController,
        to_target_dir: &Vector3<f32>,
        distance: f32,
        actor_front: &Vector3<f32>,
        actor_left: &Vector3<f32>,
        bound_box_radius: f32
    ) -> bool {
        let controller_data = ptr_as_ref(ship_controller._controller_data.as_ptr());
        let ground_velocty = math::make_vector_xz(ship_controller.get_velocity());
        let ground_speed = ship_controller.get_ground_speed();
        let breaking_time = ground_speed / controller_data._ground_acceleration;
        let breaking_distance = ground_speed * 0.5 * breaking_time;
        if distance.max(breaking_distance) <= bound_box_radius {
            return true;
        }

        if breaking_distance <= distance {
            let to_target_dot_velocity = to_target_dir.dot(&ground_velocty);
            let velocity_along_target = to_target_dir * to_target_dot_velocity;
            let (side_velocity_dir_along_target, side_speed_along_target) = math::safe_normalize_with_norm(&(ground_velocty - velocity_along_target));
            let mut side_move_time = side_speed_along_target / controller_data._ground_acceleration;
            let mut to_target_move_time = (2.0 * distance - to_target_dot_velocity) / controller_data._ground_acceleration;
            let max_time = to_target_move_time.max(side_move_time);
            if 0.0 < max_time {
                to_target_move_time /= max_time;
                side_move_time /= max_time;
            }
            let accel = math::safe_normalize(&(to_target_dir * to_target_move_time - side_velocity_dir_along_target * side_move_time));
            let forward_accel = actor_front.dot(&accel);
            let side_accel = actor_left.dot(&accel);
            ship_controller.acceleration_forward(forward_accel);
            ship_controller.acceleration_side(side_accel);
        }
        false
    }

    pub fn update_command_actor_move(&mut self, delta_time: f32) {
        if self._command_move || self._command_rotate {
            let ship_controller = ptr_as_mut(&self.get_ship()._controller);
            let (to_target_dir, distance) = math::make_normalize_xz_with_norm(&(&self._target_position - ship_controller.get_position()));
            if distance <= 0.0 {
                self.clear_command_of_actor();
                return;
            }

            let front = math::make_normalize_xz(self.get_ship().get_transform().get_front());
            let left = math::make_normalize_xz(self.get_ship().get_transform().get_left());

            if self._command_rotate {
                if ActorController::roate_to_target(ship_controller, &to_target_dir, &left, &front, delta_time) {
                    self._command_rotate = false;
                }
            }

            if self._command_move {
                if ActorController::move_to_target(
                    ship_controller,
                    &to_target_dir,
                    distance,
                    &front,
                    &left,
                    self.get_bound_box()._radius
                ) {
                    self._command_move = false;
                }
            }

            if false == self._command_move && false == self._command_rotate {
                self.clear_command_of_actor();
            }
        }
    }

    pub fn update_command_actor_attack(&mut self, delta_time: f32, game_client: &GameClient) {
        if self._command_attack || self._command_rotate {
            let ship_controller = ptr_as_mut(&self.get_ship()._controller);

            if self._command_rotate {
                let (to_target_dir, distance) = math::make_normalize_xz_with_norm(&(&self._target_position - ship_controller.get_position()));
                if 0.0 < distance {
                    let front = math::make_normalize_xz(self.get_ship().get_transform().get_front());
                    let left = math::make_normalize_xz(self.get_ship().get_transform().get_left());
                    if ActorController::roate_to_target(ship_controller, &to_target_dir, &left, &front, delta_time) {
                        self._command_rotate = false;
                    }
                } else {
                    self._command_rotate = false;
                }
            }

            if self._command_attack && false == self._command_rotate {
                // fire
                let fire_start = self.get_transform().get_position().clone_owned();
                let fire_dir = (&self._target_position - ship_controller.get_position()).normalize();
                let target_position: Vector3<f32> = &fire_start + &fire_dir * CHECK_TARGET_DISTANCE_MAX;
                self._ship.ship_fire(game_client, &fire_start, &fire_dir, &target_position);

                // stop
                self.clear_command_of_actor();
            }
        }
    }

    pub fn update_actor_controller(&mut self, game_client: &GameClient, delta_time: f32) {
        if ActorControllerState::Move == self._actor_controller_state {
            self.update_command_actor_move(delta_time);
        } else if ActorControllerState::Attack == self._actor_controller_state {
            self.update_command_actor_attack(delta_time, game_client);
        } else if false == self._is_player_actor {
            let ship_controller = ptr_as_mut(&self.get_ship()._controller);
            ship_controller.set_velocity_yaw(1.0);
            ship_controller.acceleration_forward(1.0);
        }

        // update ship
        self.get_ship_mut().update_ship(game_client, delta_time);
    }
}
