use nalgebra::{ Vector2, Vector3 };
use serde::{ Serialize, Deserialize };

use rust_engine_3d::renderer::transform_object::TransformObjectData;
use rust_engine_3d::utilities::math::{TWO_PI, make_normalize_xz, make_normalize_xz_with_norm};
use rust_engine_3d::utilities::system::RcRefCell;
use crate::game_module::game_client::GameClient;
use crate::game_module::game_constants::GRAVITY;

// Declare
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ShipControllerDataType {
    ShipController,
    TankController,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct ShipControllerData {
    pub _controller_data_type: ShipControllerDataType,
    pub _max_ground_speed: f32,
    pub _ground_acceleration: f32,
    pub _floating_acceleration: f32,
    pub _side_step_roll: f32,
    pub _side_step_roll_speed: f32,
    pub _boost_acceleration: f32,
    pub _max_rotation_speed: f32,
    pub _rotation_acceleration: f32
}

impl Default for ShipControllerData {
    fn default() -> ShipControllerData {
        ShipControllerData {
            _controller_data_type: ShipControllerDataType::ShipController,
            _max_ground_speed: 50.0,
            _ground_acceleration: 50.0,
            _floating_acceleration: 30.0,
            _side_step_roll: 0.3,
            _side_step_roll_speed: 2.0,
            _boost_acceleration: 1.5,
            _max_rotation_speed: 10.0,
            _rotation_acceleration: 100.0
        }
    }
}

#[derive(Clone, Debug)]
pub struct ShipController {
    pub _controller_data: RcRefCell<ShipControllerData>,
    pub _prev_velocity: Vector3<f32>,
    pub _velocity: Vector3<f32>,
    pub _ground_speed: f32,
    pub _floating_height: f32,
    pub _acceleration: Vector3<f32>,
    pub _rotation_velocity: Vector2<f32>,
    pub _rotation_acceleration: Vector2<f32>,
    pub _position: Vector3<f32>,
    pub _rotation: Vector3<f32>,
    pub _boost: bool,
    pub _on_ground: bool,
}

// implementation
impl ShipController {
    pub fn create_ship_controller(
        controller_data: &RcRefCell<ShipControllerData>,
        postion: &Vector3<f32>,
        rotation: &Vector3<f32>,
        floating_height: f32
    ) -> ShipController {
        ShipController {
            _controller_data: controller_data.clone(),
            _prev_velocity: Vector3::zeros(),
            _velocity: Vector3::zeros(),
            _ground_speed: 0.0,
            _floating_height: floating_height,
            _acceleration: Vector3::zeros(),
            _rotation_acceleration: Vector2::zeros(),
            _rotation_velocity: Vector2::zeros(),
            _position: postion.clone_owned(),
            _rotation: rotation.clone_owned(),
            _boost: false,
            _on_ground: false,
        }
    }

    pub fn boost_on(&mut self) { self._boost = true; }
    pub fn acceleration_side(&mut self, acceleration: f32) { self._acceleration.x = acceleration; }
    pub fn acceleration_vertical(&mut self, acceleration: f32) { self._acceleration.y = acceleration; }
    pub fn acceleration_forward(&mut self, acceleration: f32) { self._acceleration.z = acceleration; }
    pub fn get_acceleration(&self) -> &Vector3<f32> { &self._acceleration }
    pub fn set_acceleration(&mut self, acceleration: &Vector3<f32>) { self._acceleration.clone_from(acceleration); }
    pub fn acceleration_pitch(&mut self, acceleration: f32) { self._rotation_acceleration.x = acceleration; }
    pub fn acceleration_yaw(&mut self, acceleration: f32) { self._rotation_acceleration.y = acceleration; }
    pub fn get_velocity_pitch(&self) -> f32 { self._rotation_velocity.x as f32 }
    pub fn set_velocity_pitch(&mut self, pitch: f32) { self._rotation_velocity.x = pitch; }
    pub fn get_velocity_yaw(&self) -> f32 { self._rotation_velocity.y as f32 }
    pub fn set_velocity_yaw(&mut self, yaw: f32) { self._rotation_velocity.y = yaw; }
    pub fn get_ground_speed(&self) -> f32 { self._ground_speed }
    pub fn get_velocity(&self) -> &Vector3<f32> { &self._velocity }
    pub fn set_velocity(&mut self, velocity: &Vector3<f32>) { self._velocity.clone_from(velocity); }
    pub fn get_position(&self) -> &Vector3<f32> { &self._position }
    pub fn set_position(&mut self, position: &Vector3<f32>) { self._position.clone_from(position); }
    pub fn get_rotation(&self) -> &Vector3<f32> { &self._rotation }
    pub fn get_pitch(&self) -> f32 { self._rotation.x }
    pub fn get_yaw(&self) -> f32 { self._rotation.y }
    pub fn get_roll(&self) -> f32 { self._rotation.z }
    pub fn set_rotation(&mut self, rotation: &Vector3<f32>) { self._rotation.clone_from(rotation); }
    pub fn set_pitch(&mut self, pitch: f32) { self._rotation.x = pitch; }
    pub fn set_yaw(&mut self, yaw: f32) { self._rotation.y = yaw; }
    pub fn set_roll(&mut self, roll: f32) { self._rotation.z = roll; }
    pub fn update_controller(&mut self, game_client: &GameClient, transform: &TransformObjectData, delta_time: f32) {
        let mut goal_roll = 0.0;

        let controller_data = self._controller_data.borrow();
        let boost_acceleration = if self._boost { controller_data._boost_acceleration } else { 1.0 };
        let dir_forward = make_normalize_xz(transform.get_front());
        let dir_side = make_normalize_xz(transform.get_left());

        if 0.0 != self._acceleration.x || 0.0 != self._acceleration.z {
            self._velocity += dir_side * self._acceleration.x * controller_data._ground_acceleration * boost_acceleration * delta_time;
            goal_roll = -controller_data._side_step_roll * self._acceleration.x;
        }

        if 0.0 != self._acceleration.y {
            self._velocity.y += self._acceleration.y * controller_data._floating_acceleration * boost_acceleration * delta_time;
        }

        if 0.0 != self._acceleration.z {
            self._velocity += dir_forward * self._acceleration.z * controller_data._ground_acceleration * boost_acceleration * delta_time;
        }

        // ground speed
        if 0.0 != self._velocity.x || 0.0 != self._velocity.z {
            let mut ground_velocity = Vector3::new(self._velocity.x, 0f32, self._velocity.z);
            let mut ground_speed = ground_velocity.norm();
            if controller_data._max_ground_speed < ground_speed {
                ground_velocity = ground_velocity / ground_speed * controller_data._max_ground_speed;
                ground_speed = controller_data._max_ground_speed;
            }

            let acceleration_dir = make_normalize_xz(&(self._acceleration.x * &dir_side + self._acceleration.z * &dir_forward));
            let velocity_along_acceleration = acceleration_dir * acceleration_dir.dot(&ground_velocity);
            let (reduce_velocity_dir, mut reduce_velocity_speed) = make_normalize_xz_with_norm(&(&ground_velocity - &velocity_along_acceleration));

            // friction
            reduce_velocity_speed = 0f32.max(reduce_velocity_speed - controller_data._ground_acceleration * delta_time);
            let ground_velocity = velocity_along_acceleration + reduce_velocity_dir * reduce_velocity_speed;

            self._velocity.x = ground_velocity.x;
            self._velocity.z = ground_velocity.z;
            self._ground_speed = ground_speed;
        }

        // apply gravity
        if 0.0 == self._acceleration.y && false == self._on_ground {
            self._velocity.y -= GRAVITY * delta_time;
        }

        // apply velocity
        let mut position = &self._position + &self._velocity * delta_time;
        if position != self._position || false == self._on_ground {
            self._on_ground = false;
            let project_scene_manager = game_client.get_project_scene_manager();
            let floating_height = project_scene_manager.get_height_bilinear(&position, 0) + self._floating_height;
            if position.y < floating_height {
                position.y = floating_height;
                self._velocity.y = 0.0;
                self._on_ground = true;
            }
            self._position = position;
        }

        if 0.0 != self._rotation_acceleration.x || 0.0 != self._rotation_acceleration.y {
            // rotation acceleration
            self._rotation_velocity += &self._rotation_acceleration * controller_data._rotation_acceleration * delta_time;
            let rotation_speed: f32 = self._rotation_velocity.norm();
            if controller_data._max_rotation_speed < rotation_speed {
                self._rotation_velocity = &self._rotation_velocity / rotation_speed * controller_data._max_rotation_speed;
            }
        } else if 0.0 != self._rotation_velocity.x || 0.0 != self._rotation_velocity.y {
            // rotation damping
            let rotation_speed: f32 = self._rotation_velocity.norm();
            let rotation_damping = controller_data._rotation_acceleration * delta_time;
            self._rotation_velocity = &self._rotation_velocity / rotation_speed * 0.0f32.max(rotation_speed - rotation_damping);
        }

        // roll
        let mut roll = self._rotation.z;
        if goal_roll != roll {
            let roll_diff = goal_roll - roll;
            let sign = if 0.0 <= roll_diff { 1.0 } else { -1.0 };
            let roll_speed = controller_data._side_step_roll_speed * delta_time * sign;
            if roll_diff.abs() < roll_speed.abs() {
                roll = goal_roll;
            } else {
                roll += roll_speed * roll_diff.abs() / controller_data._side_step_roll;
            }
            self._rotation.z = roll % TWO_PI;
        }

        // pitch, yaw
        self._rotation.x = (self._rotation.x + self.get_velocity_pitch() * delta_time) % TWO_PI;
        self._rotation.y = (self._rotation.y + self.get_velocity_yaw() * delta_time) % TWO_PI;

        // reset
        self._prev_velocity.clone_from(&self._velocity);
        self._acceleration = Vector3::zeros();
        self._rotation_acceleration = Vector2::zeros();
        self._boost = false;
    }
}