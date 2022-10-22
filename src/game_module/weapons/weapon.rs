use nalgebra::Vector3;
use serde::{ Serialize, Deserialize };

use rust_engine_3d::renderer::render_object::{RenderObjectData, RenderObjectCreateInfo};
use rust_engine_3d::renderer::transform_object::TransformObjectData;
use rust_engine_3d::utilities::system::{RcRefCell, newRcRefCell};
use crate::application::project_scene_manager::ProjectSceneManager;
use crate::game_module::actors::actor::ActorController;
use crate::game_module::game_client::GameClient;
use crate::game_module::game_constants::{FIRE_PITCH_MIN, FIRE_PITCH_MAX};
use crate::game_module::weapons::bullet::{BulletType, BulletData};


#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug, Copy)]
pub enum WeaponType {
    BeamEmitter,
    Gatling,
    LaserEmitter,
    PlasmaEmitter,
    Shotgun,
}

pub const WEAPON_TYPES: [WeaponType; 5] = [
    WeaponType::BeamEmitter,
    WeaponType::Gatling,
    WeaponType::LaserEmitter,
    WeaponType::PlasmaEmitter,
    WeaponType::Shotgun
];

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct WeaponSlotData {
    pub _position: Vector3<f32>,
    pub _rotation: Vector3<f32>,
    pub _scale: Vector3<f32>,
}

impl Default for WeaponSlotData {
    fn default() -> WeaponSlotData {
        WeaponSlotData {
            _position: Vector3::zeros(),
            _rotation: Vector3::zeros(),
            _scale: Vector3::new(1.0, 1.0, 1.0),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct WeaponDataCreateInfo {
    pub _weapon_type: WeaponType,
    pub _rate_of_fire: f32,
    pub _bullet_amount: i32,
    pub _bullet_data_name: String,
    pub _model_data_name: String,
    pub _muzzle_position: Vector3<f32>,
}

impl Default for WeaponDataCreateInfo {
    fn default() -> WeaponDataCreateInfo {
        WeaponDataCreateInfo {
            _weapon_type: WeaponType::BeamEmitter,
            _rate_of_fire: 1.0,
            _bullet_amount: 1,
            _bullet_data_name: "".to_string(),
            _model_data_name: "".to_string(),
            _muzzle_position: Vector3::zeros(),
        }
    }
}

#[derive(Clone)]
pub struct WeaponData {
    pub _weapon_data_name: String,
    pub _weapon_type: WeaponType,
    pub _rate_of_fire: f32,
    pub _bullet_amount: i32,
    pub _bullet_data: RcRefCell<BulletData>,
    pub _model_data_name: String,
    pub _muzzle_position: Vector3<f32>,
}

pub trait WeaponTrait {
    fn initialize_weapon(&mut self);
    fn remove_weapon(&mut self, project_scene_manager: &mut ProjectSceneManager);
    fn get_owner_actor(&self) -> &ActorController;
    fn get_bullet_type(&self) -> BulletType;
    fn get_bullet_data(&self) -> &BulletData;
    fn get_weapon_type(&self) -> WeaponType;
    fn get_weapon_data(&self) -> &WeaponData;
    fn get_weapon_render_object(&self) -> &RcRefCell<RenderObjectData>;
    fn weapon_fire(&mut self, game_client: &GameClient, fire_start: &Vector3<f32>, fire_dir: &Vector3<f32>, target_position: &Vector3<f32>);
    fn update_weapon(&mut self, ship_transform_object: &TransformObjectData, delta_time: f32);
}

pub struct BeamEmitter {
    pub _owner_actor: *const ActorController,
    pub _weapon_data: RcRefCell<WeaponData>,
    pub _weapon_slot_transform: TransformObjectData,
    pub _transform_object: TransformObjectData,
    pub _muzzle_position: Vector3<f32>,
    pub _weapon_render_object: RcRefCell<RenderObjectData>,
}

// Implementation
impl WeaponData {
    pub fn create_weapon_data(weapon_data_name: &str, weapon_data_create_info: &WeaponDataCreateInfo, bullet_data: &RcRefCell<BulletData>) -> RcRefCell<WeaponData> {
        newRcRefCell(WeaponData {
            _weapon_data_name: weapon_data_name.to_string(),
            _weapon_type: weapon_data_create_info._weapon_type,
            _rate_of_fire: weapon_data_create_info._rate_of_fire,
            _bullet_amount: weapon_data_create_info._bullet_amount,
            _bullet_data: bullet_data.clone(),
            _model_data_name: weapon_data_create_info._model_data_name.clone(),
            _muzzle_position: weapon_data_create_info._muzzle_position.clone_owned(),
        })
    }
}

impl BeamEmitter {
    pub fn create_beam_emitter(
        owner_actor: *const ActorController,
        weapon_data: &RcRefCell<WeaponData>,
        weapon_slot_transform: &TransformObjectData,
        weapon_render_object: &RcRefCell<RenderObjectData>,
    ) -> Box<BeamEmitter> {
        Box::new(BeamEmitter {
            _weapon_data: weapon_data.clone(),
            _owner_actor: owner_actor,
            _weapon_slot_transform: weapon_slot_transform.clone(),
            _transform_object: TransformObjectData::new_transform_object_data(),
            _weapon_render_object: weapon_render_object.clone(),
            _muzzle_position: Vector3::zeros(),
        })
    }
}

impl WeaponTrait for BeamEmitter {
    fn initialize_weapon(&mut self) {
    }
    fn remove_weapon(&mut self, project_scene_manager: &mut ProjectSceneManager) {
        project_scene_manager.remove_skeletal_render_object(&self._weapon_render_object.borrow()._render_object_name);
    }
    fn get_owner_actor(&self) -> &ActorController { unsafe { &*self._owner_actor } }
    fn get_bullet_type(&self) -> BulletType { self.get_bullet_data()._bullet_type }
    fn get_bullet_data(&self) -> &BulletData { unsafe { &*self.get_weapon_data()._bullet_data.as_ptr() } }
    fn get_weapon_type(&self) -> WeaponType { self.get_weapon_data()._weapon_type }
    fn get_weapon_data(&self) -> &WeaponData { unsafe { &*self._weapon_data.as_ptr() } }
    fn get_weapon_render_object(&self) -> &RcRefCell<RenderObjectData> { &self._weapon_render_object }
    fn weapon_fire(&mut self, game_client: &GameClient, fire_start: &Vector3<f32>, fire_dir: &Vector3<f32>, target_position: &Vector3<f32>) {
        let d: f32 = fire_dir.dot(&(&self._muzzle_position - fire_start));
        let new_target_position: Vector3<f32> = &self._muzzle_position + (target_position - fire_start) - fire_dir * d;
        let to_target: Vector3<f32> = (new_target_position - &self._muzzle_position).normalize();
        let muzzle_pitch: f32 = FIRE_PITCH_MIN.max(FIRE_PITCH_MAX.min(-to_target.y.asin()));
        let muzzle_front = self._transform_object.get_front();
        let rotation: Vector3<f32> = Vector3::new(muzzle_pitch, muzzle_front.x.atan2(muzzle_front.z), 0.0);
        let render_object_create_info = RenderObjectCreateInfo {
            _model_data_name: self.get_bullet_data()._model_data_name.clone(),
            _position: self._muzzle_position.clone_owned(),
            _rotation: rotation,
            ..Default::default()
        };

        game_client.get_weapon_manager_mut().fire_bullet(self, &render_object_create_info);
    }
    fn update_weapon(&mut self, ship_transform_object: &TransformObjectData, _delta_time: f32) {
        let weapon_world_matrix = &ship_transform_object._matrix * &self._weapon_slot_transform._matrix;
        self._transform_object.set_position_rotation_scale(&weapon_world_matrix);
        if self._transform_object.update_transform_object() {
            let muzzle_position = &self.get_weapon_data()._muzzle_position;
            self._muzzle_position =
                self._transform_object.get_left() * muzzle_position.x +
                self._transform_object.get_up() * muzzle_position.y +
                self._transform_object.get_front() * muzzle_position.z +
                self._transform_object.get_position();
        }
        self._weapon_render_object.borrow_mut()._transform_object.set_position_rotation_scale(&weapon_world_matrix);
    }
}