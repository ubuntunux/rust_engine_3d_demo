use nalgebra::{ Vector2 };

use rust_engine_3d::application::scene_manager::ProjectSceneManagerBase;
use rust_engine_3d::renderer::ui::{ProjectUIManagerBase, Widget, WidgetDefault};
use rust_engine_3d::utilities::system::{ptr_as_ref, ptr_as_mut};
use crate::game_module::game_client::GameClient;
use crate::game_module::ui_widgets::hud::{CrossHair, TargetHud, PlayerHud, SelectionArea};
use crate::renderer::project_ui::ProjectUIManager;

pub struct GameUIManager {
    pub _game_client: *const GameClient,
    pub _project_ui_manager: *const ProjectUIManager,
    pub _crosshair: Option<CrossHair>,
    pub _target_hud: Option<TargetHud>,
    pub _player_hud: Option<PlayerHud>,
    pub _selection_area: Option<Box<SelectionArea>>
}

impl GameUIManager {
    pub fn create_game_ui_manager() -> Box<GameUIManager> {
        Box::new(GameUIManager {
            _game_client: std::ptr::null(),
            _project_ui_manager: std::ptr::null(),
            _crosshair: None,
            _target_hud: None,
            _player_hud: None,
            _selection_area: None,
        })
    }

    pub fn get_project_ui_manager(&self) -> &ProjectUIManager { ptr_as_ref(self._project_ui_manager) }
    pub fn get_project_ui_manager_mut(&self) -> &mut ProjectUIManager { ptr_as_mut(self._project_ui_manager) }
    pub fn get_game_client(&self) -> &GameClient { ptr_as_ref(self._game_client) }
    pub fn get_game_client_mut(&self) -> &mut GameClient { ptr_as_mut(self._game_client) }
    pub fn initialize_game_ui_manager(&mut self, game_client: &GameClient) {
        self._game_client = game_client;
        self._project_ui_manager = game_client.get_project_ui_manager();

        let project_resources = game_client.get_project_resources();
        let root_widget = game_client.get_project_ui_manager().get_root_widget_mut();
        let window_size = &game_client.get_project_application().get_engine_application()._window_size;
        let window_center = Vector2::<f32>::new(window_size.x as f32 * 0.5, window_size.y as f32 * 0.5,);

        self._crosshair = Some(CrossHair::create_crosshair(project_resources, root_widget, &window_center));
        self._target_hud = Some(TargetHud::create_target_hud(root_widget, &window_center));
        self._player_hud = Some(PlayerHud::create_player_hud(root_widget, &Vector2::new(window_size.x as f32 - 200.0, window_center.y as f32)));
        self._selection_area = Some(SelectionArea::create_selection_area(root_widget, window_size));
    }

    pub fn destroy_game_ui_manager(&mut self) {
    }

    pub fn get_crosshair_widget_mut(&mut self) -> &mut WidgetDefault {
        ptr_as_mut(self._crosshair.as_ref().unwrap()._widget)
    }

    pub fn show_selection_area(&mut self, show: bool) {
        let selection_area_widget = self._selection_area.as_ref().unwrap()._selection_area_layout.as_ref();
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
        self._crosshair.as_mut().unwrap()._pos.clone_from(pos);
    }

    pub fn update_game_ui(&mut self, _delta_time: f32) {
        let game_client = ptr_as_ref(self._game_client);
        let main_camera = game_client.get_project_scene_manager().get_main_camera();
        let window_size = &game_client.get_project_application().get_engine_application()._window_size;

        // Cross Hair
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

        // Player Hud
        let actor_manager = game_client.get_actor_manager();
        let player_actor = actor_manager.get_player_actor();
        let player_ship = player_actor.get_ship();
        let player_hud = self._player_hud.as_mut().unwrap();
        player_hud._hull_point_widget.update_hull_point_widget(player_ship.get_hull_point() / 2.0, player_ship.get_max_hull_point());
        player_hud._shield_point_widget.update_shield_point_widget(player_ship.get_shield_point() / 2.0, player_ship.get_max_shield_point());

        // Target Hud
        let target_hud = self._target_hud.as_mut().unwrap();
        let player_actor_pos = player_actor.get_transform().get_position();
        for (_id, actor) in actor_manager._actors.iter() {
            if false == actor.is_player_actor() {
                let actor_pos = actor.get_transform().get_position();
                let distance = (actor_pos - player_actor_pos).norm();
                let ship = actor.get_ship();
                let clamp: bool = true;
                let screen_pos: Vector2<f32> = main_camera.convert_world_to_screen(actor_pos, clamp);
                let target_widget = ptr_as_mut(target_hud._widget).get_ui_component_mut();
                target_widget.set_center(screen_pos.x, screen_pos.y);

                let target_distance = ptr_as_mut(target_hud._distance).get_ui_component_mut();
                target_distance.set_text(&format!("{}m", distance as i32));

                target_hud._hull_point_widget.update_hull_point_widget(ship.get_hull_point() / 2.0, ship.get_max_hull_point());
                target_hud._shield_point_widget.update_shield_point_widget(ship.get_shield_point() / 2.0, ship.get_max_shield_point());
                break;
            }
        }
    }
}