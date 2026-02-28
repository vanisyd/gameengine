use crate::engine::ecs::world::World;
use crate::engine::game::Game;
use crate::engine::input::InputState;
use crate::engine::render::renderer::Renderer;
use crate::{WIN_HEIGHT, WIN_WIDTH};
use softbuffer::{Context, Surface};
use std::rc::Rc;
use std::time::Instant;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::{ElementState, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::PhysicalKey;
use winit::window::{Window, WindowAttributes, WindowButtons, WindowId};

#[derive(Default)]
pub struct App {
    window: Option<Rc<Window>>,
    surface: Option<Surface<Rc<Window>, Rc<Window>>>,
    pub game: Option<Box<dyn Game>>,
    last_update: Option<Instant>,
    renderer: Option<Renderer>,
    input_state: InputState,
    pub world: Option<World>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attrs = WindowAttributes::default()
            .with_title("Engine")
            .with_resizable(true)
            .with_inner_size(PhysicalSize::new(WIN_WIDTH, WIN_HEIGHT))
            .with_enabled_buttons(WindowButtons::CLOSE);

        let window = Rc::new(event_loop.create_window(window_attrs).unwrap());
        let context = Context::new(window.clone()).unwrap();
        let surface = Surface::new(&context, window.clone()).unwrap();

        self.window = Some(window);
        self.surface = Some(surface);
        if self.last_update.is_none() {
            self.last_update = Some(Instant::now());
        }

        let render_buffer = vec![0u32; (WIN_WIDTH * WIN_HEIGHT) as usize];
        self.renderer = Some(Renderer::new(render_buffer, WIN_WIDTH, WIN_HEIGHT));
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        let surface = self.surface.as_mut().unwrap();

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                let now = Instant::now();
                let delta_time = if let Some(last_update) = self.last_update {
                    (now - last_update).as_secs_f32()
                } else {
                    0.0
                };
                self.last_update = Some(now);

                let world = self.world.as_mut().unwrap();

                // Update
                if let Some(game) = &mut self.game {
                    game.update(delta_time, &self.input_state, world);
                    self.input_state.clear();
                }

                if let Some(window) = &self.window {
                    window.request_redraw();
                }

                // Render
                if let Some(renderer) = &mut self.renderer {
                    if let Some(game) = &mut self.game {
                        game.render(renderer, world);
                    }

                    let mut surface_buffer = surface.buffer_mut().unwrap();

                    surface_buffer.copy_from_slice(renderer.buf_as_slice());
                    surface_buffer.present().unwrap();
                }
            }
            WindowEvent::KeyboardInput {
                device_id: _,
                event: keyboard_event,
                is_synthetic: _,
            } => {
                if let PhysicalKey::Code(key_code) = keyboard_event.physical_key {
                    match keyboard_event.state {
                        ElementState::Pressed => {
                            if keyboard_event.repeat == true {
                                // info!("{:?} repeat!", key_code);
                            }
                            self.input_state.key_pressed(key_code)
                        }
                        ElementState::Released => self.input_state.key_released(key_code),
                    };
                }
            }
            WindowEvent::MouseInput {
                device_id: _,
                state: mouse_state,
                button: mouse_btn,
            } => {
                self.input_state.mouse_pressed = mouse_state.is_pressed();
            },
            WindowEvent::CursorMoved { device_id: _, position: pos } => {
                self.input_state.mouse_pos = (pos.x, pos.y);
            }
            _ => {}
        }
    }
}

/*impl Default for App {
    fn default() -> Self {
        Self {
            window: None,
            surface: None,
            game: None,
            last_update: Instant::
        }
    }
}*/
