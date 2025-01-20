use std::ffi::{CStr, CString};

use ash::{
    khr::surface,
    vk::{self, PhysicalDeviceType, QueueFlags, SurfaceKHR, KHR_SWAPCHAIN_NAME},
    Entry, Instance,
};
use winit::{
    raw_window_handle::{HasDisplayHandle, HasWindowHandle},
    window::Window,
};

const ENABLE_VALIDATION_LAYERS: bool = true;
const VALIDATION_LAYERS: [&str; 1] = ["VK_LAYER_KHRONOS_validation"];

pub struct GraphicsSystem {
    instance: Instance,
}

impl GraphicsSystem {
    pub fn new(window: &Window) -> Self {
        let entry = unsafe { Entry::load().expect("Failed to load default Vulkan library") };
        let instance = Self::create_instance(&entry, &window);

        let surface_instance = surface::Instance::new(&entry, &instance);
        let surface_khr = unsafe {
            ash_window::create_surface(
                &entry,
                &instance,
                window.display_handle().unwrap().as_raw(),
                window.window_handle().unwrap().as_raw(),
                None,
            )
            .expect("Failed to create Surface")
        };

        // let debug_report_callback = setup_debug_messenger(&entry, &instance);

        let (physical_device, queue_families_indices) = Self::best_physical_device(
            &instance,
            &surface_instance,
            surface_khr,
            Vec::from([KHR_SWAPCHAIN_NAME]),
            QueueFlags::GRAPHICS,
        );

        // let (device, graphics_queue, present_queue) =
        //     Self::create_device(
        //         &instance,
        //         physical_device,
        //         queue_families_indices,
        //     );

        Self { instance }
    }

    fn check_validation_layer_support(entry: &Entry) -> bool {
        let layer_properties = unsafe { entry.enumerate_instance_layer_properties() }
            .expect("Failed to enumerate instance layer properties");

        'validation_loop: for validation_layer in VALIDATION_LAYERS {
            for layer in layer_properties.iter() {
                let layer_name = unsafe { CStr::from_ptr(layer.layer_name.as_ptr() as *const i8) };

                if layer_name.to_str().unwrap() == validation_layer {
                    // let layer_description =
                    //     unsafe { CStr::from_ptr(layer.description.as_ptr() as *const i8) };
                    // println!("{:?}: {:?}", layer_name, layer_description);
                    continue 'validation_loop;
                }
            }
            println!("{:?} not found in layer properties", validation_layer);
            return false;
        }

        true
    }

    fn create_instance(entry: &Entry, window: &Window) -> Instance {
        let application_name = CString::new("Rust_Vulkan").unwrap();
        let engine_name = CString::new("Rust_Vulkan_Engine").unwrap();
        let app_info = vk::ApplicationInfo::default()
            .application_name(application_name.as_c_str())
            .application_version(vk::make_api_version(0, 0, 1, 0))
            .engine_name(engine_name.as_c_str())
            .engine_version(vk::make_api_version(0, 0, 1, 0))
            .api_version(vk::make_api_version(0, 1, 4, 0));

        let required_extensions =
            ash_window::enumerate_required_extensions(window.display_handle().unwrap().as_raw())
                .expect("Failed to enumerade required extensions");

        let enabled_validation_layers = VALIDATION_LAYERS
            .into_iter()
            .map(|s| s.as_bytes().as_ptr() as *const i8)
            .collect::<Vec<_>>();

        let create_info = vk::InstanceCreateInfo::default()
            .application_info(&app_info)
            .enabled_extension_names(required_extensions);

        if ENABLE_VALIDATION_LAYERS && Self::check_validation_layer_support(entry) {
            let _ = create_info.enabled_layer_names(enabled_validation_layers.as_slice());
        }

        unsafe {
            entry
                .create_instance(&create_info, None)
                .expect("Failed to create Vulkan instance")
        }
    }

    fn best_physical_device(
        instance: &Instance,
        surface: &surface::Instance,
        surface_hkr: SurfaceKHR,
        extensions: Vec<&CStr>,
        queue_flags: QueueFlags,
    ) -> (vk::PhysicalDevice, u32) {
        let physical_devices =
            unsafe { instance.enumerate_physical_devices() }.expect("No physical devices found");

        physical_devices
            .into_iter()
            .filter(|physical_device| {
                let extension_properties =
                    unsafe { instance.enumerate_device_extension_properties(*physical_device) }
                        .expect("Couldn't enumerate physical device's extension properties");

                for &extension in extensions.iter() {
                    if extension_properties
                        .iter()
                        .find(|device_extension| {
                            let extension_name =
                                unsafe { CStr::from_ptr(device_extension.extension_name.as_ptr()) };

                            if extension_name == extension {
                                return true;
                            }
                            false
                        })
                        .is_none()
                    {
                        return false;
                    }
                }

                true
            })
            .filter_map(|physical_device| {
                let queue_family_properties = unsafe {
                    instance.get_physical_device_queue_family_properties(physical_device)
                };

                queue_family_properties
                    .iter()
                    .enumerate()
                    .position(|(index, queue)| {
                        let has_queue_flags = queue.queue_flags.contains(queue_flags);

                        let supports_surface = unsafe {
                            surface.get_physical_device_surface_support(
                                physical_device,
                                index as u32,
                                surface_hkr,
                            )
                        }
                        .expect("Failed to check for surface support on physical device");

                        has_queue_flags && supports_surface
                    })
                    .map(|queue| (physical_device, queue as u32))
            })
            .min_by_key(|(physical_device, _)| {
                let device_properties =
                    unsafe { instance.get_physical_device_properties(*physical_device) };

                match device_properties.device_type {
                    PhysicalDeviceType::DISCRETE_GPU => 0,
                    PhysicalDeviceType::INTEGRATED_GPU => 1,
                    PhysicalDeviceType::VIRTUAL_GPU => 2,
                    PhysicalDeviceType::CPU => 3,
                    _ => 4,
                }
            })
            .expect("No suitable device found")
    }

    // fn create_device(instance: &Instance, physical_device: vk::PhysicalDevice) {
    //     // vk::DeviceCreateInfo::default().
    //     let device =
    //         unsafe { instance.create_device(physical_device, create_info, allocation_callbacks) };
    // }
}
