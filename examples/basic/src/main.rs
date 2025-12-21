use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
    dpi::PhysicalSize,
};
use tracing::{error, info};
use vers_engine::renderer::vulkan::{
    entry::VulkanEntry,
    instance::VulkanInstance,
    surface::VulkanSurface,
    physical_device::VulkanPhysicalDevice,
    device::VulkanDevice,
    swapchain::VulkanSwapchain,
    render_pass::VulkanRenderPass,
    framebuffer::VulkanFramebuffers,
    renderer::VulkanRenderer,
};

// Cornflower blue — a classic clear color
const CLEAR_COLOR: [f32; 4] = [0.392, 0.584, 0.929, 1.0];

struct VulkanContext {
    renderer:        VulkanRenderer,
    framebuffers:    VulkanFramebuffers,
    render_pass:     VulkanRenderPass,
    swapchain:       VulkanSwapchain,
    device:          VulkanDevice,
    physical_device: VulkanPhysicalDevice,
    surface:         VulkanSurface,
    instance:        VulkanInstance,
    entry:           VulkanEntry,
}

impl VulkanContext {
    fn new(window: &Window) -> anyhow::Result<Self> {
        let size = window.inner_size();

        let entry           = VulkanEntry::new()?;
        let instance        = VulkanInstance::new(&entry, window)?;
        let surface         = VulkanSurface::new(&entry, &instance, window, window)?;
        let physical_device = VulkanPhysicalDevice::select(&instance, &surface)?;
        let device          = VulkanDevice::new(&instance, &physical_device)?;
        let swapchain       = VulkanSwapchain::new(
            &instance, &physical_device, &device, &surface,
            (size.width, size.height),
        )?;
        let render_pass  = VulkanRenderPass::new(&device, &swapchain)?;
        let framebuffers = VulkanFramebuffers::new(&device, &render_pass, &swapchain)?;
        let renderer     = VulkanRenderer::new(&device, &physical_device, &swapchain)?;

        info!(gpu = %physical_device.name(), frames_in_flight = swapchain.config.image_count, "Vulkan initialized");

        Ok(Self {
            renderer,
            framebuffers,
            render_pass,
            swapchain,
            device,
            physical_device,
            surface,
            instance,
            entry,
        })
    }

    fn draw(&mut self, window_size: (u32, u32)) {
        match self.renderer.draw_frame(
            &self.device,
            &self.instance,
            &self.physical_device,
            &self.surface,
            &mut self.swapchain,
            &mut self.framebuffers,
            &self.render_pass,
            window_size,
            CLEAR_COLOR,
        ) {
            Ok(_)  => {}
            Err(e) => error!("draw_frame error: {e}"),
        }
    }

    fn wait_idle(&self) {
        unsafe { self.device.device.device_wait_idle().ok() };
    }
}

#[derive(Default)]
struct App {
    window: Option<Window>,
    vulkan: Option<VulkanContext>,
    window_size: (u32, u32),
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.vulkan.is_some() {
            return;
        }

        let window = event_loop
            .create_window(
                Window::default_attributes()
                    .with_title(format!("VERS v{}", env!("CARGO_PKG_VERSION"))),
            )
            .expect("Failed to create window");

        let size = window.inner_size();
        self.window_size = (size.width, size.height);

        let vulkan = VulkanContext::new(&window).expect("Failed to initialize Vulkan");

        self.window = Some(window);
        self.vulkan = Some(vulkan);

        // Poll continuously for rendering
        event_loop.set_control_flow(ControlFlow::Poll);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                if let Some(v) = &self.vulkan { v.wait_idle(); }
                self.vulkan = None;
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                self.window_size = (size.width, size.height);
                // draw_frame handles recreate internally on next frame
            }
            WindowEvent::RedrawRequested => {
                if let Some(vulkan) = &mut self.vulkan {
                    vulkan.draw(self.window_size);
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _: &ActiveEventLoop) {
        // Request a redraw every time the event loop is idle (Poll mode)
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let event_loop = EventLoop::new().unwrap();
    event_loop.run_app(&mut App::default()).unwrap();
}