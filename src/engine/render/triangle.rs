use super::{Point, Color, Renderer};

pub struct Triangle {
    points: [Point; 3],
    color: Color
}

impl Triangle {
    pub fn new(p0: Point, p1: Point, p2: Point, color: Color) -> Self {
        let mut points = [p0, p1, p2];
        points.sort_by_key(|p| (p.1, p.0));
        Self { points, color }
    }

    pub fn fill(&self, renderer: &mut Renderer) {
        let (top, mid, bot) = (self.points[0], self.points[1], self.points[2]);

        let (top_x, top_y) = (top.0 as isize, top.1 as isize);
        let (bot_x, bot_y) = (bot.0 as isize, bot.1 as isize);
        let (mid_x, mid_y) = (mid.0 as isize, mid.1 as isize);

        let v_top_dist = mid_y - top_y;
        let v_bot_dist = bot_y - mid_y;
        let v_mid_dist = bot_y - top_y;
        let h_top_dist = bot_x - top_x;
        let h_mid_dist = mid_x - top_x;
        let h_bot_dist = bot_x - mid_x;

        for y in top_y..mid_y {
            let x_edge1 = if v_top_dist != 0 {
                top_x + h_mid_dist * (y - top_y) / v_top_dist
            } else {
                top_x
            };

            let x_edge2 = if v_mid_dist != 0 {
                top_x + h_top_dist * (y - top_y) / v_mid_dist
            } else {
                top_x
            };
            renderer.draw_line(
                (x_edge1 as usize, y as usize),
                (x_edge2 as usize, y as usize),
                self.color
            );
        }

        for y in mid_y..=bot_y {
            let x_edge1 = if v_bot_dist != 0 {
                mid_x + h_bot_dist * (y - mid_y) / v_bot_dist
            } else {
                mid_x
            };

            let x_edge2 = if v_mid_dist != 0 {
                top_x + h_top_dist * (y - top_y) / v_mid_dist
            } else {
                top_x
            };

            renderer.draw_line(
                (x_edge1 as usize, y as usize),
                (x_edge2 as usize, y as usize),
                self.color
            );
        }
    }

    pub fn outline(&self, renderer: &mut Renderer) {
        let (top, mid, bot) = (self.points[0], self.points[1], self.points[2]);

        renderer.draw_line(top, bot, self.color);
        renderer.draw_line(bot, mid, self.color);
        renderer.draw_line(mid, top, self.color);
    }
}