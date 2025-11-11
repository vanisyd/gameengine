use crate::engine::render::rect::Rectangle;
use crate::WIN_WIDTH;

pub mod game;
mod tetromino;

pub(crate) const COLORS: [u32; 4] = [
    4289854247, //orange
    4293600047, //yellow
    4285190477, //green
    4282666183 //blue
];
pub(crate) type Shape = Rectangle;
pub(crate) const BLOCK_SIZE: usize = (WIN_WIDTH / 28) as usize;