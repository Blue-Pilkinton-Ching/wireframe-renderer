use std::sync::Arc;

mod simulation;

use simulation::*;

use pixels::{Pixels, SurfaceTexture};
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::{MouseButton, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowAttributes, WindowId},
};

#[derive(Default)]
struct App {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
    simulation: Option<Simulation>,
    mouse_state: MouseState,
}

#[derive(Default)]
struct MouseState {
    pub position: (usize, usize),
    pub left_button_pressed: bool,
    pub right_button_pressed: bool,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        println!("Resumed");
        let window_attributes: WindowAttributes = WindowAttributes::default()
            .with_inner_size(winit::dpi::Size::Physical(PhysicalSize {
                width: 800,
                height: 800,
            }))
            .with_resizable(false);

        self.window = Some(Arc::new(
            event_loop.create_window(window_attributes).unwrap(),
        ));

        let surface_texture = SurfaceTexture::new(800, 800, self.window.clone().unwrap());

        self.pixels = Some(Pixels::new(800, 800, surface_texture).unwrap());

        self.simulation = Some(Simulation::new(Size {
            width: 200,
            height: 200,
        }));

        self.window.as_ref().unwrap().request_redraw();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CursorMoved {
                device_id: _,
                position,
            } => {
                self.mouse_state.position = (position.x as usize, position.y as usize);
            }
            WindowEvent::MouseInput {
                device_id: _,
                state,
                button,
            } => match button {
                MouseButton::Left => self.mouse_state.left_button_pressed = state.is_pressed(),
                MouseButton::Right => self.mouse_state.right_button_pressed = state.is_pressed(),
                _ => (),
            },
            WindowEvent::CloseRequested => {
                println!("Window closed");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                let simulation = self.simulation.as_mut().unwrap();

                // Input
                if self.mouse_state.left_button_pressed {
                    let (x, y) = self.mouse_state.position;
                    simulation.add_sand(
                        x / (800 / simulation.get_size().width),
                        y / (800 / simulation.get_size().height),
                    );
                }

                if self.mouse_state.right_button_pressed {
                    let (x, y) = self.mouse_state.position;
                    simulation.add_air(
                        x / (800 / simulation.get_size().width),
                        y / (800 / simulation.get_size().height),
                    );
                }

                // Logic
                simulation.tick();

                // Render
                let pixels: &mut Pixels<'static> = self.pixels.as_mut().unwrap();
                let texture = pixels.frame_mut();

                for state_change in simulation.get_state_changes() {
                    // Pixel x and y in simulation space
                    let (sim_x, sim_y) = simulation.index_to_x_and_y(state_change.state_index);

                    // Pixel x and y in pixels space
                    let pixel_x = sim_x * 4 * (800 / simulation.get_size().width);
                    let pixel_y = sim_y * 4 * 800 * (800 / simulation.get_size().height);

                    for x in 0..4 {
                        for y in 0..4 {
                            let offset_x = pixel_x + (x * 4);
                            let offset_y = pixel_y + (y * 800 * 4);
                            let index = (offset_y + offset_x) as usize;

                            match state_change.new_element {
                                Element::Air => {
                                    texture[index] = 0x00;
                                    texture[index + 1] = 0x00;
                                    texture[index + 2] = 0x00;
                                    texture[index + 3] = 0xFF;
                                }
                                Element::Sand => {
                                    texture[index] = 0xFF;
                                    texture[index + 1] = 0xFF;
                                    texture[index + 2] = 0xFF;
                                    texture[index + 3] = 0xFF;
                                }
                            }
                        }
                    }
                }

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
