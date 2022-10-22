use std::collections::HashMap;
use std::rc::Rc;

use rust_engine_3d::application::audio_manager::AudioLoop;
use rust_engine_3d::effect::effect_data::EffectCreateInfo;
use rust_engine_3d::renderer::render_object::RenderObjectCreateInfo;
use rust_engine_3d::utilities::system::{ptr_as_mut, ptr_as_ref};
use crate::game_module::game_client::GameClient;
use crate::game_module::weapons::bullet::Bullet;
use crate::game_module::weapons::weapon::WeaponTrait;


pub struct WeaponManager {
    pub _game_client: *const GameClient,
    pub _id_generator: u64,
    pub _bullets_array: HashMap<u64, Rc<Bullet>>,
}

impl WeaponManager {
    pub fn create_weapon_manager() -> Box<WeaponManager> {
        Box::new(WeaponManager {
            _game_client: std::ptr::null(),
            _id_generator: 0,
            _bullets_array: HashMap::new(),
        })
    }
    pub fn initialize_weapon_manager(&mut self, game_client: &GameClient) {
        self._game_client = game_client;
    }
    pub fn destroy_weapon_manager(&mut self) {
        self._bullets_array.clear();
    }
    pub fn get_game_client(&self) -> &GameClient { ptr_as_ref(self._game_client) }
    pub fn get_game_client_mut(&self) -> &mut GameClient { ptr_as_mut(self._game_client) }
    pub fn generate_id(&mut self) -> u64 {
        let id = self._id_generator;
        self._id_generator += 1;
        id
    }
    pub fn regist_bullets(&mut self, bullet: &Rc<Bullet>) -> u64 {
        let id = self.generate_id();
        self._bullets_array.insert(id, bullet.clone());
        id
    }
    pub fn unregist_bullets(&mut self, id: u64) {
        self._bullets_array.remove(&id);
    }
    pub fn fire_bullet(&mut self, weapon_ptr: *const dyn WeaponTrait, render_object_create_info: &RenderObjectCreateInfo) {
        let bullet_render_object = self.get_game_client().get_project_scene_manager_mut().add_static_render_object("bullet", render_object_create_info);
        self.get_game_client().get_audio_manager_mut().create_audio_instance("assaultrifle1", AudioLoop::ONCE);
        let weapon = ptr_as_ref(weapon_ptr);
        let bullet = Bullet::create_bullet(
            weapon.get_owner_actor(),
            weapon.get_owner_actor().get_velocity(),
            weapon.get_bullet_data(),
            &bullet_render_object
        );
        self.regist_bullets(&bullet);
    }

    pub fn update_weapon_manager(&mut self, delta_time: f32) {
        let game_client = ptr_as_ref(self._game_client);
        let project_scene_manager = game_client.get_project_scene_manager_mut();
        let audio_manager = game_client.get_audio_manager_mut();
        let actor_manager = game_client.get_actor_manager_mut();

        // update bullet
        let mut dead_bullets: Vec<(u64, *const Bullet)> = Vec::new();
        for (id, bullet_ptr) in self._bullets_array.iter() {
            let bullet = ptr_as_mut(bullet_ptr.as_ref());

            bullet.update_bullet(delta_time, project_scene_manager);

            // check hit
            if bullet._is_alive {
                let bullet_position = bullet.get_transform_object().get_position();
                let is_player_actor = bullet.get_owner_actor().is_player_actor();
                for actor_wrapper in actor_manager._actors.values() {
                    let actor = ptr_as_mut(actor_wrapper.as_ref());
                    if is_player_actor != actor.is_player_actor() {
                        let is_hit = {
                            let actor_bound_box = actor.get_bound_box();
                            let to_actor = &actor_bound_box._center - bullet_position;
                            to_actor.norm_squared() <= (actor_bound_box._radius * actor_bound_box._radius)
                        };

                        if is_hit {
                            actor_manager.remove_actor(project_scene_manager, actor);
                            bullet._is_alive = false;
                            bullet._is_collided = true;
                            break;
                        }
                    }
                }
            }

            if false == bullet._is_alive {
                dead_bullets.push((*id, bullet_ptr.as_ref()));
            }
        }

        // destroy bullets
        for (id, bullet_ptr) in dead_bullets.iter() {
            let bullet = ptr_as_mut(*bullet_ptr);
            if bullet._is_collided {
                let bullet_transform = bullet.get_transform_object();
                let bullet_data = bullet.get_bullet_data();
                let bullet_destroy_effect_count = bullet_data._bullet_destroy_effects.len();
                if 0 < bullet_destroy_effect_count {
                    let effect_index: usize = if 1 < bullet_destroy_effect_count { rand::random::<usize>() % bullet_destroy_effect_count } else { 0 };
                    let effect_create_info = EffectCreateInfo {
                        _effect_position: bullet_transform.get_position().clone_owned(),
                        _effect_rotation: bullet_transform.get_rotation().clone_owned(),
                        _effect_data_name: bullet_data._bullet_destroy_effects[effect_index].clone(),
                        ..Default::default()
                    };
                    project_scene_manager.add_effect(&effect_create_info._effect_data_name, &effect_create_info);
                }

                if false == bullet_data._bullet_destroy_sound_bank.is_empty() {
                    audio_manager.create_audio_instance_from_bank(&bullet_data._bullet_destroy_sound_bank, AudioLoop::ONCE);
                }
            }
            project_scene_manager.remove_static_render_object(&bullet._bullet_render_object.borrow()._render_object_name);

            self.unregist_bullets(*id);
        }
    }


    /*
    pub fn destroy(self) {
        for bullet in self.bullets:
            bullet.destroy(self.scene_manager)
    }

    pub fn set_damage(self, bullet, target_actor) {
        target_actor.set_damage(bullet.damage)
    }

    pub fn update_bullets(self, delta_time, actors) {
        player_actor = self.actor_manager.player_actor
        for bullet in self.bullets:
            if bullet.actor is player_actor:
                for actor in actors:
                    if bullet.check_collide(actor):
                        self.set_damage(bullet, actor)
            else:
                if bullet.check_collide(player_actor):
                    self.set_damage(bullet, player_actor)
            bullet.update_bullet(self.core_manager.debug_line_manager, delta_time)
    }
*/
}