use ash::vk;
use log::LevelFilter;
use nalgebra::Vector2;
use rust_engine_3d::audio::audio_manager::AudioManager;
use rust_engine_3d::constants;
use rust_engine_3d::core::engine_core::{
    self, ApplicationBase, EngineCore, WindowMode,
};
use rust_engine_3d::effect::effect_manager::EffectManager;
use rust_engine_3d::renderer::renderer_data::RendererData;
use rust_engine_3d::resource::resource::CallbackLoadRenderPassCreateInfo;
use rust_engine_3d::utilities::system::{ptr_as_mut, ptr_as_ref};
use winit::event::VirtualKeyCode;

use crate::game_module::character::character_manager::CharacterManager;
use crate::game_module::game_client::GameClient;
use crate::game_module::game_constants;
use crate::game_module::game_controller::GameController;
use crate::game_module::game_resource::GameResources;
use crate::game_module::game_scene_manager::GameSceneManager;
use crate::game_module::game_ui_manager::GameUIManager;
use crate::render_pass;

pub struct Application {
    pub _engine_core: *const EngineCore,
    pub _audio_manager: *const AudioManager,
    pub _effect_manager: *const EffectManager,
    pub _renderer_data: *const RendererData,
    pub _character_manager: Box<CharacterManager>,
    pub _game_resources: Box<GameResources>,
    pub _game_scene_manager: Box<GameSceneManager>,
    pub _game_ui_manager: Box<GameUIManager>,
    pub _game_controller: Box<GameController>,
    pub _game_client: Box<GameClient>,
    pub _is_game_mode: bool,
}

impl ApplicationBase for Application {
    fn initialize_application(
        &mut self,
        engine_core: &EngineCore,
        window_size: &Vector2<i32>,
    ) {
        // engine managers
        self._engine_core = engine_core;
        self._audio_manager = engine_core.get_audio_manager();
        self._effect_manager = engine_core.get_effect_manager();
        self._renderer_data = engine_core.get_renderer_context().get_renderer_data();

        // initialize project managers
        let application = ptr_as_ref(self);
        self.get_game_resources_mut().initialize_game_resources(engine_core.get_engine_resources());
        self.get_game_resources_mut().load_game_resources(engine_core.get_renderer_context());
        self.get_character_manager_mut().initialize_character_manager(application);
        self.get_game_scene_manager_mut().initialize_game_scene_manager(application, engine_core, window_size);
        self.get_game_ui_manager_mut().initialize_game_ui_manager(engine_core, application);
        self.get_game_controller_mut().initialize_game_controller(application);
        self.get_game_client_mut().initialize_game_client(engine_core, application);

        // start game
        self.get_game_ui_manager_mut().build_game_ui(window_size);
        self.get_game_ui_manager_mut().show_ui(false);
        self.set_game_mode(self._is_game_mode);
        self.get_game_client_mut().start_game();
    }

    fn terminate_application(&mut self) {
        self._game_scene_manager.close_game_scene_data();
        self._game_client.destroy_game_client();
        self._game_scene_manager.destroy_game_scene_manager();
        self._game_resources.destroy_game_resources();
    }

    fn get_render_pass_create_info_callback(&self) -> *const CallbackLoadRenderPassCreateInfo {
        static CALLBACK: CallbackLoadRenderPassCreateInfo = render_pass::render_pass::get_render_pass_data_create_infos;
        &CALLBACK
    }

    fn update_event(&mut self) {
        let engine_core = ptr_as_ref(self._engine_core);
        let time_data = &engine_core._time_data;
        let mouse_move_data = &engine_core._mouse_move_data;
        let mouse_input_data = &engine_core._mouse_input_data;
        let keyboard_input_data = &engine_core._keyboard_input_data;

        if engine_core._keyboard_input_data.get_key_pressed(VirtualKeyCode::Tab)
        {
            self.toggle_game_mode();
        }

        if false == self._is_game_mode {
            const MOUSE_DELTA_RATIO: f32 = 500.0;
            let delta_time = time_data._delta_time;
            let _mouse_pos = &mouse_move_data._mouse_pos;
            let mouse_delta_x = mouse_move_data._mouse_pos_delta.x as f32
                / engine_core._window_size.x as f32
                * MOUSE_DELTA_RATIO;
            let mouse_delta_y = mouse_move_data._mouse_pos_delta.y as f32
                / engine_core._window_size.y as f32
                * MOUSE_DELTA_RATIO;
            let btn_left: bool = mouse_input_data._btn_l_hold;
            let btn_right: bool = mouse_input_data._btn_r_hold;
            let btn_r_pressed: bool = mouse_input_data._btn_r_pressed;
            let btn_r_released: bool = mouse_input_data._btn_r_released;
            let _btn_middle: bool = mouse_input_data._btn_m_hold;

            if btn_r_pressed {
                self.get_engine_core_mut().set_grab_mode(true);
            } else if btn_r_released {
                self.get_engine_core_mut().set_grab_mode(false);
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
            let scene_manager = self.get_game_scene_manager().get_scene_manager();
            let main_camera = scene_manager.get_main_camera_mut();
            let mut main_light = scene_manager._main_light.borrow_mut();
            let camera_move_speed_multiplier = if modifier_keys_shift { 2.0 } else { 1.0 };
            let move_speed: f32 = game_constants::EDITOR_CAMERA_MOVE_SPEED
                * camera_move_speed_multiplier
                * delta_time as f32;
            let pan_speed = game_constants::EDITOR_CAMERA_PAN_SPEED * camera_move_speed_multiplier;
            let rotation_speed = game_constants::EDITOR_CAMERA_ROTATION_SPEED;

            if released_key_left_bracket {
                self.get_renderer_data_mut().prev_debug_render_target();
            } else if released_key_right_bracket {
                self.get_renderer_data_mut().next_debug_render_target();
            }

            if released_key_subtract {
                self.get_renderer_data_mut()
                    .prev_debug_render_target_miplevel();
            } else if released_key_equals {
                self.get_renderer_data_mut()
                    .next_debug_render_target_miplevel();
            }

            if pressed_key_comma {
                main_light._transform_object.rotation_pitch(rotation_speed);
            } else if pressed_key_period {
                main_light._transform_object.rotation_pitch(-rotation_speed);
            }

            if btn_left && btn_right {
                main_camera
                    ._transform_object
                    .move_right(pan_speed * mouse_delta_x);
                main_camera
                    ._transform_object
                    .move_up(-pan_speed * mouse_delta_y);
            } else if btn_right {
                main_camera
                    ._transform_object
                    .rotation_pitch(rotation_speed * mouse_delta_y);
                main_camera
                    ._transform_object
                    .rotation_yaw(rotation_speed * mouse_delta_x);
            }

            if pressed_key_z {
                main_camera
                    ._transform_object
                    .rotation_roll(-rotation_speed * delta_time as f32 * 100.0);
            } else if pressed_key_c {
                main_camera
                    ._transform_object
                    .rotation_roll(rotation_speed * delta_time as f32 * 100.0);
            }

            if pressed_key_w {
                main_camera._transform_object.move_front(move_speed);
            } else if pressed_key_s {
                main_camera._transform_object.move_front(-move_speed);
            }

            if pressed_key_a {
                main_camera._transform_object.move_right(-move_speed);
            } else if pressed_key_d {
                main_camera._transform_object.move_right(move_speed);
            }

            if pressed_key_q {
                main_camera._transform_object.move_up(-move_speed);
            } else if pressed_key_e {
                main_camera._transform_object.move_up(move_speed);
            }
        }
    }

    fn update_application(&mut self, delta_time: f64) {
        let engine_core = ptr_as_ref(self._engine_core);
        let font_manager = engine_core.get_font_manager_mut();
        font_manager.clear_logs();

        // update managers
        if self._is_game_mode {
            self._game_client.update_game_mode(delta_time);
            self.get_game_ui_manager_mut().set_crosshair_pos(&engine_core._mouse_move_data._mouse_pos);
        }
        self._game_scene_manager.update_game_scene_manager(engine_core, delta_time);
        self._character_manager.update_character_manager(engine_core, delta_time);
        self._game_ui_manager.as_mut().update_game_ui(delta_time);
    }
}

impl Application {
    pub fn get_engine_core(&self) -> &EngineCore {
        ptr_as_ref(self._engine_core)
    }
    pub fn get_engine_core_mut(&self) -> &mut EngineCore {
        ptr_as_mut(self._engine_core)
    }
    pub fn get_effect_manager(&self) -> &EffectManager {
        ptr_as_ref(self._effect_manager)
    }
    pub fn get_effect_manager_mut(&self) -> &mut EffectManager {
        ptr_as_mut(self._effect_manager)
    }
    pub fn get_game_resources(&self) -> &GameResources {
        ptr_as_ref(self._game_resources.as_ref())
    }
    pub fn get_game_resources_mut(&self) -> &mut GameResources { ptr_as_mut(self._game_resources.as_ref()) }
    pub fn get_character_manager(&self) -> &CharacterManager {
        self._character_manager.as_ref()
    }
    pub fn get_character_manager_mut(&mut self) -> &mut CharacterManager { self._character_manager.as_mut() }
    pub fn get_game_scene_manager(&self) -> &GameSceneManager {
        self._game_scene_manager.as_ref()
    }
    pub fn get_game_scene_manager_mut(&mut self) -> &mut GameSceneManager { self._game_scene_manager.as_mut() }
    pub fn get_renderer_data(&self) -> &RendererData {
        ptr_as_ref(self._renderer_data)
    }
    pub fn get_renderer_data_mut(&self) -> &mut RendererData {
        ptr_as_mut(self._renderer_data)
    }
    pub fn get_game_ui_manager(&self) -> &GameUIManager { ptr_as_ref(self._game_ui_manager.as_ref()) }
    pub fn get_game_ui_manager_mut(&self) -> &mut GameUIManager { ptr_as_mut(self._game_ui_manager.as_ref()) }
    pub fn get_audio_manager(&self) -> &AudioManager {
        ptr_as_ref(self._audio_manager)
    }
    pub fn get_audio_manager_mut(&self) -> &mut AudioManager {
        ptr_as_mut(self._audio_manager)
    }
    pub fn get_game_controller(&self) -> &GameController { self._game_controller.as_ref() }
    pub fn get_game_controller_mut(&self) -> &mut GameController { ptr_as_mut(self._game_controller.as_ref()) }
    pub fn get_game_client(&self) -> &GameClient {
        self._game_client.as_ref()
    }
    pub fn get_game_client_mut(&self) -> &mut GameClient { ptr_as_mut(self._game_client.as_ref()) }
    pub fn toggle_game_mode(&mut self) {
        self.set_game_mode(!self._is_game_mode);
    }
    pub fn set_game_mode(&mut self, is_game_mode: bool) {
        self._is_game_mode = is_game_mode;
        self.get_game_client_mut().set_game_mode(is_game_mode);
        self.get_engine_core_mut().set_grab_mode(is_game_mode);
        self.get_engine_core_mut().get_ui_manager_mut().set_visible_world_axis(!is_game_mode);
    }
}

pub fn run_application() {
    // application setting
    let app_name: String = "Stone Age".to_string();
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
            constants::REQUIRED_INSTANCE_LAYERS = vec!["VK_LAYER_KHRONOS_validation".to_string()];
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
        // render option
        constants::RENDER_OCEAN = true;
    }

    // create project application & managers
    let game_resources = GameResources::create_game_resources();
    let game_scene_manager = GameSceneManager::create_game_scene_manager();
    let character_manager = CharacterManager::create_character_manager();
    let game_ui_manager = GameUIManager::create_game_ui_manager();
    let game_controller = GameController::create_game_controller();
    let game_client = GameClient::create_game_client();
    let application = Application {
        _engine_core: std::ptr::null(),
        _renderer_data: std::ptr::null(),
        _effect_manager: std::ptr::null(),
        _audio_manager: std::ptr::null(),
        _game_resources: game_resources,
        _game_scene_manager: game_scene_manager,
        _character_manager: character_manager,
        _game_ui_manager: game_ui_manager,
        _game_controller: game_controller,
        _game_client: game_client,
        _is_game_mode: false,
    };

    // run
    engine_core::run_application(
        app_name,
        app_version,
        initial_window_size,
        window_mode,
        log_level,
        &application, // TODO: Remove
    );
}
