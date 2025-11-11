use crate::engine::render::{Color, Point, Position, Size};
use super::triangle::Triangle;
use super::rect::Rectangle;

pub struct Renderer {
    buffer: Vec<u32>,
    width: usize,
    height: usize
}

pub struct Line { start: Point, end: Point, color: Color }
impl Default for Line {
    fn default() -> Self {
        Self {
            start: (0, 0),
            end: (0, 0),
            color: Renderer::rgb_to_u32(255, 255, 255)
        }
    }
}

impl Renderer {
    pub fn new(buffer: Vec<u32>, width: u32, height: u32) -> Self {
        Self {
            buffer,
            width: width as usize,
            height: height as usize
        }
    }

    pub fn triangle(&mut self, p0: Point, p1: Point, p2: Point, color: Color) -> Triangle {
        Triangle::new(p0, p1, p2, color)
    }

    pub fn rect(&mut self, position: Position, size: Size, color: Color) -> Rectangle {
        Rectangle::new(position, size, color)
    }

    pub fn put_pixel(&mut self, x: usize, y: usize, color: Color) {
        let idx = y * self.width + x;
        if idx > self.buffer.len() {
            return;
        }

        self.buffer[idx] = color;
    }

    pub fn draw_line(&mut self, start: Point, end: Point, color: Color) {
        let dx: isize = end.0.abs_diff(start.0) as isize;
        let dy: isize = end.1.abs_diff(start.1) as isize;
        let delta_1: isize;
        let delta_2: isize;
        let mut delta: isize;
        if dy <= dx {
            delta_1 = 2 * dy;
            delta_2 = 2 * (dy - dx);
            delta = (2*dy - dx);
        } else {
            delta_1 = 2*dx;
            delta_2 = 2 * (dx - dy);
            delta = (2*dx - dy);
        }

        let step_x: isize = if end.0 > start.0 {
            1
        } else {
            -1
        };
        let step_y: isize = if end.1 > start.1 {
            1
        } else {
            -1
        };

        let mut x = start.0;
        let mut y = start.1;
        self.put_pixel(x, y, color);
        while x != end.0 || y != end.1 {
            if delta >= 0 {
                delta += delta_2;
                if dx > dy {
                    y = y.wrapping_add_signed(step_y);
                } else {
                    x = x.wrapping_add_signed(step_x);
                }
            } else {
                delta += delta_1;
            }
            self.put_pixel(x, y, color);
            if dx > dy {
                x = x.wrapping_add_signed(step_x);
            } else {
                y = y.wrapping_add_signed(step_y);
            }
        }
    }

    pub fn buf_as_slice(&self) -> &[u32] {
        self.buffer.as_slice()
    }

    pub fn rgb_to_u32(r: u8, g: u8, b: u8) -> u32 {
        (r as u32) << 16 | (g as u32) << 8 | (b as u32)
    }
}