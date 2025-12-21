use ash::vk;
use thiserror::Error;

use super::device::VulkanDevice;
use super::render_pass::VulkanRenderPass;
use super::swapchain::VulkanSwapchain;

#[derive(Debug, Error)]
pub enum FramebufferError {
    #[error("Vulkan error: {0}")]
    Vulkan(#[from] vk::Result),
}

pub struct VulkanFramebuffers {
    pub framebuffers: Vec<vk::Framebuffer>,
    device:           ash::Device,
}

impl VulkanFramebuffers {
    pub fn new(
        device:      &VulkanDevice,
        render_pass: &VulkanRenderPass,
        swapchain:   &VulkanSwapchain,
    ) -> Result<Self, FramebufferError> {
        let extent = swapchain.config.extent;

        let framebuffers = swapchain
            .image_views
            .iter()
            .map(|&view| {
                // One framebuffer per swapchain image view.
                // The attachment order must match the render pass attachments.
                let attachments = [view];

                let create_info = vk::FramebufferCreateInfo::default()
                    .render_pass(render_pass.render_pass)
                    .attachments(&attachments)
                    .width(extent.width)
                    .height(extent.height)
                    .layers(1);

                unsafe { device.device.create_framebuffer(&create_info, None) }
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            framebuffers,
            device: device.device.clone(),
        })
    }

    /// Recreate all framebuffers after a swapchain resize.
    pub fn recreate(
        &mut self,
        device:      &VulkanDevice,
        render_pass: &VulkanRenderPass,
        swapchain:   &VulkanSwapchain,
    ) -> Result<(), FramebufferError> {
        self.destroy_framebuffers();
        let new = Self::new(device, render_pass, swapchain)?;
        self.framebuffers = new.framebuffers.clone();
        std::mem::forget(new); // device clone is already in self
        Ok(())
    }

    fn destroy_framebuffers(&self) {
        for &fb in &self.framebuffers {
            unsafe { self.device.destroy_framebuffer(fb, None) };
        }
    }
}

impl Drop for VulkanFramebuffers {
    fn drop(&mut self) {
        self.destroy_framebuffers();
    }
}