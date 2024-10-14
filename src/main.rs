mod buffers;
mod depth;
mod descriptor;
mod device;
mod math;
mod model;
mod obj;
mod pipeline;
mod swapchain;
mod textures;
mod vertex;

use anyhow::{anyhow, Result};
use descriptor::{Mat4, UniformBufferObject};
use device::{create_logical_device, pick_physical_device};
use log::*;
use math::{perspective, vec2, vec3, Deg, Vec2, Vec3};
use std::collections::HashSet;
use std::ffi::CStr;
use std::mem::size_of;
use std::os::raw::c_void;
use std::ptr::copy_nonoverlapping as memcpy;
use std::time::Instant;
use vertex::Vertex;
use winit::keyboard::Key;

use winit::dpi::LogicalSize;
use winit::event::{ElementState, Event, KeyEvent, MouseButton, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

use vulkanalia::loader::{LibloadingLoader, LIBRARY};
use vulkanalia::prelude::v1_2::*;
use vulkanalia::vk::{ExtDebugUtilsExtension, KhrSurfaceExtension, KhrSwapchainExtension};
use vulkanalia::window as vk_window;
use vulkanalia::Version;

pub const PORTABILITY_MACOS_VERSION: Version = Version::new(1, 3, 216);

pub const VALIDATION_ENABLED: bool = cfg!(debug_assertions);

pub const VALIDATION_LAYER: vk::ExtensionName =
    vk::ExtensionName::from_bytes(b"VK_LAYER_KHRONOS_validation");

pub const MAX_FRAMES_IN_FLIGHT: usize = 2;

fn main() -> Result<()> {
    pretty_env_logger::init();

    let obj_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("./resources/texture_cube.obj"));
    let texture_path = std::env::args()
        .nth(2)
        .unwrap_or_else(|| String::from("./resources/orange_texture.png"));

    // Window

    let event_loop = EventLoop::new()?;
    let window = WindowBuilder::new()
        .with_title("scop")
        .with_inner_size(LogicalSize::new(1024, 768))
        .build(&event_loop)?;

    // App

    let mut app = unsafe { App::create(&window, obj_path, texture_path)? };
    let mut minimized = false;

    event_loop.run(move |event, elwt| {
        match event {
            // Request a redraw when all events were processed.
            Event::AboutToWait => window.request_redraw(),
            Event::WindowEvent { event, .. } => match event {
                // Render a frame if our Vulkan app is not being destroyed.
                WindowEvent::RedrawRequested if !elwt.exiting() && !minimized => {
                    unsafe { app.render(&window) }.unwrap()
                }
                // Destroy our Vulkan app.
                WindowEvent::CloseRequested => {
                    elwt.exit();
                    unsafe {
                        app.destroy();
                    }
                }
                WindowEvent::Resized(size) => {
                    if size.width == 0 || size.height == 0 {
                        minimized = true;
                    } else {
                        minimized = false;
                        app.resized = true;
                    }
                }
                // Client input
                WindowEvent::MouseWheel { delta, .. } => match delta {
                    winit::event::MouseScrollDelta::LineDelta(_, y) => {
                        let value = y as f32 * 0.1;
                        if app.controls.zoom + value > 0.0 {
                            app.controls.zoom += value;
                        }
                    }
                    winit::event::MouseScrollDelta::PixelDelta(pos) => {
                        let value = pos.y as f32 * 0.01;
                        if app.controls.zoom + value > 0.0 {
                            app.controls.zoom += value;
                        }
                    }
                },
                WindowEvent::MouseInput { state, button, .. } => {
                    if button == MouseButton::Left {
                        app.controls.mouse_pressed = state == ElementState::Pressed;
                    }
                }
                WindowEvent::CursorMoved { position, .. } => {
                    if app.controls.mouse_pressed {
                        let delta_x = position.x as f32 - app.controls.last_mouse_pos.x;
                        let delta_y = position.y as f32 - app.controls.last_mouse_pos.y;
                        app.controls.rotation.x += delta_x as f32 * 0.1;
                        app.controls.rotation.y += delta_y as f32 * -0.1;
                    }
                    app.controls.last_mouse_pos.x = position.x as f32;
                    app.controls.last_mouse_pos.y = position.y as f32;
                }
                WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            logical_key: key,
                            state,
                            ..
                        },
                    ..
                } => match (key.as_ref(), state) {
                    (Key::Character("w"), ElementState::Pressed) => app.controls.rotation.y += 1.0,
                    (Key::Character("s"), ElementState::Pressed) => app.controls.rotation.y -= 1.0,
                    (Key::Character("a"), ElementState::Pressed) => app.controls.rotation.x -= 1.0,
                    (Key::Character("d"), ElementState::Pressed) => app.controls.rotation.x += 1.0,
                    (Key::Character("r"), ElementState::Pressed) => {
                        app.controls.auto_rotate = !app.controls.auto_rotate
                    }
                    (Key::Character("f"), ElementState::Pressed) => {
                        app.data.wireframe = !app.data.wireframe;
                        unsafe {
                            let _ = app.recreate_swapchain(&window);
                        }
                    }
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        }
    })?;

    Ok(())
}

/// The controls for our Vulkan app.
#[derive(Clone, Debug, Default)]
struct Controls {
    zoom: f32,
    rotation: Vec2,
    auto_rotate: bool,
    mouse_pressed: bool,
    last_mouse_pos: Vec2,
}

/// Our Vulkan app.
#[derive(Clone, Debug)]
pub struct App {
    entry: Entry,
    instance: Instance,
    data: AppData,
    device: Device,
    frame: usize,
    resized: bool,
    start: Instant,
    controls: Controls,
}

impl App {
    /// Creates our Vulkan app.
    unsafe fn create(window: &Window, obj_path: String, texture_path: String) -> Result<Self> {
        let loader = LibloadingLoader::new(LIBRARY)?;
        let entry = Entry::new(loader).map_err(|err| anyhow!(err))?;
        let mut data = AppData::default();
        let instance = create_instance(window, &entry, &mut data)?;
        data.surface = vk_window::create_surface(&instance, &window, &window)?;
        pick_physical_device(&instance, &mut data)?;
        let device = create_logical_device(&entry, &instance, &mut data)?;
        swapchain::create_swapchain(window, &instance, &device, &mut data)?;
        swapchain::create_swapchain_image_views(&device, &mut data)?;
        pipeline::create_render_pass(&instance, &device, &mut data)?;
        descriptor::create_descriptor_set_layout(&device, &mut data)?;
        pipeline::create(&device, &mut data)?;
        buffers::create_command_pool(&instance, &device, &mut data)?;
        depth::create_depth_objects(&instance, &device, &mut data)?;
        buffers::create_framebuffers(&device, &mut data)?;
        textures::create_texture_image(&instance, &device, &mut data, texture_path)?;
        textures::create_texture_image_view(&device, &mut data)?;
        textures::create_texture_sampler(&device, &mut data)?;
        model::load_model(&mut data, obj_path)?;
        vertex::create_vertex_buffer(&instance, &device, &mut data)?;
        vertex::create_index_buffer(&instance, &device, &mut data)?;
        descriptor::create_uniform_buffers(&instance, &device, &mut data)?;
        descriptor::create_descriptor_pool(&device, &mut data)?;
        descriptor::create_descriptor_sets(&device, &mut data)?;
        buffers::create_command_buffers(&device, &mut data)?;
        buffers::create_sync_objects(&device, &mut data)?;
        Ok(Self {
            entry,
            instance,
            data,
            device,
            frame: 0,
            resized: false,
            start: Instant::now(),
            controls: Controls {
                zoom: 1.0,
                rotation: vec2(0.0, 45.0),
                auto_rotate: false,
                ..Default::default()
            },
        })
    }

    /// Renders a frame for our Vulkan app.
    unsafe fn render(&mut self, window: &Window) -> Result<()> {
        let in_flight_fence = self.data.in_flight_fences[self.frame];

        self.device
            .wait_for_fences(&[in_flight_fence], true, u64::MAX)?;

        let result = self.device.acquire_next_image_khr(
            self.data.swapchain,
            u64::MAX,
            self.data.image_available_semaphores[self.frame],
            vk::Fence::null(),
        );

        let image_index = match result {
            Ok((image_index, _)) => image_index as usize,
            Err(vk::ErrorCode::OUT_OF_DATE_KHR) => return self.recreate_swapchain(window),
            Err(error) => return Err(anyhow!("Failed to acquire next image: {}", error)),
        };

        if !self.data.images_in_flight[image_index as usize].is_null() {
            self.device.wait_for_fences(
                &[self.data.images_in_flight[image_index as usize]],
                true,
                u64::MAX,
            )?;
        }

        self.data.images_in_flight[image_index as usize] = in_flight_fence;

        self.update_uniform_buffer(image_index)?;

        let wait_semaphores = [self.data.image_available_semaphores[self.frame]];
        let wait_stages = [vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
        let command_buffers = [self.data.command_buffers[image_index]];
        let signal_semaphores = [self.data.render_finished_semaphores[self.frame]];
        let submit_info = vk::SubmitInfo::builder()
            .wait_semaphores(&wait_semaphores)
            .wait_dst_stage_mask(&wait_stages)
            .command_buffers(&command_buffers)
            .signal_semaphores(&signal_semaphores);

        self.device.reset_fences(&[in_flight_fence])?;

        self.device
            .queue_submit(self.data.graphics_queue, &[submit_info], in_flight_fence)?;

        let swapchains = [self.data.swapchain];
        let image_indices = [image_index as u32];
        let present_info = vk::PresentInfoKHR::builder()
            .wait_semaphores(&signal_semaphores)
            .swapchains(&swapchains)
            .image_indices(&image_indices);

        let result = self
            .device
            .queue_present_khr(self.data.present_queue, &present_info);
        let changed = result == Ok(vk::SuccessCode::SUBOPTIMAL_KHR)
            || result == Err(vk::ErrorCode::OUT_OF_DATE_KHR);

        if self.resized || changed {
            self.resized = false;
            self.recreate_swapchain(window)?;
        } else if let Err(e) = result {
            return Err(anyhow!("Failed to present queue: {}", e));
        }

        self.frame = (self.frame + 1) % MAX_FRAMES_IN_FLIGHT;

        Ok(())
    }

    /// Destroys our Vulkan app.
    #[rustfmt::skip]
    unsafe fn destroy(&mut self) {
        self.device.device_wait_idle().unwrap();

        self.destroy_swapchain();

        self.data.in_flight_fences.iter().for_each(|f| self.device.destroy_fence(*f, None));
        self.data.render_finished_semaphores.iter().for_each(|s| self.device.destroy_semaphore(*s, None));
        self.data.image_available_semaphores.iter().for_each(|s| self.device.destroy_semaphore(*s, None));

        self.device.destroy_sampler(self.data.texture_sampler, None);
        self.device.destroy_image_view(self.data.texture_image_view, None);
        self.device.destroy_image(self.data.texture_image, None);
        self.device.free_memory(self.data.texture_image_memory, None);
        self.device.destroy_buffer(self.data.vertex_buffer, None);
        self.device.free_memory(self.data.vertex_buffer_memory, None);
        self.device.destroy_buffer(self.data.index_buffer, None);
        self.device.free_memory(self.data.index_buffer_memory, None);
        self.device.destroy_command_pool(self.data.command_pool, None);
        self.device.destroy_descriptor_set_layout(self.data.descriptor_set_layout, None);
        self.device.destroy_device(None);
        self.instance.destroy_surface_khr(self.data.surface, None);

        if VALIDATION_ENABLED {
            self.instance
                .destroy_debug_utils_messenger_ext(self.data.messenger, None);
        }

        self.instance.destroy_instance(None);
    }

    unsafe fn update_uniform_buffer(&mut self, image_index: usize) -> Result<()> {
        let time = self.start.elapsed().as_secs_f32();

        let num_vertices = self.data.vertices.len() as u32;
        let mut sum = Vec3::default();
        for vertex in &self.data.vertices {
            sum += vertex.pos;
        }
        sum /= num_vertices as f32;

        let model = Mat4::from_axis_angle(
            vec3(0.0, 1.0, 0.0),
            if self.controls.auto_rotate { time } else { 1.0 },
        ) * Mat4::from_translation(-sum);

        let theta_x = self.controls.rotation.x * (std::f32::consts::PI / 180.0);
        let theta_y = self.controls.rotation.y * (std::f32::consts::PI / 180.0);
        let radius: f32 = 20.0 * self.controls.zoom;

        let camera: Vec3 = vec3(
            radius * theta_x.cos() * theta_y.sin() + 0.1,
            radius * theta_y.cos() + 0.1,
            radius * theta_x.sin() * theta_y.sin() + 0.1,
        );

        let view = Mat4::look_at_rh(camera, sum, vec3(0.0, 1.0, 0.0));

        #[rustfmt::skip]
        let correction = Mat4::new(
            1.0, 0.0,       0.0, 0.0,
            0.0, 1.0,       0.0, 0.0,
            0.0, 0.0, 1.0 / 2.0, 0.0,
            0.0, 0.0, 1.0 / 2.0, 1.0,
        );

        let proj = correction
            * perspective(
                Deg(45.0),
                self.data.swapchain_extent.width as f32 / self.data.swapchain_extent.height as f32,
                0.1,
                100.0,
            );

        let ubo = UniformBufferObject { model, view, proj };

        let memory = self.device.map_memory(
            self.data.uniform_buffers_memory[image_index],
            0,
            size_of::<UniformBufferObject>() as u64,
            vk::MemoryMapFlags::empty(),
        )?;

        memcpy(&ubo, memory.cast(), 1);

        self.device
            .unmap_memory(self.data.uniform_buffers_memory[image_index]);

        Ok(())
    }
}

/// The Vulkan handles and associated properties used by our Vulkan app.
#[derive(Clone, Debug, Default)]
pub struct AppData {
    // Debug
    messenger: vk::DebugUtilsMessengerEXT,
    // Surface
    surface: vk::SurfaceKHR,
    // Physical Device / Logical Device
    physical_device: vk::PhysicalDevice,
    graphics_queue: vk::Queue,
    present_queue: vk::Queue,
    // Swapchain
    swapchain_format: vk::Format,
    swapchain_extent: vk::Extent2D,
    swapchain: vk::SwapchainKHR,
    swapchain_images: Vec<vk::Image>,
    swapchain_images_views: Vec<vk::ImageView>,
    // Pipeline
    render_pass: vk::RenderPass,
    descriptor_set_layout: vk::DescriptorSetLayout,
    pipeline_layout: vk::PipelineLayout,
    pipeline: vk::Pipeline,
    // Framebuffers
    framebuffers: Vec<vk::Framebuffer>,
    // Command Pool
    command_pool: vk::CommandPool,
    // Command Buffers
    command_buffers: Vec<vk::CommandBuffer>,
    // Semaphores for each frame in flight.
    image_available_semaphores: Vec<vk::Semaphore>,
    render_finished_semaphores: Vec<vk::Semaphore>,
    in_flight_fences: Vec<vk::Fence>,
    images_in_flight: Vec<vk::Fence>,
    // Vertex Buffer
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    vertex_buffer: vk::Buffer,
    vertex_buffer_memory: vk::DeviceMemory,
    index_buffer: vk::Buffer,
    index_buffer_memory: vk::DeviceMemory,
    uniform_buffers: Vec<vk::Buffer>,
    uniform_buffers_memory: Vec<vk::DeviceMemory>,
    // Descriptor
    descriptor_pool: vk::DescriptorPool,
    descriptor_sets: Vec<vk::DescriptorSet>,
    // Textures
    mip_levels: u32,
    texture_image: vk::Image,
    texture_image_memory: vk::DeviceMemory,
    texture_image_view: vk::ImageView,
    texture_sampler: vk::Sampler,
    // Depth image
    depth_image: vk::Image,
    depth_image_memory: vk::DeviceMemory,
    depth_image_view: vk::ImageView,
    // Rasterization parameters
    wireframe: bool,
}

/// Creates a Vulkan instance.
unsafe fn create_instance(window: &Window, entry: &Entry, data: &mut AppData) -> Result<Instance> {
    let app_info = vk::ApplicationInfo::builder()
        .application_name(b"scop\0")
        .application_version(vk::make_version(1, 0, 0))
        .engine_name(b"No Engine\0")
        .engine_version(vk::make_version(1, 0, 0))
        .api_version(vk::make_version(1, 0, 0));

    let mut extensions = vk_window::get_required_instance_extensions(window)
        .iter()
        .map(|ext| ext.as_ptr())
        .collect::<Vec<_>>();

    if VALIDATION_ENABLED {
        extensions.push(vk::EXT_DEBUG_UTILS_EXTENSION.name.as_ptr());
    }

    let flags = if cfg!(target_os = "macos") && entry.version()? >= PORTABILITY_MACOS_VERSION {
        info!("Enabling extensions for macOS portability.");
        extensions.push(
            vk::KHR_GET_PHYSICAL_DEVICE_PROPERTIES2_EXTENSION
                .name
                .as_ptr(),
        );
        extensions.push(vk::KHR_PORTABILITY_ENUMERATION_EXTENSION.name.as_ptr());
        vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR
    } else {
        vk::InstanceCreateFlags::empty()
    };

    let available_layers = entry
        .enumerate_instance_layer_properties()?
        .iter()
        .map(|layer| layer.layer_name)
        .collect::<HashSet<_>>();

    if VALIDATION_ENABLED && !available_layers.contains(&VALIDATION_LAYER) {
        return Err(anyhow!("Validation layer requested but not supported."));
    }

    let enabled_layer_names = if VALIDATION_ENABLED {
        vec![VALIDATION_LAYER.as_ptr()]
    } else {
        Vec::new()
    };

    let mut instance_info = vk::InstanceCreateInfo::builder()
        .application_info(&app_info)
        .enabled_extension_names(&extensions)
        .flags(flags)
        .enabled_layer_names(&enabled_layer_names);

    let mut debug_info = vk::DebugUtilsMessengerCreateInfoEXT::builder()
        .message_severity(vk::DebugUtilsMessageSeverityFlagsEXT::all())
        .message_type(
            vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
                | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
        ) // Can't use all() because it might include additional extensions (EXT_DEVICE_ADDRESS_BINDING_REPORT_EXTENSION)
        .user_callback(Some(debug_callback));

    if VALIDATION_ENABLED {
        instance_info = instance_info.push_next(&mut debug_info);
    }

    let instance = entry.create_instance(&instance_info, None)?;

    if VALIDATION_ENABLED {
        data.messenger = instance.create_debug_utils_messenger_ext(&debug_info, None)?;
    }

    Ok(instance)
}

extern "system" fn debug_callback(
    severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    type_: vk::DebugUtilsMessageTypeFlagsEXT,
    data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _: *mut c_void,
) -> vk::Bool32 {
    let data = unsafe { *data };
    let message = unsafe { CStr::from_ptr(data.message) }.to_string_lossy();

    if severity >= vk::DebugUtilsMessageSeverityFlagsEXT::ERROR {
        error!("({:?}) {}", type_, message);
    } else if severity >= vk::DebugUtilsMessageSeverityFlagsEXT::WARNING {
        warn!("({:?}) {}", type_, message);
    } else if severity >= vk::DebugUtilsMessageSeverityFlagsEXT::INFO {
        debug!("({:?}) {}", type_, message);
    } else {
        trace!("({:?}) {}", type_, message);
    }

    vk::FALSE
}
