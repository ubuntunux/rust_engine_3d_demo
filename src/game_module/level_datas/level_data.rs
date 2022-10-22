use serde::{ Serialize, Deserialize };

use crate::game_module::level_datas::spawn_point::{ SpawnPointType, ShipSpawnPointData };

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(default)]
pub struct LevelData {
    pub _spawn_point_datas: Vec<SpawnPointType>,
}

impl LevelData {
    pub fn get_test_level_data() -> LevelData {
        LevelData {
            _spawn_point_datas: vec![
                SpawnPointType::Player(ShipSpawnPointData {
                    _ship_data_name: "scout".to_string(),
                    ..Default::default()
                }),
                SpawnPointType::NonPlayer(ShipSpawnPointData {
                    _ship_data_name: "tank".to_string(),
                    ..Default::default()
                })
            ],
            ..Default::default()
        }
    }
}