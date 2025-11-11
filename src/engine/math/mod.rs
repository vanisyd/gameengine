#[derive(Default)]
pub struct Vec2 { start: Point, end: Point }
#[derive(Default)]
pub struct Point { x: usize, y: usize }

pub struct Rect {
    points: Vec<Point>,
    pos: Option<Point>,
    slope: i8
}
