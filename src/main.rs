use std::sync::Arc;

use pixels::{Pixels, SurfaceTexture};
use winit::{
    application::ApplicationHandler,
    dpi::{PhysicalSize, Size},
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowAttributes, WindowId},
};

#[derive(Default)]
struct App {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
    frame: u64,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        println!("Resumed");
        let window_attributes: WindowAttributes = WindowAttributes::default()
            .with_inner_size(Size::Physical(PhysicalSize {
                width: 800,
                height: 800,
            }))
            .with_resizable(false);

        self.window = Some(Arc::new(
            event_loop.create_window(window_attributes).unwrap(),
        ));

        let surface_texture = SurfaceTexture::new(800, 800, self.window.clone().unwrap());

        self.pixels = Some(Pixels::new(800, 800, surface_texture).unwrap());

        let pixels: &mut Pixels<'static> = self.pixels.as_mut().unwrap();

        pixels.render().unwrap();

        self.window.as_ref().unwrap().request_redraw();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("Window closed");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                let pixels: &mut Pixels<'static> = self.pixels.as_mut().unwrap();

                let texture = pixels.frame_mut();

                let tex_x = (self.frame * 8) % (800 * 4);
                let tex_y = ((self.frame * 8) / (800 * 4)) * 800 * 4 * 8;

                for x in 0..8 {
                    for y in 0..8 {
                        let offset_x = tex_x + (x * 4);
                        let offset_y = tex_y + (y * 800 * 4);
                        let index = (offset_y + offset_x) as usize;

                        texture[index] = 0xFF;
                        texture[index + 1] = 0x00;
                        texture[index + 2] = 0x00;
                        texture[index + 3] = 0xFF;
                    }
                }

                self.frame += 4;

                pixels.render().unwrap();

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
