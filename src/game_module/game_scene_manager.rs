use std::collections::HashMap;

use nalgebra::{Vector2, Vector3};
use rust_engine_3d::core::engine_core::EngineCore;
use rust_engine_3d::effect::effect_manager::EffectManager;
use rust_engine_3d::scene::render_object::RenderObjectCreateInfo;
use rust_engine_3d::scene::scene_manager::SceneManager;
use rust_engine_3d::utilities::system::{newRcRefCell, ptr_as_mut, ptr_as_ref, RcRefCell};
use serde::{Deserialize, Serialize};

use crate::application::application::Application;
use crate::game_module::character::block::{Block, BlockCreateInfo};
use crate::game_module::character::character::CharacterCreateInfo;
use crate::game_module::character::character_manager::CharacterManager;
use crate::game_module::game_resource::GameResources;

type BlockCreateInfoMap = HashMap<String, BlockCreateInfo>;
type CharacterCreateInfoMap = HashMap<String, CharacterCreateInfo>;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(default)]
pub struct GameSceneDataCreateInfo {
    pub _scene_data_name: String,
    pub _blocks: BlockCreateInfoMap,
    pub _player: CharacterCreateInfoMap,
    pub _characters: CharacterCreateInfoMap,
    pub _start_point: Vector3<f32>,
}

pub struct GameSceneManager {
    pub _effect_manager: *const EffectManager,
    pub _scene_manager: *const SceneManager,
    pub _game_resources: *const GameResources,
    pub _character_manager: *const CharacterManager,
    pub _game_scene_name: String,
    pub _blocks: HashMap<u64, RcRefCell<Block>>,
    pub _block_id_generator: u64
}

impl GameSceneManager {
    pub fn get_scene_manager(&self) -> &SceneManager {
        ptr_as_ref(self._scene_manager)
    }
    pub fn get_scene_manager_mut(&self) -> &mut SceneManager {
        ptr_as_mut(self._scene_manager)
    }

    pub fn create_game_scene_manager() -> Box<GameSceneManager> {
        Box::new(GameSceneManager {
            _effect_manager: std::ptr::null(),
            _scene_manager: std::ptr::null(),
            _game_resources: std::ptr::null(),
            _character_manager: std::ptr::null(),
            _game_scene_name: String::new(),
            _blocks: HashMap::new(),
            _block_id_generator: 0,
        })
    }

    pub fn initialize_game_scene_manager(
        &mut self,
        application: &Application,
        engine_core: &EngineCore,
        window_size: &Vector2<i32>,
    ) {
        log::info!("initialize_game_scene_manager");
        self._scene_manager = engine_core.get_scene_manager();
        self._effect_manager = engine_core.get_effect_manager();
        self._character_manager = application.get_character_manager();
        self._game_resources = application.get_game_resources();
        engine_core.get_scene_manager_mut().initialize_scene_manager(
            engine_core.get_renderer_context(),
            engine_core.get_effect_manager(),
            engine_core.get_engine_resources(),
            window_size,
        )
    }

    pub fn generate_block_id(&mut self) -> u64 {
        let id = self._block_id_generator;
        self._block_id_generator += 1;
        id
    }

    pub fn register_block(&mut self, block: &RcRefCell<Block>) {
        self._blocks.insert(block.borrow().get_block_id(), block.clone());
    }

    pub fn unregister_block(&mut self, block: &RcRefCell<Block>) {
        self._blocks.remove(&block.borrow().get_block_id());
    }

    pub fn create_block(&mut self, block_name: &str, block_create_info: &BlockCreateInfo) {
        let game_resources = ptr_as_ref(self._game_resources);
        let block_data = game_resources.get_block_data(block_create_info._block_data_name.as_str());
        let render_object_create_info = RenderObjectCreateInfo {
            _model_data_name: block_data.borrow()._model_data_name.clone(),
            ..Default::default()
        };
        let render_object_data = self.get_scene_manager_mut().add_static_render_object(
            block_name,
            &render_object_create_info
        );
        let block_id = self.generate_block_id();
        let block = newRcRefCell(Block::create_block(
            block_id,
            block_name,
            block_data,
            &render_object_data,
            &block_create_info._position,
            &block_create_info._rotation,
            &block_create_info._scale
        ));
        self._blocks.insert(block_id, block.clone());
    }

    pub fn open_game_scene_data(&mut self, game_scene_data_name: &str) {
        log::info!("open_game_scene_data: {:?}", game_scene_data_name);
        self._game_scene_name = String::from(game_scene_data_name);
        let game_resources = ptr_as_ref(self._game_resources);

        if false == game_resources.has_game_scene_data(game_scene_data_name) {
            // TODO
        }

        // load scene
        let game_scene_data = game_resources.get_game_scene_data(game_scene_data_name).borrow();
        let scene_data_name = &game_scene_data._scene_data_name;
        self.get_scene_manager_mut()
            .open_scene_data(scene_data_name);

        // create blocks
        for (block_name, block_create_info) in game_scene_data._blocks.iter() {
            self.create_block(block_name, block_create_info);
        }

        // create player
        let character_manager = ptr_as_mut(self._character_manager);
        for (character_name, character_create_info) in game_scene_data._player.iter() {
            let _character = character_manager.create_character(character_name, character_create_info, true);
        }

        // create npc
        for (character_name, character_create_info) in game_scene_data._characters.iter() {
            let _character = character_manager.create_character(character_name, character_create_info, false);
        }
    }

    pub fn close_game_scene_data(&mut self) {
        self.get_scene_manager_mut().close_scene_data();
    }

    pub fn destroy_game_scene_manager(&mut self) {
        self.get_scene_manager_mut().destroy_scene_manager();
    }

    pub fn update_game_scene_manager(&mut self, _engine_core: &EngineCore, _delta_time: f64) {
    }
}
