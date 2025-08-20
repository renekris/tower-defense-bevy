use tower_defense_bevy::components::*;
use tower_defense_bevy::resources::*;
use bevy::prelude::*;

#[test]
fn test_path_progress_creation() {
    let progress = PathProgress::new();
    assert_eq!(progress.current, 0.0);
    assert!(progress.current >= 0.0 && progress.current <= 1.0);
}

#[test]
fn test_path_progress_advance() {
    let mut progress = PathProgress::new();
    progress.advance(0.25);
    assert_eq!(progress.current, 0.25);
}

#[test]
fn test_path_progress_advance_beyond_end() {
    let mut progress = PathProgress::new();
    progress.advance(1.5);
    assert_eq!(progress.current, 1.0);
}

#[test]
fn test_path_progress_is_complete() {
    let mut progress = PathProgress::new();
    assert!(!progress.is_complete());
    
    progress.advance(1.0);
    assert!(progress.is_complete());
}

#[test]
fn test_enemy_movement_calculation() {
    // Test that we can calculate movement distance based on speed and time
    let enemy = Enemy { speed: 100.0, ..Default::default() };
    let delta_time = 0.5; // half a second
    let distance = enemy.speed * delta_time;
    assert_eq!(distance, 50.0);
}

#[test]
fn test_path_distance_to_progress_conversion() {
    let path = EnemyPath::new(vec![
        Vec2::new(0.0, 0.0),
        Vec2::new(100.0, 0.0),
        Vec2::new(100.0, 100.0),
    ]);
    
    let total_length = path.total_length();
    assert_eq!(total_length, 200.0); // 100 + 100
    
    // 50 units along a 200 unit path should be 0.25 progress
    let progress = 50.0 / total_length;
    assert_eq!(progress, 0.25);
}