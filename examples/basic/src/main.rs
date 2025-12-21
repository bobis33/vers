use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};
use vers_engine::vulkan::{VulkanEntry, VulkanInstance, VulkanSurface};

#[derive(Default)]
struct App {
    window: Option<Window>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(Window::default_attributes().with_title("vers"))
            .unwrap();

        let entry = VulkanEntry::new().unwrap();
        let instance = VulkanInstance::new(&entry, &window).unwrap();
        let surface = VulkanSurface::new(&entry, &instance, &window, &window).unwrap();

        println!("Surface créée : {:?}", surface.surface);
        self.window = Some(window);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        if event == WindowEvent::CloseRequested {
            event_loop.exit();
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.run_app(&mut App::default()).unwrap();
}

