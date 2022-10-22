use rust_engine_3d::renderer::ui::{ProjectUIManagerBase, UIManager, UIWidgetTypes, Widget, UILayoutType, Orientation, HorizontalAlign, VerticalAlign, WidgetDefault};
use rust_engine_3d::renderer::renderer_context::RendererContext;
use rust_engine_3d::resource::resource::EngineResources;
use rust_engine_3d::vulkan_context::vulkan_context::{ get_color32 };
use rust_engine_3d::utilities::system::{ptr_as_mut};


pub struct ProjectUIManager {
    pub _ui_manager: *const UIManager,
    pub _root_widget: *const dyn Widget,
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
        let root = self.get_root_widget_mut();
        let btn0 = UIManager::create_widget("btn0", UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(btn0.as_ref()).get_ui_component_mut();
        ui_component.set_pos(25.0,255.0);
        ui_component.set_size(200.0, 100.0);
        ui_component.set_color(get_color32(255, 255, 255, 255));
        ui_component.set_font_color(get_color32(0, 0, 0, 255));
        ui_component.set_border_color(get_color32(255, 0, 0, 255));
        ui_component.set_margine(5.0);
        ui_component.set_round(10.0);
        ui_component.set_border(5.0);
        ui_component.set_dragable(true);
        ui_component.set_touchable(true);
        ui_component.set_expandable(true);
        ui_component.set_resizable(true);
        ui_component.set_text("btn0\nbtn0 Child Test");
        ui_component.set_material_instance(&engine_resources.get_material_instance_data("ui/render_ui_test"));
        root.add_widget(&btn0);

        let btn0_0 = UIManager::create_widget("btn0_0", UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(btn0_0.as_ref()).get_ui_component_mut();
        ui_component.set_pos(0.0, 5.0);
        ui_component.set_size(100.0, 50.0);
        ui_component.set_color(get_color32(255, 128, 128, 255));
        ui_component.set_font_color(get_color32(255, 255, 255, 255));
        ui_component.set_border_color(get_color32(0, 0, 0, 255));
        ui_component.set_margine(5.0);
        ui_component.set_round(10.0);
        ui_component.set_border(5.0);
        ui_component.set_dragable(true);
        ui_component.set_touchable(true);
        ui_component.set_expandable(true);
        ui_component.set_resizable(true);
        ui_component.set_text("btn0_0\nbtn0_0 Test");
        ptr_as_mut(btn0.as_ref()).add_widget(&btn0_0);

        let btn0_0_0 = UIManager::create_widget("btn0_0_0", UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(btn0_0_0.as_ref()).get_ui_component_mut();
        ui_component.set_pos(0.0, 5.0);
        ui_component.set_size(200.0, 100.0);
        ui_component.set_color(get_color32(128, 128, 255, 255));
        ui_component.set_font_color(get_color32(0, 0, 0, 255));
        ui_component.set_border_color(get_color32(0, 0, 0, 128));
        ui_component.set_margine(5.0);
        ui_component.set_round(10.0);
        ui_component.set_border(5.0);
        ui_component.set_dragable(true);
        ui_component.set_touchable(true);
        ui_component.set_expandable(true);
        ui_component.set_resizable(true);
        ui_component.set_text("btn0_0_0\nbtn0_0_0 Test");
        ptr_as_mut(btn0_0.as_ref()).add_widget(&btn0_0_0);

        //
        let btn0_1 = UIManager::create_widget("btn0_1", UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(btn0_1.as_ref()).get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::BoxLayout);
        ui_component.set_layout_orientation(Orientation::VERTICAL);
        ui_component.set_halign(HorizontalAlign::RIGHT);
        ui_component.set_valign(VerticalAlign::BOTTOM);
        ui_component.set_pos(100.0, 50.0);
        ui_component.set_size(100.0, 100.0);
        ui_component.set_color(get_color32(255, 128, 128, 255));
        ui_component.set_font_color(get_color32(255, 255, 255, 255));
        ui_component.set_border_color(get_color32(0, 0, 0, 255));
        ui_component.set_margine(5.0);
        ui_component.set_round(10.0);
        ui_component.set_border(5.0);
        ui_component.set_dragable(true);
        ui_component.set_touchable(true);
        ui_component.set_expandable(true);
        ui_component.set_resizable(true);
        ui_component.set_text("btn0_1\nbtn0_1 Test");
        ptr_as_mut(btn0.as_ref()).add_widget(&btn0_1);

        let btn0_1_0 = UIManager::create_widget("btn0_1_0", UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(btn0_1_0.as_ref()).get_ui_component_mut();
        ui_component.set_pos(0.0, 5.0);
        ui_component.set_size(50.0, 75.0);
        ui_component.set_color(get_color32(255, 128, 255, 255));
        ui_component.set_font_color(get_color32(0, 0, 0, 255));
        ui_component.set_border_color(get_color32(0, 0, 0, 128));
        ui_component.set_margine(5.0);
        ui_component.set_round(10.0);
        ui_component.set_border(5.0);
        ui_component.set_dragable(true);
        ui_component.set_touchable(true);
        ui_component.set_expandable(true);
        ui_component.set_resizable(true);
        ui_component.set_text("btn0_1_0\nbtn0_1_0 Test");
        ptr_as_mut(btn0_1.as_ref()).add_widget(&btn0_1_0);

        let btn0_1_1 = UIManager::create_widget("btn0_1_1", UIWidgetTypes::Default);
        let ui_component = ptr_as_mut(btn0_1_1.as_ref()).get_ui_component_mut();
        ui_component.set_halign(HorizontalAlign::RIGHT);
        ui_component.set_valign(VerticalAlign::BOTTOM);
        ui_component.set_pos(0.0, 5.0);
        ui_component.set_size(150.0, 50.0);
        ui_component.set_color(get_color32(128, 128, 255, 255));
        ui_component.set_font_color(get_color32(0, 0, 0, 255));
        ui_component.set_border_color(get_color32(0, 0, 0, 128));
        ui_component.set_margine(5.0);
        ui_component.set_margine_top(40.0);
        ui_component.set_padding_top(40.0);
        ui_component.set_round(10.0);
        ui_component.set_border(5.0);
        ui_component.set_dragable(true);
        ui_component.set_touchable(true);
        ui_component.set_resizable(true);
        ui_component.set_text("btn0_1_1\nbtn0_1_1 Test");
        ptr_as_mut(btn0_1.as_ref()).add_widget(&btn0_1_1);
    }
}

impl ProjectUIManager {
    pub fn create_project_ui_manager() -> Box<ProjectUIManager> {
        Box::new(ProjectUIManager {
            _ui_manager: std::ptr::null(),
            _root_widget: std::ptr::null() as *const WidgetDefault,
        })
    }
}