use bevy::prelude::*;
use crate::resources::TowerType;

#[derive(Component)]
pub struct Projectile {
    pub damage: f32,
    pub speed: f32,
    pub target_entity: Entity,
    pub target_position: Vec2,   // Where the target was when fired
    pub tower_type: TowerType,   // For different projectile behaviors
}

impl Projectile {
    pub fn new(damage: f32, speed: f32, target_entity: Entity, target_position: Vec2, tower_type: TowerType) -> Self {
        Self {
            damage,
            speed,
            target_entity,
            target_position,
            tower_type,
        }
    }
}

// Removed Default implementation to prevent Entity::PLACEHOLDER usage
// This forces explicit construction through Projectile::new() which is safer
// and prevents accidental creation of projectiles with invalid entity references