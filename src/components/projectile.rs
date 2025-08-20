use bevy::prelude::*;

#[derive(Component)]
pub struct Projectile {
    pub damage: f32,
    pub speed: f32,
    pub target: Option<Entity>,
}

impl Default for Projectile {
    fn default() -> Self {
        Self {
            damage: 10.0,
            speed: 200.0,
            target: None,
        }
    }
}