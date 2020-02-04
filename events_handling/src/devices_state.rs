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
}
