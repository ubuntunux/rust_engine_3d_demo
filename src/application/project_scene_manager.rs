use std::collections::HashMap;

use ash::Device;
use nalgebra::{
    Vector3,
    Vector4
};
use serde::{ Serialize, Deserialize };

use rust_engine_3d::constants;
use rust_engine_3d::application::application::TimeData;
use rust_engine_3d::application::scene_manager::{ ProjectSceneManagerBase, SceneManagerData };
use rust_engine_3d::renderer::effect::{ EffectCreateInfo, EffectInstance, EffectManagerData, ProjectEffectManagerBase };
use rust_engine_3d::renderer::font::FontManager;
use rust_engine_3d::renderer::renderer::RendererData;
use rust_engine_3d::renderer::camera::{ CameraCreateInfo, CameraObjectData};
use rust_engine_3d::renderer::light::{ DirectionalLightCreateInfo, DirectionalLightData };
use rust_engine_3d::renderer::render_element::{ RenderElementData };
use rust_engine_3d::renderer::render_object::{ RenderObjectCreateInfo, RenderObjectData, AnimationPlayArgs };
use rust_engine_3d::renderer::light::LightConstants;
use rust_engine_3d::resource::resource::Resources;
use rust_engine_3d::utilities::system::{ self, RcRefCell, newRcRefCell };
use rust_engine_3d::utilities::bounding_box::BoundingBox;

use crate::application_constants;
use crate::renderer::project_effect::ProjectEffectManager;
use crate::renderer::project_renderer::ProjectRenderer;
use crate::resource::project_resource::ProjectResources;

type CameraObjectMap = HashMap<String, RcRefCell<CameraObjectData>>;
type DirectionalLightObjectMap = HashMap<String, RcRefCell<DirectionalLightData>>;
type EffectIDMap = HashMap<String, i64>;
type RenderObjectMap = HashMap<String, RcRefCell<RenderObjectData>>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SceneDataCreateInfo {
    _cameras: HashMap<String, CameraCreateInfo>,
    _directional_lights: HashMap<String, DirectionalLightCreateInfo>,
    _effects: HashMap<String, EffectCreateInfo>,
    _static_objects: HashMap<String, RenderObjectCreateInfo>,
    _skeletal_objects: HashMap<String, RenderObjectCreateInfo>,
}

impl Default for SceneDataCreateInfo {
    fn default() -> SceneDataCreateInfo {
        SceneDataCreateInfo {
            _cameras: HashMap::new(),
            _directional_lights: HashMap::new(),
            _effects: HashMap::new(),
            _static_objects: HashMap::new(),
            _skeletal_objects: HashMap::new(),
        }
    }
}

#[derive(Clone)]
pub struct ProjectSceneManager {
    pub _scene_manager_data: *const SceneManagerData,
    pub _project_resources: *const ProjectResources,
    pub _project_renderer: *const ProjectRenderer,
    pub _project_effect_manager: *const ProjectEffectManager,
    pub _window_width: u32,
    pub _window_height: u32,
    pub _scene_name: String,
    pub _main_camera: RcRefCell<CameraObjectData>,
    pub _main_light: RcRefCell<DirectionalLightData>,
    pub _capture_height_map: RcRefCell<DirectionalLightData>,
    pub _light_probe_cameras: Vec<RcRefCell<CameraObjectData>>,
    pub _camera_object_map: CameraObjectMap,
    pub _directional_light_object_map: DirectionalLightObjectMap,
    pub _effect_id_map: EffectIDMap,
    pub _static_render_object_map: RenderObjectMap,
    pub _skeletal_render_object_map: RenderObjectMap,
    pub _static_render_elements: Vec<RenderElementData>,
    pub _static_shadow_render_elements: Vec<RenderElementData>,
    pub _skeletal_render_elements: Vec<RenderElementData>,
    pub _skeletal_shadow_render_elements: Vec<RenderElementData>,
}


impl ProjectSceneManagerBase for ProjectSceneManager {
    fn initialize_project_scene_manager(
        &mut self,
        window_width: u32,
        window_height: u32,
        scene_manager_data: &SceneManagerData,
        renderer_data: &RendererData,
        resources: &Resources,
        effect_manager_data: *const EffectManagerData,
    ) {
        self._project_renderer = renderer_data._project_renderer as *const ProjectRenderer;
        self._scene_manager_data = scene_manager_data;
        self._project_resources = resources._project_resources as *const ProjectResources;
        self._project_effect_manager = unsafe { (*effect_manager_data)._project_effect_manager as *const ProjectEffectManager };

        self.resized_window(window_width, window_height);
    }

    fn initialize_scene_graphics_data(&self) {
    }

    fn destroy_scene_graphics_data(&self, _device: &Device) {
    }

    fn get_window_size(&self) -> (u32, u32) {
        (self._window_width, self._window_height)
    }
    fn set_window_size(&mut self, width: u32, height: u32) {
        self._window_width = width;
        self._window_height = height;
    }
    fn resized_window(&mut self, width: u32, height: u32) {
        self.set_window_size(width, height);
        self._main_camera.borrow_mut().set_aspect(width, height);
    }

    fn create_default_scene_data(&self, scene_data_name: &String) {
        let mut scene_data_create_info = SceneDataCreateInfo {
            _cameras: HashMap::new(),
            _directional_lights: HashMap::new(),
            _effects: HashMap::new(),
            _static_objects: HashMap::new(),
            _skeletal_objects: HashMap::new(),
        };

        scene_data_create_info._cameras.insert(
            String::from("main_camera"),
            CameraCreateInfo {
                position: Vector3::new(2.0, 2.0, -1.0),
                rotation: Vector3::new(-0.157, 1.3, 0.0),
                ..Default::default()
            }
        );

        let pitch: f32 = -std::f32::consts::PI * 0.47;
        scene_data_create_info._directional_lights.insert(
            String::from("main_light"),
            DirectionalLightCreateInfo {
                _position: Vector3::zeros(),
                _rotation: Vector3::new(pitch, 0.0, 0.3),
                _light_constants: LightConstants {
                    _light_direction: Vector3::new(pitch, 0.0, 0.3),
                    ..Default::default()
                },
                ..Default::default()
            }
        );

        scene_data_create_info._effects.insert(
            String::from("effect0"),
            EffectCreateInfo {
                _effect_data_name: String::from("default"),
                _effect_position: Vector3::new(0.0, 4.0, 0.0),
                ..Default::default()
            }
        );

        scene_data_create_info._effects.insert(
            String::from("effect1"),
            EffectCreateInfo {
                _effect_data_name: String::from("test"),
                _effect_position: Vector3::new(4.0, 4.0, 0.0),
                ..Default::default()
            }
        );

        scene_data_create_info._effects.insert(
            String::from("effect2"),
            EffectCreateInfo {
                _effect_data_name: String::from("test2"),
                _effect_position: Vector3::new(8.0, 4.0, 0.0),
                ..Default::default()
            }
        );
        scene_data_create_info._static_objects.insert(
            String::from("sponza"),
            RenderObjectCreateInfo {
                _model_data_name: String::from("sponza/sponza"),
                _position: Vector3::new(0.0, 0.0, 0.0),
                _scale: Vector3::new(0.1, 0.1, 0.1),
                ..Default::default()
            }
        );
        scene_data_create_info._static_objects.insert(
            String::from("sphere"),
            RenderObjectCreateInfo {
                _model_data_name: String::from("sphere"),
                _position: Vector3::new(-2.0, 1.0, 0.0),
                _scale: Vector3::new(1.0, 1.0, 1.0),
                ..Default::default()
            }
        );

        for i in 0..3 {
            scene_data_create_info._skeletal_objects.insert(
                format!("skeletal_{}", i),
                RenderObjectCreateInfo {
                    _model_data_name: String::from("skeletal"),
                    _position: Vector3::new(i as f32, 1.0, 0.0),
                    _scale: Vector3::new(0.01, 0.01, 0.01),
                    ..Default::default()
                }
            );
        }

        self.get_project_resources_mut().save_scene_data(scene_data_name, &scene_data_create_info);
    }

    fn open_scene_data(&mut self, resources: &Resources, scene_data_name: &String) {
        self._scene_name = scene_data_name.clone();

        self.initialize_light_probe_cameras();

        let project_resources = unsafe { &*self._project_resources };

        if false == project_resources.has_scene_data(scene_data_name) {
            self.create_default_scene_data(scene_data_name);
        }

        let scene_data_create_info = project_resources.get_scene_data(scene_data_name).borrow();

        // cameras
        for (index, (object_name, camera_create_info)) in scene_data_create_info._cameras.iter().enumerate() {
            let camera_create_info = CameraCreateInfo {
                window_width: self._window_width,
                window_height: self._window_height,
                ..camera_create_info.clone()
            };
            let camera_object = self.add_camera_object(object_name, &camera_create_info);
            if 0 == index {
                self._main_camera = camera_object;
            }
        }

        // lights
        for (index, (object_name, light_create_info)) in scene_data_create_info._directional_lights.iter().enumerate() {
            let light_create_info = DirectionalLightCreateInfo {
                _position: light_create_info._position.clone() as Vector3<f32>,
                _rotation: light_create_info._rotation.clone() as Vector3<f32>,
                _light_constants: LightConstants {
                    _light_position: light_create_info._light_constants._light_position.clone() as Vector3<f32>,
                    _light_direction: light_create_info._light_constants._light_direction.clone() as Vector3<f32>,
                    _light_color: light_create_info._light_constants._light_color.clone() as Vector3<f32>,
                    ..Default::default()
                },
                ..Default::default()
            };
            let light_object = self.add_light_object(object_name, &light_create_info);
            if 0 == index {
                self._main_light = light_object;
            }
        }

        // effects
        for (object_name, effect_create_info) in scene_data_create_info._effects.iter() {
            self.add_effect(object_name, effect_create_info);
        }

        // static objects
        for (object_name, render_object_create_info) in scene_data_create_info._static_objects.iter() {
            self.add_static_render_object(object_name, render_object_create_info);
        }

        // skeletal objects
        for (object_name, render_object_create_info) in scene_data_create_info._skeletal_objects.iter() {
            self.add_skeletal_render_object(object_name, render_object_create_info);
        }
    }

    fn close_scene_data(&mut self, _device: &Device) {
        self._camera_object_map.clear();
        self._directional_light_object_map.clear();
        self._effect_id_map.clear();
        self._static_render_object_map.clear();
        self._skeletal_render_object_map.clear();
        self._static_render_elements.clear();
        self._static_shadow_render_elements.clear();
        self._skeletal_render_elements.clear();
        self._skeletal_shadow_render_elements.clear();
    }

    fn save_scene_data(&mut self) {
        let mut scene_data_create_info = SceneDataCreateInfo {
            _cameras: HashMap::new(),
            _directional_lights: HashMap::new(),
            _effects: HashMap::new(),
            _static_objects: HashMap::new(),
            _skeletal_objects: HashMap::new(),
        };
        // cameras
        for camera_object in self._camera_object_map.values() {
            let camera = camera_object.borrow();
            let camera_create_info = CameraCreateInfo {
                fov: camera._fov,
                near: camera._near,
                far: camera._far,
                position: camera._transform_object.get_position().clone() as Vector3<f32>,
                rotation: camera._transform_object.get_rotation().clone() as Vector3<f32>,
                ..Default::default()
            };
            scene_data_create_info._cameras.insert(camera._name.clone(), camera_create_info);
        }
        // lights
        for light_object in self._directional_light_object_map.values() {
            let light = light_object.borrow();
            let light_create_info = DirectionalLightCreateInfo {
                _position: light._transform_object.get_position().clone() as Vector3<f32>,
                _rotation: light._transform_object.get_rotation().clone() as Vector3<f32>,
                ..Default::default()
            };
            scene_data_create_info._directional_lights.insert(light._light_name.clone(), light_create_info);
        }
        // effects
        for (effect_name, effect_id) in self._effect_id_map.iter() {
            let effect = self.get_project_effect_manager().get_effect(*effect_id).unwrap().borrow();
            let effect_create_info = EffectCreateInfo {
                _effect_position: effect._effect_transform.get_position().clone() as Vector3<f32>,
                _effect_rotation: effect._effect_transform.get_rotation().clone() as Vector3<f32>,
                _effect_scale: effect._effect_transform.get_scale().clone() as Vector3<f32>,
                _effect_data_name: effect._effect_data.borrow()._effect_data_name.clone(),
            };
            scene_data_create_info._effects.insert(effect_name.clone(), effect_create_info);
        }
        // static objects
        for static_object in self._static_render_object_map.values() {
            let object = static_object.borrow();
            let static_object_create_info = RenderObjectCreateInfo {
                _model_data_name: object._model_data.borrow()._model_data_name.clone(),
                _position: object._transform_object.get_position().clone() as Vector3<f32>,
                _rotation: object._transform_object.get_rotation().clone() as Vector3<f32>,
                _scale: object._transform_object.get_scale().clone() as Vector3<f32>,
            };
            scene_data_create_info._static_objects.insert(object._render_object_name.clone(), static_object_create_info);
        }
        // skeletal objects
        for skeletal_object in self._skeletal_render_object_map.values() {
            let object = skeletal_object.borrow();
            let skeletal_object_create_info = RenderObjectCreateInfo {
                _model_data_name: object._model_data.borrow()._model_data_name.clone(),
                _position: object._transform_object.get_position().clone() as Vector3<f32>,
                _rotation: object._transform_object.get_rotation().clone() as Vector3<f32>,
                _scale: object._transform_object.get_scale().clone() as Vector3<f32>,
            };
            scene_data_create_info._skeletal_objects.insert(object._render_object_name.clone(), skeletal_object_create_info);
        }

        self.get_project_resources_mut().save_scene_data(&self._scene_name, &scene_data_create_info);
    }

    fn destroy_project_scene_manager(&mut self, device: &Device) {
        self.destroy_scene_graphics_data(device);
    }

    fn update_project_scene_manager(&mut self, time_data: &TimeData, font_manager: &mut FontManager) {
        let delta_time: f64 = time_data._delta_time;

        let mut main_camera = self._main_camera.borrow_mut();
        main_camera.update_camera_object_data();
        let camera_position = &main_camera.get_camera_position();

        let mut main_light = self._main_light.borrow_mut();
        main_light.update_light_data(camera_position);

        let mut capture_height_map = self._capture_height_map.borrow_mut();
        capture_height_map.update_light_data(camera_position);

        for (_key, render_object_data) in self._static_render_object_map.iter() {
            render_object_data.borrow_mut().update_render_object_data(delta_time as f32);
        }

        for (_key, render_object_data) in self._skeletal_render_object_map.iter() {
            render_object_data.borrow_mut().update_render_object_data(delta_time as f32);
        }

        self.get_project_effect_manager_mut().update_effects(delta_time as f32);

        // gather render elements
        ProjectSceneManager::gather_render_elements(
            &main_camera,
            &main_light,
            &self._static_render_object_map,
            &mut self._static_render_elements,
            &mut self._static_shadow_render_elements
        );

        ProjectSceneManager::gather_render_elements(
            &main_camera,
            &main_light,
            &self._skeletal_render_object_map,
            &mut self._skeletal_render_elements,
            &mut self._skeletal_shadow_render_elements
        );

        // debug text
        font_manager.clear_logs();
        font_manager.log(format!("{:.2}fps / {:.3}ms", time_data._average_fps, time_data._average_frame_time));
        font_manager.log(format!("StaticMesh: {:?}, Shadow: {:?}", self._static_render_elements.len(), self._static_shadow_render_elements.len()));
        font_manager.log(format!("SkeletalMesh: {:?}, Shadow: {:?}", self._skeletal_render_elements.len(), self._skeletal_shadow_render_elements.len()));
    }
}

impl ProjectSceneManager {
    pub fn create_project_scene_manager() -> Box<ProjectSceneManager> {
        let default_camera = CameraObjectData::create_camera_object_data(&String::from("default_camera"), &CameraCreateInfo::default());
        let light_probe_camera_create_info = CameraCreateInfo {
            fov: 90.0,
            window_width: application_constants::LIGHT_PROBE_SIZE,
            window_height: application_constants::LIGHT_PROBE_SIZE,
            enable_jitter: false,
            ..Default::default()
        };
        let light_probe_cameras = vec![
            system::newRcRefCell(CameraObjectData::create_camera_object_data(&String::from("light_probe_camera0"), &light_probe_camera_create_info)),
            system::newRcRefCell(CameraObjectData::create_camera_object_data(&String::from("light_probe_camera1"), &light_probe_camera_create_info)),
            system::newRcRefCell(CameraObjectData::create_camera_object_data(&String::from("light_probe_camera2"), &light_probe_camera_create_info)),
            system::newRcRefCell(CameraObjectData::create_camera_object_data(&String::from("light_probe_camera3"), &light_probe_camera_create_info)),
            system::newRcRefCell(CameraObjectData::create_camera_object_data(&String::from("light_probe_camera4"), &light_probe_camera_create_info)),
            system::newRcRefCell(CameraObjectData::create_camera_object_data(&String::from("light_probe_camera5"), &light_probe_camera_create_info))
        ];
        let default_light = DirectionalLightData::create_light_data(&String::from("default_light"), &DirectionalLightCreateInfo::default());
        let capture_height_map = DirectionalLightData::create_light_data(
            &String::from("capture_height_map"),
            &DirectionalLightCreateInfo {
                _rotation: Vector3::new(std::f32::consts::PI * -0.5, 0.0, 0.0),
                _shadow_dimensions: Vector4::new(
                    application_constants::CAPTURE_HEIGHT_MAP_DISTANCE,
                    application_constants::CAPTURE_HEIGHT_MAP_DISTANCE,
                    -application_constants::CAPTURE_HEIGHT_MAP_DEPTH,
                    application_constants::CAPTURE_HEIGHT_MAP_DEPTH
                ),
                ..Default::default()
            }
        );
        Box::new(ProjectSceneManager {
            _scene_manager_data: std::ptr::null(),
            _project_resources: std::ptr::null(),
            _project_renderer: std::ptr::null(),
            _project_effect_manager: std::ptr::null(),
            _window_width: default_camera._window_width,
            _window_height: default_camera._window_height,
            _scene_name: String::new(),
            _main_camera: system::newRcRefCell(default_camera),
            _main_light: system::newRcRefCell(default_light),
            _capture_height_map: system::newRcRefCell(capture_height_map),
            _light_probe_cameras: light_probe_cameras,
            _camera_object_map: HashMap::new(),
            _directional_light_object_map: HashMap::new(),
            _effect_id_map: HashMap::default(),
            _static_render_object_map: HashMap::new(),
            _skeletal_render_object_map: HashMap::new(),
            _static_render_elements: Vec::new(),
            _static_shadow_render_elements: Vec::new(),
            _skeletal_render_elements: Vec::new(),
            _skeletal_shadow_render_elements: Vec::new(),
        })
    }
    pub fn get_scene_manager_data(&self) -> &SceneManagerData { unsafe { &*self._scene_manager_data } }
    pub fn get_scene_manager_data_mut(&self) -> &mut SceneManagerData { unsafe { &mut *(self._scene_manager_data as *mut SceneManagerData) } }
    pub fn get_project_renderer(&self) -> &ProjectRenderer { unsafe { &*self._project_renderer } }
    pub fn get_project_renderer_mut(&self) -> &mut ProjectRenderer { unsafe { &mut *(self._project_renderer as *mut ProjectRenderer) } }
    pub fn get_project_resources(&self) -> &ProjectResources { unsafe { &*self._project_resources } }
    pub fn get_project_resources_mut(&self) -> &mut ProjectResources { unsafe { &mut *(self._project_resources as *mut ProjectResources) } }
    pub fn get_engine_resources(&self) -> &Resources { self.get_project_resources().get_engine_resources() }
    pub fn get_engine_resources_mut(&self) -> &mut Resources { self.get_project_resources().get_engine_resources_mut() }
    pub fn get_project_effect_manager(&self) -> &ProjectEffectManager { unsafe { &*self._project_effect_manager } }
    pub fn get_project_effect_manager_mut(&self) -> &mut ProjectEffectManager { unsafe { &mut *(self._project_effect_manager as *mut ProjectEffectManager) } }
    pub fn get_main_camera(&self) -> &RcRefCell<CameraObjectData> { &self._main_camera }
    pub fn get_light_probe_camera(&self, index: usize) -> &RcRefCell<CameraObjectData> { &self._light_probe_cameras[index] }
    pub fn add_camera_object(&mut self, object_name: &str, camera_create_info: &CameraCreateInfo) -> RcRefCell<CameraObjectData> {
        let new_object_name = system::generate_unique_name(&self._camera_object_map, object_name);
        let camera_object_data = newRcRefCell(CameraObjectData::create_camera_object_data(&new_object_name, camera_create_info));
        self._camera_object_map.insert(new_object_name, camera_object_data.clone());
        camera_object_data
    }
    pub fn get_main_light(&self) -> &RcRefCell<DirectionalLightData> {
        &self._main_light
    }

    pub fn get_capture_height_map(&self) -> &RcRefCell<DirectionalLightData> {
        &self._capture_height_map
    }

    pub fn add_light_object(&mut self, object_name: &str, light_create_info: &DirectionalLightCreateInfo) -> RcRefCell<DirectionalLightData> {
        let new_object_name = system::generate_unique_name(&self._directional_light_object_map, object_name);
        let light_object_data = newRcRefCell(DirectionalLightData::create_light_data(&new_object_name, light_create_info));
        self._directional_light_object_map.insert(new_object_name, light_object_data.clone());
        light_object_data
    }

    pub fn add_static_render_object(&mut self, object_name: &str, render_object_create_info: &RenderObjectCreateInfo) -> RcRefCell<RenderObjectData> {
        let model_data = self.get_engine_resources().get_model_data(&render_object_create_info._model_data_name);
        let new_object_name = system::generate_unique_name(&self._static_render_object_map, &object_name);
        let render_object_data = newRcRefCell(RenderObjectData::create_render_object_data(&new_object_name, &model_data, &render_object_create_info));
        self._static_render_object_map.insert(new_object_name, render_object_data.clone());
        render_object_data
    }

    pub fn add_skeletal_render_object(&mut self, object_name: &str, render_object_create_info: &RenderObjectCreateInfo) -> RcRefCell<RenderObjectData> {
        let model_data = self.get_engine_resources().get_model_data(&render_object_create_info._model_data_name);
        let new_object_name = system::generate_unique_name(&self._skeletal_render_object_map, &object_name);
        let render_object_data = newRcRefCell(RenderObjectData::create_render_object_data(&new_object_name, model_data, &render_object_create_info));
        self._skeletal_render_object_map.insert(new_object_name, render_object_data.clone());
        render_object_data
    }

    pub fn add_effect(&mut self, object_name: &str, effect_create_info: &EffectCreateInfo) -> i64 {
        let new_object_name = system::generate_unique_name(&self._effect_id_map, &object_name);
        let effect_id = self.get_project_effect_manager_mut().create_effect(effect_create_info);
        self._effect_id_map.insert(new_object_name, effect_id);
        effect_id
    }

    pub fn get_static_render_object(&self, object_name: &str) -> Option<&RcRefCell<RenderObjectData>> {
        self._static_render_object_map.get(object_name)
    }

    pub fn get_skeletal_render_object(&self, object_name: &str) -> Option<&RcRefCell<RenderObjectData>> {
        self._skeletal_render_object_map.get(object_name)
    }

    pub fn get_static_render_elements(&self) -> &Vec<RenderElementData> {
        &self._static_render_elements
    }

    pub fn get_static_shadow_render_elements(&self) -> &Vec<RenderElementData> {
        &self._static_shadow_render_elements
    }

    pub fn get_skeletal_render_elements(&self) -> &Vec<RenderElementData> {
        &self._skeletal_render_elements
    }

    pub fn get_skeletal_shadow_render_elements(&self) -> &Vec<RenderElementData> {
        &self._skeletal_shadow_render_elements
    }

    pub fn get_effect(&self, effect_id: i64) -> Option<&RcRefCell<EffectInstance>> {
        self.get_project_effect_manager().get_effect(effect_id)
    }

    pub fn view_frustum_culling_geometry(camera: &CameraObjectData, geometry_bound_box: &BoundingBox) -> bool {
        let to_geometry = &geometry_bound_box._center - camera.get_camera_position();
        for plane in camera._view_frustum_planes.iter() {
            let d = plane.dot(&to_geometry);
            if geometry_bound_box._radius < d {
                return true;
            }
        }
        false
    }

    pub fn shadow_culling(light: &DirectionalLightData, geometry_bound_box: &BoundingBox) -> bool {
        let shadow_view_projection = light.get_shadow_view_projection();
        let bound_min: Vector4<f32> = shadow_view_projection * Vector4::new(geometry_bound_box._min.x, geometry_bound_box._min.y, geometry_bound_box._min.z, 1.0);
        let bound_max: Vector4<f32> = shadow_view_projection * Vector4::new(geometry_bound_box._max.x, geometry_bound_box._max.y, geometry_bound_box._max.z, 1.0);
        let minimum: Vector3<f32> = Vector3::new(bound_min.x.min(bound_max.x), bound_min.y.min(bound_max.y), bound_min.z.min(bound_max.z));
        let maximum: Vector3<f32> = Vector3::new(bound_min.x.max(bound_max.x), bound_min.y.max(bound_max.y), bound_min.z.max(bound_max.z));
        if maximum.x < -1.0 || 1.0 < minimum.x || maximum.y < -1.0 || 1.0 < minimum.y || maximum.z < -1.0 || 1.0 < minimum.z {
            return true;
        }
        false
    }

    pub fn gather_render_elements(
        camera: &CameraObjectData,
        light: &DirectionalLightData,
        render_object_map: &RenderObjectMap,
        render_elements: &mut Vec<RenderElementData>,
        render_shadow_elements: &mut Vec<RenderElementData>,
    ) {
        render_elements.clear();
        render_shadow_elements.clear();
        for (_key, render_object_data) in render_object_map.iter() {
            let render_object_data_ref = &render_object_data.borrow();
            let mode_data = render_object_data_ref.get_model_data().borrow();
            let mesh_data = mode_data.get_mesh_data().borrow();
            let geometry_datas = mesh_data.get_geomtry_datas();
            let geometry_bound_boxes = &render_object_data_ref._geometry_bound_boxes;
            let material_instance_datas = mode_data.get_material_instance_datas();
            for index in 0..geometry_datas.len() {
                if false == ProjectSceneManager::view_frustum_culling_geometry(camera, &geometry_bound_boxes[index]) {
                    render_elements.push(RenderElementData {
                        _render_object: render_object_data.clone(),
                        _geometry_data: geometry_datas[index].clone(),
                        _material_instance_data: material_instance_datas[index].clone(),
                    })
                }

                if false == ProjectSceneManager::shadow_culling(light, &geometry_bound_boxes[index]) {
                    render_shadow_elements.push(RenderElementData {
                        _render_object: render_object_data.clone(),
                        _geometry_data: geometry_datas[index].clone(),
                        _material_instance_data: material_instance_datas[index].clone(),
                    })
                }
            }
        }
    }

    pub fn initialize_light_probe_cameras(&mut self) {
        let pi = std::f32::consts::PI;
        let half_pi = std::f32::consts::PI * 0.5;
        let rotations: [Vector3<f32>; constants::CUBE_LAYER_COUNT] = [
            Vector3::new(0.0, half_pi, 0.0),
            Vector3::new(0.0, -half_pi, 0.0),
            Vector3::new(-half_pi, 0.0, 0.0),
            Vector3::new(half_pi, 0.0, 0.0),
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(0.0, pi, 0.0)
        ];
        let inverse_front = Vector3::new(1.0, 1.0, -1.0);
        for i in 0..constants::CUBE_LAYER_COUNT {
            self._light_probe_cameras[i].borrow_mut()._transform_object.set_rotation(&rotations[i]);
            self._light_probe_cameras[i].borrow_mut()._transform_object.set_scale(&inverse_front);
            self._light_probe_cameras[i].borrow_mut().update_camera_object_data();
        }
    }
}