use std::collections::HashSet;
use winit::keyboard::KeyCode;

#[derive(Default)]
pub struct InputState {
    keys_down: HashSet<KeyCode>,
    keys_pressed: HashSet<KeyCode>,
    keys_released: HashSet<KeyCode>,
    pub mouse_pos: (f64, f64),
    pub mouse_pressed: bool
}

impl InputState {
    pub fn key_down(&mut self, key: KeyCode) {
        self.keys_down.insert(key);
    }

    pub fn key_pressed(&mut self, key: KeyCode) {
        self.keys_pressed.insert(key);
    }

    pub fn key_released(&mut self, key: KeyCode) {
        self.keys_released.insert(key);
    }

    pub fn clear(&mut self) {
        self.keys_down.clear();
        self.keys_pressed.clear();
        self.keys_released.clear();
    }

    pub fn is_pressed(&self, key: &KeyCode) -> bool {
        self.keys_pressed.get(key).is_some()
    }

    pub fn mouse_pressed(&self) -> bool {
        self.mouse_pressed
    }
}