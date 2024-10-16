use crate::swapchain;

use std::collections::HashSet;

use anyhow::{anyhow, Ok, Result};
use log::*;
use thiserror::Error;

use vulkanalia::{prelude::v1_2::*, vk::KhrSurfaceExtension};

use crate::{AppData, PORTABILITY_MACOS_VERSION, VALIDATION_ENABLED, VALIDATION_LAYER};

const DEVICE_EXTENSIONS: &[vk::ExtensionName] = &[vk::KHR_SWAPCHAIN_EXTENSION.name];

#[derive(Debug, Error)]
#[error("{0}")]
pub struct SuitabilityError(pub &'static str);

// Create a logical device.
pub unsafe fn create_logical_device(
    entry: &Entry,
    instance: &Instance,
    data: &mut AppData,
) -> Result<Device> {
    let indices = QueueFamilyIndices::get(instance, data, data.physical_device)?;

    let mut unique_indices = std::collections::HashSet::new();
    unique_indices.insert(indices.graphics);
    unique_indices.insert(indices.present);

    let queue_priorities = &[1.0];
    let queue_infos = unique_indices
        .iter()
        .map(|&index| {
            vk::DeviceQueueCreateInfo::builder()
                .queue_family_index(index)
                .queue_priorities(queue_priorities)
        })
        .collect::<Vec<_>>();

    let layers = if VALIDATION_ENABLED {
        vec![VALIDATION_LAYER.as_ptr()]
    } else {
        vec![]
    };

    let mut extensions = DEVICE_EXTENSIONS
        .iter()
        .map(|ext| ext.as_ptr())
        .collect::<Vec<_>>();

    // Required by Vulkan SDK on macOS since 1.3.216.
    if cfg!(target_os = "macos") && entry.version()? >= PORTABILITY_MACOS_VERSION {
        extensions.push(vk::KHR_PORTABILITY_SUBSET_EXTENSION.name.as_ptr());
    }

    let features = vk::PhysicalDeviceFeatures::builder()
        .fill_mode_non_solid(true)
        .sampler_anisotropy(true);

    let device_info = vk::DeviceCreateInfo::builder()
        .queue_create_infos(&queue_infos)
        .enabled_layer_names(&layers)
        .enabled_extension_names(&extensions)
        .enabled_features(&features);

    let device = instance.create_device(data.physical_device, &device_info, None)?;
    data.graphics_queue = device.get_device_queue(indices.graphics, 0);
    data.present_queue = device.get_device_queue(indices.present, 0);

    Ok(device)
}

// Picks a physical device.
pub unsafe fn pick_physical_device(instance: &Instance, data: &mut AppData) -> Result<()> {
    let mut best_score = 0;
    let mut best_physical_device = None;

    for physical_device in instance.enumerate_physical_devices()? {
        let properties = instance.get_physical_device_properties(physical_device);

        if let Err(error) = check_physical_device(instance, data, physical_device) {
            warn!(
                "Skipping physical device (`{}`): {}",
                properties.device_name, error
            );
        } else {
            debug!(
                "Found physical device (`{}`) with device type: {:?}",
                properties.device_name, properties.device_type
            );
            let score = calculate_physical_device_score(&properties);
            if score > best_score {
                best_score = score;
                best_physical_device = Some(physical_device);
            }
        }
    }

    if let Some(physical_device) = best_physical_device {
        let properties = instance.get_physical_device_properties(physical_device);
        info!(
            "Selected physical device (`{}`) with device type: {:?} with score: {}",
            properties.device_name, properties.device_type, best_score
        );
        data.physical_device = physical_device;
        Ok(())
    } else {
        Err(anyhow!("Failed to find suitable physical device."))
    }
}

fn calculate_physical_device_score(properties: &vk::PhysicalDeviceProperties) -> u32 {
    let mut score = 0;

    match properties.device_type {
        vk::PhysicalDeviceType::DISCRETE_GPU => score += 5,
        vk::PhysicalDeviceType::INTEGRATED_GPU => score += 2,
        vk::PhysicalDeviceType::VIRTUAL_GPU => score += 1,
        _ => (),
    }

    score
}

unsafe fn check_physical_device(
    instance: &Instance,
    data: &AppData,
    physical_device: vk::PhysicalDevice,
) -> Result<()> {
    QueueFamilyIndices::get(instance, data, physical_device)?;
    check_physical_device_extensions(instance, physical_device)?;
    let support = swapchain::SwapchainSupport::get(instance, data, physical_device)?;
    if support.formats().is_empty() || support.present_modes().is_empty() {
        return Err(anyhow!(SuitabilityError(
            "Missing required swapchain support."
        )));
    }
    let features = instance.get_physical_device_features(physical_device);
    if features.sampler_anisotropy != vk::TRUE {
        return Err(anyhow!(SuitabilityError("No sampler anisotropy")));
    }
    Ok(())
}

unsafe fn check_physical_device_extensions(
    instance: &Instance,
    physical_device: vk::PhysicalDevice,
) -> Result<()> {
    let available_extensions = instance
        .enumerate_device_extension_properties(physical_device, None)?
        .iter()
        .map(|extension| extension.extension_name)
        .collect::<HashSet<_>>();

    if DEVICE_EXTENSIONS
        .iter()
        .all(|ext| available_extensions.contains(ext))
    {
        Ok(())
    } else {
        Err(anyhow!(SuitabilityError(
            "Missing required device extensions."
        )))
    }
}

#[derive(Copy, Clone, Debug)]
pub struct QueueFamilyIndices {
    graphics: u32,
    present: u32,
}

impl QueueFamilyIndices {
    pub fn graphics(&self) -> u32 {
        self.graphics
    }
    pub fn present(&self) -> u32 {
        self.present
    }
    pub unsafe fn get(
        instance: &Instance,
        data: &AppData,
        physical_device: vk::PhysicalDevice,
    ) -> Result<Self> {
        let properties = instance.get_physical_device_queue_family_properties(physical_device);

        let graphics = properties
            .iter()
            .position(|p| p.queue_flags.contains(vk::QueueFlags::GRAPHICS))
            .map(|index| index as u32);

        let mut present = None;
        for (index, _) in properties.iter().enumerate() {
            if instance.get_physical_device_surface_support_khr(
                physical_device,
                index as u32,
                data.surface,
            )? {
                present = Some(index as u32);
                break;
            }
        }

        if let (Some(graphics), Some(present)) = (graphics, present) {
            Ok(Self { graphics, present })
        } else {
            Err(anyhow!(SuitabilityError(
                "Missing required queue families."
            )))
        }
    }
}

pub unsafe fn get_memory_type_index(
    instance: &Instance,
    data: &AppData,
    properties: vk::MemoryPropertyFlags,
    requirements: vk::MemoryRequirements,
) -> Result<u32> {
    let memory_properties = instance.get_physical_device_memory_properties(data.physical_device);

    (0..memory_properties.memory_type_count)
        .find(|i| {
            let suitable = (requirements.memory_type_bits & (1 << i)) != 0;
            let memory_type = memory_properties.memory_types[*i as usize];

            suitable && memory_type.property_flags.contains(properties)
        })
        .ok_or_else(|| anyhow!("Failed to find suitable memory type."))
}
