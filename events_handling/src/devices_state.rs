use std::collections::HashSet;
use crate::{Key, Button};

pub struct DevicesState
{
    pub keyboard_state: HashSet<Key>,
    pub mouse_state: HashSet<Button>,
    pub mouse_move: (f64, f64),
    pub mouse_scroll: (f32, f32),

}

impl DevicesState
{
    pub fn new() -> Self
    {
        Self
        {
            keyboard_state: HashSet::new(),
            mouse_state: HashSet::new(),
            mouse_move: (0., 0.),
            mouse_scroll: (0., 0.)
        }
    }

    pub fn key_pressed(&self, key: Key) -> bool
    {
        self.keyboard_state.contains(&key)
    }
    pub fn button_pressed(&self, button: Button) -> bool
    {
        self.mouse_state.contains(&button)
    }
    pub fn mouse_motion(&self) -> (f64, f64)
    {
        self.mouse_move
    }
    pub fn mouse_scroll(&self) -> (f32, f32)
    {
        self.mouse_scroll
    }
}
