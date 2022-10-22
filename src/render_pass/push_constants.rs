use std::fmt::Debug;
use nalgebra::{Vector4};
use serde::{Serialize, Deserialize};
use serde_json;
use rust_engine_3d::renderer::push_constants::{PushConstantParameter, PushConstant, PushConstantName};
use rust_engine_3d::utilities::json::convert_json_value_to_push_constant_parameter;

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct PushConstant_RenderShip {
    pub _transform_matrix_offset: u32,
    pub _bone_count: u32,
    pub _reserved0: u32,
    pub _reserved1: u32,
    pub _color: Vector4<f32>
}

impl Default for PushConstant_RenderShip {
    fn default() -> PushConstant_RenderShip {
        PushConstant_RenderShip {
            _transform_matrix_offset: 0,
            _bone_count: 0,
            _reserved0: 0,
            _reserved1: 0,
            _color: Vector4::new(1.0, 1.0, 1.0, 1.0)
        }
    }
}

impl PushConstantName for PushConstant_RenderShip {
    fn get_push_constant_name(&self) -> &str {
        "PushConstant_RenderShip"
    }
}

impl PushConstant for PushConstant_RenderShip {
    fn set_push_constant_parameter(&mut self, key: &str, value: &PushConstantParameter) {
        if "_transform_matrix_offset" == key {
            if let PushConstantParameter::Int(transform_matrix_offset) = value {
                self._transform_matrix_offset = *transform_matrix_offset as u32;
            }
        } else if "_bone_count" == key {
            if let PushConstantParameter::Int(bone_count) = value {
                self._bone_count = *bone_count as u32;
            }
        } else {
            panic!("Not implemented for {:?}", key);
        }
    }

    fn update_material_parameters(&mut self, material_parameters: &serde_json::Map<String, serde_json::Value>) {
        if let PushConstantParameter::Float4(value) = convert_json_value_to_push_constant_parameter(material_parameters, "_color") {
            self._color = value;
        }
    }
}