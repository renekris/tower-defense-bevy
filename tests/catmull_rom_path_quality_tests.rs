use tower_defense_bevy::resources::EnemyPath;
use tower_defense_bevy::systems::path_generation::*;
use bevy::prelude::*;

/// Tests for validating Catmull-Rom spline path quality and smoothness
/// These tests ensure that the visual improvements don't compromise gameplay mechanics

#[test]
fn test_smooth_paths_are_natural_looking() {
    // Create a zigzag path that would benefit from smoothing
    let waypoints = vec![
        Vec2::new(0.0, 0.0),
        Vec2::new(50.0, 0.0),
        Vec2::new(50.0, 50.0),
        Vec2::new(100.0, 50.0),
        Vec2::new(100.0, 100.0),
        Vec2::new(150.0, 100.0),
    ];
    let path = EnemyPath::new(waypoints.clone());
    
    // Sample positions along the smooth path
    let mut smooth_positions = Vec::new();
    for i in 0..=20 {
        let progress = i as f32 / 20.0;
        smooth_positions.push(path.get_smooth_position_at_progress(progress));
    }
    
    // Validate smoothness by checking that direction changes are gradual
    let mut max_direction_change = 0.0f32;
    for i in 2..smooth_positions.len() {
        let dir1 = (smooth_positions[i-1] - smooth_positions[i-2]).normalize();
        let dir2 = (smooth_positions[i] - smooth_positions[i-1]).normalize();
        
        // Calculate angle between directions (dot product gives cosine of angle)
        let dot = dir1.dot(dir2).clamp(-1.0, 1.0);
        let angle_change = dot.acos();
        max_direction_change = max_direction_change.max(angle_change);
    }
    
    // Maximum direction change should be reasonable (less than 90 degrees)
    assert!(max_direction_change < std::f32::consts::PI / 2.0, 
        "Direction changes should be gradual, got max change: {} radians", max_direction_change);
}

#[test] 
fn test_smooth_paths_pass_through_strategic_waypoints() {
    // Generate a realistic game path
    let enemy_path = generate_level_path(5);
    
    // Verify that smooth interpolation still passes through all original waypoints
    let total_segments = enemy_path.waypoints.len() - 1;
    for i in 0..enemy_path.waypoints.len() {
        let progress = i as f32 / total_segments as f32;
        let smooth_pos = enemy_path.get_smooth_position_at_progress(progress);
        let expected = enemy_path.waypoints[i];
        
        // Should be within floating point precision of the waypoint
        let distance = (smooth_pos - expected).length();
        assert!(distance < 0.01, 
            "Smooth path must pass through waypoint {} (distance: {})", i, distance);
    }
}

#[test]
fn test_smooth_path_length_balance() {
    // Test different path configurations to ensure balanced path lengths
    let test_paths = vec![
        // Simple straight path
        vec![Vec2::new(0.0, 0.0), Vec2::new(100.0, 0.0)],
        // L-shaped path  
        vec![Vec2::new(0.0, 0.0), Vec2::new(50.0, 0.0), Vec2::new(50.0, 50.0)],
        // Complex zigzag
        vec![
            Vec2::new(0.0, 0.0), Vec2::new(25.0, 25.0), 
            Vec2::new(50.0, 0.0), Vec2::new(75.0, 25.0), Vec2::new(100.0, 0.0)
        ],
    ];
    
    for (i, waypoints) in test_paths.iter().enumerate() {
        let path = EnemyPath::new(waypoints.clone());
        let linear_length = calculate_linear_path_length(&path);
        let smooth_length = calculate_smooth_path_length(&path);
        
        // Smooth path should be reasonably close to linear path length
        // Too short = cutting corners, too long = inefficient
        let length_ratio = smooth_length / linear_length;
        assert!(length_ratio >= 0.85 && length_ratio <= 1.15,
            "Path {} smooth length ratio should be 0.85-1.15, got: {}", i, length_ratio);
    }
}

#[test]
fn test_smooth_paths_avoid_impossible_difficulty() {
    // Generate paths for various waves and ensure they remain defendable
    for wave in 1..=10 {
        let path = generate_level_path(wave);
        let smooth_length = calculate_smooth_path_length(&path);
        
        // Validate minimum path length to ensure it's not too short to defend
        let min_expected_length = 200.0; // Minimum viable defense distance
        assert!(smooth_length >= min_expected_length,
            "Wave {} path too short for defense: {} < {}", wave, smooth_length, min_expected_length);
        
        // Validate maximum path length to ensure reasonable game time
        let max_expected_length = 1500.0; // Maximum reasonable path length
        assert!(smooth_length <= max_expected_length,
            "Wave {} path too long: {} > {}", wave, smooth_length, max_expected_length);
    }
}

#[test]
fn test_corner_curvature_prevents_instant_direction_changes() {
    // Create a path with sharp 90-degree turns
    let waypoints = vec![
        Vec2::new(0.0, 0.0),
        Vec2::new(100.0, 0.0),   // Sharp right turn
        Vec2::new(100.0, 100.0), // Sharp up turn
        Vec2::new(200.0, 100.0), // Sharp right turn
    ];
    let path = EnemyPath::new(waypoints);
    
    // Sample densely around the sharp corners
    let corner_regions = vec![
        (0.25, 0.4),   // Around first corner
        (0.6, 0.75),   // Around second corner
    ];
    
    for (start_progress, end_progress) in corner_regions {
        let mut previous_direction = Vec2::ZERO;
        let mut first_iteration = true;
        
        // Sample 20 points around the corner
        for i in 0..=20 {
            let progress = start_progress + (end_progress - start_progress) * (i as f32 / 20.0);
            let pos1 = path.get_smooth_position_at_progress(progress);
            let pos2 = path.get_smooth_position_at_progress(progress + 0.01);
            let direction = (pos2 - pos1).normalize();
            
            if !first_iteration {
                let direction_change = (direction - previous_direction).length();
                // Direction changes should be gradual, not instant
                assert!(direction_change < 0.3,
                    "Direction change too abrupt at progress {}: {}", progress, direction_change);
            }
            
            previous_direction = direction;
            first_iteration = false;
        }
    }
}

#[test]
fn test_smooth_path_consistency_across_samples() {
    // Verify that sampling the smooth path at different densities gives consistent results
    let path = generate_level_path(3);
    
    // Sample at two different densities
    let low_density_samples = sample_path_at_density(&path, 10);
    let high_density_samples = sample_path_at_density(&path, 50);
    
    // Extract corresponding samples from high density to match low density positions
    for (i, &low_sample) in low_density_samples.iter().enumerate() {
        let corresponding_high_index = i * 5; // 50/10 = 5
        if corresponding_high_index < high_density_samples.len() {
            let high_sample = high_density_samples[corresponding_high_index];
            let difference = (low_sample - high_sample).length();
            
            assert!(difference < 0.5,
                "Inconsistent sampling at index {}: low={:?}, high={:?}, diff={}",
                i, low_sample, high_sample, difference);
        }
    }
}

#[test] 
fn test_path_smoothing_preserves_total_travel_time() {
    // Ensure smooth paths don't significantly alter enemy travel time
    let test_speeds = vec![50.0, 100.0, 150.0]; // Different enemy speeds
    
    for wave in 1..=5 {
        let path = generate_level_path(wave);
        
        for &speed in &test_speeds {
            let linear_time = calculate_linear_path_length(&path) / speed;
            let smooth_time = calculate_smooth_path_length(&path) / speed;
            
            // Travel time difference should be minimal (within 20%)
            let time_ratio = smooth_time / linear_time;
            assert!(time_ratio >= 0.8 && time_ratio <= 1.2,
                "Wave {} speed {} travel time ratio should be 0.8-1.2, got: {}",
                wave, speed, time_ratio);
        }
    }
}

// Helper functions for path testing

/// Calculate the total length of a path using linear interpolation
fn calculate_linear_path_length(path: &EnemyPath) -> f32 {
    path.total_length()
}

/// Calculate the total length of a path using smooth interpolation
fn calculate_smooth_path_length(path: &EnemyPath) -> f32 {
    let mut total_length = 0.0;
    let samples = 1000; // High resolution for accuracy
    
    for i in 0..samples {
        let progress1 = i as f32 / samples as f32;
        let progress2 = (i + 1) as f32 / samples as f32;
        
        let pos1 = path.get_smooth_position_at_progress(progress1);
        let pos2 = path.get_smooth_position_at_progress(progress2);
        
        total_length += pos1.distance(pos2);
    }
    
    total_length
}

/// Sample a path at a specific density and return positions
fn sample_path_at_density(path: &EnemyPath, sample_count: usize) -> Vec<Vec2> {
    let mut samples = Vec::new();
    
    for i in 0..=sample_count {
        let progress = i as f32 / sample_count as f32;
        samples.push(path.get_smooth_position_at_progress(progress));
    }
    
    samples
}

#[test]
fn test_deterministic_smooth_paths() {
    // Ensure smooth paths are deterministic for save/load consistency
    let path1 = generate_level_path(7);
    let path2 = generate_level_path(7);
    
    // Sample both paths and verify they're identical
    for i in 0..=100 {
        let progress = i as f32 / 100.0;
        let pos1 = path1.get_smooth_position_at_progress(progress);
        let pos2 = path2.get_smooth_position_at_progress(progress);
        
        let difference = (pos1 - pos2).length();
        assert!(difference < 0.001,
            "Smooth paths should be deterministic at progress {}: diff={}", progress, difference);
    }
}