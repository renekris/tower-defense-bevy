use tower_defense_bevy::components::Projectile;
use bevy::prelude::Entity;

#[test]
fn test_projectile_creation() {
    let projectile = Projectile::default();
    assert_eq!(projectile.damage, 10.0);
    assert_eq!(projectile.speed, 200.0);
    assert_eq!(projectile.target, None);
}

#[test]
fn test_projectile_with_target() {
    let target_entity = Entity::from_raw(42);
    let projectile = Projectile {
        damage: 15.0,
        speed: 250.0,
        target: Some(target_entity),
    };
    
    assert_eq!(projectile.damage, 15.0);
    assert_eq!(projectile.speed, 250.0);
    assert_eq!(projectile.target, Some(target_entity));
}

#[test]
fn test_projectile_no_target() {
    let projectile = Projectile {
        damage: 20.0,
        speed: 180.0,
        target: None,
    };
    
    assert_eq!(projectile.target, None);
    assert!(projectile.target.is_none());
}

#[test]
fn test_projectile_target_assignment() {
    let mut projectile = Projectile::default();
    assert!(projectile.target.is_none());
    
    let new_target = Entity::from_raw(123);
    projectile.target = Some(new_target);
    
    assert!(projectile.target.is_some());
    assert_eq!(projectile.target.unwrap(), new_target);
}

#[test]
fn test_projectile_damage_variations() {
    let weak_projectile = Projectile {
        damage: 5.0,
        ..Default::default()
    };
    
    let strong_projectile = Projectile {
        damage: 50.0,
        ..Default::default()
    };
    
    assert!(weak_projectile.damage < strong_projectile.damage);
}