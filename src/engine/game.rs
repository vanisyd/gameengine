use crate::engine::ecs::world::World;
use crate::engine::input::InputState;
use super::render::renderer::Renderer;

pub trait Game {
    fn update(&mut self, delta_time: f32, input_state: &InputState, world: &mut World);
    fn render(&self, renderer: &mut Renderer, world: &mut World);
}