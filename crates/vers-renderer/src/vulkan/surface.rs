use ash::{khr, vk};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use thiserror::Error;

use super::instance::VulkanInstance;
use super::entry::VulkanEntry;

#[derive(Debug, Error)]
pub enum SurfaceError {
    #[error("Vulkan error: {0}")]
    Vulkan(#[from] vk::Result),
}

pub struct VulkanSurface {
    pub(crate) loader: khr::surface::Instance,
    pub surface: vk::SurfaceKHR,
}

impl VulkanSurface {
    pub fn new(
        entry: &VulkanEntry,
        instance: &VulkanInstance,
        window: &dyn HasWindowHandle,
        display: &dyn HasDisplayHandle,
    ) -> Result<Self, SurfaceError> {
        let loader = khr::surface::Instance::new(
            &entry.entry,
            &instance.instance,
        );

        let surface = unsafe {
            ash_window::create_surface(
                &entry.entry,
                &instance.instance,
                display.display_handle().unwrap().as_raw(),
                window.window_handle().unwrap().as_raw(),
                None,
            )?
        };

        Ok(Self { loader, surface })
    }
}

impl Drop for VulkanSurface {
    fn drop(&mut self) {
        unsafe { self.loader.destroy_surface(self.surface, None) };
    }
}