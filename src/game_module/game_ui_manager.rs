use std::rc::Rc;

use rust_engine_3d::scene::ui::{UIManager, Widget};
use crate::game_module::game_client::GameClient;
use crate::game_module::widgets::hud::{Crosshair, PlayerHud, SelectionArea, TargetHud};

pub struct GameUIManager {
    pub _ui_manager: *const UIManager,
    pub _game_client: *const GameClient,
    pub _root_widget: *const dyn Widget,
    pub _game_ui_layout: *const dyn Widget,
    pub _ui_switch: Option<Box<UISwitch>>,
    pub _crosshair: Option<Box<Crosshair>>,
    pub _target_hud: Option<Box<TargetHud>>,
    pub _player_hud: Option<Box<PlayerHud>>,
    pub _selection_area: Option<Box<SelectionArea>>,
}

pub struct UISwitch {
    pub _ui_switch_widget: Rc<dyn Widget>,
}