use std::rc::Rc;

use rust_engine_3d::scene::ui::{UIManager, Widget};

pub struct ProjectUIManager {
    pub _ui_manager: *const UIManager,
    pub _root_widget: *const dyn Widget,
    pub _game_ui_layout: *const dyn Widget,
    pub _ui_switch: Option<UISwitch>,
    pub _ui_world_axis: Option<UIWorldAxis>,
}

pub struct UISwitch {
    pub _ui_switch_widget: Rc<dyn Widget>,
}

pub struct UIWorldAxis {
    pub _widget_axis_x: Rc<dyn Widget>,
    pub _widget_axis_y: Rc<dyn Widget>,
    pub _widget_axis_z: Rc<dyn Widget>,
}
