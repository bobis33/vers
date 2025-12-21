use ash::vk;
use thiserror::Error;

use super::device::VulkanDevice;
use super::swapchain::VulkanSwapchain;

#[derive(Debug, Error)]
pub enum RenderPassError {
    #[error("Vulkan error: {0}")]
    Vulkan(#[from] vk::Result),
}

pub struct VulkanRenderPass {
    pub render_pass: vk::RenderPass,
    device:          ash::Device,
}

impl VulkanRenderPass {
    pub fn new(
        device:    &VulkanDevice,
        swapchain: &VulkanSwapchain,
    ) -> Result<Self, RenderPassError> {
        // --- Attachment description -----------------------------------------
        // One attachment: the swapchain color image.
        // - loadOp  = CLEAR  : clear to our color at the start of the pass
        // - storeOp = STORE  : keep the result so it can be presented
        // - initial = UNDEFINED : we don't care about previous contents
        // - final   = PRESENT_SRC_KHR : image must be in this layout to present
        let color_attachment = vk::AttachmentDescription::default()
            .format(swapchain.config.format.format)
            .samples(vk::SampleCountFlags::TYPE_1)
            .load_op(vk::AttachmentLoadOp::CLEAR)
            .store_op(vk::AttachmentStoreOp::STORE)
            .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
            .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
            .initial_layout(vk::ImageLayout::UNDEFINED)
            .final_layout(vk::ImageLayout::PRESENT_SRC_KHR);

        // --- Subpass --------------------------------------------------------
        // One subpass that writes to attachment 0 as a color target.
        // layout = COLOR_ATTACHMENT_OPTIMAL : optimal for writing color
        let color_attachment_ref = vk::AttachmentReference::default()
            .attachment(0)
            .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL);

        let color_refs = [color_attachment_ref];

        let subpass = vk::SubpassDescription::default()
            .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
            .color_attachments(&color_refs);

        // --- Subpass dependency ---------------------------------------------
        // Ensures the swapchain image has finished being read for presentation
        // before we start writing to it again in the next frame.
        //
        // EXTERNAL → subpass 0
        //   wait on: COLOR_ATTACHMENT_OUTPUT stage (presentation is done)
        //   before:  COLOR_ATTACHMENT_OUTPUT stage (we write color)
        let dependency = vk::SubpassDependency::default()
            .src_subpass(vk::SUBPASS_EXTERNAL)
            .dst_subpass(0)
            .src_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .src_access_mask(vk::AccessFlags::empty())
            .dst_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .dst_access_mask(vk::AccessFlags::COLOR_ATTACHMENT_WRITE);

        let attachments  = [color_attachment];
        let subpasses    = [subpass];
        let dependencies = [dependency];

        let create_info = vk::RenderPassCreateInfo::default()
            .attachments(&attachments)
            .subpasses(&subpasses)
            .dependencies(&dependencies);

        let render_pass = unsafe {
            device.device.create_render_pass(&create_info, None)?
        };

        Ok(Self {
            render_pass,
            device: device.device.clone(),
        })
    }
}

impl Drop for VulkanRenderPass {
    fn drop(&mut self) {
        unsafe { self.device.destroy_render_pass(self.render_pass, None) };
    }
}