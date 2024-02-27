use std::os::raw::c_void;

use nalgebra::Vector2;
use rust_engine_3d::core::engine_core::EngineCore;
use rust_engine_3d::resource::resource::EngineResources;
use rust_engine_3d::scene::ui::{
    CallbackTouchEvent, HorizontalAlign, UIComponentInstance, UIManager,
    UIWidgetTypes, VerticalAlign, Widget, WidgetDefault,
};
use rust_engine_3d::utilities::system::{ptr_as_mut, ptr_as_ref};
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;

use crate::application::application::Application;
use crate::game_module::game_client::GameClient;
use crate::game_module::game_ui_manager::*;
use crate::game_module::widgets::hud::{Crosshair, PlayerHud, SelectionArea, TargetHud};

impl GameUIManager {
    pub fn create_game_ui_manager() -> Box<GameUIManager> {
        Box::new(GameUIManager {
            _ui_manager: std::ptr::null(),
            _game_client: std::ptr::null(),
            _root_widget: std::ptr::null() as *const WidgetDefault,
            _game_ui_layout: std::ptr::null() as *const WidgetDefault,
            _ui_switch: None,
            _crosshair: None,
            _target_hud: None,
            _player_hud: None,
            _selection_area: None,
        })
    }

    pub fn game_ui_layout(&self) -> *const dyn Widget {
        self._game_ui_layout
    }
}

impl UISwitch {
    pub fn create_ui_switch(
        _engine_resources: &EngineResources,
        root_widget: &mut dyn Widget,
        game_ui_widget: &dyn Widget,
    ) -> UISwitch {
        let ui_switch_widget = UIManager::create_widget("ui_switch", UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(ui_switch_widget.as_ref()).get_ui_component_mut();
        ui_component.set_text("UI On/Off");
        ui_component.set_pos_hint_x(Some(0.5));
        ui_component.set_pos_hint_y(Some(0.0));
        ui_component.set_size(150.0, 50.0);
        ui_component.set_font_size(20.0);
        ui_component.set_color(get_color32(128, 128, 255, 128));
        ui_component.set_font_color(get_color32(255, 255, 255, 255));
        ui_component.set_border_color(get_color32(0, 0, 0, 128));
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_margin(5.0);
        ui_component.set_round(10.0);
        ui_component.set_border(2.0);
        ui_component.set_touchable(true);
        //ui_component.set_material_instance(&engine_resources.get_material_instance_data("ui/render_ui_test"));

        static TOUCH_DOWN: CallbackTouchEvent = UISwitch::touch_down;
        ui_component.set_callback_touch_down(&TOUCH_DOWN);
        ui_component.set_user_data(
            game_ui_widget.get_ui_component() as *const UIComponentInstance as *const c_void,
        );
        root_widget.add_widget(&ui_switch_widget);

        let ui_text_widget = UIManager::create_widget("ui_text_widget", UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(ui_text_widget.as_ref()).get_ui_component_mut();
        ui_component.set_text("Tab: Toggle GameMode <-> NavigationMode\nNavigationMode\n\tMove:W,A,S,D\n\tRotation:Hold Mouse Right Click\nGameMode\n\tMove: A,S,Left,Right\n\tJump:SpaceBar\n\tAttack:Mouse Left Click\n");
        ui_component.set_pos_hint_x(Some(0.0));
        ui_component.set_pos_hint_y(Some(0.0));
        ui_component.set_size_hint_x(Some(1.0));
        ui_component.set_size_hint_y(Some(1.0));
        // ui_component.set_font_size(20.0);
        ui_component.set_color(get_color32(128, 128, 255, 0));
        ui_component.set_font_color(get_color32(255, 255, 255, 255));
        // ui_component.set_border_color(get_color32(0, 0, 0, 128));
        // ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::BOTTOM);
        ui_component.set_margin(10.0);
        // ui_component.set_round(10.0);
        // ui_component.set_border(2.0);
        // ui_component.set_touchable(true);
        root_widget.add_widget(&ui_text_widget);

        let ui_switch = UISwitch {
            _ui_switch_widget: ui_switch_widget,
        };

        ui_switch
    }

    pub fn touch_down(
        ui_component: &mut UIComponentInstance,
        _touched_pos: &Vector2<f32>,
        _touched_pos_delta: &Vector2<f32>,
    ) -> bool {
        let game_ui_component =
            ptr_as_mut(ui_component.get_user_data() as *const UIComponentInstance);
        game_ui_component.set_visible(!game_ui_component.get_visible());
        true
    }
}

impl GameUIManager {
    pub fn initialize_game_ui_manager(&mut self, engine_core: &EngineCore, application: &Application) {
        log::info!("initialize_game_ui_manager");
        self._game_client = application.get_game_client();
        self._ui_manager = engine_core.get_ui_manager();
        self._root_widget = ptr_as_ref(self._ui_manager).get_root_ptr();
    }
    pub fn destroy_game_ui_manager(&mut self) {
    }
    pub fn get_game_client(&self) -> &GameClient {
        ptr_as_ref(self._game_client)
    }
    pub fn get_game_client_mut(&self) -> &mut GameClient {
        ptr_as_mut(self._game_client)
    }
    pub fn get_ui_manager(&self) -> &UIManager {
        ptr_as_ref(self._ui_manager)
    }
    pub fn get_ui_manager_mut(&self) -> &mut UIManager {
        ptr_as_mut(self._ui_manager)
    }
    pub fn get_root_widget(&self) -> &dyn Widget {
        ptr_as_ref(self._root_widget)
    }
    pub fn get_root_widget_mut(&self) -> &mut dyn Widget {
        ptr_as_mut(self._root_widget as *mut dyn Widget)
    }
    pub fn build_game_ui(&mut self, window_size: &Vector2<i32>) {
        log::info!("build_game_ui");
        let game_client = ptr_as_ref(self._game_client);
        let game_resources = game_client.get_game_resources();
        let engine_resources = game_resources.get_engine_resources();

        // create layout
        let game_ui_layout = UIManager::create_widget("game ui layout", UIWidgetTypes::Default);
        let game_ui_layout_mut = ptr_as_mut(game_ui_layout.as_ref());
        let ui_component = game_ui_layout_mut.get_ui_component_mut();
        ui_component.set_size_hint_x(Some(1.0));
        ui_component.set_size_hint_y(Some(1.0));
        ui_component.set_renderable(false);

        let root_widget_mut = ptr_as_mut(self._root_widget);
        root_widget_mut.add_widget(&game_ui_layout);
        self._game_ui_layout = game_ui_layout.as_ref();

        let window_center =
            Vector2::<f32>::new(window_size.x as f32 * 0.5, window_size.y as f32 * 0.5);

        self._ui_switch = Some(Box::new(UISwitch::create_ui_switch(
            engine_resources,
            root_widget_mut,
            game_ui_layout_mut,
        )));
        self._crosshair = Some(Box::new(Crosshair::create_crosshair(
            game_resources,
            game_ui_layout_mut,
            &window_center,
        )));
        self._target_hud = Some(Box::new(TargetHud::create_target_hud(
            game_ui_layout_mut,
            &window_center,
        )));
        self._player_hud = Some(Box::new(PlayerHud::create_player_hud(
            game_ui_layout_mut,
            &Vector2::new(window_size.x as f32 - 200.0, window_center.y),
        )));
        self._selection_area = Some(SelectionArea::create_selection_area(
            game_ui_layout_mut,
            window_size,
        ));
    }

    pub fn get_crosshair_widget_mut(&mut self) -> &mut WidgetDefault {
        ptr_as_mut(self._crosshair.as_ref().unwrap()._widget)
    }

    pub fn show_ui(&mut self, show: bool) {
        if false == self._game_ui_layout.is_null() {
            let game_ui_layout_mut = ptr_as_mut(self._game_ui_layout);
            game_ui_layout_mut.get_ui_component_mut().set_visible(show);
        }
    }

    pub fn show_selection_area(&mut self, show: bool) {
        let selection_area_widget = self
            ._selection_area
            .as_ref()
            .unwrap()
            ._selection_area_layout
            .as_ref();
        let ui_component = ptr_as_mut(selection_area_widget.get_ui_component());
        ui_component.set_visible(show);
    }

    pub fn show_crosshair(&mut self, show: bool) {
        let ui_component = self.get_crosshair_widget_mut().get_ui_component_mut();
        ui_component.set_visible(show);
    }

    pub fn set_crosshair_tracking_mouse(&mut self, tracking: bool) {
        self._crosshair.as_mut().unwrap()._tracking_mouse = tracking;
    }

    pub fn set_crosshair_pos(&mut self, pos: &Vector2<i32>) {
        if self._crosshair.is_some() {
            self._crosshair.as_mut().unwrap()._pos.clone_from(pos);
        }
    }

    pub fn update_game_ui(&mut self, _delta_time: f64) {
        let game_client = ptr_as_ref(self._game_client);
        let window_size = &game_client
            .get_application()
            .get_engine_core()
            ._window_size;

        // Cross Hair
        if self._crosshair.is_some() {
            let crosshair = self._crosshair.as_ref().unwrap();
            let crosshair_widget = ptr_as_mut(crosshair._widget);
            if crosshair_widget.get_ui_component().get_visible() {
                let crosshair_pos_x: i32;
                let crosshair_pos_y: i32;

                if crosshair._tracking_mouse {
                    crosshair_pos_x = crosshair._pos.x;
                    crosshair_pos_y = crosshair._pos.y;
                } else {
                    crosshair_pos_x = window_size.x / 2;
                    crosshair_pos_y = window_size.y / 2;
                }
                let ui_component = crosshair_widget.get_ui_component_mut();
                ui_component.set_center(crosshair_pos_x as f32, crosshair_pos_y as f32);
            }
        }
    }
}
