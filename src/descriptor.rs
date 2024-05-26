use anyhow::{anyhow, Ok, Result};
use vulkanalia::prelude::v1_2::*;

use crate::AppData;

type Mat4 = cgmath::Matrix4<f32>;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct UniformBufferObject {
    model: Mat4,
    view: Mat4,
    proj: Mat4,
}

pub unsafe fn create_descriptor_set_layout(device: &Device, data: &mut AppData) -> Result<()> {
    let ubo_binding = vk::DescriptorSetLayoutBinding::builder()
        .binding(0)
        .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
        .descriptor_count(1)
        .stage_flags(vk::ShaderStageFlags::VERTEX);

    let bindings = [ubo_binding];

    let layout_info = vk::DescriptorSetLayoutCreateInfo::builder().bindings(&bindings);

    data.descriptor_set_layout = device.create_descriptor_set_layout(&layout_info, None)?;

    Ok(())
}
