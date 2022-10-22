use nalgebra::Vector3;
use serde::{ Serialize, Deserialize };

use rust_engine_3d::renderer::render_object::{RenderObjectData, RenderObjectCreateInfo};
use rust_engine_3d::renderer::transform_object::TransformObjectData;
use rust_engine_3d::utilities::system::{RcRefCell, newRcRefCell, ptr_as_ref, ptr_as_mut};
use crate::application::project_scene_manager::ProjectSceneManager;
use crate::game_module::actor_manager::calc_floating_height;
use crate::game_module::actors::actor::ActorController;
use crate::game_module::game_client::GameClient;
use crate::game_module::ship::ship_controller::{ShipController, ShipControllerData};
use crate::game_module::weapons::weapon::{WeaponTrait, WeaponData, BeamEmitter, WeaponSlotData};
use rust_engine_3d::utilities::bounding_box::BoundingBox;

#[derive(Serialize, Deserialize,Clone, Copy, Debug, PartialEq)]
pub enum ShipDataType {
    Scout,
    Tank,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct ShipDataCreateInfo {
    pub _ship_type: ShipDataType,
    pub _model_data_name: String,
    pub _hull_armor: f32,
    pub _shield_armor: f32,
    pub _max_hull: f32,
    pub _max_shields: f32,
    pub _weapon_solts: Vec<WeaponSlotData>,
    pub _controller_data_name: String,
}

impl Default for ShipDataCreateInfo {
    fn default() -> ShipDataCreateInfo {
        ShipDataCreateInfo {
            _ship_type: ShipDataType::Scout,
            _model_data_name: "".to_string(),
            _hull_armor: 0.0,
            _shield_armor: 0.0,
            _max_hull: 100.0,
            _max_shields: 10.0,
            _weapon_solts: vec![WeaponSlotData::default()],
            _controller_data_name: "".to_string(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ShipData {
    pub _ship_name: String,
    pub _ship_type: ShipDataType,
    pub _model_data_name: String,
    pub _hull_armor: f32,
    pub _shield_armor: f32,
    pub _max_hull: f32,
    pub _max_shields: f32,
    pub _weapon_solts: Vec<WeaponSlotData>,
    pub _contoller_data: RcRefCell<ShipControllerData>,
}

pub struct ShipInstance {
    pub _ship_data: RcRefCell<ShipData>,
    pub _hull: f32,
    pub _shields: f32,
    pub _render_object: RcRefCell<RenderObjectData>,
    pub _transform_object: *mut TransformObjectData,
    pub _controller: ShipController,
    pub _weapons: Vec<Box<dyn WeaponTrait>>,
    pub _current_weapons: Vec<*const dyn WeaponTrait>,
}

// Implementation
impl ShipData {
    pub fn create_ship_data(ship_data_name: &str, ship_data_create_info: &ShipDataCreateInfo, controller_data: &RcRefCell<ShipControllerData>) -> RcRefCell<ShipData> {
        newRcRefCell(ShipData {
            _ship_name: ship_data_name.to_string(),
            _ship_type: ship_data_create_info._ship_type,
            _model_data_name: ship_data_create_info._model_data_name.clone(),
            _hull_armor: ship_data_create_info._hull_armor,
            _shield_armor: ship_data_create_info._shield_armor,
            _max_hull: ship_data_create_info._max_hull,
            _max_shields: ship_data_create_info._max_shields,
            _weapon_solts: ship_data_create_info._weapon_solts.clone(),
            _contoller_data: controller_data.clone(),
        })
    }
}

impl ShipInstance {
    pub fn create_ship_instance(
        ship_data: &RcRefCell<ShipData>,
        render_object: &RcRefCell<RenderObjectData>
    ) -> ShipInstance {
        let transform_object: &TransformObjectData = &render_object.borrow()._transform_object;
        let floating_height = calc_floating_height(&render_object.borrow());
        ShipInstance {
            _ship_data: ship_data.clone(),
            _hull: 0.0,
            _shields: 0.0,
            _render_object: render_object.clone(),
            _transform_object: (transform_object as *const TransformObjectData as *mut TransformObjectData).clone(),
            _controller: ShipController::create_ship_controller(
                &ship_data.borrow()._contoller_data,
                transform_object.get_position(),
                transform_object.get_rotation(),
                floating_height
            ),
            _weapons: Vec::new(),
            _current_weapons: Vec::new(),
        }
    }

    pub fn initialize_ship_instance(&mut self, owner_actor: *const ActorController, project_scene_manager: &mut ProjectSceneManager) {
        let ship_data = unsafe { &*self._ship_data.as_ptr() };
        self._hull = ship_data._max_hull;
        self._shields = ship_data._max_shields;

        // add weapons
        for weapon_slot in self._ship_data.borrow()._weapon_solts.iter() {
            let weapon_data: RcRefCell<WeaponData> = project_scene_manager.get_project_resources().get_weapon_data("beam_emitter").clone();
            let mut weapon_slot_transform = TransformObjectData::new_transform_object_data();
            weapon_slot_transform.set_position(&weapon_slot._position);
            weapon_slot_transform.set_rotation(&weapon_slot._rotation);
            weapon_slot_transform.set_scale(&weapon_slot._scale);
            weapon_slot_transform.update_transform_object();
            let render_object_create_info = RenderObjectCreateInfo {
                _model_data_name: weapon_data.borrow()._model_data_name.clone(),
                _position: self.get_transform().get_position() + &weapon_slot._position,
                ..Default::default()
            };
            let weapon_render_object = project_scene_manager.add_skeletal_render_object("weapon", &render_object_create_info);
            let weapon = BeamEmitter::create_beam_emitter(
                owner_actor,
                &weapon_data,
                &weapon_slot_transform,
                &weapon_render_object,
            );
            self._current_weapons.push(weapon.as_ref());
            self._weapons.push(weapon);
        }
    }
    pub fn remove_ship_instance(&mut self, project_scene_manager: &mut ProjectSceneManager) {
        project_scene_manager.remove_skeletal_render_object(&self._render_object.borrow()._render_object_name);
        for weapon in self._weapons.iter_mut() {
            weapon.remove_weapon(project_scene_manager);
        }
        self._weapons.clear();
    }
    pub fn get_ship_data(&self) -> &ShipData { ptr_as_ref(self._ship_data.as_ptr()) }
    pub fn get_controller(&self) -> &ShipController {
        &self._controller
    }
    pub fn get_controller_mut(&mut self) -> &mut ShipController { &mut self._controller }
    pub fn get_bound_box(&self) -> &BoundingBox { &ptr_as_ref(self._render_object.as_ptr())._bound_box }
    pub fn get_transform(&self) -> &TransformObjectData { ptr_as_ref(self._transform_object) }
    pub fn get_transform_mut(&self) -> &mut TransformObjectData { ptr_as_mut(self._transform_object) }
    pub fn get_current_weapons(&self) -> &Vec<*const dyn WeaponTrait> {
        &self._current_weapons
    }
    pub fn get_hull_point(&self) -> f32 {
        self._hull
    }
    pub fn get_max_hull_point(&self) -> f32 {
        self.get_ship_data()._max_hull
    }
    pub fn get_shield_point(&self) -> f32 {
        self._shields
    }
    pub fn get_max_shield_point(&self) -> f32 {
        self.get_ship_data()._max_shields
    }
    pub fn ship_fire(&mut self, game_client: &GameClient, fire_start: &Vector3<f32>, fire_dir: &Vector3<f32>, target_position: &Vector3<f32>) {
        for weapon in self._current_weapons.iter() {
            let weapon: &mut dyn WeaponTrait = unsafe { &mut *(*weapon as *mut dyn WeaponTrait) };
            weapon.weapon_fire(game_client, fire_start, fire_dir, target_position);
        }
    }
    pub fn update_ship(&mut self, game_client: &GameClient, delta_time: f32) {
        let ship_transform = ptr_as_mut(self._transform_object);

        self._controller.update_controller(game_client, ship_transform, delta_time);

        ship_transform.set_rotation(self._controller.get_rotation());
        ship_transform.set_position(self._controller.get_position());
        ship_transform.update_matrix();

        for weapon in self._weapons.iter_mut() {
            weapon.update_weapon(ship_transform, delta_time);
        }
    }
}