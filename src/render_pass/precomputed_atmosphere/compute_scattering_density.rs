use std::path::PathBuf;

use ash::vk;
use rust_engine_3d::utilities::system::enum_to_string;
use rust_engine_3d::vulkan_context::framebuffer::{ self, FramebufferDataCreateInfo, RenderTargetInfo };
use rust_engine_3d::vulkan_context::geometry_buffer::{ VertexData, StaticVertexData };
use rust_engine_3d::vulkan_context::render_pass::{
    RenderPassDataCreateInfo,
    PipelineDataCreateInfo,
    ImageAttachmentDescription,
};
use rust_engine_3d::vulkan_context::descriptor::{
    DescriptorDataCreateInfo,
    DescriptorResourceType,
};
use rust_engine_3d::vulkan_context::vulkan_context::{ self, BlendMode };

use crate::renderer::precomputed_atmosphere::{ DEFAULT_USE_COMBINED_TEXTURES, PushConstant_PrecomputedAtmosphere };
use crate::renderer::render_target::RenderTargetType;
use crate::renderer::project_renderer::ProjectRenderer;
use crate::renderer::shader_buffer_datas::ShaderBufferDataType;

pub fn get_framebuffer_data_create_info(project_renderer: &ProjectRenderer) -> FramebufferDataCreateInfo {
    let render_target = project_renderer.get_render_target(RenderTargetType::PRECOMPUTED_ATMOSPHERE_DELTA_SCATTERING_DENSITY);
    framebuffer::create_framebuffer_data_create_info(
        &[RenderTargetInfo { _texture_data: render_target, _target_layer: 0, _target_mip_level: 0, _clear_value: None, }],
        &[],
        &[]
    )
}

pub fn get_render_pass_data_create_info(project_renderer: &ProjectRenderer) -> RenderPassDataCreateInfo {
    let render_pass_name = String::from("compute_scattering_density");
    let framebuffer_data_create_info = get_framebuffer_data_create_info(project_renderer);
    let mut color_attachment_descriptions: Vec<ImageAttachmentDescription> = Vec::new();
    for format in framebuffer_data_create_info._framebuffer_color_attachment_formats.iter() {
        color_attachment_descriptions.push(
            ImageAttachmentDescription {
                _attachment_image_format: *format,
                _attachment_load_operation: vk::AttachmentLoadOp::DONT_CARE,
                _attachment_store_operation: vk::AttachmentStoreOp::STORE,
                _attachment_final_layout: vk::ImageLayout::GENERAL,
                _attachment_reference_layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
                ..Default::default()
            }
        );
    }
    let subpass_dependencies = vec![
        vk::SubpassDependency {
            src_subpass: vk::SUBPASS_EXTERNAL,
            dst_subpass: 0,
            src_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
            dst_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
            src_access_mask: vk::AccessFlags::empty(),
            dst_access_mask: vk::AccessFlags::COLOR_ATTACHMENT_READ | vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
            dependency_flags: vk::DependencyFlags::BY_REGION,
        }
    ];
    let pipeline_data_create_infos = vec![
        PipelineDataCreateInfo {
            _pipeline_data_create_info_name: String::from("default"),
            _pipeline_vertex_shader_file: PathBuf::from("precomputed_atmosphere/render_atmosphere.vert"),
            _pipeline_fragment_shader_file: PathBuf::from("precomputed_atmosphere/compute_scattering_density.frag"),
            _pipeline_shader_defines: vec![
                format!("COMBINED_SCATTERING_TEXTURES={:?}", if DEFAULT_USE_COMBINED_TEXTURES { 1 } else { 0 }),
            ],
            _pipeline_bind_point: vk::PipelineBindPoint::GRAPHICS,
            _pipeline_color_blend_modes: vec![vulkan_context::get_color_blend_mode(BlendMode::None); color_attachment_descriptions.len()],
            _pipeline_cull_mode: vk::CullModeFlags::BACK,
            _pipeline_front_face: vk::FrontFace::COUNTER_CLOCKWISE,
            _vertex_input_bind_descriptions: StaticVertexData::get_vertex_input_binding_descriptions(),
            _vertex_input_attribute_descriptions: StaticVertexData::create_vertex_input_attribute_descriptions(),
            _push_constant_ranges: vec![vk::PushConstantRange {
                stage_flags: vk::ShaderStageFlags::ALL,
                offset: 0,
                size: std::mem::size_of::<PushConstant_PrecomputedAtmosphere>() as u32,
            }],
            _descriptor_data_create_infos: vec![
                DescriptorDataCreateInfo {
                    _descriptor_binding_index: 0,
                    _descriptor_name: enum_to_string(&ShaderBufferDataType::SceneConstants),
                    _descriptor_resource_type: DescriptorResourceType::UniformBuffer,
                    _descriptor_shader_stage: vk::ShaderStageFlags::VERTEX | vk::ShaderStageFlags::FRAGMENT,
                    ..Default::default()
                },
                DescriptorDataCreateInfo {
                    _descriptor_binding_index: 1,
                    _descriptor_name: enum_to_string(&ShaderBufferDataType::ViewConstants),
                    _descriptor_resource_type: DescriptorResourceType::UniformBuffer,
                    _descriptor_shader_stage: vk::ShaderStageFlags::VERTEX | vk::ShaderStageFlags::FRAGMENT,
                    ..Default::default()
                },
                DescriptorDataCreateInfo {
                    _descriptor_binding_index: 8,
                    _descriptor_name: enum_to_string(&RenderTargetType::PRECOMPUTED_ATMOSPHERE_TRANSMITTANCE),
                    _descriptor_resource_type: DescriptorResourceType::RenderTarget,
                    _descriptor_shader_stage: vk::ShaderStageFlags::FRAGMENT,
                    ..Default::default()
                },
                DescriptorDataCreateInfo {
                    _descriptor_binding_index: 9,
                    _descriptor_name: enum_to_string(&RenderTargetType::PRECOMPUTED_ATMOSPHERE_DELTA_IRRADIANCE),
                    _descriptor_resource_type: DescriptorResourceType::RenderTarget,
                    _descriptor_shader_stage: vk::ShaderStageFlags::FRAGMENT,
                    ..Default::default()
                },
                DescriptorDataCreateInfo {
                    _descriptor_binding_index: 11,
                    _descriptor_name: enum_to_string(&RenderTargetType::PRECOMPUTED_ATMOSPHERE_DELTA_MIE_SCATTERING),
                    _descriptor_resource_type: DescriptorResourceType::RenderTarget,
                    _descriptor_shader_stage: vk::ShaderStageFlags::FRAGMENT,
                    ..Default::default()
                },
                DescriptorDataCreateInfo {
                    _descriptor_binding_index: 12,
                    _descriptor_name: enum_to_string(&RenderTargetType::PRECOMPUTED_ATMOSPHERE_DELTA_RAYLEIGH_SCATTERING),
                    _descriptor_resource_type: DescriptorResourceType::RenderTarget,
                    _descriptor_shader_stage: vk::ShaderStageFlags::FRAGMENT,
                    ..Default::default()
                },
                DescriptorDataCreateInfo {
                    _descriptor_binding_index: 14,
                    _descriptor_name: enum_to_string(&RenderTargetType::PRECOMPUTED_ATMOSPHERE_DELTA_RAYLEIGH_SCATTERING), // equal to DELTA_MULTIPLE_SCATTERING_TEXTURE
                    _descriptor_resource_type: DescriptorResourceType::RenderTarget,
                    _descriptor_shader_stage: vk::ShaderStageFlags::FRAGMENT,
                    ..Default::default()
                },
            ],
            ..Default::default()
        }
    ];

    RenderPassDataCreateInfo  {
        _render_pass_create_info_name: render_pass_name.clone(),
        _render_pass_framebuffer_create_info: framebuffer_data_create_info,
        _color_attachment_descriptions: color_attachment_descriptions,
        _depth_attachment_descriptions: Vec::new(),
        _resolve_attachment_descriptions: Vec::new(),
        _subpass_dependencies: subpass_dependencies,
        _pipeline_data_create_infos: pipeline_data_create_infos,
    }
}