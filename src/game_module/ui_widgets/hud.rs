use std::os::raw::c_void;

use nalgebra::Vector2;

use rust_engine_3d::renderer::ui::*;
use rust_engine_3d::resource::resource::ProjectResourcesBase;
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use crate::game_module::ui_widgets::hit_point_widgets::{ShieldPointWidget, HullPointWidget};
use crate::resource::project_resource::ProjectResources;
use rust_engine_3d::utilities::system::{ptr_as_mut, ptr_as_ref};
use std::rc::Rc;


pub struct TargetHud {
    pub _widget: *const WidgetDefault,
    pub _distance: *const WidgetDefault,
    pub _hull_point_widget: HullPointWidget,
    pub _shield_point_widget: ShieldPointWidget,
}

pub struct PlayerHud {
    pub _widget: *const WidgetDefault,
    pub _hull_point_widget: HullPointWidget,
    pub _shield_point_widget: ShieldPointWidget,
}

pub struct CrossHair {
    pub _widget: *const WidgetDefault,
    pub _pos: Vector2<i32>,
    pub _tracking_mouse: bool,
}

pub struct SelectionArea {
    pub _selection_area_layout: Rc<dyn Widget>,
    pub _selection_widget: Rc<dyn Widget>,
    pub _drag_mouse: bool,
}

// CrossHair
impl CrossHair {
    pub fn create_crosshair(project_resources: &ProjectResources, root_widget: &mut dyn Widget, window_center: &Vector2<f32>) -> CrossHair {
        let crosshair_widget = UIManager::create_widget("cursor", UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(crosshair_widget.as_ref()).get_ui_component_mut();
        let ui_size = 50.0f32;
        ui_component.set_pos(window_center.x - ui_size * 0.5, window_center.y - ui_size * 0.5);
        ui_component.set_size(ui_size, ui_size);
        ui_component.set_material_instance(&project_resources.get_material_instance_data("ui/crosshair"));
        root_widget.add_widget(&crosshair_widget);

        CrossHair {
            _widget: crosshair_widget.as_ref() as *const dyn Widget as *const WidgetDefault,
            _pos: Vector2::zeros(),
            _tracking_mouse: true,
        }
    }
}

// TargetHud
impl TargetHud {
    pub fn create_target_hud(root_widget: &mut dyn Widget, center: &Vector2<f32>) -> TargetHud {
        let hud_layer_width: f32 = 100.0;
        let hud_layer_height: f32 = 100.0;
        let hud_layer_padding: f32 = 10.0;
        let hud_ui_width: f32 = 100.0;
        let hud_ui_height: f32 = 25.0;
        let hud_ui_margine: f32 = 2.0;
        let hud_ui_padding: f32 = 4.0;

        let target_widget = UIManager::create_widget("target_widget", UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(target_widget.as_ref()).get_ui_component_mut();
        ui_component.set_size(hud_layer_width, hud_layer_height);
        ui_component.set_center(center.x, center.y);
        ui_component.set_layout_type(UILayoutType::BoxLayout);
        ui_component.set_layout_orientation(Orientation::VERTICAL);
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_expandable(true);
        ui_component.set_padding(hud_layer_padding);
        ui_component.set_color(get_color32(255, 255, 255, 10));
        ui_component.set_opacity(0.5);
        root_widget.add_widget(&target_widget);

        let target_distance = UIManager::create_widget("target_distance", UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(target_distance.as_ref()).get_ui_component_mut();
        ui_component.set_text("100m");
        ui_component.set_size(hud_ui_width, hud_ui_height);
        ui_component.set_halign(HorizontalAlign::LEFT);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_color(get_color32(255, 0, 0, 20));
        ui_component.set_font_color(get_color32(255, 255, 255, 255));
        ui_component.set_margine(hud_ui_margine);
        ui_component.set_padding(hud_ui_padding);
        ui_component.set_expandable(true);
        ptr_as_mut(target_widget.as_ref()).add_widget(&target_distance);

        TargetHud {
            _widget: target_widget.as_ref() as *const dyn Widget as *const WidgetDefault,
            _distance: target_distance.as_ref() as *const dyn Widget as *const WidgetDefault,
            _hull_point_widget: HullPointWidget::create_hull_point_widget(ptr_as_mut(target_widget.as_ref())),
            _shield_point_widget: ShieldPointWidget::create_shield_point_widget(ptr_as_mut(target_widget.as_ref()))
        }
    }
}

// PlayerHud
impl PlayerHud {
    pub fn create_player_hud(root_widget: &mut dyn Widget, pos: &Vector2<f32>) -> PlayerHud {
        let hud_layer_width: f32 = 100.0;
        let hud_layer_height: f32 = 100.0;
        let hud_layer_padding: f32 = 10.0;

        let player_widget = UIManager::create_widget("player_widget", UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(player_widget.as_ref()).get_ui_component_mut();
        ui_component.set_size(hud_layer_width, hud_layer_height);
        ui_component.set_pos(pos.x, pos.y);
        ui_component.set_layout_type(UILayoutType::BoxLayout);
        ui_component.set_layout_orientation(Orientation::VERTICAL);
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_expandable(true);
        ui_component.set_padding(hud_layer_padding);
        ui_component.set_color(get_color32(255, 255, 255, 10));
        root_widget.add_widget(&player_widget);

        PlayerHud {
            _widget: player_widget.as_ref() as *const dyn Widget as *const WidgetDefault,
            _hull_point_widget: HullPointWidget::create_hull_point_widget(ptr_as_mut(player_widget.as_ref())),
            _shield_point_widget: ShieldPointWidget::create_shield_point_widget(ptr_as_mut(player_widget.as_ref())),
        }
    }
}


// Selection Area
impl SelectionArea {
    pub fn create_selection_area(root_widget: &mut dyn Widget, window_size: &Vector2<i32>) -> Box<SelectionArea> {
        let selection_area_layout = UIManager::create_widget("selection_area_layout", UIWidgetTypes::Default);
        let layout_ui_component = ptr_as_mut(selection_area_layout.as_ref()).get_ui_component_mut();
        layout_ui_component.set_size(window_size.x as f32 - 200.0, window_size.y as f32 - 200.0);
        layout_ui_component.set_pos(0.0, 0.0);
        layout_ui_component.set_color(get_color32(0, 0, 0, 0));
        layout_ui_component.set_border_color(get_color32(255, 255, 0, 255));
        layout_ui_component.set_border(2.0);
        layout_ui_component.set_touchable(true);
        static TOUCH_DOWN: CallbackTouchEvent = SelectionArea::touch_down;
        static TOUCH_MOVE: CallbackTouchEvent = SelectionArea::touch_move;
        static TOUCH_UP: CallbackTouchEvent = SelectionArea::touch_up;
        layout_ui_component.set_callback_touch_down(&TOUCH_DOWN);
        layout_ui_component.set_callback_touch_move(&TOUCH_MOVE);
        layout_ui_component.set_callback_touch_up(&TOUCH_UP);
        root_widget.add_widget(&selection_area_layout);

        let selection_widget = UIManager::create_widget("selection_area_widget", UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(selection_widget.as_ref()).get_ui_component_mut();
        ui_component.set_color(get_color32(255, 255, 0, 128));
        ui_component.set_border_color(get_color32(255, 255, 0, 255));
        ui_component.set_size(0f32, 0f32);
        ui_component.set_round(5.0);
        ui_component.set_border(2.0);
        ui_component.set_visible(false);
        ptr_as_mut(selection_area_layout.as_ref()).add_widget(&selection_widget);

        let selection_area = Box::new(SelectionArea {
            _selection_area_layout: selection_area_layout,
            _selection_widget: selection_widget,
            _drag_mouse: false,
        });

        // set user data
        layout_ui_component.set_user_data(selection_area.as_ref() as *const SelectionArea as *const c_void);

        selection_area
    }

    pub fn touch_down(ui_component: &mut UIComponentInstance, touched_pos: &Vector2<f32>, _touched_pos_delta: &Vector2<f32>) -> bool {
        let selection_area = ptr_as_ref(ui_component.get_user_data() as *const SelectionArea);
        let selection_widget = selection_area._selection_widget.as_ref();
        let selection_ui_component = ptr_as_mut(selection_widget).get_ui_component_mut();
        selection_ui_component.set_pos(touched_pos.x, touched_pos.y);
        selection_ui_component.set_size(0f32, 0f32);
        selection_ui_component.set_visible(true);
        true
    }

    pub fn touch_move(ui_component: &mut UIComponentInstance, touched_pos: &Vector2<f32>, _touched_pos_delta: &Vector2<f32>) -> bool {
        let touch_start_pos: &Vector2<f32> = ui_component.get_touch_start_pos();
        let size: Vector2<f32> = touch_start_pos - touched_pos;

        let selection_area = ptr_as_ref(ui_component.get_user_data() as *const SelectionArea);
        let selection_widget = selection_area._selection_widget.as_ref();
        let selection_ui_component = ptr_as_mut(selection_widget).get_ui_component_mut();
        selection_ui_component.set_pos_x(touch_start_pos.x - 0f32.max(size.x));
        selection_ui_component.set_pos_y(touch_start_pos.y - 0f32.max(size.y));
        selection_ui_component.set_size(size.x.abs(), size.y.abs());
        true
    }

    pub fn touch_up(ui_component: &mut UIComponentInstance, touched_pos: &Vector2<f32>, _touched_pos_delta: &Vector2<f32>) -> bool {
        let selection_area = ptr_as_ref(ui_component.get_user_data() as *const SelectionArea);
        let selection_widget = selection_area._selection_widget.as_ref();
        let selection_ui_component = ptr_as_mut(selection_widget).get_ui_component_mut();
        selection_ui_component.set_pos(touched_pos.x, touched_pos.y);
        selection_ui_component.set_size(0f32, 0f32);
        selection_ui_component.set_visible(false);
        true
    }
}