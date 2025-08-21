use tower_defense_bevy::components::Projectile;
use tower_defense_bevy::resources::TowerType;
use bevy::prelude::{Entity, Vec2};

#[test]
fn test_projectile_creation() {
    let projectile = Projectile::default();
    assert_eq!(projectile.damage, 10.0);
    assert_eq!(projectile.speed, 200.0);
    assert_eq!(projectile.target_entity, Entity::PLACEHOLDER);
    assert_eq!(projectile.target_position, Vec2::ZERO);
    assert_eq!(projectile.tower_type, TowerType::Basic);
}

#[test]
fn test_projectile_with_target() {
    let target_entity = Entity::from_raw(42);
    let target_pos = Vec2::new(100.0, 200.0);
    let projectile = Projectile::new(15.0, 250.0, target_entity, target_pos, TowerType::Laser);
    
    assert_eq!(projectile.damage, 15.0);
    assert_eq!(projectile.speed, 250.0);
    assert_eq!(projectile.target_entity, target_entity);
    assert_eq!(projectile.target_position, target_pos);
    assert_eq!(projectile.tower_type, TowerType::Laser);
}

#[test]
fn test_projectile_no_target() {
    let projectile = Projectile::new(20.0, 180.0, Entity::PLACEHOLDER, Vec2::ZERO, TowerType::Basic);
    
    assert_eq!(projectile.target_entity, Entity::PLACEHOLDER);
    assert_eq!(projectile.target_position, Vec2::ZERO);
}

#[test]
fn test_projectile_target_assignment() {
    let mut projectile = Projectile::default();
    assert_eq!(projectile.target_entity, Entity::PLACEHOLDER);
    
    let new_target = Entity::from_raw(123);
    let new_position = Vec2::new(50.0, 75.0);
    projectile.target_entity = new_target;
    projectile.target_position = new_position;
    
    assert_eq!(projectile.target_entity, new_target);
    assert_eq!(projectile.target_position, new_position);
}

#[test]
fn test_projectile_damage_variations() {
    let weak_projectile = Projectile::new(5.0, 200.0, Entity::PLACEHOLDER, Vec2::ZERO, TowerType::Basic);
    let strong_projectile = Projectile::new(50.0, 200.0, Entity::PLACEHOLDER, Vec2::ZERO, TowerType::Tesla);
    
    assert!(weak_projectile.damage < strong_projectile.damage);
    assert_eq!(weak_projectile.tower_type, TowerType::Basic);
    assert_eq!(strong_projectile.tower_type, TowerType::Tesla);
}