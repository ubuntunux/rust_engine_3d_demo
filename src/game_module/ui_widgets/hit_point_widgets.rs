use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use rust_engine_3d::renderer::ui::{UIManager, Widget, UIWidgetTypes, HorizontalAlign, VerticalAlign, UILayoutType, WidgetDefault};
use rust_engine_3d::utilities::system::ptr_as_mut;

const WIDGET_UI_WIDTH: f32 = 120.0;
const WIDGET_UI_HEIGHT: f32 = 24.0;
const WIDGET_UI_MARGINE: f32 = 2.0;
const WIDGET_UI_PADDING: f32 = 2.0;

pub struct HullPointWidget {
    pub _hull_point_layer: *const WidgetDefault,
    pub _hull_point_bar: *const WidgetDefault,
}

pub struct ShieldPointWidget {
    pub _shield_point_layer: *const WidgetDefault,
    pub _shield_point_bar: *const WidgetDefault,
}


// Implementation
fn create_hit_point_layer_widget(parent_widget: &mut dyn Widget) -> *const WidgetDefault {
    let hit_point_layer = UIManager::create_widget("hit_point_layer", UIWidgetTypes::Default);
    let ui_component = ptr_as_mut(hit_point_layer.as_ref()).get_ui_component_mut();
    ui_component.set_layout_type(UILayoutType::BoxLayout);
    ui_component.set_text("Point");
    ui_component.set_size(WIDGET_UI_WIDTH, WIDGET_UI_HEIGHT);
    ui_component.set_halign(HorizontalAlign::LEFT);
    ui_component.set_valign(VerticalAlign::CENTER);
    ui_component.set_color(get_color32(50, 50, 50, 255));
    ui_component.set_font_color(get_color32(255, 255, 255, 255));
    ui_component.set_border_color(get_color32(0, 0, 0, 255));
    ui_component.set_round(5.0);
    ui_component.set_border(2.0);
    ui_component.set_margine(WIDGET_UI_MARGINE);
    ui_component.set_padding(WIDGET_UI_PADDING);
    parent_widget.add_widget(&hit_point_layer);
    hit_point_layer.as_ref() as *const dyn Widget as *const WidgetDefault
}

fn create_hit_point_bar_widget(parent_widget: &mut dyn Widget, color: u32) -> *const WidgetDefault {
    let hull_point_bar = UIManager::create_widget("hit_point_bar", UIWidgetTypes::Default);
    let ui_component = ptr_as_mut(hull_point_bar.as_ref()).get_ui_component_mut();
    ui_component.set_size_hint_x(Some(0.5));
    ui_component.set_size_hint_y(Some(1.0));
    ui_component.set_halign(HorizontalAlign::LEFT);
    ui_component.set_valign(VerticalAlign::CENTER);
    ui_component.set_color(color);
    ui_component.set_round(1.0);
    parent_widget.add_widget(&hull_point_bar);
    hull_point_bar.as_ref() as *const dyn Widget as *const WidgetDefault
}

impl HullPointWidget {
    pub fn create_hull_point_widget(parent_widget: &mut dyn Widget) -> HullPointWidget {
        let hull_point_layer = create_hit_point_layer_widget(parent_widget);
        let hull_point_bar = create_hit_point_bar_widget(ptr_as_mut(hull_point_layer), get_color32(255, 75, 0, 75));
        HullPointWidget {
            _hull_point_layer: hull_point_layer,
            _hull_point_bar: hull_point_bar
        }
    }

    pub fn update_hull_point_widget(&self, hull_point: f32, max_hull_point: f32) {
        let hull_point_ratio = 1.0f32.min(hull_point / max_hull_point);
        let hull_point_ui = ptr_as_mut(self._hull_point_layer).get_ui_component_mut();
        hull_point_ui.set_text(&format!("Hull: {}", hull_point as i32));
        let hull_point_bar = ptr_as_mut(self._hull_point_bar).get_ui_component_mut();
        hull_point_bar.set_size_hint_x(Some(hull_point_ratio));
    }
}

impl ShieldPointWidget {
    pub fn create_shield_point_widget(parent_widget: &mut dyn Widget) -> ShieldPointWidget {
        let shield_point_layer = create_hit_point_layer_widget(parent_widget);
        let shield_point_bar = create_hit_point_bar_widget(ptr_as_mut(shield_point_layer), get_color32(75, 75, 255, 75));
        ShieldPointWidget {
            _shield_point_layer: shield_point_layer,
            _shield_point_bar: shield_point_bar
        }
    }

    pub fn update_shield_point_widget(&self, shield_point: f32, max_shield_point: f32) {
        let shield_point_ratio = 1.0f32.min(shield_point / max_shield_point);
        let shield_point_ui = ptr_as_mut(self._shield_point_layer).get_ui_component_mut();
        shield_point_ui.set_text(&format!("Shield: {}", shield_point as i32));
        let shield_point_bar = ptr_as_mut(self._shield_point_bar).get_ui_component_mut();
        shield_point_bar.set_size_hint_x(Some(shield_point_ratio));
    }
}
