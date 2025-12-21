use ash::{vk, Instance};
use raw_window_handle::HasDisplayHandle;
use std::ffi::CStr;
use thiserror::Error;

use super::entry::VulkanEntry;

#[derive(Debug, Error)]
pub enum InstanceError {
    #[error("Vulkan error: {0}")]
    Vulkan(#[from] vk::Result),
    #[error("Required extension not available: {0}")]
    MissingExtension(String),
}

pub struct VulkanInstance {
    pub(crate) instance: Instance,
}

impl VulkanInstance {
    pub fn new(
        entry: &VulkanEntry,
        display_handle: &dyn HasDisplayHandle,
    ) -> Result<Self, InstanceError> {
        // Extensions requises par la surface (Wayland, XCB, Win32...)
        let surface_extensions = ash_window::enumerate_required_extensions(
            display_handle.display_handle().unwrap().as_raw(),
        )?;

        let app_info = vk::ApplicationInfo::default()
            .api_version(vk::API_VERSION_1_3);

        let create_info = vk::InstanceCreateInfo::default()
            .application_info(&app_info)
            .enabled_extension_names(surface_extensions);

        let instance = unsafe {
            entry.entry.create_instance(&create_info, None)?
        };

        Ok(Self { instance })
    }
}

impl Drop for VulkanInstance {
    fn drop(&mut self) {
        unsafe { self.instance.destroy_instance(None) };
    }
}