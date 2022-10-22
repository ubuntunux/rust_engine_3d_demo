use std::collections::HashMap;
use std::rc::Rc;

use rust_engine_3d::renderer::render_object::{RenderObjectData, RenderObjectCreateInfo};
use rust_engine_3d::utilities::system::{ptr_as_mut, ptr_as_ref};
use crate::application::project_scene_manager::ProjectSceneManager;
use crate::game_module::actors::actor::ActorController;
use crate::game_module::game_client::GameClient;
use crate::game_module::level_datas::spawn_point::{SpawnPointType, ShipSpawnPointData};


pub type ActorMap = HashMap<u64, Rc<ActorController>>;

pub struct ActorManager {
    pub _game_client: *const GameClient,
    pub _id_generator: u64,
    pub _player_actor: *const ActorController,
    pub _actors: ActorMap,
}

pub fn calc_floating_height(render_object: &RenderObjectData) -> f32 {
    render_object._bound_box._size.y * 0.5 + 2.0
}

impl ActorManager {
    pub fn create_actor_manager() -> Box<ActorManager> {
        Box::new(ActorManager {
            _game_client: std::ptr::null(),
            _id_generator: 0,
            _player_actor: std::ptr::null(),
            _actors: HashMap::new(),
        })
    }

    pub fn initialize_actor_manager(&mut self, game_client: &GameClient) {
        self._game_client = game_client;
    }
    pub fn destroy_actor_manager(&mut self) {

    }
    pub fn get_game_client(&self) -> &GameClient { ptr_as_ref(self._game_client) }
    pub fn get_game_client_mut(&self) -> &mut GameClient { ptr_as_mut(self._game_client) }
    pub fn generate_id(&mut self) -> u64 {
        let id = self._id_generator;
        self._id_generator += 1;
        id
    }

    pub fn create_actor(&mut self, game_client: &GameClient, spawn_point_data: &ShipSpawnPointData, is_player_actor: bool) {
        let id = self.generate_id();
        let project_scene_manager = game_client.get_project_scene_manager_mut();

        // create ship render object
        let ship_data = game_client.get_project_resources().get_ship_data(&spawn_point_data._ship_data_name);
        let render_object_create_info = RenderObjectCreateInfo {
            _model_data_name: ship_data.borrow()._model_data_name.clone(),
            _position: spawn_point_data._position.clone_owned(),
            _rotation: spawn_point_data._rotation.clone_owned(),
            ..Default::default()
        };

        // regist ship render object
        let actor_render_object = project_scene_manager.add_skeletal_render_object(
            if is_player_actor { "Player" } else { "Enemy" },
            &render_object_create_info
        );

        // create actor
        let actor = ActorController::create_actor_controller(id, &ship_data, &actor_render_object, is_player_actor);
        if is_player_actor {
            self._player_actor = actor.as_ref();
        };
        ptr_as_mut(actor.as_ref()).initialize_actor(project_scene_manager);

        // regist actor
        self._actors.insert(id, actor);
    }
    pub fn remove_actor(&mut self, project_scene_manager: &mut ProjectSceneManager, actor: &mut ActorController) {
        actor.remove_actor(project_scene_manager);
        self._actors.remove(&actor.get_actor_id());
    }
    pub fn get_player_actor(&self) -> &ActorController {
        ptr_as_ref(self._player_actor)
    }
    pub fn get_player_actor_mut(&self) -> &mut ActorController { ptr_as_mut(self._player_actor) }
    pub fn spawn_actors(&mut self) {
        let game_client = ptr_as_ref(self._game_client);
        let level_data = game_client.get_project_scene_manager().get_level_data();
        for spawn_point_type in level_data._spawn_point_datas.iter() {
            match spawn_point_type {
                SpawnPointType::Player(spawn_point_data) => self.create_actor(game_client, spawn_point_data, true),
                SpawnPointType::NonPlayer(spawn_point_data) => self.create_actor(game_client, spawn_point_data, false),
                _ => (),
            }
        }
    }

    pub fn update_actor_manager(&mut self, delta_time: f32) {
        let game_client = ptr_as_ref(self._game_client);
        for actor_ref in self._actors.values() {
            let actor = ptr_as_mut(actor_ref.as_ref());
            if actor._is_player_actor {
                actor.update_actor_controller(game_client, delta_time);
            }
        }
    }
}