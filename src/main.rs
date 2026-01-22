use std::sync::Arc;

use pixels::{Pixels, SurfaceTexture, wgpu::Color};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowAttributes, WindowId},
};

#[derive(Default)]
struct App {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        println!("Resumed");
        let window_attributes = WindowAttributes::default();

        self.window = Some(Arc::new(
            event_loop.create_window(window_attributes).unwrap(),
        ));

        let surface_texture = SurfaceTexture::new(320, 240, self.window.clone().unwrap());

        self.pixels = Some(Pixels::new(800, 800, surface_texture).unwrap());

        let frame = self.pixels.as_mut().unwrap().clear_color(Color::RED);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("Window closed");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                print!("Redraw requested\n");
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => {}
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();

    event_loop.run_app(&mut app).unwrap();
}
