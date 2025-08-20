use bevy::prelude::*;

#[derive(Component)]
pub struct Tower {
    pub damage: f32,
    pub range: f32,
    pub fire_rate: f32,
    pub last_shot: f32,
}

impl Default for Tower {
    fn default() -> Self {
        Self {
            damage: 10.0,
            range: 100.0,
            fire_rate: 1.0,
            last_shot: 0.0,
        }
    }
}