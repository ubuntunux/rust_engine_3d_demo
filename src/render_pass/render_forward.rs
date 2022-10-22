use std::path::PathBuf;
use ash::vk;
use rust_engine_3d::renderer::push_constants::{ PushConstant_RenderObject };
use rust_engine_3d::renderer::renderer_data::RenderObjectType;
use rust_engine_3d::resource::resource::RenderPassDataCreateInfoMap;
use rust_engine_3d::vulkan_context::render_pass::PipelinePushConstantData;

pub fn get_render_pass_data_create_info(render_object_type: RenderObjectType, render_pass_data_create_info_map: &mut RenderPassDataCreateInfoMap) {
    let render_pass_name = match render_object_type {
        RenderObjectType::Static => String::from("render_pass_static_forward"),
        RenderObjectType::Skeletal => String::from("render_pass_skeletal_forward"),
    };
    let render_pass_data_create_info = render_pass_data_create_info_map.get_mut(&*render_pass_name).unwrap();
    let mut pipeline_data_create_info = render_pass_data_create_info.get_pipeline_data_create_info_clone("render_object");
    pipeline_data_create_info._pipeline_data_create_info_name = String::from("render_ship");
    pipeline_data_create_info._pipeline_vertex_shader_file = PathBuf::from("render_ship.vert");
    pipeline_data_create_info._pipeline_fragment_shader_file = PathBuf::from("render_ship.frag");
    pipeline_data_create_info._push_constant_datas = vec![
        PipelinePushConstantData {
            _stage_flags: vk::ShaderStageFlags::ALL,
            _offset: 0,
            _push_constant: Box::new(PushConstant_RenderObject::default())
        }
    ];
    render_pass_data_create_info._pipeline_data_create_infos.push(pipeline_data_create_info);
}