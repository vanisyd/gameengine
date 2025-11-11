mod games;
mod engine;
mod app;

use env_logger::Builder;
use winit::event_loop::{ControlFlow, EventLoop};
use crate::app::App;
use crate::engine::ecs::world::World;
use crate::games::tetris::game::TetrisGame;

const WIN_WIDTH: u32 = 1280;
const WIN_HEIGHT: u32 = 960;

fn main() {
    Builder::new()
        .filter_level(log::LevelFilter::Debug)
        .init();

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);

    let tetris = Box::new(TetrisGame::new());
    let mut app = App::default();
    app.game = Some(tetris);
    app.world = Some(World::new());
    event_loop.run_app(&mut app).unwrap();
}
