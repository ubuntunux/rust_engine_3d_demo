use std::path::PathBuf;

use ash::vk;
use rust_engine_3d::vulkan_context::framebuffer::{ self, FramebufferDataCreateInfo, RenderTargetInfo };
use rust_engine_3d::vulkan_context::geometry_buffer::{ VertexData, StaticVertexData };
use rust_engine_3d::vulkan_context::render_pass::{
    RenderPassDataCreateInfo,
    PipelineDataCreateInfo,
    ImageAttachmentDescription,
    DepthStencilStateCreateInfo,
};
use rust_engine_3d::vulkan_context::descriptor::{
    DescriptorDataCreateInfo,
    DescriptorResourceType,
};
use rust_engine_3d::vulkan_context::vulkan_context::{ self, BlendMode };

use crate::renderer::fft_ocean::PushConstant_FFT_Init;
use crate::renderer::render_target::RenderTargetType;
use crate::renderer::project_renderer::ProjectRenderer;

pub fn get_framebuffer_data_create_info(project_renderer: &ProjectRenderer) -> FramebufferDataCreateInfo {
    let render_target = project_renderer.get_render_target(RenderTargetType::FFT_A);
    let render_target_infos: Vec<RenderTargetInfo> = (0..render_target._image_layers).map(|index| {
        RenderTargetInfo {
            _texture_data: render_target,
            _target_layer: index,
            _target_mip_level: 0,
            _clear_value: Some(vulkan_context::get_color_clear_value(0.0, 0.0, 0.0, 0.0)),
        }
    }).collect();

    framebuffer::create_framebuffer_data_create_info(&render_target_infos, &[], &[])
}

pub fn get_render_pass_data_create_info(project_renderer: &ProjectRenderer) -> RenderPassDataCreateInfo {
    let render_pass_name = String::from("render_fft_init");
    let framebuffer_data_create_info = get_framebuffer_data_create_info(project_renderer);
    let sample_count = framebuffer_data_create_info._framebuffer_sample_count;
    let mut color_attachment_descriptions: Vec<ImageAttachmentDescription> = Vec::new();
    for format in framebuffer_data_create_info._framebuffer_color_attachment_formats.iter() {
        color_attachment_descriptions.push(
            ImageAttachmentDescription {
                _attachment_image_format: *format,
                _attachment_image_samples: sample_count,
                _attachment_load_operation: vk::AttachmentLoadOp::CLEAR,
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
            _pipeline_data_create_info_name: String::from("render_fft_init"),
            _pipeline_vertex_shader_file: PathBuf::from("render_quad.vert"),
            _pipeline_fragment_shader_file: PathBuf::from("fft_ocean/render_fft_init.frag"),
            _pipeline_bind_point: vk::PipelineBindPoint::GRAPHICS,
            _pipeline_shader_defines: Vec::new(),
            _pipeline_dynamic_states: vec![vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR],
            _pipeline_sample_count: sample_count,
            _pipeline_cull_mode: vk::CullModeFlags::BACK,
            _pipeline_front_face: vk::FrontFace::COUNTER_CLOCKWISE,
            _pipeline_color_blend_modes: vec![vulkan_context::get_color_blend_mode(BlendMode::None); color_attachment_descriptions.len()],
            _depth_stencil_state_create_info: DepthStencilStateCreateInfo::default(),
            _vertex_input_bind_descriptions: StaticVertexData::get_vertex_input_binding_descriptions(),
            _vertex_input_attribute_descriptions: StaticVertexData::create_vertex_input_attribute_descriptions(),
            _push_constant_ranges: vec![vk::PushConstantRange {
                stage_flags: vk::ShaderStageFlags::ALL,
                offset: 0,
                size: std::mem::size_of::<PushConstant_FFT_Init>() as u32,
            }],
            _descriptor_data_create_infos: vec![
                DescriptorDataCreateInfo {
                    _descriptor_binding_index: 0,
                    _descriptor_name: String::from("texture_spectrum_1_2"),
                    _descriptor_resource_type: DescriptorResourceType::Texture,
                    _descriptor_shader_stage: vk::ShaderStageFlags::FRAGMENT,
                    ..Default::default()
                },
                DescriptorDataCreateInfo {
                    _descriptor_binding_index: 1,
                    _descriptor_name: String::from("texture_spectrum_3_4"),
                    _descriptor_resource_type: DescriptorResourceType::Texture,
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