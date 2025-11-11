use crate::engine::render::renderer::Renderer;

pub mod renderer;
pub mod triangle;
pub mod rect;

pub type Point = (usize, usize);
pub type Color = u32;
pub type Position = (usize, usize);
pub type Size = (usize, usize);
pub type Rotation = (usize);

pub trait Renderable {
    fn render(&self, renderer: &mut Renderer);
}