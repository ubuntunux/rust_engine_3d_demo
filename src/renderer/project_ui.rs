use std::os::raw::c_void;
use std::rc::Rc;
use nalgebra::Vector2;
use rust_engine_3d::renderer::ui::{ProjectUIManagerBase, UIManager, UIWidgetTypes, Widget, WidgetDefault, CallbackTouchEvent, UIComponentInstance, HorizontalAlign, VerticalAlign};
use rust_engine_3d::renderer::renderer_context::RendererContext;
use rust_engine_3d::resource::resource::EngineResources;
use rust_engine_3d::vulkan_context::vulkan_context::{ get_color32 };
use rust_engine_3d::utilities::system::{ptr_as_mut};

// Declaration
pub struct ProjectUIManager {
    pub _ui_manager: *const UIManager,
    pub _root_widget: *const dyn Widget,
    pub _game_ui_layout: *const dyn Widget,
    pub _ui_switch: Option<UISwitch>
}

pub struct UISwitch {
    pub _ui_switch_widget: Rc<dyn Widget>
}

// Implemetation
impl ProjectUIManager {
    pub fn create_project_ui_manager() -> Box<ProjectUIManager> {
        Box::new(ProjectUIManager {
            _ui_manager: std::ptr::null(),
            _root_widget: std::ptr::null() as *const WidgetDefault,
            _game_ui_layout: std::ptr::null() as *const WidgetDefault,
            _ui_switch: None
        })
    }

    pub fn game_ui_layout(&self) -> *const dyn Widget {
        self._game_ui_layout
    }
}

impl ProjectUIManagerBase for ProjectUIManager {
    fn get_ui_manager(&self) -> &UIManager {
        unsafe { &*(self._ui_manager) }
    }

    fn get_ui_manager_mut(&self) -> &mut UIManager {
        unsafe { &mut *(self._ui_manager as *mut UIManager) }
    }

    fn get_root_widget(&self) -> &dyn Widget {
        unsafe { &*self._root_widget }
    }

    fn get_root_widget_mut(&self) -> &mut dyn Widget {
        unsafe { &mut *(self._root_widget as *mut dyn Widget) }
    }

    fn initialize_project_ui_manager(&mut self, ui_manager: &UIManager) {
        self._ui_manager = ui_manager;
        self._root_widget = self.get_ui_manager().get_root_ptr();
    }

    fn build_ui(&mut self, _renderer_context: &RendererContext, engine_resources: &EngineResources) {
        let game_ui_layout = UIManager::create_widget("game ui layout", UIWidgetTypes::Default);
        let game_ui_layout_mut = ptr_as_mut(game_ui_layout.as_ref());
        let ui_component = game_ui_layout_mut.get_ui_component_mut();
        ui_component.set_size_hint_x(Some(1.0));
        ui_component.set_size_hint_y(Some(1.0));
        ui_component.set_renderable(false);

        self._game_ui_layout = game_ui_layout.as_ref();

        let root_widget_mut = ptr_as_mut(self._root_widget);
        root_widget_mut.add_widget(&game_ui_layout);

        self._ui_switch = Some(UISwitch::create_ui_switch(engine_resources, root_widget_mut, game_ui_layout_mut));
    }
}

impl UISwitch {
    pub fn create_ui_switch(_engine_resources: &EngineResources, root_widget: &mut dyn Widget, game_ui_widget: &dyn Widget) -> UISwitch {
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
        ui_component.set_margine(5.0);
        ui_component.set_round(10.0);
        ui_component.set_border(2.0);
        ui_component.set_touchable(true);
        //ui_component.set_material_instance(&engine_resources.get_material_instance_data("ui/render_ui_test"));

        static TOUCH_DOWN: CallbackTouchEvent = UISwitch::touch_down;
        ui_component.set_callback_touch_down(&TOUCH_DOWN);
        ui_component.set_user_data(game_ui_widget.get_ui_component() as *const UIComponentInstance as *const c_void);
        root_widget.add_widget(&ui_switch_widget);

        let ui_switch = UISwitch {
            _ui_switch_widget: ui_switch_widget,
        };

        ui_switch
    }

    pub fn touch_down(ui_component: &mut UIComponentInstance, _touched_pos: &Vector2<f32>, _touched_pos_delta: &Vector2<f32>) -> bool {
        let game_ui_component = ptr_as_mut(ui_component.get_user_data() as *const UIComponentInstance);
        game_ui_component.set_visible(!game_ui_component.get_visible());
        true
    }
}