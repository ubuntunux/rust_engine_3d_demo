use std::fs::{ File };
use std::io::prelude::*;
use std::path::{ Path, PathBuf };

use serde_json::{ self };

use rust_engine_3d::resource::resource::{PROJECT_RESOURCE_PATH, ResourceData, ResourceDataMap, ProjectResourcesBase, EngineResources, get_unique_resource_name, RenderPassDataCreateInfoMap};
use rust_engine_3d::effect::effect_data::EffectData;
use rust_engine_3d::renderer::renderer_context::{ RendererContext };
use rust_engine_3d::utilities::system::{ self, RcRefCell, newRcRefCell };
use rust_engine_3d::renderer::font::FontData;
use rust_engine_3d::renderer::model::ModelData;
use rust_engine_3d::renderer::mesh::MeshData;
use rust_engine_3d::vulkan_context::texture::TextureData;
use rust_engine_3d::renderer::material::MaterialData;
use rust_engine_3d::renderer::material_instance::MaterialInstanceData;
use crate::application::project_scene_manager::SceneDataCreateInfo;
use crate::render_pass::render_pass;

pub const SCENE_FILE_PATH: &str = "scenes";
pub const EXT_SCENE: &str = "scene";
pub const DEFAULT_GAME_DATA_NAME: &str = "default";

pub type SceneDataCreateInfoMap = ResourceDataMap<SceneDataCreateInfo>;

#[derive(Clone)]
pub struct ProjectResources {
    _engine_resources: *const EngineResources,
    _scene_data_create_infos_map: SceneDataCreateInfoMap
}

impl ProjectResourcesBase for ProjectResources {
    fn initialize_project_resources(&mut self, engine_resources: &EngineResources) {
        self._engine_resources = engine_resources;
    }

    fn load_project_resources(&mut self, renderer_context: &RendererContext) {
        self.load_scene_data_list(renderer_context);
    }

    fn destroy_project_resources(&mut self, renderer_context: &RendererContext) {
        self.unload_scene_data_list(renderer_context);
    }
    fn load_graphics_data_list(&mut self, _renderer_context: &RendererContext) {
    }
    fn unload_graphics_data_list(&mut self, _renderer_context: &RendererContext) {
    }
    fn load_render_pass_data_create_infos(&mut self, renderer_context: &RendererContext, render_pass_data_create_info_map: &mut RenderPassDataCreateInfoMap) {
        render_pass::get_render_pass_data_create_infos(renderer_context, render_pass_data_create_info_map);
    }
    fn regist_resource(&mut self) {
    }
    fn unregist_resource(&mut self) {
    }
    fn has_audio_data(&self, resource_name: &str) -> bool {
        self.get_engine_resources().has_audio_data(resource_name)
    }
    fn get_audio_data(&self, resource_name: &str) -> &ResourceData {
        self.get_engine_resources_mut().get_audio_data(resource_name)
    }
    fn has_audio_bank_data(&self, resource_name: &str) -> bool {
        self.get_engine_resources().has_audio_bank_data(resource_name)
    }
    fn get_audio_bank_data(&self, resource_name: &str) -> &ResourceData {
        self.get_engine_resources_mut().get_audio_bank_data(resource_name)
    }
    fn has_effect_data(&self, resource_name: &str) -> bool {
        self.get_engine_resources().has_effect_data(resource_name)
    }
    fn get_effect_data(&self, resource_name: &str) -> &RcRefCell<EffectData> {
        self.get_engine_resources().get_effect_data(resource_name)
    }
    fn get_default_font_data(&self) -> &RcRefCell<FontData> {
        self.get_engine_resources().get_default_font_data()
    }
    fn get_font_data(&self, resource_name: &str) -> &RcRefCell<FontData> {
        self.get_engine_resources().get_font_data(resource_name)
    }
    fn has_model_data(&self, resource_name: &str) -> bool {
        self.get_engine_resources().has_model_data(resource_name)
    }
    fn get_model_data(&self, resource_name: &str) -> &RcRefCell<ModelData> {
        self.get_engine_resources().get_model_data(resource_name)
    }
    fn has_mesh_data(&self, resource_name: &str) -> bool {
        self.get_engine_resources().has_mesh_data(resource_name)
    }
    fn get_mesh_data(&self, resource_name: &str) -> &RcRefCell<MeshData> {
        self.get_engine_resources().get_mesh_data(resource_name)
    }
    fn has_texture_data(&self, resource_name: &str) -> bool {
        self.get_engine_resources().has_texture_data(resource_name)
    }
    fn get_texture_data(&self, resource_name: &str) -> &RcRefCell<TextureData> {
        self.get_engine_resources().get_texture_data(resource_name)
    }
    fn has_material_data(&self, resource_name: &str) -> bool {
        self.get_engine_resources().has_material_data(resource_name)
    }
    fn get_material_data(&self, resource_name: &str) -> &RcRefCell<MaterialData> {
        self.get_engine_resources().get_material_data(resource_name)
    }
    fn has_material_instance_data(&self, resource_name: &str) -> bool {
        self.get_engine_resources().has_material_instance_data(resource_name)
    }
    fn get_material_instance_data(&self, resource_name: &str) -> &RcRefCell<MaterialInstanceData> {
        self.get_engine_resources().get_material_instance_data(resource_name)
    }
}

impl ProjectResources {
    pub fn create_project_resources() -> Box<ProjectResources> {
        Box::new(ProjectResources {
            _engine_resources: std::ptr::null(),
            _scene_data_create_infos_map: SceneDataCreateInfoMap::new(),
        })
    }
    pub fn get_engine_resources(&self) -> &EngineResources {
        unsafe { &*self._engine_resources }
    }
    pub fn get_engine_resources_mut(&self) -> &mut EngineResources {
        unsafe { &mut *(self._engine_resources as *mut EngineResources) }
    }
    pub fn collect_resources(&self, dir: &Path, extensions: &[&str]) -> Vec<PathBuf> {
        self.get_engine_resources().collect_resources(dir, extensions)
    }

    // SceneData
    pub fn load_scene_data_list(&mut self, _renderer_context: &RendererContext) {
        log::info!("    load_scene_data_list");
        let scene_directory = PathBuf::from(SCENE_FILE_PATH);
        let scene_data_files: Vec<PathBuf> = self.collect_resources(&scene_directory, &[EXT_SCENE]);
        for scene_data_file in scene_data_files {
            let scene_data_name = get_unique_resource_name(&self._scene_data_create_infos_map, &scene_directory, &scene_data_file);
            let loaded_contents = system::load(&scene_data_file);
            let scene_data_create_info: SceneDataCreateInfo = serde_json::from_reader(loaded_contents).expect("Failed to deserialize.");
            self._scene_data_create_infos_map.insert(scene_data_name.clone(), newRcRefCell(scene_data_create_info));
        }
    }

    pub fn unload_scene_data_list(&mut self, _renderer_context: &RendererContext) {
        self._scene_data_create_infos_map.clear();
    }

    pub fn save_scene_data(&mut self, scene_data_name: &str, scene_data_create_info: &SceneDataCreateInfo) {
        let mut scene_data_filepath = PathBuf::from(PROJECT_RESOURCE_PATH);
        scene_data_filepath.push(SCENE_FILE_PATH);
        scene_data_filepath.push(scene_data_name);
        scene_data_filepath.set_extension(EXT_SCENE);
        let mut write_file = File::create(&scene_data_filepath).expect("Failed to create file");
        let mut write_contents: String = serde_json::to_string(&scene_data_create_info).expect("Failed to serialize.");
        write_contents = write_contents.replace(",\"", ",\n\"");
        write_file.write(write_contents.as_bytes()).expect("Failed to write");

        self._scene_data_create_infos_map.insert(String::from(scene_data_name), newRcRefCell(scene_data_create_info.clone()));
    }

    pub fn has_scene_data(&self, resource_name: &str) -> bool {
        self._scene_data_create_infos_map.get(resource_name).is_some()
    }

    pub fn get_scene_data(&self, resource_name: &str) -> &RcRefCell<SceneDataCreateInfo> {
        self._scene_data_create_infos_map.get(resource_name).unwrap()
    }
}
