use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use rust_engine_3d::renderer::renderer_context::RendererContext;
use rust_engine_3d::resource::resource::{EngineResources, get_unique_resource_name, APPLICATION_RESOURCE_PATH, ResourceDataMap};
use rust_engine_3d::utilities::system::{self, newRcRefCell, ptr_as_mut, ptr_as_ref, RcRefCell};
use serde_json::{self};
use crate::game_module::character::block::BlockData;

use crate::game_module::character::character::CharacterData;
use crate::game_module::game_scene_manager::GameSceneDataCreateInfo;

pub const GAME_SCENE_FILE_PATH: &str = "game_data/game_scenes";
pub const BLOCK_DATA_FILE_PATH: &str = "game_data/blocks";
pub const CHARACTER_DATA_FILE_PATH: &str = "game_data/characters";

pub const EXT_GAME_DATA: &str = "data";
pub const EXT_GAME_SCENE: &str = "game_scene";

pub const DEFAULT_GAME_DATA_NAME: &str = "default";

pub type GameSceneDataCreateInfoMap = ResourceDataMap<GameSceneDataCreateInfo>;
pub type BlockDataMap = ResourceDataMap<BlockData>;
pub type CharacterDataMap = ResourceDataMap<CharacterData>;

#[derive(Clone)]
pub struct GameResources {
    _engine_resources: *const EngineResources,
    _game_scene_data_create_infos_map: GameSceneDataCreateInfoMap,
    _block_data_map: BlockDataMap,
    _character_data_map: CharacterDataMap,
}

impl GameResources {
    pub fn create_game_resources() -> Box<GameResources> {
        Box::new(GameResources {
            _engine_resources: std::ptr::null(),
            _game_scene_data_create_infos_map: GameSceneDataCreateInfoMap::new(),
            _block_data_map: BlockDataMap::new(),
            _character_data_map: CharacterDataMap::new(),
        })
    }
    pub fn get_engine_resources(&self) -> &EngineResources {
        ptr_as_ref(self._engine_resources)
    }
    pub fn get_engine_resources_mut(&self) -> &mut EngineResources {
        ptr_as_mut(self._engine_resources)
    }
    pub fn collect_resources(&self, dir: &Path, extensions: &[&str]) -> Vec<PathBuf> {
        self.get_engine_resources().collect_resources(dir, extensions)
    }
    pub fn initialize_game_resources(&mut self, engine_resources: &EngineResources) {
        self._engine_resources = engine_resources;
    }
    pub fn load_game_resources(&mut self, renderer_context: &RendererContext) {
        self.load_game_scene_data(renderer_context);
        self.load_game_data();
    }
    pub fn destroy_game_resources(&mut self) {
        self.unload_game_data();
        self.unload_game_scene_data();
    }
    pub fn load_game_scene_data(&mut self, _renderer_context: &RendererContext) {
        log::info!("    load_game_scene_data");
        let game_scene_directory = PathBuf::from(GAME_SCENE_FILE_PATH);
        let game_scene_data_files: Vec<PathBuf> = self.collect_resources(&game_scene_directory, &[EXT_GAME_SCENE]);
        for game_scene_data_file in game_scene_data_files {
            let game_scene_data_name = get_unique_resource_name(&self._game_scene_data_create_infos_map, &game_scene_directory, &game_scene_data_file);
            let loaded_contents = system::load(&game_scene_data_file);
            let game_scene_data_create_info: GameSceneDataCreateInfo = serde_json::from_reader(loaded_contents).expect("Failed to deserialize.");
            self._game_scene_data_create_infos_map.insert(game_scene_data_name.clone(), newRcRefCell(game_scene_data_create_info));
        }
    }

    pub fn unload_game_scene_data(&mut self) {
        self._game_scene_data_create_infos_map.clear();
    }

    pub fn save_game_scene_data(&mut self, game_scene_data_name: &str, game_scene_data_create_info: &GameSceneDataCreateInfo) {
        let mut game_scene_data_filepath = PathBuf::from(APPLICATION_RESOURCE_PATH);
        game_scene_data_filepath.push(GAME_SCENE_FILE_PATH);
        game_scene_data_filepath.push(game_scene_data_name);
        game_scene_data_filepath.set_extension(EXT_GAME_DATA);
        let mut write_file = File::create(&game_scene_data_filepath).expect("Failed to create file");
        let mut write_contents: String = serde_json::to_string(&game_scene_data_create_info).expect("Failed to serialize.");
        write_contents = write_contents.replace(",\"", ",\n\"");
        write_file.write(write_contents.as_bytes()).expect("Failed to write");

        self._game_scene_data_create_infos_map.insert(String::from(game_scene_data_name), newRcRefCell(game_scene_data_create_info.clone()));
    }

    pub fn has_game_scene_data(&self, resource_name: &str) -> bool {
        self._game_scene_data_create_infos_map.get(resource_name).is_some()
    }

    pub fn get_game_scene_data(&self, resource_name: &str) -> &RcRefCell<GameSceneDataCreateInfo> {
        self._game_scene_data_create_infos_map.get(resource_name).unwrap()
    }

    // Game Data
    fn load_game_data(&mut self) {
        log::info!("    load_game_data");
        self.load_block_data();
        self.load_character_data();
    }

    fn unload_game_data(&mut self) {
        self.unload_character_data();
        self.unload_block_data();
    }

    // block data
    fn load_block_data(&mut self) {
        let game_data_directory = PathBuf::from(BLOCK_DATA_FILE_PATH);

        // load_block_data
        let game_data_files: Vec<PathBuf> = self.collect_resources(&game_data_directory, &[EXT_GAME_DATA]);
        for game_data_file in game_data_files {
            let block_data_name = get_unique_resource_name(&self._block_data_map, &game_data_directory, &game_data_file);
            let loaded_contents = system::load(&game_data_file);
            let block_data: BlockData = serde_json::from_reader(loaded_contents).expect("Failed to deserialize.");
            self._block_data_map.insert(block_data_name.clone(), newRcRefCell(block_data));
        }
    }

    fn unload_block_data(&mut self) {
        self._block_data_map.clear();
    }

    pub fn has_block_data(&self, resource_name: &str) -> bool {
        self._block_data_map.get(resource_name).is_some()
    }

    pub fn get_block_data(&self, resource_name: &str) -> &RcRefCell<BlockData> {
        self._block_data_map.get(resource_name).unwrap()
    }

    // character data
    fn load_character_data(&mut self) {
        let game_data_directory = PathBuf::from(CHARACTER_DATA_FILE_PATH);

        // load_character_data
        let game_data_files: Vec<PathBuf> = self.collect_resources(&game_data_directory, &[EXT_GAME_DATA]);
        for game_data_file in game_data_files {
            let character_data_name = get_unique_resource_name(&self._character_data_map, &game_data_directory, &game_data_file);
            let loaded_contents = system::load(&game_data_file);
            let character_data: CharacterData = serde_json::from_reader(loaded_contents).expect("Failed to deserialize.");
            self._character_data_map.insert(character_data_name.clone(), newRcRefCell(character_data));
        }
    }

    fn unload_character_data(&mut self) {
        self._character_data_map.clear();
    }

    pub fn has_character_data(&self, resource_name: &str) -> bool {
        self._character_data_map.get(resource_name).is_some()
    }

    pub fn get_character_data(&self, resource_name: &str) -> &RcRefCell<CharacterData> {
        self._character_data_map.get(resource_name).unwrap()
    }
}
