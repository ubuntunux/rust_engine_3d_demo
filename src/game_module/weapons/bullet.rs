use std::rc::Rc;

use nalgebra::Vector3;
use serde::{ Serialize, Deserialize };

use rust_engine_3d::renderer::render_object::RenderObjectData;
use rust_engine_3d::renderer::transform_object::TransformObjectData;
use rust_engine_3d::utilities::system::RcRefCell;
use crate::application::project_scene_manager::ProjectSceneManager;
use crate::game_module::actors::actor::ActorController;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug, Copy)]
pub enum BulletType {
    Beam,
    Gatling,
    Laser,
    Plasma,
    Shotgun
}

pub const BULLET_TYPES: [BulletType; 5] = [
    BulletType::Beam,
    BulletType::Gatling,
    BulletType::Laser,
    BulletType::Plasma,
    BulletType::Shotgun
];

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct BulletData {
    pub _bullet_type: BulletType,
    pub _shield_damage: f32,
    pub _hull_damage: f32,
    pub _bullet_speed: f32,
    pub _bullet_range: f32,
    pub _bullet_life_time: f32,
    pub _bullet_destroy_effects: Vec<String>,
    pub _bullet_destroy_sound_bank: String,
    pub _model_data_name: String
}

impl Default for BulletData {
    fn default() -> BulletData {
        BulletData {
            _bullet_type: BulletType::Beam,
            _shield_damage: 1.0,
            _hull_damage: 1.0,
            _bullet_speed: 100.0,
            _bullet_range: 100.0,
            _bullet_life_time: 10.0,
            _bullet_destroy_effects: Vec::new(),
            _bullet_destroy_sound_bank: "".to_string(),
            _model_data_name: "".to_string(),
        }
    }
}

pub struct Bullet {
    pub _bullet_data: *const BulletData,
    pub _owner_actor: *const ActorController,
    pub _is_alive: bool,
    pub _is_collided: bool,
    pub _elapsed_time: f32,
    pub _transform: *const TransformObjectData,
    pub _initial_position: Vector3<f32>,
    pub _initial_velocity: Vector3<f32>,
    pub _bullet_render_object: RcRefCell<RenderObjectData>,
}


// Implementation
impl Bullet {
    pub fn create_bullet(
        owner_actor: *const ActorController,
        initial_velocity: &Vector3<f32>,
        bullet_data: *const BulletData,
        bullet_render_object: &RcRefCell<RenderObjectData>,
    ) -> Rc<Bullet> {
        Rc::new(Bullet {
            _owner_actor: owner_actor,
            _transform: &bullet_render_object.borrow()._transform_object,
            _initial_position: bullet_render_object.borrow()._transform_object.get_position().clone_owned(),
            _initial_velocity: initial_velocity.clone_owned(),
            _bullet_data: bullet_data,
            _elapsed_time: 0.0,
            _is_alive: true,
            _is_collided: false,
            _bullet_render_object: bullet_render_object.clone(),
        })
    }
    pub fn get_owner_actor(&self) -> &ActorController {
        unsafe { &*self._owner_actor }
    }
    pub fn get_owner_actor_mut(&self) -> &mut ActorController { unsafe { &mut *(self._owner_actor as *mut ActorController) } }
    pub fn get_bullet_type(&self) -> BulletType { self.get_bullet_data()._bullet_type }
    pub fn get_bullet_data(&self) -> &BulletData { unsafe { &*self._bullet_data } }
    pub fn get_transform_object(&self) -> &TransformObjectData { unsafe { &*self._transform } }
    pub fn get_transform_object_mut(&self) -> &mut TransformObjectData { unsafe { &mut *(self._transform as *mut TransformObjectData) } }
    pub fn update_bullet(&mut self, delta_time: f32, project_scene_manager: &ProjectSceneManager) -> bool {
        if self._is_alive {
            let bullet_data = unsafe { &*self._bullet_data };

            // move bullet
            let transform = unsafe { &mut *(self._transform as *mut TransformObjectData) };
            let velocity = (&self._initial_velocity + transform.get_front() * bullet_data._bullet_speed) * delta_time;
            transform.move_position(&velocity);

            let current_position = transform.get_position();
            // check bullet range
            if self._is_alive {
                let move_distance = (current_position - &self._initial_position).norm();
                if bullet_data._bullet_life_time < self._elapsed_time || bullet_data._bullet_range < move_distance {
                    self._is_alive = false;
                }
            }

            // check bullet collision
            if self._is_alive {
                let floating_height = project_scene_manager.get_height_bilinear(current_position, 0);
                if current_position.y < floating_height {
                    self._is_alive = false;
                    self._is_collided = true;
                }
            }

            self._elapsed_time += delta_time;
        }
        self._is_alive
    }
}