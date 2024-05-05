use anyhow::{Ok, Result};

use vulkanalia::prelude::v1_2::*;

use crate::device::QueueFamilyIndices;
use crate::AppData;

pub unsafe fn create_framebuffers(device: &Device, data: &mut AppData) -> Result<()> {
    data.framebuffers = data
        .swapchain_images_views
        .iter()
        .map(|image_view| {
            let attachments = [*image_view];
            let framebuffer_info = vk::FramebufferCreateInfo::builder()
                .render_pass(data.render_pass)
                .attachments(&attachments)
                .width(data.swapchain_extent.width)
                .height(data.swapchain_extent.height)
                .layers(1);
            device.create_framebuffer(&framebuffer_info, None)
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(())
}

pub unsafe fn create_command_pool(
    instance: &Instance,
    device: &Device,
    data: &mut AppData,
) -> Result<()> {
    let indices = QueueFamilyIndices::get(instance, data, data.physical_device)?;

    let pool_info = vk::CommandPoolCreateInfo::builder()
        .flags(vk::CommandPoolCreateFlags::empty())
        .queue_family_index(indices.graphics());

    data.command_pool = device.create_command_pool(&pool_info, None)?;
    Ok(())
}

pub unsafe fn create_command_buffers(device: &Device, data: &mut AppData) -> Result<()> {
    let allocate_info = vk::CommandBufferAllocateInfo::builder()
        .command_pool(data.command_pool)
        .level(vk::CommandBufferLevel::PRIMARY)
        .command_buffer_count(data.framebuffers.len() as u32);

    data.command_buffers = device.allocate_command_buffers(&allocate_info)?;

    for (i, &command_buffer) in data.command_buffers.iter().enumerate() {
        let info = vk::CommandBufferBeginInfo::builder();

        device.begin_command_buffer(command_buffer, &info)?;

        let render_area = vk::Rect2D::builder()
            .offset(vk::Offset2D::default())
            .extent(data.swapchain_extent);

        let color_clear_value = vk::ClearValue {
            color: vk::ClearColorValue {
                float32: [0.0, 0.0, 0.0, 1.0],
            },
        };
        let clear_values = [color_clear_value];
        let info = vk::RenderPassBeginInfo::builder()
            .render_pass(data.render_pass)
            .framebuffer(data.framebuffers[i])
            .render_area(render_area)
            .clear_values(&clear_values);

        device.cmd_begin_render_pass(command_buffer, &info, vk::SubpassContents::INLINE);
        device.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::GRAPHICS, data.pipeline);
        device.cmd_draw(command_buffer, 3, 1, 0, 0);
        device.cmd_end_render_pass(command_buffer);

        device.end_command_buffer(command_buffer)?;
    }
    Ok(())
}

pub unsafe fn create_sync_objects(device: &Device, data: &mut AppData) -> Result<()> {
	let semaphore_info = vk::SemaphoreCreateInfo::builder();

	data.image_available_semaphore = device.create_semaphore(&semaphore_info, None)?;
	data.render_finished_semaphore = device.create_semaphore(&semaphore_info, None)?;

	Ok(())
}