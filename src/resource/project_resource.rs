use std::path::{Path, PathBuf};

use crate::render_pass::render_pass;
use rust_engine_3d::effect::effect_data::EffectData;
use rust_engine_3d::renderer::renderer_context::RendererContext;
use rust_engine_3d::resource::resource::{
    EngineResources, ProjectResourcesBase, RenderPassDataCreateInfoMap, ResourceData,
};
use rust_engine_3d::scene::font::FontData;
use rust_engine_3d::scene::material::MaterialData;
use rust_engine_3d::scene::material_instance::MaterialInstanceData;
use rust_engine_3d::scene::mesh::MeshData;
use rust_engine_3d::scene::model::ModelData;
use rust_engine_3d::scene::scene_manager::SceneDataCreateInfo;
use rust_engine_3d::utilities::system::{ptr_as_mut, ptr_as_ref, RcRefCell};
use rust_engine_3d::vulkan_context::texture::TextureData;

pub const DEFAULT_GAME_DATA_NAME: &str = "default";

#[derive(Clone)]
pub struct ProjectResources {
    _engine_resources: *const EngineResources,
}

impl ProjectResourcesBase for ProjectResources {
    fn initialize_project_resources(&mut self, engine_resources: &EngineResources) {
        self._engine_resources = engine_resources;
    }
    fn load_project_resources(&mut self, _renderer_context: &RendererContext) {}
    fn destroy_project_resources(&mut self, _renderer_context: &RendererContext) {}
    fn load_graphics_data_list(&mut self, _renderer_context: &RendererContext) {}
    fn unload_graphics_data_list(&mut self, _renderer_context: &RendererContext) {}
    fn load_render_pass_data_create_infos(
        &mut self,
        renderer_context: &RendererContext,
        render_pass_data_create_info_map: &mut RenderPassDataCreateInfoMap,
    ) {
        render_pass::get_render_pass_data_create_infos(
            renderer_context,
            render_pass_data_create_info_map,
        );
    }
    fn regist_resource(&mut self) {}
    fn unregist_resource(&mut self) {}
    fn has_audio_data(&self, resource_name: &str) -> bool {
        self.get_engine_resources().has_audio_data(resource_name)
    }
    fn get_audio_data(&self, resource_name: &str) -> &ResourceData {
        self.get_engine_resources_mut()
            .get_audio_data(resource_name)
    }
    fn has_audio_bank_data(&self, resource_name: &str) -> bool {
        self.get_engine_resources()
            .has_audio_bank_data(resource_name)
    }
    fn get_audio_bank_data(&self, resource_name: &str) -> &ResourceData {
        self.get_engine_resources_mut()
            .get_audio_bank_data(resource_name)
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
        self.get_engine_resources()
            .has_material_instance_data(resource_name)
    }
    fn get_material_instance_data(&self, resource_name: &str) -> &RcRefCell<MaterialInstanceData> {
        self.get_engine_resources()
            .get_material_instance_data(resource_name)
    }
}

impl ProjectResources {
    pub fn create_project_resources() -> Box<ProjectResources> {
        Box::new(ProjectResources {
            _engine_resources: std::ptr::null(),
        })
    }
    pub fn get_engine_resources(&self) -> &EngineResources {
        ptr_as_ref(self._engine_resources)
    }
    pub fn get_engine_resources_mut(&self) -> &mut EngineResources {
        ptr_as_mut(self._engine_resources as *mut EngineResources)
    }
    pub fn collect_resources(&self, dir: &Path, extensions: &[&str]) -> Vec<PathBuf> {
        self.get_engine_resources()
            .collect_resources(dir, extensions)
    }

    pub fn has_scene_data(&self, resource_name: &str) -> bool {
        self.get_engine_resources().has_scene_data(resource_name)
    }

    pub fn get_scene_data(&self, resource_name: &str) -> &RcRefCell<SceneDataCreateInfo> {
        self.get_engine_resources().get_scene_data(resource_name)
    }
}
