use tower_defense_bevy::systems::path_generation::*;
use tower_defense_bevy::systems::unified_grid::*;
use tower_defense_bevy::systems::enemy_system::*;
use tower_defense_bevy::resources::*;
use tower_defense_bevy::components::*;
use bevy::prelude::*;

/// Integration test simulating the complete coordinate pipeline
#[cfg(test)]
mod integration_pipeline {
    use super::*;

    #[test]
    fn test_complete_coordinate_pipeline() {
        // === STEP 1: Setup unified coordinate system ===
        let unified_grid = UnifiedGridSystem::default();
        let path_grid = PathGrid::new_unified();
        
        // Verify they use the same coordinate system
        assert_eq!(path_grid.width, unified_grid.grid_width);
        assert_eq!(path_grid.height, unified_grid.grid_height);
        assert_eq!(path_grid.cell_size, unified_grid.cell_size);
        
        // === STEP 2: Generate path using path generation system ===
        let wave_number = 1;
        let seed = wave_number as u64 * 12345 + 67890;
        let grid_path = generate_random_strategic_path(seed, &path_grid);
        
        // Convert to enemy path (this is what enemies actually use)
        let enemy_path = path_grid.to_enemy_path(grid_path.clone());
        
        // === STEP 3: Verify path coordinates align with unified grid ===
        for (i, &grid_pos) in grid_path.iter().enumerate() {
            // Check that grid position is valid
            assert!(grid_pos.x < unified_grid.grid_width, 
                "Grid position x {} out of bounds", grid_pos.x);
            assert!(grid_pos.y < unified_grid.grid_height, 
                "Grid position y {} out of bounds", grid_pos.y);
            
            // Check that world position matches between systems
            let path_world_pos = path_grid.grid_to_world(grid_pos);
            let unified_world_pos = grid_to_world(grid_pos, &unified_grid);
            let enemy_waypoint = enemy_path.waypoints[i];
            
            // All three should be identical
            let path_diff = path_world_pos - unified_world_pos;
            let enemy_diff = enemy_waypoint - path_world_pos;
            
            assert!(path_diff.length() < 0.001, 
                "Path vs Unified grid mismatch at {}: {:?} vs {:?}", 
                i, path_world_pos, unified_world_pos);
            assert!(enemy_diff.length() < 0.001,
                "Enemy waypoint vs path grid mismatch at {}: {:?} vs {:?}",
                i, enemy_waypoint, path_world_pos);
        }
        
        // === STEP 4: Simulate enemy movement ===
        let mut enemy = Enemy::for_wave(wave_number);
        let mut path_progress = PathProgress::new();
        let mut enemy_transform = Transform::from_translation(Vec3::ZERO);
        
        // Test enemy movement at various progress points
        let test_progress_values = [0.0, 0.1, 0.25, 0.5, 0.75, 0.9, 1.0];
        
        for &progress in &test_progress_values {
            path_progress.current = progress;
            let enemy_position = enemy_path.get_position_at_progress(progress);
            enemy_transform.translation = enemy_position.extend(0.0);
            
            // Verify enemy position can be converted back to grid coordinates
            let grid_pos_option = world_to_grid(enemy_position, &unified_grid);
            assert!(grid_pos_option.is_some(), 
                "Enemy position {:?} at progress {} not convertible to grid", 
                enemy_position, progress);
            
            // Verify position is within reasonable game bounds
            assert!(enemy_position.x >= -600.0 && enemy_position.x <= 600.0,
                "Enemy X position {} out of bounds at progress {}", 
                enemy_position.x, progress);
            assert!(enemy_position.y >= -400.0 && enemy_position.y <= 400.0,
                "Enemy Y position {} out of bounds at progress {}",
                enemy_position.y, progress);
        }
        
        // === STEP 5: Test tower placement zones ===
        let tower_zones = generate_placement_zones(wave_number);
        
        for (i, zone) in tower_zones.iter().enumerate() {
            let (world_min, world_max) = zone.world_bounds;
            
            // Verify zone bounds can be converted to grid coordinates
            let grid_min = world_to_grid(world_min, &unified_grid);
            let grid_max = world_to_grid(world_max, &unified_grid);
            
            assert!(grid_min.is_some(), "Tower zone {} min bounds invalid", i);
            assert!(grid_max.is_some(), "Tower zone {} max bounds invalid", i);
            
            // Verify zone is within game bounds
            assert!(world_min.x >= -600.0 && world_max.x <= 600.0,
                "Tower zone {} X bounds out of range: {} to {}", 
                i, world_min.x, world_max.x);
            assert!(world_min.y >= -400.0 && world_max.y <= 400.0,
                "Tower zone {} Y bounds out of range: {} to {}",
                i, world_min.y, world_max.y);
        }
        
        // === STEP 6: Test debug visualization coordinates ===
        // This simulates what debug visualization would show
        for (i, &grid_pos) in grid_path.iter().enumerate() {
            // Debug visualization uses the same coordinate conversion
            let debug_world_pos = path_grid.grid_to_world(grid_pos);
            let actual_enemy_waypoint = enemy_path.waypoints[i];
            
            // Debug visualization should show enemies at the exact same positions
            let debug_diff = debug_world_pos - actual_enemy_waypoint;
            assert!(debug_diff.length() < 0.001,
                "Debug visualization would show incorrect position at waypoint {}: 
                debug={:?} vs actual={:?}", i, debug_world_pos, actual_enemy_waypoint);
        }
        
        // === STEP 7: Test coordinate conversion edge cases ===
        // Test boundary conversions
        let boundary_tests = vec![
            (GridPos::new(0, 0), "bottom-left corner"),
            (GridPos::new(17, 9), "top-right corner"),
            (GridPos::new(8, 4), "center"),
            (GridPos::new(0, 4), "left edge center"),
            (GridPos::new(17, 4), "right edge center"),
        ];
        
        for (grid_pos, description) in boundary_tests {
            // Convert to world and back
            let world_pos = grid_to_world(grid_pos, &unified_grid);
            let back_to_grid = world_to_grid(world_pos, &unified_grid);
            
            assert_eq!(back_to_grid, Some(grid_pos),
                "Round-trip coordinate conversion failed for {}: {:?} -> {:?} -> {:?}",
                description, grid_pos, world_pos, back_to_grid);
            
            // Verify PathGrid produces same result
            let path_world_pos = path_grid.grid_to_world(grid_pos);
            let path_back_to_grid = path_grid.world_to_grid(path_world_pos);
            
            assert_eq!(path_back_to_grid, Some(grid_pos),
                "PathGrid round-trip failed for {}: {:?} -> {:?} -> {:?}",
                description, grid_pos, path_world_pos, path_back_to_grid);
        }
        
        println!("✅ Complete coordinate pipeline test passed!");
        println!("   - Generated {} waypoints", grid_path.len());
        println!("   - Path length: {:.1} units", enemy_path.total_length());
        println!("   - Tower zones: {}", tower_zones.len());
        println!("   - All coordinates align perfectly between systems");
    }
    
    /// Test multiple waves to ensure coordinate consistency across different paths
    #[test]
    fn test_multi_wave_coordinate_consistency() {
        let unified_grid = UnifiedGridSystem::default();
        
        for wave in 1..=10 {
            let enemy_path = generate_level_path_with_params(wave, 0.2);
            let tower_zones = generate_placement_zones(wave);
            
            // Verify all waypoints are within unified grid bounds
            for (i, &waypoint) in enemy_path.waypoints.iter().enumerate() {
                let grid_pos = world_to_grid(waypoint, &unified_grid);
                
                assert!(grid_pos.is_some(),
                    "Wave {} waypoint {} at {:?} outside unified grid",
                    wave, i, waypoint);
                
                // Verify round-trip conversion
                if let Some(pos) = grid_pos {
                    let back_to_world = grid_to_world(pos, &unified_grid);
                    let diff = waypoint - back_to_world;
                    
                    // Allow some tolerance for grid snapping
                    assert!(diff.length() < 32.1, // Half cell size + epsilon
                        "Wave {} coordinate inconsistency at waypoint {}: {:?} vs {:?} (diff: {:.3})",
                        wave, i, waypoint, back_to_world, diff.length());
                }
            }
            
            // Verify tower zones are valid
            for (i, zone) in tower_zones.iter().enumerate() {
                let (min_world, max_world) = zone.world_bounds;
                let min_grid = world_to_grid(min_world, &unified_grid);
                let max_grid = world_to_grid(max_world, &unified_grid);
                
                assert!(min_grid.is_some() && max_grid.is_some(),
                    "Wave {} tower zone {} bounds outside unified grid: {:?} to {:?}",
                    wave, i, min_world, max_world);
            }
        }
        
        println!("✅ Multi-wave coordinate consistency test passed!");
        println!("   - Tested waves 1-10");
        println!("   - All paths and tower zones within unified grid bounds");
    }
    
    /// Performance test to ensure coordinate conversions are fast enough
    #[test]
    fn test_coordinate_conversion_performance() {
        let unified_grid = UnifiedGridSystem::default();
        let path_grid = PathGrid::new_unified();
        
        let start_time = std::time::Instant::now();
        
        // Test 10,000 coordinate conversions (simulating heavy usage)
        for i in 0..10_000 {
            let grid_x = i % unified_grid.grid_width;
            let grid_y = (i / unified_grid.grid_width) % unified_grid.grid_height;
            let grid_pos = GridPos::new(grid_x, grid_y);
            
            // Forward conversion
            let world_pos1 = grid_to_world(grid_pos, &unified_grid);
            let world_pos2 = path_grid.grid_to_world(grid_pos);
            
            // Reverse conversion
            let _back_to_grid1 = world_to_grid(world_pos1, &unified_grid);
            let _back_to_grid2 = path_grid.world_to_grid(world_pos2);
            
            // Verify consistency (sample check)
            if i % 1000 == 0 {
                let diff = world_pos1 - world_pos2;
                assert!(diff.length() < 0.001, "Coordinate conversion inconsistency");
            }
        }
        
        let elapsed = start_time.elapsed();
        println!("✅ Performance test completed!");
        println!("   - 10,000 conversions in {:.3}ms", elapsed.as_secs_f64() * 1000.0);
        println!("   - Average: {:.1}ns per conversion", elapsed.as_nanos() as f64 / 40_000.0);
        
        // Should complete in reasonable time (less than 10ms on modern hardware)
        assert!(elapsed.as_millis() < 10, 
            "Coordinate conversions too slow: {}ms", elapsed.as_millis());
    }
}