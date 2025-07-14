use tower_defense_bevy::components::GamePosition;
use bevy::prelude::Vec2;

#[test]
fn test_position_creation() {
    let pos = GamePosition::new(10.0, 20.0);
    assert_eq!(pos.x, 10.0);
    assert_eq!(pos.y, 20.0);
}

#[test]
fn test_position_default() {
    let pos = GamePosition::default();
    assert_eq!(pos.x, 0.0);
    assert_eq!(pos.y, 0.0);
}

#[test]
fn test_distance_calculation() {
    let pos1 = GamePosition::new(0.0, 0.0);
    let pos2 = GamePosition::new(3.0, 4.0);
    
    let distance = pos1.distance_to(&pos2);
    assert_eq!(distance, 5.0); // 3-4-5 triangle
}

#[test]
fn test_distance_to_self() {
    let pos = GamePosition::new(5.0, 10.0);
    let distance = pos.distance_to(&pos);
    assert_eq!(distance, 0.0);
}

#[test]
fn test_to_vec3() {
    let pos = GamePosition::new(1.0, 2.0);
    let vec3 = pos.to_vec3();
    assert_eq!(vec3.x, 1.0);
    assert_eq!(vec3.y, 2.0);
    assert_eq!(vec3.z, 0.0);
}

#[test]
fn test_from_vec2() {
    let vec2 = Vec2::new(7.0, 8.0);
    let pos: GamePosition = vec2.into();
    assert_eq!(pos.x, 7.0);
    assert_eq!(pos.y, 8.0);
}

#[test]
fn test_to_vec2() {
    let pos = GamePosition::new(3.0, 6.0);
    let vec2: Vec2 = pos.into();
    assert_eq!(vec2.x, 3.0);
    assert_eq!(vec2.y, 6.0);
}