use super::triangle::Triangle;
use super::{Position, Size, Color, Renderer};

const MOD_SCANLINE: u8 = 1;
const MOD_TRIANGLE: u8 = 2;

#[derive(Clone, Debug)]
pub struct Rectangle {
    pos: Position,
    size: Size,
    color: Color,
    mode: u8,
    outline_color: Option<Color>
}

impl Rectangle {
    pub fn new(position: Position, size: Size, color: Color) -> Self {
        Self {
            pos: position,
            size,
            color,
            mode: MOD_SCANLINE,
            outline_color: None
        }
    }

    pub fn fill(&self, renderer: &mut Renderer) {
        match self.mode {
            MOD_SCANLINE => self.scanline_fill(renderer),
            MOD_TRIANGLE => self.triangle_fill(renderer),
            _ => unreachable!()
        }

        if self.outline_color.is_some() {
            self.outline(renderer);
        }
    }

    pub fn with_outline(&mut self, color: Color) -> &mut Self {
        self.outline_color = Some(color);
        self
    }

    pub fn outline(&self, renderer: &mut Renderer) {
        let color = self.outline_color.ok_or(self.color).unwrap();

        renderer.draw_line(
            (self.pos.0 + self.size.0, self.pos.1 + self.size.1),
            (self.pos.0 + self.size.0, self.pos.1),
            color
        ); // right
        renderer.draw_line(
            (self.pos.0, self.pos.1 + self.size.1),
            (self.pos.0, self.pos.1),
            color
        ); // left
        renderer.draw_line(
            (self.pos.0, self.pos.1),
            (self.pos.0 + self.size.0, self.pos.1),
            color
        ); // top
        renderer.draw_line(
            (self.pos.0, self.pos.1 + self.size.1),
            (self.pos.0 + self.size.0, self.pos.1 + self.size.1),
            color
        ); // bottom
    }

    fn as_triangles(&self) -> (Triangle, Triangle) {
        let bot_triangle = {
            let p0 = (self.pos.0 + self.size.0, self.pos.1 + self.size.1);
            let p1 = (self.pos.0, self.pos.1 + self.size.1);
            let p2 = (self.pos.0 + self.size.0, self.pos.1);

            Triangle::new(p0, p1, p2, self.color)
        };

        let top_triangle = {
            let p0 = (self.pos.0, self.pos.1 + self.size.1);
            let p1 = (self.pos.0 + self.size.0, self.pos.1);
            let p2 = (self.pos.0, self.pos.1);

            Triangle::new(p0, p1, p2, self.color)
        };

        (top_triangle, bot_triangle)
    }

    pub fn get_position(&self) -> Position {
        self.pos
    }

    pub fn get_size(&self) -> Size {
        self.size
    }

    pub fn with_position(&mut self, position: Position) -> &mut Self {
        self.pos = position;
        self
    }

    fn scanline_fill(&self, renderer: &mut Renderer) {
        let top = self.pos.1 + self.size.1;
        let edge = self.pos.0 + self.size.0;
        for y in self.pos.1..=top {
            renderer.draw_line((self.pos.0, y), (edge, y), self.color);
        }
    }

    fn triangle_fill(&self, renderer: &mut Renderer) {
        let (top_triangle, bot_triangle) = self.as_triangles();
        top_triangle.fill(renderer);
        bot_triangle.fill(renderer);
    }
}