use log::LevelFilter;
use nalgebra::Vector2;

use ash::vk;
use winit::event::VirtualKeyCode;

use rust_engine_3d::constants;
use rust_engine_3d::application::application::{
    self,
    ProjectApplicationBase,
    EngineApplication,
    WindowMode
};
use rust_engine_3d::application::audio_manager::AudioManager;
use rust_engine_3d::application::scene_manager::ProjectSceneManagerBase;
use rust_engine_3d::effect::effect_manager::EffectManager;
use rust_engine_3d::renderer::renderer_data::RendererData;
use rust_engine_3d::utilities::system::{ptr_as_ref, ptr_as_mut};
use crate::application_constants;
use crate::application::project_scene_manager::ProjectSceneManager;
use crate::game_module::game_client::GameClient;
use crate::renderer::project_ui::ProjectUIManager;
use crate::resource::project_resource::ProjectResources;

pub struct ProjectApplication {
    pub _engine_application: *const EngineApplication,
    pub _audio_manager: *const AudioManager,
    pub _effect_manager: *const EffectManager,
    pub _renderer_data: *const RendererData,
    pub _project_resources: Box<ProjectResources>,
    pub _project_scene_manager: Box<ProjectSceneManager>,
    pub _project_ui_manager: Box<ProjectUIManager>,
    pub _game_client: Box<GameClient>,
    pub _is_game_mode: bool
}

impl ProjectApplicationBase for ProjectApplication {
    fn initialize_project_application(&mut self, engine_application: &EngineApplication, window_size: &Vector2<i32>) {
        self._engine_application = engine_application;
        self._audio_manager = engine_application.get_audio_manager();
        self._effect_manager = engine_application.get_effect_manager();
        self._renderer_data = engine_application.get_renderer_context().get_renderer_data();

        self.get_project_scene_manager_mut().initialize_project_scene_manager(
            engine_application.get_renderer_context(),
            engine_application.get_effect_manager(),
            engine_application.get_engine_resources(),
            window_size,
        );
        self.get_game_client_mut().initialize_game_client(self);

        // start game
        self.get_game_client_mut().start_game();
    }

    fn terminate_project_application(&mut self) {
        // close scene
        self._project_scene_manager.close_scene_data();

        // destroy managers
        self._game_client.destroy_game_client();
        self._project_scene_manager.destroy_project_scene_manager();
    }

    fn resized_window(&self, width: i32, height: i32) {
        self._project_scene_manager.resized_window(width, height);
    }

    fn update_event(&mut self) {
        if self.get_engine_application()._keyboard_input_data.get_key_pressed(VirtualKeyCode::Tab) {
            self.toggle_game_mode();
        }

        if self._is_game_mode {
            // game mode
            self.get_game_client_mut().update_event();
        } else {
            // editor mode
            let engine_application = self.get_engine_application();
            let time_data = &engine_application._time_data;
            let mouse_move_data = &engine_application._mouse_move_data;
            let mouse_input_data = &engine_application._mouse_input_data;
            let keyboard_input_data = &engine_application._keyboard_input_data;

            const MOUSE_DELTA_RATIO: f32 = 500.0;
            let delta_time = time_data._delta_time;
            let _mouse_pos = &mouse_move_data._mouse_pos;
            let mouse_delta_x = mouse_move_data._mouse_pos_delta.x as f32 / engine_application._window_size.x as f32 * MOUSE_DELTA_RATIO;
            let mouse_delta_y = mouse_move_data._mouse_pos_delta.y as f32 / engine_application._window_size.y as f32 * MOUSE_DELTA_RATIO;
            let btn_left: bool = mouse_input_data._btn_l_hold;
            let btn_right: bool = mouse_input_data._btn_r_hold;
            let btn_r_pressed: bool = mouse_input_data._btn_r_pressed;
            let btn_r_released: bool = mouse_input_data._btn_r_released;
            let _btn_middle: bool = mouse_input_data._btn_m_hold;

            if btn_r_pressed {
                self.get_engine_application_mut().set_grab_mode(true);
            } else if btn_r_released {
                self.get_engine_application_mut().set_grab_mode(false);
            }

            let pressed_key_a = keyboard_input_data.get_key_hold(VirtualKeyCode::A);
            let pressed_key_d = keyboard_input_data.get_key_hold(VirtualKeyCode::D);
            let pressed_key_w = keyboard_input_data.get_key_hold(VirtualKeyCode::W);
            let pressed_key_s = keyboard_input_data.get_key_hold(VirtualKeyCode::S);
            let pressed_key_q = keyboard_input_data.get_key_hold(VirtualKeyCode::Q);
            let pressed_key_e = keyboard_input_data.get_key_hold(VirtualKeyCode::E);
            let pressed_key_z = keyboard_input_data.get_key_hold(VirtualKeyCode::Z);
            let pressed_key_c = keyboard_input_data.get_key_hold(VirtualKeyCode::C);
            let pressed_key_comma = keyboard_input_data.get_key_hold(VirtualKeyCode::Comma);
            let pressed_key_period = keyboard_input_data.get_key_hold(VirtualKeyCode::Period);
            let released_key_left_bracket = keyboard_input_data.get_key_released(VirtualKeyCode::LBracket);
            let released_key_right_bracket = keyboard_input_data.get_key_released(VirtualKeyCode::RBracket);
            let released_key_subtract = keyboard_input_data.get_key_released(VirtualKeyCode::Minus);
            let released_key_equals = keyboard_input_data.get_key_released(VirtualKeyCode::Equals);
            let modifier_keys_shift = keyboard_input_data.get_key_hold(VirtualKeyCode::LShift);

            let main_camera = self.get_project_scene_manager().get_main_camera_mut();
            let mut main_light = self.get_project_scene_manager()._main_light.borrow_mut();
            let camera_move_speed_multiplier = if modifier_keys_shift { 2.0 } else { 1.0 };
            let move_speed: f32 = application_constants::CAMERA_MOVE_SPEED * camera_move_speed_multiplier * delta_time as f32;
            let pan_speed = application_constants::CAMERA_PAN_SPEED * camera_move_speed_multiplier;
            let _rotation_speed = application_constants::CAMERA_ROTATION_SPEED;

            if released_key_left_bracket {
                self.get_renderer_data_mut().prev_debug_render_target();
            } else if released_key_right_bracket {
                self.get_renderer_data_mut().next_debug_render_target();
            }

            if released_key_subtract {
                self.get_renderer_data_mut().prev_debug_render_target_miplevel();
            } else if released_key_equals {
                self.get_renderer_data_mut().next_debug_render_target_miplevel();
            }

            #[cfg(target_os = "android")]
                let rotation_speed = 0.02 * delta_time as f32;
            #[cfg(not(target_os = "android"))]
                let rotation_speed = delta_time as f32;

            if pressed_key_comma {
                main_light._transform_object.rotation_pitch(rotation_speed);
            } else if pressed_key_period {
                main_light._transform_object.rotation_pitch(-rotation_speed);
            }

            if btn_left && btn_right {
                main_camera._transform_object.move_left(-pan_speed * mouse_delta_x as f32);
                main_camera._transform_object.move_up(pan_speed * mouse_delta_y as f32);
            }
            else if btn_right {
                main_camera._transform_object.rotation_pitch(-rotation_speed * mouse_delta_y as f32);
                main_camera._transform_object.rotation_yaw(-rotation_speed * mouse_delta_x as f32);
            }

            if pressed_key_z {
                main_camera._transform_object.rotation_roll(-rotation_speed * delta_time as f32);
            }
            else if pressed_key_c {
                main_camera._transform_object.rotation_roll(rotation_speed * delta_time as f32);
            }

            if pressed_key_w {
                main_camera._transform_object.move_front(-move_speed);
            }
            else if pressed_key_s {
                main_camera._transform_object.move_front(move_speed);
            }

            if pressed_key_a {
                main_camera._transform_object.move_left(-move_speed);
            }
            else if pressed_key_d {
                main_camera._transform_object.move_left(move_speed);
            }

            if pressed_key_q {
                main_camera._transform_object.move_up(-move_speed);
            }
            else if pressed_key_e {
                main_camera._transform_object.move_up(move_speed);
            }
        }
    }

    fn update_project_application(&mut self) {
        if self._is_game_mode {
            self._game_client.update_game_client();
        }

        let engine_application = unsafe { &*self._engine_application };
        self._project_scene_manager.update_project_scene_manager(engine_application);
    }
}

impl ProjectApplication {
    pub fn get_engine_application(&self) -> &EngineApplication { ptr_as_ref(self._engine_application) }
    pub fn get_engine_application_mut(&self) -> &mut EngineApplication { ptr_as_mut(self._engine_application) }
    pub fn get_effect_manager(&self) -> &EffectManager { ptr_as_ref(self._effect_manager) }
    pub fn get_effect_manager_mut(&self) -> &mut EffectManager { ptr_as_mut(self._effect_manager) }
    pub fn get_project_resources(&self) -> &ProjectResources { ptr_as_ref(self._project_resources.as_ref()) }
    pub fn get_project_resources_mut(&self) -> &mut ProjectResources { ptr_as_mut(self._project_resources.as_ref()) }
    pub fn get_project_scene_manager(&self) -> &ProjectSceneManager { ptr_as_ref(self._project_scene_manager.as_ref()) }
    pub fn get_project_scene_manager_mut(&self) -> &mut ProjectSceneManager { ptr_as_mut(self._project_scene_manager.as_ref()) }
    pub fn get_renderer_data(&self) -> &RendererData { ptr_as_ref(self._renderer_data) }
    pub fn get_renderer_data_mut(&self) -> &mut RendererData { ptr_as_mut(self._renderer_data) }
    pub fn get_project_ui_manager(&self) -> &ProjectUIManager { ptr_as_ref(self._project_ui_manager.as_ref()) }
    pub fn get_project_ui_manager_mut(&self) -> &mut ProjectUIManager { ptr_as_mut(self._project_ui_manager.as_ref()) }
    pub fn get_audio_manager(&self) -> &AudioManager { ptr_as_ref(self._audio_manager) }
    pub fn get_audio_manager_mut(&self) -> &mut AudioManager { ptr_as_mut(self._audio_manager) }
    pub fn get_game_client(&self) -> &GameClient { ptr_as_ref(self._game_client.as_ref()) }
    pub fn get_game_client_mut(&self) -> &mut GameClient { ptr_as_mut(self._game_client.as_ref()) }
    pub fn toggle_game_mode(&mut self) { self.set_game_mode(!self._is_game_mode); }
    pub fn set_game_mode(&mut self, is_game_mode: bool) {
        self._is_game_mode = is_game_mode;
        self.get_engine_application_mut().set_grab_mode(is_game_mode);
    }
}

pub fn run_project_application() {
    // application setting
    let app_name: String = "Destruction Zone".to_string();
    let app_version: u32 = 1;
    let initial_window_size: Vector2<i32> = Vector2::new(1024, 768);
    let window_mode = WindowMode::WindowMode;
    let log_level = LevelFilter::Info;

    // vulkan setting
    let vulkan_api_version: u32;
    let enable_immediate_mode: bool;
    let is_concurrent_mode: bool;
    let enable_validation_layer = true;

    #[cfg(target_os = "android")]
    {
        vulkan_api_version = vk::make_version(1, 0, 0);
        enable_immediate_mode = false;
        is_concurrent_mode = false;
    }
    #[cfg(not(target_os = "android"))]
    {
        vulkan_api_version = vk::make_api_version(0, 1, 2, 126);
        enable_immediate_mode = true;
        is_concurrent_mode = true;
    }

    unsafe {
        constants::VULKAN_API_VERSION = vulkan_api_version;
        constants::DEBUG_MESSAGE_LEVEL = vk::DebugUtilsMessageSeverityFlagsEXT::WARNING;
        if enable_validation_layer {
            constants::REQUIRED_INSTANCE_LAYERS = vec!["VK_LAYER_MESA_device_select".to_string()];
            //constants::REQUIRED_INSTANCE_LAYERS = vec!["VK_LAYER_LUNARG_standard_validation".to_string()];
        }
        constants::REQUIRED_DEVICE_EXTENSIONS = vec![
            "VK_KHR_swapchain".to_string(),
            "VK_KHR_buffer_device_address".to_string(),
            "VK_KHR_deferred_host_operations".to_string(),
        ];
        // ray tracing
        constants::USE_RAY_TRACING = false;
        constants::REQUIRED_RAY_TRACING_EXTENSIONS = vec![
            "VK_NV_ray_tracing".to_string(),
            "VK_KHR_ray_query".to_string(),
            "VK_KHR_ray_tracing_pipeline".to_string(),
            "VK_KHR_acceleration_structure".to_string(),
        ];
        constants::ENABLE_IMMEDIATE_MODE = enable_immediate_mode;
        constants::IS_CONCURRENT_MODE = is_concurrent_mode;
        constants::METER_PER_UNIT = 1.0;
        constants::NEAR = 0.1;
        constants::FAR = 2000.0;
        constants::FOV = 60.0;
        // shadow
        constants::SHADOW_MAP_SIZE = 2048;
        constants::SHADOW_SAMPLES = 8;
        constants::SHADOW_DISTANCE = 500.0;
        constants::SHADOW_DEPTH = 2000.0;
        // capture height map
        constants::CAPTURE_HEIGHT_MAP_DISTANCE = 1000.0;
        constants::CAPTURE_HEIGHT_MAP_DEPTH = 2000.0;
        // effect
        constants::MAX_EMITTER_COUNT = 1024;
        constants::MAX_PARTICLE_COUNT = 262144;
    }

    // create project application & managers
    let project_resources = ProjectResources::create_project_resources();
    let project_scene_manager = ProjectSceneManager::create_project_scene_manager();
    let project_ui_manager = ProjectUIManager::create_project_ui_manager();
    let game_client = GameClient::create_game_client();
    let application = ProjectApplication {
        _engine_application: std::ptr::null(),
        _renderer_data: std::ptr::null(),
        _effect_manager: std::ptr::null(),
        _audio_manager: std::ptr::null(),
        _project_resources: project_resources,
        _project_scene_manager: project_scene_manager,
        _project_ui_manager: project_ui_manager,
        _game_client: game_client,
        _is_game_mode: false,
    };

    // run
    application::run_application(
        app_name,
        app_version,
        initial_window_size,
        window_mode,
        log_level,
        &application,
        application.get_project_resources(),
        application.get_project_scene_manager(),
        application.get_project_ui_manager()
    );
}