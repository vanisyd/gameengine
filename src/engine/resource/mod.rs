use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Sprite {
    filename: String,
    coords: Vec<SpriteCoordinates>
}

#[derive(Serialize, Deserialize)]
struct SpriteCoordinates {
    start_x: usize,
    start_y: usize,
    end_x: usize,
    end_y: usize
}