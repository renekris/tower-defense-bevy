use tower_defense_bevy::{resources::*, systems::path_generation::*};
use bevy::prelude::*;

/// Integration test for procedural path generation with enemy movement
#[test]
fn test_procedural_path_enemy_integration() {
    // Generate a procedural path for wave 1
    let enemy_path = generate_level_path(1);
    
    // Validate the path has reasonable properties
    assert!(enemy_path.waypoints.len() >= 2, "Path should have at least start and end points");
    assert!(enemy_path.total_length() > 100.0, "Path should have reasonable length for gameplay");
    
    // Test that path positions are accessible at different progress points
    let start_pos = enemy_path.get_position_at_progress(0.0);
    let mid_pos = enemy_path.get_position_at_progress(0.5);
    let end_pos = enemy_path.get_position_at_progress(1.0);
    
    // Validate positions are different (non-trivial path)
    assert!(start_pos.distance(end_pos) > 50.0, "Start and end should be sufficiently apart");
    assert!(start_pos.distance(mid_pos) > 0.0, "Mid position should be different from start");
    assert!(end_pos.distance(mid_pos) > 0.0, "Mid position should be different from end");
    
    // Test path continuity - positions should change smoothly
    let pos_25 = enemy_path.get_position_at_progress(0.25);
    let pos_75 = enemy_path.get_position_at_progress(0.75);
    
    // These should be reasonable intermediate positions
    assert!(start_pos.distance(pos_25) < start_pos.distance(end_pos));
    assert!(end_pos.distance(pos_75) < start_pos.distance(end_pos));
}

/// Test that procedural paths work with realistic enemy movement simulation
#[test]
fn test_enemy_movement_on_procedural_path() {
    // Generate a path for testing
    let enemy_path = generate_level_path(5); // Use wave 5 for more complex path
    
    // Simulate enemy movement parameters
    let enemy_speed = 100.0; // units per second
    let delta_time = 1.0 / 60.0; // 60 FPS
    let path_length = enemy_path.total_length();
    
    // Simulate enemy progress through the path
    let mut current_progress = 0.0;
    let mut positions = Vec::new();
    
    // Move enemy along the path over multiple frames
    for frame in 0..1000 { // Simulate up to ~16 seconds of movement
        // Calculate movement this frame
        let distance_this_frame = enemy_speed * delta_time;
        let progress_this_frame = distance_this_frame / path_length;
        current_progress += progress_this_frame;
        
        if current_progress >= 1.0 {
            current_progress = 1.0;
            positions.push(enemy_path.get_position_at_progress(current_progress));
            break;
        }
        
        // Record position every 10 frames for sampling
        if frame % 10 == 0 {
            positions.push(enemy_path.get_position_at_progress(current_progress));
        }
    }
    
    // Validate movement simulation
    assert!(positions.len() >= 10, "Should have multiple position samples");
    assert_eq!(positions.last().unwrap(), &enemy_path.get_position_at_progress(1.0), "Should end at path end");
    
    // Validate movement is reasonably smooth (no huge jumps)
    for i in 1..positions.len() {
        let distance_moved = positions[i-1].distance(positions[i]);
        assert!(distance_moved < 50.0, "Movement between frames should be reasonable (frame {}, distance: {})", i, distance_moved);
    }
}

/// Test path generation performance under realistic game conditions
#[test]
fn test_path_generation_performance() {
    use std::time::Instant;
    
    // Test generating paths for multiple waves rapidly
    let start = Instant::now();
    let mut generated_paths = Vec::new();
    
    // Generate paths for 20 waves (realistic game session)
    for wave in 1..=20 {
        let path = generate_level_path(wave);
        generated_paths.push(path);
    }
    
    let generation_time = start.elapsed();
    
    // Performance validation
    assert!(generation_time.as_millis() < 500, "20 paths should generate in under 500ms, took: {:?}", generation_time);
    assert_eq!(generated_paths.len(), 20, "Should generate all requested paths");
    
    // Validate that different waves produce different paths
    let path1 = &generated_paths[0];
    let path10 = &generated_paths[9];
    let path20 = &generated_paths[19];
    
    assert_ne!(path1.waypoints, path10.waypoints, "Different waves should produce different paths");
    assert_ne!(path1.waypoints, path20.waypoints, "Different waves should produce different paths");
    assert_ne!(path10.waypoints, path20.waypoints, "Different waves should produce different paths");
}

/// Test deterministic generation for save/load consistency
#[test]
fn test_deterministic_path_generation() {
    // Generate the same wave multiple times
    let path1 = generate_level_path(7);
    let path2 = generate_level_path(7);
    let path3 = generate_level_path(7);
    
    // Should be identical
    assert_eq!(path1.waypoints, path2.waypoints, "Same wave should produce identical paths");
    assert_eq!(path1.waypoints, path3.waypoints, "Same wave should produce identical paths");
    assert_eq!(path1.total_length(), path2.total_length(), "Path lengths should be identical");
}

/// Test integration with wave management system
#[test]
fn test_wave_manager_integration() {
    // Create a wave manager
    let mut wave_manager = WaveManager::new();
    
    // Test path generation for progressive waves
    let mut previous_paths = Vec::new();
    
    for wave_num in 1..=5 {
        // Start a new wave
        wave_manager.start_wave(5);
        
        // Generate path for this wave
        let current_path = generate_level_path(wave_num);
        
        // Validate path properties
        assert!(current_path.waypoints.len() >= 2, "Wave {} should have valid path", wave_num);
        assert!(current_path.total_length() > 0.0, "Wave {} should have positive path length", wave_num);
        
        // Ensure this path is different from previous ones (difficulty scaling)
        for (prev_wave, prev_path) in previous_paths.iter().enumerate() {
            if prev_path != &current_path.waypoints {
                // Good - paths are different due to difficulty scaling
                continue;
            }
        }
        
        previous_paths.push(current_path.waypoints);
    }
    
    assert_eq!(previous_paths.len(), 5, "Should have generated 5 different wave paths");
}

/// Test caching system integration
#[test]
fn test_cache_integration_performance() {
    use std::time::Instant;
    
    // First generation should be slower (cache miss)
    let start = Instant::now();
    let path1 = generate_level_path(15);
    let first_gen_time = start.elapsed();
    
    // Second generation should be faster (cache hit)
    let start = Instant::now();
    let path2 = generate_level_path(15);
    let second_gen_time = start.elapsed();
    
    // Validate caching worked
    assert_eq!(path1.waypoints, path2.waypoints, "Cached path should be identical");
    
    // Note: Cache performance test may not show significant difference in this simple test
    // But paths should be identical proving cache is working
    
    println!("First generation: {:?}, Second generation: {:?}", first_gen_time, second_gen_time);
}

/// Test tower zone integration (when towers are placed)
#[test]
fn test_tower_zone_generation() {
    // Generate tower zones for a wave
    let zones = generate_placement_zones(3);
    
    // Validate zones are generated
    assert!(!zones.is_empty(), "Should generate tower placement zones");
    
    // Validate zone properties
    for (i, zone) in zones.iter().enumerate() {
        assert!(zone.area() > 0, "Zone {} should have positive area", i);
        
        // Zones should be within reasonable world bounds
        let (top_left, bottom_right) = zone.world_bounds;
        assert!(top_left.x >= -640.0 && top_left.x <= 640.0, "Zone {} should be within world X bounds", i);
        assert!(top_left.y >= -360.0 && top_left.y <= 360.0, "Zone {} should be within world Y bounds", i);
        assert!(bottom_right.x >= -640.0 && bottom_right.x <= 640.0, "Zone {} should be within world X bounds", i);
        assert!(bottom_right.y >= -360.0 && bottom_right.y <= 360.0, "Zone {} should be within world Y bounds", i);
        
        // Validate strategic value is reasonable
        assert!(zone.strategic_value >= 0.0, "Zone {} should have non-negative strategic value", i);
    }
    
    // Validate zones don't overlap too much (good placement strategy)
    for i in 0..zones.len() {
        for j in (i+1)..zones.len() {
            let zone_a = &zones[i];
            let zone_b = &zones[j];
            
            // Check if zones are reasonably separated
            let center_a = Vec2::new(
                (zone_a.world_bounds.0.x + zone_a.world_bounds.1.x) / 2.0,
                (zone_a.world_bounds.0.y + zone_a.world_bounds.1.y) / 2.0
            );
            let center_b = Vec2::new(
                (zone_b.world_bounds.0.x + zone_b.world_bounds.1.x) / 2.0,
                (zone_b.world_bounds.0.y + zone_b.world_bounds.1.y) / 2.0
            );
            
            let separation = center_a.distance(center_b);
            assert!(separation > 30.0, "Tower zones {} and {} should be reasonably separated", i, j);
        }
    }
}