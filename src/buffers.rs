use anyhow::{Ok, Result};

use vulkanalia::prelude::v1_2::*;

use crate::device::QueueFamilyIndices;
use crate::vertex::{get_memory_type_index, INDICES};
use crate::{AppData, MAX_FRAMES_IN_FLIGHT};

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
        device.cmd_bind_pipeline(
            command_buffer,
            vk::PipelineBindPoint::GRAPHICS,
            data.pipeline,
        );

        // Bind vertex buffer
        let vertex_buffers = [data.vertex_buffer];
        let offsets = [0];
        device.cmd_bind_vertex_buffers(command_buffer, 0, &vertex_buffers, &offsets);
        device.cmd_bind_index_buffer(command_buffer, data.index_buffer, 0, vk::IndexType::UINT16);

        device.cmd_bind_descriptor_sets(
            command_buffer,
            vk::PipelineBindPoint::GRAPHICS,
            data.pipeline_layout,
            0,
            &[data.descriptor_sets[i]],
            &[],
        );
        device.cmd_draw_indexed(command_buffer, INDICES.len() as u32, 1, 0, 0, 0);
        device.cmd_end_render_pass(command_buffer);

        device.end_command_buffer(command_buffer)?;
    }
    Ok(())
}

pub unsafe fn create_sync_objects(device: &Device, data: &mut AppData) -> Result<()> {
    let semaphore_info = vk::SemaphoreCreateInfo::builder();
    let fence_info = vk::FenceCreateInfo::builder().flags(vk::FenceCreateFlags::SIGNALED);

    for _ in 0..MAX_FRAMES_IN_FLIGHT {
        data.image_available_semaphores
            .push(device.create_semaphore(&semaphore_info, None)?);
        data.render_finished_semaphores
            .push(device.create_semaphore(&semaphore_info, None)?);

        data.in_flight_fences
            .push(device.create_fence(&fence_info, None)?);
    }

    data.images_in_flight = data
        .swapchain_images
        .iter()
        .map(|_| vk::Fence::null())
        .collect();

    Ok(())
}

pub unsafe fn create_buffer(
    instance: &Instance,
    device: &Device,
    data: &mut AppData,
    size: vk::DeviceSize,
    usage: vk::BufferUsageFlags,
    properties: vk::MemoryPropertyFlags,
) -> Result<(vk::Buffer, vk::DeviceMemory)> {
    let buffer_info = vk::BufferCreateInfo::builder()
        .size(size)
        .usage(usage)
        .sharing_mode(vk::SharingMode::EXCLUSIVE);

    let buffer = device.create_buffer(&buffer_info, None)?;

    let memory_requirements = device.get_buffer_memory_requirements(buffer);

    let memory_type_index = get_memory_type_index(instance, data, properties, memory_requirements)?;

    let allocate_info = vk::MemoryAllocateInfo::builder()
        .allocation_size(memory_requirements.size)
        .memory_type_index(memory_type_index);

    let buffer_memory = device.allocate_memory(&allocate_info, None)?;

    device.bind_buffer_memory(buffer, buffer_memory, 0)?;

    Ok((buffer, buffer_memory))
}

pub unsafe fn copy_buffer(
    device: &Device,
    data: &AppData,
    source: vk::Buffer,
    destination: vk::Buffer,
    size: vk::DeviceSize,
) -> Result<()> {
    let command_buffer = begin_single_time_commands(device, data)?;

    let copy_region = vk::BufferCopy::builder()
        .src_offset(0)
        .dst_offset(0)
        .size(size);
    device.cmd_copy_buffer(command_buffer, source, destination, &[copy_region]);

    end_single_time_commands(device, data, command_buffer)?;

    Ok(())
}

pub unsafe fn begin_single_time_commands(
    device: &Device,
    data: &AppData,
) -> Result<vk::CommandBuffer> {
    let allocate_info = vk::CommandBufferAllocateInfo::builder()
        .level(vk::CommandBufferLevel::PRIMARY)
        .command_pool(data.command_pool)
        .command_buffer_count(1);

    let command_buffer = device.allocate_command_buffers(&allocate_info)?[0];

    let begin_info =
        vk::CommandBufferBeginInfo::builder().flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);

    device.begin_command_buffer(command_buffer, &begin_info)?;

    Ok(command_buffer)
}

pub unsafe fn end_single_time_commands(
    device: &Device,
    data: &AppData,
    command_buffer: vk::CommandBuffer,
) -> Result<()> {
    device.end_command_buffer(command_buffer)?;

    let command_buffers = [command_buffer];
    let submit_info = vk::SubmitInfo::builder().command_buffers(&command_buffers);

    device.queue_submit(data.graphics_queue, &[submit_info], vk::Fence::null())?;
    device.queue_wait_idle(data.graphics_queue)?;

    device.free_command_buffers(data.command_pool, &command_buffers);

    Ok(())
}
