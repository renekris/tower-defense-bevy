use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy {
    pub speed: f32,
    pub path_index: usize,
    pub reward: u32,
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            speed: 50.0,
            path_index: 0,
            reward: 10,
        }
    }
}