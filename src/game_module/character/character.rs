use nalgebra::Vector3;
use rust_engine_3d::scene::mesh::MeshData;
use rust_engine_3d::scene::render_object::RenderObjectData;
use rust_engine_3d::utilities::system::RcRefCell;
use serde::{Deserialize, Serialize};
use crate::game_module::character::animation_blend_mask::AnimationBlendMasks;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MoveAnimationState {
    NONE,
    IDLE,
    WALK,
    JUMP,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ActionAnimationState {
    NONE,
    ATTACK
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum SpawnPointType {
    None,
    Player(SpawnPointData),
    NonPlayer(SpawnPointData),
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(default)]
pub struct SpawnPointData {
    pub _character_data_name: String,
    pub _position: Vector3<f32>,
    pub _rotation: Vector3<f32>
}

#[derive(Serialize, Deserialize,Clone, Copy, Debug, PartialEq)]
pub enum CharacterDataType {
    UrsusArctos,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct CharacterData {
    pub _character_type: CharacterDataType,
    pub _model_data_name: String,
    pub _idle_animation_mesh: String,
    pub _walk_animation_mesh: String,
    pub _jump_animation_mesh: String,
    pub _attack_animation_mesh: String,
    pub _max_hp: i32,
}

pub struct CharacterProperty {
    pub _hp: f32,
}

pub struct CharacterController {
    pub _position: Vector3<f32>,
    pub _rotation: Vector3<f32>,
    pub _scale: Vector3<f32>,
    pub _velocity: Vector3<f32>,
    pub _is_ground: bool,
    pub _is_jump: bool,
    pub _move_direction: f32
}

pub struct CharacterBehavior {
    pub _move_time: f32
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct CharacterCreateInfo {
    pub _character_data_name: String,
    pub _position: Vector3<f32>,
    pub _rotation: Vector3<f32>,
    pub _scale: Vector3<f32>,
}

pub struct Character {
    pub _character_name: String,
    pub _character_id: u64,
    pub _is_player: bool,
    pub _character_data: RcRefCell<CharacterData>,
    pub _render_object: RcRefCell<RenderObjectData>,
    pub _character_property: Box<CharacterProperty>,
    pub _controller: Box<CharacterController>,
    pub _behavior: Box<CharacterBehavior>,
    pub _move_animation_state: MoveAnimationState,
    pub _action_animation_state: ActionAnimationState,
    pub _idle_animation: RcRefCell<MeshData>,
    pub _walk_animation: RcRefCell<MeshData>,
    pub _jump_animation: RcRefCell<MeshData>,
    pub _attack_animation: RcRefCell<MeshData>,
    pub _animation_blend_masks: *const AnimationBlendMasks
}