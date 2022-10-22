use rust_engine_3d::renderer::renderer_data::RenderObjectType;
use rust_engine_3d::renderer::renderer_context::RendererContext;
use rust_engine_3d::resource::resource::RenderPassDataCreateInfoMap;
use crate::render_pass::{render_gbuffer, render_forward, render_shadow};

pub fn get_render_pass_data_create_infos(_renderer_context: &RendererContext, render_pass_data_create_info_map: &mut RenderPassDataCreateInfoMap) {
    render_forward::get_render_pass_data_create_info(RenderObjectType::Skeletal, render_pass_data_create_info_map);
    render_gbuffer::get_render_pass_data_create_info(RenderObjectType::Skeletal, render_pass_data_create_info_map);
    render_shadow::get_render_pass_data_create_info(RenderObjectType::Skeletal, render_pass_data_create_info_map);
}