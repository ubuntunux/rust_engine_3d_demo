use nalgebra::{Vector3};
use rust_engine_3d::scene::animation::AnimationPlayArgs;
use rust_engine_3d::scene::mesh::MeshData;
use rust_engine_3d::scene::render_object::{AnimationLayer, RenderObjectData};
use rust_engine_3d::utilities::bounding_box::BoundingBox;
use rust_engine_3d::utilities::system::{ptr_as_mut, ptr_as_ref, RcRefCell};
use crate::game_module::character::animation_blend_mask::AnimationBlendMasks;

use crate::game_module::character::character::*;
use crate::game_module::game_constants::*;


impl Default for CharacterData {
    fn default() -> CharacterData {
        CharacterData {
            _character_type: CharacterDataType::UrsusArctos,
            _model_data_name: String::default(),
            _idle_animation_mesh: String::default(),
            _walk_animation_mesh: String::default(),
            _jump_animation_mesh: String::default(),
            _attack_animation_mesh: String::default(),
            _max_hp: 100,
        }
    }
}

impl CharacterProperty {
    pub fn create_character_property() -> CharacterProperty {
        CharacterProperty {
            _hp: 0.0,
        }
    }
}

impl CharacterController {
    pub fn create_character_controller() -> CharacterController {
        CharacterController {
            _position: Vector3::zeros(),
            _rotation: Vector3::zeros(),
            _scale: Vector3::new(1.0, 1.0, 1.0),
            _velocity: Vector3::zeros(),
            _is_jump: false,
            _is_ground: false,
            _move_direction: 0.0
        }
    }

    pub fn initialize(&mut self) {
        self._position = Vector3::zeros();
        self._rotation = Vector3::zeros();
        self._scale = Vector3::new(1.0, 1.0, 1.0);
        self._velocity = Vector3::zeros();
        self._is_ground = true;
        self._is_jump = false;
        self._move_direction = 0.0;
    }

    pub fn is_stop(&self) -> bool {
        self._velocity.x == 0.0 && self._velocity.y == 0.0
    }

    pub fn set_move_walk(&mut self, is_left: bool) {
        self._move_direction = if is_left { -1.0 } else { 1.0 };
    }

    pub fn set_move_jump(&mut self) {
        if self._is_ground {
            self._is_jump = true;
        }
    }

    pub fn get_direction(&self) -> f32 {
        if self._rotation.y.is_sign_positive() { -1.0 } else { 1.0 }
    }

    pub fn set_direction(&mut self, direction: f32) {
        self._rotation.y = direction * std::f32::consts::PI * -0.5;
    }

    pub fn set_on_ground(&mut self, ground_height: f32) {
        self._position.y = ground_height;
        self._is_ground = true;
        self._velocity.y = 0.0;
    }

    pub fn update_character_controller(&mut self, _actor_bound_box: &BoundingBox, blocks: &Vec<*const RenderObjectData>, delta_time: f32) {
        let prev_position = self._position.clone_owned();

        // move on ground
        if 0.0 != self._move_direction {
            self._velocity.x = self._move_direction * PLAYER_MOVE_SPEED;
            self._position.x += self._velocity.x * delta_time;
            self._rotation.y = self._move_direction * std::f32::consts::PI * -0.5;
        } else {
            self._velocity.x = 0.0;
        }

        if self._is_jump {
            self._velocity.y = PLAYER_JUMP_SPEED;
            self._is_ground = false;
        }

        // fall
        self._velocity.y -= GRAVITY * delta_time;
        self._position.y += self._velocity.y * delta_time;
        if self._position.y <= GROUND_HEIGHT {
            self.set_on_ground(GROUND_HEIGHT);
        }

        for block in blocks.iter() {
            let block_bound_box = &ptr_as_ref(*block)._bound_box;
            if block_bound_box.collide_bound_box(&self._position) {
                if block_bound_box.collide_bound_box_x(&prev_position) && block_bound_box._max.y <= prev_position.y {
                    self.set_on_ground(block_bound_box._max.y)
                } else if block_bound_box.collide_bound_box_y(&prev_position) && false == block_bound_box.collide_bound_box_x(&prev_position) {
                    self._position.x = prev_position.x;
                }
            }
        }

        // reset
        self._is_jump = false;
        self._move_direction = 0.0;
    }
}

impl CharacterBehavior {
    pub fn create_character_behavior() -> CharacterBehavior {
        CharacterBehavior {
            _move_time: 0.0,
        }
    }

    pub fn update_behavior(&mut self, character: &mut Character, delta_time: f32) {
        character.set_move_walk(self._move_time < 2.0);
        self._move_time += delta_time;
        if 4.0 <= self._move_time {
            self._move_time = 0.0;
        }
    }
}


impl Character {
    pub fn create_character_instance(
        character_id: u64,
        is_player: bool,
        character_name: &str,
        character_data: &RcRefCell<CharacterData>,
        render_object: &RcRefCell<RenderObjectData>,
        idle_animation: &RcRefCell<MeshData>,
        walk_animation: &RcRefCell<MeshData>,
        jump_animation: &RcRefCell<MeshData>,
        attack_animation: &RcRefCell<MeshData>,
        animation_blend_masks: *const AnimationBlendMasks,
        position: &Vector3<f32>,
        rotation: &Vector3<f32>,
        scale: &Vector3<f32>
    ) -> Character {
        let mut character = Character {
            _character_id: character_id,
            _is_player: is_player,
            _character_name: String::from(character_name),
            _character_data: character_data.clone(),
            _render_object: render_object.clone(),
            _character_property: Box::new(CharacterProperty::create_character_property()),
            _controller: Box::new(CharacterController::create_character_controller()),
            _behavior: Box::new(CharacterBehavior::create_character_behavior()),
            _move_animation_state: MoveAnimationState::NONE,
            _action_animation_state: ActionAnimationState::NONE,
            _idle_animation: idle_animation.clone(),
            _walk_animation: walk_animation.clone(),
            _jump_animation: jump_animation.clone(),
            _attack_animation: attack_animation.clone(),
            _animation_blend_masks: animation_blend_masks
        };
        character._controller._position.clone_from(position);
        character._controller._rotation.clone_from(rotation);
        character._controller._scale.clone_from(scale);
        character
    }
    pub fn get_character_id(&self) -> u64 { self._character_id }

    pub fn set_move_animation(&mut self, move_animation_state: MoveAnimationState) {
        let mut animation_info = AnimationPlayArgs::default();
        let mut render_object = self._render_object.borrow_mut();
        match move_animation_state {
            MoveAnimationState::IDLE => {
                render_object.set_animation(&self._idle_animation, &animation_info, AnimationLayer::BaseLayer);
            },
            MoveAnimationState::WALK => {
                render_object.set_animation(&self._walk_animation, &animation_info, AnimationLayer::BaseLayer);
            },
            MoveAnimationState::JUMP => {
                animation_info._animation_loop = false;
                render_object.set_animation(&self._jump_animation, &animation_info, AnimationLayer::BaseLayer);
            },
            _ => ()
        }
        self._move_animation_state = move_animation_state;
        self.update_animation_blend_masks();
    }

    pub fn set_action_animation(&mut self, action_animation_state: ActionAnimationState) {
        let mut animation_info = AnimationPlayArgs::default();
        let mut render_object = self._render_object.borrow_mut();
        let additive_animation_play_info = render_object.get_animation_play_info(AnimationLayer::AdditiveLayer);
        match action_animation_state {
            ActionAnimationState::ATTACK => {
                if self._action_animation_state == ActionAnimationState::NONE || CONTINUOUS_ATTACK_TIME < additive_animation_play_info._animation_play_time {
                    animation_info._animation_loop = false;
                    animation_info._force_animation_setting = true;
                    animation_info._animation_fade_out_time = 0.1;
                    render_object.set_animation(&self._attack_animation, &animation_info, AnimationLayer::AdditiveLayer);
                }
            },
            _ => ()
        }
        self._action_animation_state = action_animation_state;
        self.update_animation_blend_masks();
    }

    pub fn is_move_state(&self, move_state: MoveAnimationState) -> bool {
        move_state == self._move_animation_state
    }

    pub fn set_move_idle(&mut self) {
        self.set_move_animation(MoveAnimationState::IDLE);
    }

    pub fn set_move_walk(&mut self, is_left: bool) {
        self._controller.set_move_walk(is_left);
        if false == self.is_move_state(MoveAnimationState::WALK) && self._controller._is_ground {
            self.set_move_animation(MoveAnimationState::WALK);
        }
    }

    pub fn set_move_jump(&mut self) {
        if self._controller._is_ground {
            self._controller.set_move_jump();
            self.set_move_animation(MoveAnimationState::JUMP);
        }
    }

    pub fn is_action(&self, action: ActionAnimationState) -> bool {
        action == self._action_animation_state
    }

    pub fn set_action_idle(&mut self) {
        self.set_action_animation(ActionAnimationState::NONE);
    }

    pub fn set_action_attack(&mut self) {
        self.set_action_animation(ActionAnimationState::ATTACK);
    }

    pub fn is_attacking(&self) -> bool {
        if self.is_action(ActionAnimationState::ATTACK) {
            let animation_play_infos = &ptr_as_ref(self._render_object.as_ptr())._animation_play_infos;
            let animation_play_info = &animation_play_infos[AnimationLayer::AdditiveLayer as usize];
            let attack_time: f32 = 0.15;
            return animation_play_info._prev_animation_play_time < attack_time && attack_time <= animation_play_info._animation_play_time;
        }
        false
    }

    pub fn get_attack_point(&self) -> Vector3<f32> {
        self._controller._position + Vector3::new(self._controller.get_direction(), 1.0, 0.0)
    }

    pub fn get_position(&self) -> &Vector3<f32> {
        &self._controller._position
    }

    pub fn collide_bound_box(&self, pos: &Vector3<f32>) -> bool {
        self._render_object.borrow()._bound_box.collide_in_radius(pos)
    }

    pub fn update_transform(&mut self) {
        let mut render_object = self._render_object.borrow_mut();
        render_object._transform_object.set_position(&self._controller._position);
        render_object._transform_object.set_rotation(&self._controller._rotation);
        render_object._transform_object.set_scale(&self._controller._scale);
    }

    pub fn update_animation_blend_masks(&self) {
        let render_object = ptr_as_mut(self._render_object.as_ptr());
        if self.is_action(ActionAnimationState::ATTACK) {
            let additive_animation_play_info = render_object.get_animation_play_info(AnimationLayer::AdditiveLayer);
            if false == additive_animation_play_info._is_animation_end {
                if self.is_move_state(MoveAnimationState::IDLE) {
                    render_object.clear_animation_blend_masks(AnimationLayer::AdditiveLayer);
                } else {
                    render_object.set_animation_blend_masks(
                        &ptr_as_ref(self._animation_blend_masks)._upper_animation_mask,
                        AnimationLayer::AdditiveLayer
                    );
                }
            }
        }
    }

    pub fn update_character(&mut self, blocks: &Vec<*const RenderObjectData>, delta_time: f32) {
        if false == self._is_player {
            self._behavior.update_behavior(ptr_as_mut(self), delta_time);
        }

        self._controller.update_character_controller(&self._render_object.borrow()._bound_box,  blocks, delta_time);
        self.update_transform();

        let animation_play_infos = &ptr_as_ref(self._render_object.as_ptr())._animation_play_infos;

        if false == self.is_move_state(MoveAnimationState::IDLE) && self._controller.is_stop() {
            self.set_move_idle();
        }

        if self.is_action(ActionAnimationState::ATTACK) {
            if animation_play_infos[AnimationLayer::AdditiveLayer as usize]._is_animation_end {
                self.set_action_idle();
            }
        }
    }
}