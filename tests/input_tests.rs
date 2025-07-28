use tower_defense_bevy::systems::*;
use bevy::prelude::*;

#[test]
fn test_screen_to_world_conversion() {
    let screen_pos = Vec2::new(640.0, 360.0); // Center of 1280x720 screen
    let global_transform = GlobalTransform::from_xyz(0.0, 0.0, 0.0);
    let camera = Camera::default();
    let window = Window {
        resolution: (1280.0, 720.0).into(),
        ..default()
    };
    
    let world_pos = screen_to_world_position(screen_pos, &global_transform, &camera, &window);
    
    // Should be approximately (0.0, 0.0) for center of screen
    assert!((world_pos.x - 0.0).abs() < 100.0); // Allow some tolerance
    assert!((world_pos.y - 0.0).abs() < 100.0);
}

#[test]
fn test_grid_snapping() {
    let world_pos = Vec2::new(47.0, 33.0);
    let grid_size = 32.0;
    
    let snapped = snap_to_grid(world_pos, grid_size);
    
    assert_eq!(snapped, Vec2::new(32.0, 32.0));
}

#[test]
fn test_grid_snapping_negative() {
    let world_pos = Vec2::new(-47.0, -33.0);
    let grid_size = 32.0;
    
    let snapped = snap_to_grid(world_pos, grid_size);
    
    assert_eq!(snapped, Vec2::new(-64.0, -64.0));
}

#[test]
fn test_placement_validation_on_path() {
    let position = Vec2::new(0.0, 0.0); // On the enemy path
    let path_points = vec![
        Vec2::new(-100.0, 0.0),
        Vec2::new(100.0, 0.0),
    ];
    
    assert!(!is_valid_placement_position(position, &path_points, 20.0));
}

#[test]
fn test_placement_validation_off_path() {
    let position = Vec2::new(0.0, 50.0); // Off the enemy path
    let path_points = vec![
        Vec2::new(-100.0, 0.0),
        Vec2::new(100.0, 0.0),
    ];
    
    assert!(is_valid_placement_position(position, &path_points, 20.0));
}

#[test]
fn test_mouse_input_state_creation() {
    let mouse_state = MouseInputState::default();
    
    assert_eq!(mouse_state.current_position, Vec2::ZERO);
    assert_eq!(mouse_state.world_position, Vec2::ZERO);
    assert!(!mouse_state.left_clicked);
    assert!(!mouse_state.right_clicked);
    assert_eq!(mouse_state.selected_tower_type, None);
}

#[test]
fn test_placement_zones() {
    // Test grid zone (left side)
    let left_grid_pos = Vec2::new(-300.0, 100.0);
    assert!(is_in_grid_zone(left_grid_pos));
    
    // Test grid zone (right side) 
    let right_grid_pos = Vec2::new(300.0, 100.0);
    assert!(is_in_grid_zone(right_grid_pos));
    
    // Test free zone (top area)
    let top_free_pos = Vec2::new(0.0, 200.0);
    assert!(is_in_free_zone(top_free_pos));
    
    // Test free zone (bottom area)
    let bottom_free_pos = Vec2::new(0.0, -200.0);
    assert!(is_in_free_zone(bottom_free_pos));
    
    // Test restricted area (center near path)
    let restricted_pos = Vec2::new(0.0, 0.0); // Near path
    assert!(!is_in_grid_zone(restricted_pos));
    assert!(!is_in_free_zone(restricted_pos));
}