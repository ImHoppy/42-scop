use anyhow::{anyhow, Result};
use log::*;
use thiserror::Error;

use vulkanalia::prelude::v1_2::*;

use crate::{AppData, VALIDATION_ENABLED, VALIDATION_LAYER, PORTABILITY_MACOS_VERSION};

#[derive(Debug, Error)]
#[error("{0}")]
pub struct SuitabilityError(pub &'static str);


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
    Ok(())
}

#[derive(Copy, Clone, Debug)]
struct QueueFamilyIndices {
    graphics: u32,
}

impl QueueFamilyIndices {
    unsafe fn get(
        instance: &Instance,
        data: &AppData,
        physical_device: vk::PhysicalDevice,
    ) -> Result<Self> {
        let properties = instance.get_physical_device_queue_family_properties(physical_device);

        let graphics = properties
            .iter()
            .position(|p| p.queue_flags.contains(vk::QueueFlags::GRAPHICS))
            .map(|i| i as u32);

        if let Some(graphics) = graphics {
            Ok(Self { graphics })
        } else {
            Err(anyhow!(SuitabilityError(
                "Missing required queue families."
            )))
        }
    }
}
