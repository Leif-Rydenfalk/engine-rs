mod common;

use common::*;
use imgui::*;
use material_icons::*;
use simple_logger::SimpleLogger;
use std::error::Error;

const APP_NAME: &str = "creating windows";

fn main() -> Result<(), Box<dyn Error>> {
    SimpleLogger::new().init()?;

    let mut demo_window_open = true;
    System::new(APP_NAME)?.run((), move |_, ui, _| {
        if demo_window_open {
            ui.show_demo_window(&mut demo_window_open);
        }

        ui.window("Debug").always_auto_resize(true).build(|| {
            ui.text(format!("fps: {}", ui.io().framerate));
            ui.text(format!("dt (ms): {}", 1000.0 / ui.io().framerate));
            ui.separator();
            ui.checkbox("Show Imgui Demo Window", &mut demo_window_open);
        });

        ui.window("Hello World")
            .size([300.0, 400.0], Condition::FirstUseEver)
            .build(|| {
                ui.text("Hello World!");
                ui.text_wrapped("こんにちは世界！");
                ui.text(format!("{}", icon_to_char(Icon::DirectionsBike)))
            });
    })?;

    Ok(())
}

// #![warn(
//     clippy::use_self,
//     deprecated_in_future,
//     rust_2018_idioms,
//     trivial_casts,
//     trivial_numeric_casts,
//     unused_qualifications
// )]

// use std::{
//     borrow::Cow, cell::RefCell, default::Default, error::Error, ffi, ops::Drop, os::raw::c_char,
// };

// use ash::{
//     ext::debug_utils,
//     khr::{surface, swapchain},
//     vk, Device, Entry, Instance,
// };
// use winit::{
//     event::{ElementState, Event, KeyEvent, WindowEvent},
//     event_loop::{ControlFlow, EventLoop},
//     keyboard::{Key, NamedKey},
//     platform::run_on_demand::EventLoopExtRunOnDemand,
//     raw_window_handle::{HasDisplayHandle, HasWindowHandle},
//     window::WindowBuilder,
// };

// // Simple offset_of macro akin to C++ offsetof
// #[macro_export]
// macro_rules! offset_of {
//     ($base:path, $field:ident) => {{
//         #[allow(unused_unsafe)]
//         unsafe {
//             let b: $base = mem::zeroed();
//             std::ptr::addr_of!(b.$field) as isize - std::ptr::addr_of!(b) as isize
//         }
//     }};
// }
// /// Helper function for submitting command buffers. Immediately waits for the fence before the command buffer
// /// is executed. That way we can delay the waiting for the fences by 1 frame which is good for performance.
// /// Make sure to create the fence in a signaled state on the first use.
// #[allow(clippy::too_many_arguments)]
// pub fn record_submit_commandbuffer<F: FnOnce(&Device, vk::CommandBuffer)>(
//     device: &Device,
//     command_buffer: vk::CommandBuffer,
//     command_buffer_reuse_fence: vk::Fence,
//     submit_queue: vk::Queue,
//     wait_mask: &[vk::PipelineStageFlags],
//     wait_semaphores: &[vk::Semaphore],
//     signal_semaphores: &[vk::Semaphore],
//     f: F,
// ) {
//     unsafe {
//         device
//             .wait_for_fences(&[command_buffer_reuse_fence], true, u64::MAX)
//             .expect("Wait for fence failed.");

//         device
//             .reset_fences(&[command_buffer_reuse_fence])
//             .expect("Reset fences failed.");

//         device
//             .reset_command_buffer(
//                 command_buffer,
//                 vk::CommandBufferResetFlags::RELEASE_RESOURCES,
//             )
//             .expect("Reset command buffer failed.");

//         let command_buffer_begin_info = vk::CommandBufferBeginInfo::default()
//             .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);

//         device
//             .begin_command_buffer(command_buffer, &command_buffer_begin_info)
//             .expect("Begin commandbuffer");
//         f(device, command_buffer);
//         device
//             .end_command_buffer(command_buffer)
//             .expect("End commandbuffer");

//         let command_buffers = vec![command_buffer];

//         let submit_info = vk::SubmitInfo::default()
//             .wait_semaphores(wait_semaphores)
//             .wait_dst_stage_mask(wait_mask)
//             .command_buffers(&command_buffers)
//             .signal_semaphores(signal_semaphores);

//         device
//             .queue_submit(submit_queue, &[submit_info], command_buffer_reuse_fence)
//             .expect("queue submit failed.");
//     }
// }

// unsafe extern "system" fn vulkan_debug_callback(
//     message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
//     message_type: vk::DebugUtilsMessageTypeFlagsEXT,
//     p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT<'_>,
//     _user_data: *mut std::os::raw::c_void,
// ) -> vk::Bool32 {
//     let callback_data = *p_callback_data;
//     let message_id_number = callback_data.message_id_number;

//     let message_id_name = if callback_data.p_message_id_name.is_null() {
//         Cow::from("")
//     } else {
//         ffi::CStr::from_ptr(callback_data.p_message_id_name).to_string_lossy()
//     };

//     let message = if callback_data.p_message.is_null() {
//         Cow::from("")
//     } else {
//         ffi::CStr::from_ptr(callback_data.p_message).to_string_lossy()
//     };

//     println!(
//         "{message_severity:?}:\n{message_type:?} [{message_id_name} ({message_id_number})] : {message}\n",
//     );

//     vk::FALSE
// }

// pub fn find_memorytype_index(
//     memory_req: &vk::MemoryRequirements,
//     memory_prop: &vk::PhysicalDeviceMemoryProperties,
//     flags: vk::MemoryPropertyFlags,
// ) -> Option<u32> {
//     memory_prop.memory_types[..memory_prop.memory_type_count as _]
//         .iter()
//         .enumerate()
//         .find(|(index, memory_type)| {
//             (1 << index) & memory_req.memory_type_bits != 0
//                 && memory_type.property_flags & flags == flags
//         })
//         .map(|(index, _memory_type)| index as _)
// }

// pub struct ExampleBase {
//     pub entry: Entry,
//     pub instance: Instance,
//     pub device: Device,
//     pub surface_loader: surface::Instance,
//     pub swapchain_loader: swapchain::Device,
//     pub debug_utils_loader: debug_utils::Instance,
//     pub window: winit::window::Window,
//     pub event_loop: RefCell<EventLoop<()>>,
//     pub debug_call_back: vk::DebugUtilsMessengerEXT,

//     pub pdevice: vk::PhysicalDevice,
//     pub device_memory_properties: vk::PhysicalDeviceMemoryProperties,
//     pub queue_family_index: u32,
//     pub present_queue: vk::Queue,

//     pub surface: vk::SurfaceKHR,
//     pub surface_format: vk::SurfaceFormatKHR,
//     pub surface_resolution: vk::Extent2D,

//     pub swapchain: vk::SwapchainKHR,
//     pub present_images: Vec<vk::Image>,
//     pub present_image_views: Vec<vk::ImageView>,

//     pub pool: vk::CommandPool,
//     pub draw_command_buffer: vk::CommandBuffer,
//     pub setup_command_buffer: vk::CommandBuffer,

//     pub depth_image: vk::Image,
//     pub depth_image_view: vk::ImageView,
//     pub depth_image_memory: vk::DeviceMemory,

//     pub present_complete_semaphore: vk::Semaphore,
//     pub rendering_complete_semaphore: vk::Semaphore,

//     pub draw_commands_reuse_fence: vk::Fence,
//     pub setup_commands_reuse_fence: vk::Fence,
// }

// impl ExampleBase {
//     pub fn render_loop<F: Fn()>(&self, f: F) -> Result<(), impl Error> {
//         self.event_loop.borrow_mut().run_on_demand(|event, elwp| {
//             elwp.set_control_flow(ControlFlow::Poll);
//             match event {
//                 Event::WindowEvent {
//                     event:
//                         WindowEvent::CloseRequested
//                         | WindowEvent::KeyboardInput {
//                             event:
//                                 KeyEvent {
//                                     state: ElementState::Pressed,
//                                     logical_key: Key::Named(NamedKey::Escape),
//                                     ..
//                                 },
//                             ..
//                         },
//                     ..
//                 } => {
//                     elwp.exit();
//                 }
//                 Event::AboutToWait => f(),
//                 _ => (),
//             }
//         })
//     }

//     pub fn new(window_width: u32, window_height: u32) -> Result<Self, Box<dyn Error>> {
//         unsafe {
//             let event_loop = EventLoop::new()?;
//             let window = WindowBuilder::new()
//                 .with_title("Ash - Example")
//                 .with_inner_size(winit::dpi::LogicalSize::new(
//                     f64::from(window_width),
//                     f64::from(window_height),
//                 ))
//                 .build(&event_loop)
//                 .unwrap();
//             let entry = Entry::linked();
//             let app_name = c"VulkanTriangle";

//             // let layer_names = [c"VK_LAYER_KHRONOS_validation"];
//             // let layers_names_raw: Vec<*const c_char> = layer_names
//             //     .iter()
//             //     .map(|raw_name| raw_name.as_ptr())
//             //     .collect();

//             let mut extension_names =
//                 ash_window::enumerate_required_extensions(window.display_handle()?.as_raw())
//                     .unwrap()
//                     .to_vec();
//             extension_names.push(debug_utils::NAME.as_ptr());

//             #[cfg(any(target_os = "macos", target_os = "ios"))]
//             {
//                 extension_names.push(ash::khr::portability_enumeration::NAME.as_ptr());
//                 // Enabling this extension is a requirement when using `VK_KHR_portability_subset`
//                 extension_names.push(ash::khr::get_physical_device_properties2::NAME.as_ptr());
//             }

//             let appinfo = vk::ApplicationInfo::default()
//                 .application_name(app_name)
//                 .application_version(0)
//                 .engine_name(app_name)
//                 .engine_version(0)
//                 .api_version(vk::make_api_version(0, 1, 0, 0));

//             let create_flags = if cfg!(any(target_os = "macos", target_os = "ios")) {
//                 vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR
//             } else {
//                 vk::InstanceCreateFlags::default()
//             };

//             let create_info = vk::InstanceCreateInfo::default()
//                 .application_info(&appinfo)
//                 // .enabled_layer_names(&layers_names_raw)
//                 .enabled_extension_names(&extension_names)
//                 .flags(create_flags);

//             let instance: Instance = entry
//                 .create_instance(&create_info, None)
//                 .expect("Instance creation error");

//             let debug_info = vk::DebugUtilsMessengerCreateInfoEXT::default()
//                 .message_severity(
//                     vk::DebugUtilsMessageSeverityFlagsEXT::ERROR
//                         | vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
//                         | vk::DebugUtilsMessageSeverityFlagsEXT::INFO,
//                 )
//                 .message_type(
//                     vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
//                         | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
//                         | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
//                 )
//                 .pfn_user_callback(Some(vulkan_debug_callback));

//             let debug_utils_loader = debug_utils::Instance::new(&entry, &instance);
//             let debug_call_back = debug_utils_loader
//                 .create_debug_utils_messenger(&debug_info, None)
//                 .unwrap();
//             let surface = ash_window::create_surface(
//                 &entry,
//                 &instance,
//                 window.display_handle()?.as_raw(),
//                 window.window_handle()?.as_raw(),
//                 None,
//             )
//             .unwrap();
//             let pdevices = instance
//                 .enumerate_physical_devices()
//                 .expect("Physical device error");
//             let surface_loader = surface::Instance::new(&entry, &instance);
//             let (pdevice, queue_family_index) = pdevices
//                 .iter()
//                 .find_map(|pdevice| {
//                     instance
//                         .get_physical_device_queue_family_properties(*pdevice)
//                         .iter()
//                         .enumerate()
//                         .find_map(|(index, info)| {
//                             let supports_graphic_and_surface =
//                                 info.queue_flags.contains(vk::QueueFlags::GRAPHICS)
//                                     && surface_loader
//                                         .get_physical_device_surface_support(
//                                             *pdevice,
//                                             index as u32,
//                                             surface,
//                                         )
//                                         .unwrap();
//                             if supports_graphic_and_surface {
//                                 Some((*pdevice, index))
//                             } else {
//                                 None
//                             }
//                         })
//                 })
//                 .expect("Couldn't find suitable device.");
//             let queue_family_index = queue_family_index as u32;
//             let device_extension_names_raw = [
//                 swapchain::NAME.as_ptr(),
//                 #[cfg(any(target_os = "macos", target_os = "ios"))]
//                 ash::khr::portability_subset::NAME.as_ptr(),
//             ];
//             let features = vk::PhysicalDeviceFeatures {
//                 shader_clip_distance: 1,
//                 ..Default::default()
//             };
//             let priorities = [1.0];

//             let queue_info = vk::DeviceQueueCreateInfo::default()
//                 .queue_family_index(queue_family_index)
//                 .queue_priorities(&priorities);

//             let device_create_info = vk::DeviceCreateInfo::default()
//                 .queue_create_infos(std::slice::from_ref(&queue_info))
//                 .enabled_extension_names(&device_extension_names_raw)
//                 .enabled_features(&features);

//             let device: Device = instance
//                 .create_device(pdevice, &device_create_info, None)
//                 .unwrap();

//             let present_queue = device.get_device_queue(queue_family_index, 0);

//             let surface_format = surface_loader
//                 .get_physical_device_surface_formats(pdevice, surface)
//                 .unwrap()[0];

//             let surface_capabilities = surface_loader
//                 .get_physical_device_surface_capabilities(pdevice, surface)
//                 .unwrap();
//             let mut desired_image_count = surface_capabilities.min_image_count + 1;
//             if surface_capabilities.max_image_count > 0
//                 && desired_image_count > surface_capabilities.max_image_count
//             {
//                 desired_image_count = surface_capabilities.max_image_count;
//             }
//             let surface_resolution = match surface_capabilities.current_extent.width {
//                 u32::MAX => vk::Extent2D {
//                     width: window_width,
//                     height: window_height,
//                 },
//                 _ => surface_capabilities.current_extent,
//             };
//             let pre_transform = if surface_capabilities
//                 .supported_transforms
//                 .contains(vk::SurfaceTransformFlagsKHR::IDENTITY)
//             {
//                 vk::SurfaceTransformFlagsKHR::IDENTITY
//             } else {
//                 surface_capabilities.current_transform
//             };
//             let present_modes = surface_loader
//                 .get_physical_device_surface_present_modes(pdevice, surface)
//                 .unwrap();
//             let present_mode = present_modes
//                 .iter()
//                 .cloned()
//                 .find(|&mode| mode == vk::PresentModeKHR::MAILBOX)
//                 .unwrap_or(vk::PresentModeKHR::FIFO);
//             let swapchain_loader = swapchain::Device::new(&instance, &device);

//             let swapchain_create_info = vk::SwapchainCreateInfoKHR::default()
//                 .surface(surface)
//                 .min_image_count(desired_image_count)
//                 .image_color_space(surface_format.color_space)
//                 .image_format(surface_format.format)
//                 .image_extent(surface_resolution)
//                 .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
//                 .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
//                 .pre_transform(pre_transform)
//                 .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
//                 .present_mode(present_mode)
//                 .clipped(true)
//                 .image_array_layers(1);

//             let swapchain = swapchain_loader
//                 .create_swapchain(&swapchain_create_info, None)
//                 .unwrap();

//             let pool_create_info = vk::CommandPoolCreateInfo::default()
//                 .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
//                 .queue_family_index(queue_family_index);

//             let pool = device.create_command_pool(&pool_create_info, None).unwrap();

//             let command_buffer_allocate_info = vk::CommandBufferAllocateInfo::default()
//                 .command_buffer_count(2)
//                 .command_pool(pool)
//                 .level(vk::CommandBufferLevel::PRIMARY);

//             let command_buffers = device
//                 .allocate_command_buffers(&command_buffer_allocate_info)
//                 .unwrap();
//             let setup_command_buffer = command_buffers[0];
//             let draw_command_buffer = command_buffers[1];

//             let present_images = swapchain_loader.get_swapchain_images(swapchain).unwrap();
//             let present_image_views: Vec<vk::ImageView> = present_images
//                 .iter()
//                 .map(|&image| {
//                     let create_view_info = vk::ImageViewCreateInfo::default()
//                         .view_type(vk::ImageViewType::TYPE_2D)
//                         .format(surface_format.format)
//                         .components(vk::ComponentMapping {
//                             r: vk::ComponentSwizzle::R,
//                             g: vk::ComponentSwizzle::G,
//                             b: vk::ComponentSwizzle::B,
//                             a: vk::ComponentSwizzle::A,
//                         })
//                         .subresource_range(vk::ImageSubresourceRange {
//                             aspect_mask: vk::ImageAspectFlags::COLOR,
//                             base_mip_level: 0,
//                             level_count: 1,
//                             base_array_layer: 0,
//                             layer_count: 1,
//                         })
//                         .image(image);
//                     device.create_image_view(&create_view_info, None).unwrap()
//                 })
//                 .collect();
//             let device_memory_properties = instance.get_physical_device_memory_properties(pdevice);
//             let depth_image_create_info = vk::ImageCreateInfo::default()
//                 .image_type(vk::ImageType::TYPE_2D)
//                 .format(vk::Format::D16_UNORM)
//                 .extent(surface_resolution.into())
//                 .mip_levels(1)
//                 .array_layers(1)
//                 .samples(vk::SampleCountFlags::TYPE_1)
//                 .tiling(vk::ImageTiling::OPTIMAL)
//                 .usage(vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT)
//                 .sharing_mode(vk::SharingMode::EXCLUSIVE);

//             let depth_image = device.create_image(&depth_image_create_info, None).unwrap();
//             let depth_image_memory_req = device.get_image_memory_requirements(depth_image);
//             let depth_image_memory_index = find_memorytype_index(
//                 &depth_image_memory_req,
//                 &device_memory_properties,
//                 vk::MemoryPropertyFlags::DEVICE_LOCAL,
//             )
//             .expect("Unable to find suitable memory index for depth image.");

//             let depth_image_allocate_info = vk::MemoryAllocateInfo::default()
//                 .allocation_size(depth_image_memory_req.size)
//                 .memory_type_index(depth_image_memory_index);

//             let depth_image_memory = device
//                 .allocate_memory(&depth_image_allocate_info, None)
//                 .unwrap();

//             device
//                 .bind_image_memory(depth_image, depth_image_memory, 0)
//                 .expect("Unable to bind depth image memory");

//             let fence_create_info =
//                 vk::FenceCreateInfo::default().flags(vk::FenceCreateFlags::SIGNALED);

//             let draw_commands_reuse_fence = device
//                 .create_fence(&fence_create_info, None)
//                 .expect("Create fence failed.");
//             let setup_commands_reuse_fence = device
//                 .create_fence(&fence_create_info, None)
//                 .expect("Create fence failed.");

//             record_submit_commandbuffer(
//                 &device,
//                 setup_command_buffer,
//                 setup_commands_reuse_fence,
//                 present_queue,
//                 &[],
//                 &[],
//                 &[],
//                 |device, setup_command_buffer| {
//                     let layout_transition_barriers = vk::ImageMemoryBarrier::default()
//                         .image(depth_image)
//                         .dst_access_mask(
//                             vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_READ
//                                 | vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE,
//                         )
//                         .new_layout(vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL)
//                         .old_layout(vk::ImageLayout::UNDEFINED)
//                         .subresource_range(
//                             vk::ImageSubresourceRange::default()
//                                 .aspect_mask(vk::ImageAspectFlags::DEPTH)
//                                 .layer_count(1)
//                                 .level_count(1),
//                         );

//                     device.cmd_pipeline_barrier(
//                         setup_command_buffer,
//                         vk::PipelineStageFlags::BOTTOM_OF_PIPE,
//                         vk::PipelineStageFlags::LATE_FRAGMENT_TESTS,
//                         vk::DependencyFlags::empty(),
//                         &[],
//                         &[],
//                         &[layout_transition_barriers],
//                     );
//                 },
//             );

//             let depth_image_view_info = vk::ImageViewCreateInfo::default()
//                 .subresource_range(
//                     vk::ImageSubresourceRange::default()
//                         .aspect_mask(vk::ImageAspectFlags::DEPTH)
//                         .level_count(1)
//                         .layer_count(1),
//                 )
//                 .image(depth_image)
//                 .format(depth_image_create_info.format)
//                 .view_type(vk::ImageViewType::TYPE_2D);

//             let depth_image_view = device
//                 .create_image_view(&depth_image_view_info, None)
//                 .unwrap();

//             let semaphore_create_info = vk::SemaphoreCreateInfo::default();

//             let present_complete_semaphore = device
//                 .create_semaphore(&semaphore_create_info, None)
//                 .unwrap();
//             let rendering_complete_semaphore = device
//                 .create_semaphore(&semaphore_create_info, None)
//                 .unwrap();

//             Ok(Self {
//                 event_loop: RefCell::new(event_loop),
//                 entry,
//                 instance,
//                 device,
//                 queue_family_index,
//                 pdevice,
//                 device_memory_properties,
//                 window,
//                 surface_loader,
//                 surface_format,
//                 present_queue,
//                 surface_resolution,
//                 swapchain_loader,
//                 swapchain,
//                 present_images,
//                 present_image_views,
//                 pool,
//                 draw_command_buffer,
//                 setup_command_buffer,
//                 depth_image,
//                 depth_image_view,
//                 present_complete_semaphore,
//                 rendering_complete_semaphore,
//                 draw_commands_reuse_fence,
//                 setup_commands_reuse_fence,
//                 surface,
//                 debug_call_back,
//                 debug_utils_loader,
//                 depth_image_memory,
//             })
//         }
//     }
// }

// impl Drop for ExampleBase {
//     fn drop(&mut self) {
//         unsafe {
//             self.device.device_wait_idle().unwrap();
//             self.device
//                 .destroy_semaphore(self.present_complete_semaphore, None);
//             self.device
//                 .destroy_semaphore(self.rendering_complete_semaphore, None);
//             self.device
//                 .destroy_fence(self.draw_commands_reuse_fence, None);
//             self.device
//                 .destroy_fence(self.setup_commands_reuse_fence, None);
//             self.device.free_memory(self.depth_image_memory, None);
//             self.device.destroy_image_view(self.depth_image_view, None);
//             self.device.destroy_image(self.depth_image, None);
//             for &image_view in self.present_image_views.iter() {
//                 self.device.destroy_image_view(image_view, None);
//             }
//             self.device.destroy_command_pool(self.pool, None);
//             self.swapchain_loader
//                 .destroy_swapchain(self.swapchain, None);
//             self.device.destroy_device(None);
//             self.surface_loader.destroy_surface(self.surface, None);
//             self.debug_utils_loader
//                 .destroy_debug_utils_messenger(self.debug_call_back, None);
//             self.instance.destroy_instance(None);
//         }
//     }
// }

// // #![warn(unused_qualifications)]

// // use std::default::Default;
// // use std::error::Error;
// use std::io::Cursor;
// use std::mem;
// use std::mem::{align_of, size_of, size_of_val}; // TODO: Remove when bumping MSRV to 1.80

// use ash::util::*;
// // use ash::vk;
// // use ash_examples::*;

// #[derive(Clone, Debug, Copy)]
// struct Vertex {
//     pos: [f32; 4],
//     color: [f32; 4],
// }

// fn main() -> Result<(), Box<dyn Error>> {
//     unsafe {
//         let base = ExampleBase::new(500, 500)?;
//         let renderpass_attachments = [
//             vk::AttachmentDescription {
//                 format: base.surface_format.format,
//                 samples: vk::SampleCountFlags::TYPE_1,
//                 load_op: vk::AttachmentLoadOp::CLEAR,
//                 store_op: vk::AttachmentStoreOp::STORE,
//                 final_layout: vk::ImageLayout::PRESENT_SRC_KHR,
//                 ..Default::default()
//             },
//             vk::AttachmentDescription {
//                 format: vk::Format::D16_UNORM,
//                 samples: vk::SampleCountFlags::TYPE_1,
//                 load_op: vk::AttachmentLoadOp::CLEAR,
//                 initial_layout: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
//                 final_layout: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
//                 ..Default::default()
//             },
//         ];
//         let color_attachment_refs = [vk::AttachmentReference {
//             attachment: 0,
//             layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
//         }];
//         let depth_attachment_ref = vk::AttachmentReference {
//             attachment: 1,
//             layout: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
//         };
//         let dependencies = [vk::SubpassDependency {
//             src_subpass: vk::SUBPASS_EXTERNAL,
//             src_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
//             dst_access_mask: vk::AccessFlags::COLOR_ATTACHMENT_READ
//                 | vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
//             dst_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
//             ..Default::default()
//         }];

//         let subpass = vk::SubpassDescription::default()
//             .color_attachments(&color_attachment_refs)
//             .depth_stencil_attachment(&depth_attachment_ref)
//             .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS);

//         let renderpass_create_info = vk::RenderPassCreateInfo::default()
//             .attachments(&renderpass_attachments)
//             .subpasses(std::slice::from_ref(&subpass))
//             .dependencies(&dependencies);

//         let renderpass = base
//             .device
//             .create_render_pass(&renderpass_create_info, None)
//             .unwrap();

//         let framebuffers: Vec<vk::Framebuffer> = base
//             .present_image_views
//             .iter()
//             .map(|&present_image_view| {
//                 let framebuffer_attachments = [present_image_view, base.depth_image_view];
//                 let frame_buffer_create_info = vk::FramebufferCreateInfo::default()
//                     .render_pass(renderpass)
//                     .attachments(&framebuffer_attachments)
//                     .width(base.surface_resolution.width)
//                     .height(base.surface_resolution.height)
//                     .layers(1);

//                 base.device
//                     .create_framebuffer(&frame_buffer_create_info, None)
//                     .unwrap()
//             })
//             .collect();

//         let index_buffer_data = [0u32, 1, 2];
//         let index_buffer_info = vk::BufferCreateInfo::default()
//             .size(size_of_val(&index_buffer_data) as u64)
//             .usage(vk::BufferUsageFlags::INDEX_BUFFER)
//             .sharing_mode(vk::SharingMode::EXCLUSIVE);

//         let index_buffer = base.device.create_buffer(&index_buffer_info, None).unwrap();
//         let index_buffer_memory_req = base.device.get_buffer_memory_requirements(index_buffer);
//         let index_buffer_memory_index = find_memorytype_index(
//             &index_buffer_memory_req,
//             &base.device_memory_properties,
//             vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
//         )
//         .expect("Unable to find suitable memorytype for the index buffer.");

//         let index_allocate_info = vk::MemoryAllocateInfo {
//             allocation_size: index_buffer_memory_req.size,
//             memory_type_index: index_buffer_memory_index,
//             ..Default::default()
//         };
//         let index_buffer_memory = base
//             .device
//             .allocate_memory(&index_allocate_info, None)
//             .unwrap();
//         let index_ptr = base
//             .device
//             .map_memory(
//                 index_buffer_memory,
//                 0,
//                 index_buffer_memory_req.size,
//                 vk::MemoryMapFlags::empty(),
//             )
//             .unwrap();
//         let mut index_slice = Align::new(
//             index_ptr,
//             align_of::<u32>() as u64,
//             index_buffer_memory_req.size,
//         );
//         index_slice.copy_from_slice(&index_buffer_data);
//         base.device.unmap_memory(index_buffer_memory);
//         base.device
//             .bind_buffer_memory(index_buffer, index_buffer_memory, 0)
//             .unwrap();

//         let vertex_input_buffer_info = vk::BufferCreateInfo {
//             size: 3 * size_of::<Vertex>() as u64,
//             usage: vk::BufferUsageFlags::VERTEX_BUFFER,
//             sharing_mode: vk::SharingMode::EXCLUSIVE,
//             ..Default::default()
//         };

//         let vertex_input_buffer = base
//             .device
//             .create_buffer(&vertex_input_buffer_info, None)
//             .unwrap();

//         let vertex_input_buffer_memory_req = base
//             .device
//             .get_buffer_memory_requirements(vertex_input_buffer);

//         let vertex_input_buffer_memory_index = find_memorytype_index(
//             &vertex_input_buffer_memory_req,
//             &base.device_memory_properties,
//             vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
//         )
//         .expect("Unable to find suitable memorytype for the vertex buffer.");

//         let vertex_buffer_allocate_info = vk::MemoryAllocateInfo {
//             allocation_size: vertex_input_buffer_memory_req.size,
//             memory_type_index: vertex_input_buffer_memory_index,
//             ..Default::default()
//         };

//         let vertex_input_buffer_memory = base
//             .device
//             .allocate_memory(&vertex_buffer_allocate_info, None)
//             .unwrap();

//         let vertices = [
//             Vertex {
//                 pos: [-1.0, 1.0, 0.0, 1.0],
//                 color: [0.0, 1.0, 0.0, 1.0],
//             },
//             Vertex {
//                 pos: [1.0, 1.0, 0.0, 1.0],
//                 color: [0.0, 0.0, 1.0, 1.0],
//             },
//             Vertex {
//                 pos: [0.0, -1.0, 0.0, 1.0],
//                 color: [1.0, 0.0, 0.0, 1.0],
//             },
//         ];

//         let vert_ptr = base
//             .device
//             .map_memory(
//                 vertex_input_buffer_memory,
//                 0,
//                 vertex_input_buffer_memory_req.size,
//                 vk::MemoryMapFlags::empty(),
//             )
//             .unwrap();

//         let mut vert_align = Align::new(
//             vert_ptr,
//             align_of::<Vertex>() as u64,
//             vertex_input_buffer_memory_req.size,
//         );
//         vert_align.copy_from_slice(&vertices);
//         base.device.unmap_memory(vertex_input_buffer_memory);
//         base.device
//             .bind_buffer_memory(vertex_input_buffer, vertex_input_buffer_memory, 0)
//             .unwrap();

//         let mut compiler = spirv_compiler::CompilerBuilder::new()
//             // // Add include dirs
//             // .with_include_dir("my-include-dir")
//             // // Add macros
//             // .with_macro("PI", Some("3.141592"))
//             // Set source language
//             .with_source_language(spirv_compiler::SourceLanguage::GLSL)
//             // Build compiler
//             .build()
//             // If shaderc fails to initialize, this returns None
//             .unwrap();

//         let vertex_code: Vec<u32> = compiler
//             .compile_from_file(
//                 "shaders/triangle.vert",
//                 spirv_compiler::ShaderKind::Vertex,
//                 false, // Set to true if shaders should be cached to filesystem
//             )
//             .unwrap();

//         let frag_code: Vec<u32> = compiler
//             .compile_from_file(
//                 "shaders/triangle.frag",
//                 spirv_compiler::ShaderKind::Fragment,
//                 false, // Set to true if shaders should be cached to filesystem
//             )
//             .unwrap();

//         let vertex_shader_info = vk::ShaderModuleCreateInfo::default().code(&vertex_code);

//         let frag_shader_info = vk::ShaderModuleCreateInfo::default().code(&frag_code);

//         let vertex_shader_module = base
//             .device
//             .create_shader_module(&vertex_shader_info, None)
//             .expect("Vertex shader module error");

//         let fragment_shader_module = base
//             .device
//             .create_shader_module(&frag_shader_info, None)
//             .expect("Fragment shader module error");

//         let layout_create_info = vk::PipelineLayoutCreateInfo::default();

//         let pipeline_layout = base
//             .device
//             .create_pipeline_layout(&layout_create_info, None)
//             .unwrap();

//         let shader_entry_name = c"main";
//         let shader_stage_create_infos = [
//             vk::PipelineShaderStageCreateInfo {
//                 module: vertex_shader_module,
//                 p_name: shader_entry_name.as_ptr(),
//                 stage: vk::ShaderStageFlags::VERTEX,
//                 ..Default::default()
//             },
//             vk::PipelineShaderStageCreateInfo {
//                 s_type: vk::StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO,
//                 module: fragment_shader_module,
//                 p_name: shader_entry_name.as_ptr(),
//                 stage: vk::ShaderStageFlags::FRAGMENT,
//                 ..Default::default()
//             },
//         ];
//         let vertex_input_binding_descriptions = [vk::VertexInputBindingDescription {
//             binding: 0,
//             stride: size_of::<Vertex>() as u32,
//             input_rate: vk::VertexInputRate::VERTEX,
//         }];
//         let vertex_input_attribute_descriptions = [
//             vk::VertexInputAttributeDescription {
//                 location: 0,
//                 binding: 0,
//                 format: vk::Format::R32G32B32A32_SFLOAT,
//                 offset: offset_of!(Vertex, pos) as u32,
//             },
//             vk::VertexInputAttributeDescription {
//                 location: 1,
//                 binding: 0,
//                 format: vk::Format::R32G32B32A32_SFLOAT,
//                 offset: offset_of!(Vertex, color) as u32,
//             },
//         ];

//         let vertex_input_state_info = vk::PipelineVertexInputStateCreateInfo::default()
//             .vertex_attribute_descriptions(&vertex_input_attribute_descriptions)
//             .vertex_binding_descriptions(&vertex_input_binding_descriptions);
//         let vertex_input_assembly_state_info = vk::PipelineInputAssemblyStateCreateInfo {
//             topology: vk::PrimitiveTopology::TRIANGLE_LIST,
//             ..Default::default()
//         };
//         let viewports = [vk::Viewport {
//             x: 0.0,
//             y: 0.0,
//             width: base.surface_resolution.width as f32,
//             height: base.surface_resolution.height as f32,
//             min_depth: 0.0,
//             max_depth: 1.0,
//         }];
//         let scissors = [base.surface_resolution.into()];
//         let viewport_state_info = vk::PipelineViewportStateCreateInfo::default()
//             .scissors(&scissors)
//             .viewports(&viewports);

//         let rasterization_info = vk::PipelineRasterizationStateCreateInfo {
//             front_face: vk::FrontFace::COUNTER_CLOCKWISE,
//             line_width: 1.0,
//             polygon_mode: vk::PolygonMode::FILL,
//             ..Default::default()
//         };
//         let multisample_state_info = vk::PipelineMultisampleStateCreateInfo {
//             rasterization_samples: vk::SampleCountFlags::TYPE_1,
//             ..Default::default()
//         };
//         let noop_stencil_state = vk::StencilOpState {
//             fail_op: vk::StencilOp::KEEP,
//             pass_op: vk::StencilOp::KEEP,
//             depth_fail_op: vk::StencilOp::KEEP,
//             compare_op: vk::CompareOp::ALWAYS,
//             ..Default::default()
//         };
//         let depth_state_info = vk::PipelineDepthStencilStateCreateInfo {
//             depth_test_enable: 1,
//             depth_write_enable: 1,
//             depth_compare_op: vk::CompareOp::LESS_OR_EQUAL,
//             front: noop_stencil_state,
//             back: noop_stencil_state,
//             max_depth_bounds: 1.0,
//             ..Default::default()
//         };
//         let color_blend_attachment_states = [vk::PipelineColorBlendAttachmentState {
//             blend_enable: 0,
//             src_color_blend_factor: vk::BlendFactor::SRC_COLOR,
//             dst_color_blend_factor: vk::BlendFactor::ONE_MINUS_DST_COLOR,
//             color_blend_op: vk::BlendOp::ADD,
//             src_alpha_blend_factor: vk::BlendFactor::ZERO,
//             dst_alpha_blend_factor: vk::BlendFactor::ZERO,
//             alpha_blend_op: vk::BlendOp::ADD,
//             color_write_mask: vk::ColorComponentFlags::RGBA,
//         }];
//         let color_blend_state = vk::PipelineColorBlendStateCreateInfo::default()
//             .logic_op(vk::LogicOp::CLEAR)
//             .attachments(&color_blend_attachment_states);

//         let dynamic_state = [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];
//         let dynamic_state_info =
//             vk::PipelineDynamicStateCreateInfo::default().dynamic_states(&dynamic_state);

//         let graphic_pipeline_info = vk::GraphicsPipelineCreateInfo::default()
//             .stages(&shader_stage_create_infos)
//             .vertex_input_state(&vertex_input_state_info)
//             .input_assembly_state(&vertex_input_assembly_state_info)
//             .viewport_state(&viewport_state_info)
//             .rasterization_state(&rasterization_info)
//             .multisample_state(&multisample_state_info)
//             .depth_stencil_state(&depth_state_info)
//             .color_blend_state(&color_blend_state)
//             .dynamic_state(&dynamic_state_info)
//             .layout(pipeline_layout)
//             .render_pass(renderpass);

//         let graphics_pipelines = base
//             .device
//             .create_graphics_pipelines(vk::PipelineCache::null(), &[graphic_pipeline_info], None)
//             .expect("Unable to create graphics pipeline");

//         let graphic_pipeline = graphics_pipelines[0];

//         let _ = base.render_loop(|| {
//             let (present_index, _) = base
//                 .swapchain_loader
//                 .acquire_next_image(
//                     base.swapchain,
//                     u64::MAX,
//                     base.present_complete_semaphore,
//                     vk::Fence::null(),
//                 )
//                 .unwrap();
//             let clear_values = [
//                 vk::ClearValue {
//                     color: vk::ClearColorValue {
//                         float32: [0.0, 0.0, 0.0, 0.0],
//                     },
//                 },
//                 vk::ClearValue {
//                     depth_stencil: vk::ClearDepthStencilValue {
//                         depth: 1.0,
//                         stencil: 0,
//                     },
//                 },
//             ];

//             let render_pass_begin_info = vk::RenderPassBeginInfo::default()
//                 .render_pass(renderpass)
//                 .framebuffer(framebuffers[present_index as usize])
//                 .render_area(base.surface_resolution.into())
//                 .clear_values(&clear_values);

//             record_submit_commandbuffer(
//                 &base.device,
//                 base.draw_command_buffer,
//                 base.draw_commands_reuse_fence,
//                 base.present_queue,
//                 &[vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT],
//                 &[base.present_complete_semaphore],
//                 &[base.rendering_complete_semaphore],
//                 |device, draw_command_buffer| {
//                     device.cmd_begin_render_pass(
//                         draw_command_buffer,
//                         &render_pass_begin_info,
//                         vk::SubpassContents::INLINE,
//                     );
//                     device.cmd_bind_pipeline(
//                         draw_command_buffer,
//                         vk::PipelineBindPoint::GRAPHICS,
//                         graphic_pipeline,
//                     );
//                     device.cmd_set_viewport(draw_command_buffer, 0, &viewports);
//                     device.cmd_set_scissor(draw_command_buffer, 0, &scissors);
//                     device.cmd_bind_vertex_buffers(
//                         draw_command_buffer,
//                         0,
//                         &[vertex_input_buffer],
//                         &[0],
//                     );
//                     device.cmd_bind_index_buffer(
//                         draw_command_buffer,
//                         index_buffer,
//                         0,
//                         vk::IndexType::UINT32,
//                     );
//                     device.cmd_draw_indexed(
//                         draw_command_buffer,
//                         index_buffer_data.len() as u32,
//                         4,
//                         0,
//                         0,
//                         1,
//                     );
//                     // Or draw without the index buffer
//                     // device.cmd_draw(draw_command_buffer, 3, 1, 0, 0);
//                     device.cmd_end_render_pass(draw_command_buffer);
//                 },
//             );
//             let wait_semaphors = [base.rendering_complete_semaphore];
//             let swapchains = [base.swapchain];
//             let image_indices = [present_index];
//             let present_info = vk::PresentInfoKHR::default()
//                 .wait_semaphores(&wait_semaphors) // &base.rendering_complete_semaphore)
//                 .swapchains(&swapchains)
//                 .image_indices(&image_indices);

//             base.swapchain_loader
//                 .queue_present(base.present_queue, &present_info)
//                 .unwrap();
//         });

//         base.device.device_wait_idle().unwrap();
//         for pipeline in graphics_pipelines {
//             base.device.destroy_pipeline(pipeline, None);
//         }
//         base.device.destroy_pipeline_layout(pipeline_layout, None);
//         base.device
//             .destroy_shader_module(vertex_shader_module, None);
//         base.device
//             .destroy_shader_module(fragment_shader_module, None);
//         base.device.free_memory(index_buffer_memory, None);
//         base.device.destroy_buffer(index_buffer, None);
//         base.device.free_memory(vertex_input_buffer_memory, None);
//         base.device.destroy_buffer(vertex_input_buffer, None);
//         for framebuffer in framebuffers {
//             base.device.destroy_framebuffer(framebuffer, None);
//         }
//         base.device.destroy_render_pass(renderpass, None);
//     }

//     Ok(())
// }
