use std::fs::{ File };
use std::io::prelude::*;
use std::path::{ Path, PathBuf };

use serde_json::{ self };

use rust_engine_3d::resource::resource::{ ResourceDataMap, ProjectResourcesBase, Resources, get_unique_resource_name };
use rust_engine_3d::renderer::renderer::{ RendererData };
use rust_engine_3d::utilities::system::{ self, RcRefCell, newRcRefCell };
use crate::application::project_scene_manager::{ SceneDataCreateInfo };

pub const SCENE_FILE_PATH: &str = "resource/scenes";

pub const EXT_SCENE: &str = "scene";

pub type SceneDataCreateInfoMap = ResourceDataMap<SceneDataCreateInfo>;

#[derive(Clone)]
pub struct ProjectResources {
    _engine_resources: *const Resources,
    _scene_data_create_infos_map: SceneDataCreateInfoMap,
}

impl ProjectResourcesBase for ProjectResources {
    fn initialize_project_resources(&mut self, engine_resources: &Resources, engine_renderer: &mut RendererData) {
        self._engine_resources = engine_resources;
        self.load_scene_datas(engine_renderer);
    }

    fn destroy_project_resources(&mut self, engine_renderer: &mut RendererData) {
        self.unload_scene_datas(engine_renderer);
    }

    fn load_graphics_datas(&mut self, _engine_renderer: &mut RendererData) {

    }

    fn unload_graphics_datas(&mut self, _engine_renderer: &mut RendererData) {

    }

    fn regist_resource(&mut self) {

    }

    fn unregist_resource(&mut self) {

    }
}

impl ProjectResources {
    pub fn create_project_resources() -> Box<ProjectResources> {
        Box::new(ProjectResources {
            _engine_resources: std::ptr::null(),
            _scene_data_create_infos_map: SceneDataCreateInfoMap::new(),
        })
    }

    pub fn get_engine_resources(&self) -> &Resources {
        unsafe { &*self._engine_resources }
    }

    pub fn get_engine_resources_mut(&self) -> &mut Resources {
        unsafe { &mut *(self._engine_resources as *mut Resources) }
    }

    pub fn collect_resources(&self, dir: &Path, extensions: &[&str]) -> Vec<PathBuf> {
        self.get_engine_resources().collect_resources(dir, extensions)
    }

    // SceneData
    pub fn load_scene_datas(&mut self, _renderer_data: &RendererData) {
        let scene_directory = PathBuf::from(SCENE_FILE_PATH);
        let scene_data_files: Vec<PathBuf> = self.collect_resources(&scene_directory, &[EXT_SCENE]);
        for scene_data_file in scene_data_files {
            let scene_data_name = get_unique_resource_name(&self._scene_data_create_infos_map, &scene_directory, &scene_data_file);
            let loaded_contents = system::load(&scene_data_file);
            let scene_data_create_info: SceneDataCreateInfo = serde_json::from_reader(loaded_contents).expect("Failed to deserialize.");
            self._scene_data_create_infos_map.insert(scene_data_name.clone(), newRcRefCell(scene_data_create_info));
        }
    }

    pub fn unload_scene_datas(&mut self, _renderer_data: &RendererData) {
        self._scene_data_create_infos_map.clear();
    }

    pub fn save_scene_data(&mut self, scene_data_name: &String, scene_data_create_info: &SceneDataCreateInfo) {
        let mut scene_data_filepath = PathBuf::from(SCENE_FILE_PATH);
        scene_data_filepath.push(scene_data_name);
        scene_data_filepath.set_extension(EXT_SCENE);
        let mut write_file = File::create(&scene_data_filepath).expect("Failed to create file");
        let mut write_contents: String = serde_json::to_string(&scene_data_create_info).expect("Failed to serialize.");
        write_contents = write_contents.replace(",\"", ",\n\"");
        write_file.write(write_contents.as_bytes()).expect("Failed to write");

        self._scene_data_create_infos_map.insert(scene_data_name.clone(), newRcRefCell(scene_data_create_info.clone()));
    }

    pub fn has_scene_data(&self, resource_name: &str) -> bool {
        self._scene_data_create_infos_map.get(resource_name).is_some()
    }

    pub fn get_scene_data(&self, resource_name: &str) -> &RcRefCell<SceneDataCreateInfo> {
        self._scene_data_create_infos_map.get(resource_name).unwrap()
    }
}
