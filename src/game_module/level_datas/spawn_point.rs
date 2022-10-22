use nalgebra::Vector3;
use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum SpawnPointType {
    None,
    Player(ShipSpawnPointData),
    NonPlayer(ShipSpawnPointData),
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(default)]
pub struct ShipSpawnPointData {
    pub _ship_data_name: String,
    pub _position: Vector3<f32>,
    pub _rotation: Vector3<f32>
}