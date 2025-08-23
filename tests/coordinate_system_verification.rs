use tower_defense_bevy::systems::path_generation::*;
use tower_defense_bevy::systems::unified_grid::*;
use tower_defense_bevy::resources::*;
use bevy::prelude::*;

#[cfg(test)]
mod coordinate_system_tests {
    use super::*;

    /// Test that PathGrid and UnifiedGrid use the same coordinate system
    #[test]
    fn test_path_grid_unified_grid_coordinate_consistency() {
        let path_grid = PathGrid::new_unified();
        let unified_grid = UnifiedGridSystem::default();
        
        // Verify dimensions match
        assert_eq!(path_grid.width, unified_grid.grid_width, "Grid widths must match");
        assert_eq!(path_grid.height, unified_grid.grid_height, "Grid heights must match");
        assert_eq!(path_grid.cell_size, unified_grid.cell_size, "Cell sizes must match");
        
        // Test coordinate conversion consistency for multiple points
        let test_positions = vec![
            GridPos::new(0, 0),      // Bottom-left corner
            GridPos::new(17, 9),     // Top-right corner  
            GridPos::new(8, 4),      // Center
            GridPos::new(0, 5),      // Left edge middle
            GridPos::new(17, 5),     // Right edge middle
        ];
        
        for grid_pos in test_positions {
            // Convert using PathGrid
            let path_world_pos = path_grid.grid_to_world(grid_pos);
            
            // Convert using UnifiedGrid
            let unified_world_pos = grid_to_world(grid_pos, &unified_grid);
            
            // They should be identical
            let diff = path_world_pos - unified_world_pos;
            assert!(diff.length() < 0.001, 
                "Coordinate mismatch at {:?}: PathGrid={:?}, UnifiedGrid={:?}", 
                grid_pos, path_world_pos, unified_world_pos);
            
            // Test reverse conversion
            let path_grid_pos = path_grid.world_to_grid(path_world_pos);
            let unified_grid_pos = world_to_grid(unified_world_pos, &unified_grid);
            
            assert_eq!(path_grid_pos, Some(grid_pos), "PathGrid world_to_grid failed");
            assert_eq!(unified_grid_pos, Some(grid_pos), "UnifiedGrid world_to_grid failed");
        }
    }
    
    /// Test that generated paths align perfectly with grid cells
    #[test]
    fn test_generated_paths_align_with_grid() {
        let wave_number = 1;
        let seed = wave_number as u64 * 12345 + 67890;
        
        // Generate path using the standard pipeline
        let grid = PathGrid::new_unified();
        let grid_path = generate_random_strategic_path(seed, &grid);
        let enemy_path = grid.to_enemy_path(grid_path.clone());
        
        // Verify all grid positions are within bounds
        for &pos in &grid_path {
            assert!(pos.x < grid.width, "Grid path x coordinate {} out of bounds", pos.x);
            assert!(pos.y < grid.height, "Grid path y coordinate {} out of bounds", pos.y);
        }
        
        // Verify all waypoints align with grid cell centers
        for (i, waypoint) in enemy_path.waypoints.iter().enumerate() {
            let grid_pos = grid_path[i];
            let expected_world_pos = grid.grid_to_world(grid_pos);
            
            let diff = *waypoint - expected_world_pos;
            assert!(diff.length() < 0.001, 
                "Waypoint {} at {:?} doesn't align with grid center {:?} (diff: {:?})", 
                i, waypoint, expected_world_pos, diff);
        }
        
        // Test that path starts at left edge and ends at right edge
        assert_eq!(grid_path[0].x, 0, "Path must start at left edge (x=0)");
        assert_eq!(grid_path[grid_path.len()-1].x, grid.width-1, "Path must end at right edge");
        
        // Verify path length is reasonable (not too short - straight lines are boring!)
        // For 32x18 grid, longer paths make for more interesting gameplay
        assert!(grid_path.len() >= 8, "Path should have at least 8 waypoints for interesting gameplay");
        assert!(grid_path.len() <= 40, "Path should be reasonable for 32x18 grid (max width is 32)");
    }
    
    /// Test enemy movement follows grid-aligned paths correctly
    #[test]
    fn test_enemy_movement_grid_alignment() {
        let wave_number = 2;
        let enemy_path = generate_level_path_with_params(wave_number, 0.15);
        
        // Test enemy movement at various progress points
        let test_progress_points = vec![0.0, 0.25, 0.5, 0.75, 1.0];
        
        for progress in test_progress_points {
            let position = enemy_path.get_position_at_progress(progress);
            
            // Position should be valid (not NaN or infinite)
            assert!(position.x.is_finite(), "Enemy position X should be finite at progress {}", progress);
            assert!(position.y.is_finite(), "Enemy position Y should be finite at progress {}", progress);
            
            // Position should be within reasonable game bounds (1280x720 screen = ±640x±360)
            assert!(position.x >= -650.0 && position.x <= 650.0, 
                "Enemy X position {} out of reasonable bounds at progress {}", position.x, progress);
            assert!(position.y >= -370.0 && position.y <= 370.0,
                "Enemy Y position {} out of reasonable bounds at progress {}", position.y, progress);
        }
        
        // Verify path has reasonable total length
        let total_length = enemy_path.total_length();
        assert!(total_length > 100.0, "Path length {} seems too short", total_length);
        assert!(total_length < 2000.0, "Path length {} seems too long", total_length);
    }
    
    /// Test coordinate conversion edge cases and boundaries
    #[test]
    fn test_coordinate_conversion_edge_cases() {
        let path_grid = PathGrid::new_unified();
        let unified_grid = UnifiedGridSystem::default();
        
        // Test boundary positions
        let boundary_positions = vec![
            Vec2::new(-640.0, -320.0),   // Far bottom-left
            Vec2::new(640.0, 320.0),     // Far top-right
            Vec2::new(0.0, 0.0),         // Center
            Vec2::new(-576.0, -288.0),   // Grid boundary bottom-left
            Vec2::new(576.0, 288.0),     // Grid boundary top-right
        ];
        
        for world_pos in boundary_positions {
            let path_grid_pos = path_grid.world_to_grid(world_pos);
            let unified_grid_pos = world_to_grid(world_pos, &unified_grid);
            
            // Results should be identical
            assert_eq!(path_grid_pos, unified_grid_pos,
                "Grid conversion mismatch at world position {:?}", world_pos);
            
            // If conversion succeeded, reverse conversion should work
            if let Some(grid_pos) = path_grid_pos {
                let path_back = path_grid.grid_to_world(grid_pos);
                let unified_back = grid_to_world(grid_pos, &unified_grid);
                
                let diff = path_back - unified_back;
                assert!(diff.length() < 0.001,
                    "Reverse conversion mismatch: {:?} vs {:?}", path_back, unified_back);
            }
        }
    }
    
    /// Test that debug visualization coordinates align with actual grid
    #[test] 
    fn test_debug_visualization_coordinate_alignment() {
        let wave_number = 3;
        let seed = wave_number as u64 * 12345 + 67890;
        
        // Generate the same path that debug visualization would use
        let grid = PathGrid::new_unified();
        let grid_path = generate_random_strategic_path(seed, &grid);
        let enemy_path = grid.to_enemy_path(grid_path.clone());
        
        // Verify debug visualization would show paths at correct positions
        for (i, &grid_pos) in grid_path.iter().enumerate() {
            let expected_world_pos = grid.grid_to_world(grid_pos);
            let actual_waypoint = enemy_path.waypoints[i];
            
            let diff = expected_world_pos - actual_waypoint;
            assert!(diff.length() < 0.001,
                "Debug visualization mismatch at grid position {:?}: expected {:?}, got {:?}",
                grid_pos, expected_world_pos, actual_waypoint);
        }
    }
    
    /// Test multiple waves use consistent coordinate system
    #[test]
    fn test_multi_wave_coordinate_consistency() {
        let unified_grid = UnifiedGridSystem::default();
        
        for wave in 1..=5 {
            let enemy_path = generate_level_path_with_params(wave, 0.2);
            
            // All waypoints should be within the unified grid bounds
            for (i, &waypoint) in enemy_path.waypoints.iter().enumerate() {
                let grid_pos = world_to_grid(waypoint, &unified_grid);
                
                assert!(grid_pos.is_some(), 
                    "Wave {} waypoint {} at {:?} is outside unified grid bounds", 
                    wave, i, waypoint);
                
                if let Some(pos) = grid_pos {
                    // Verify it can be converted back consistently
                    let back_to_world = grid_to_world(pos, &unified_grid);
                    let diff = waypoint - back_to_world;
                    
                    // Allow some tolerance for floating point precision
                    assert!(diff.length() < 32.1, // Half a cell size + epsilon
                        "Wave {} coordinate conversion inconsistency at waypoint {}: {:?} <-> {:?}",
                        wave, i, waypoint, back_to_world);
                }
            }
        }
    }
    
    /// Test that tower placement zones align with grid
    #[test]
    fn test_tower_zones_grid_alignment() {
        let wave_number = 1;
        let tower_zones = generate_placement_zones(wave_number);
        let unified_grid = UnifiedGridSystem::default();
        
        // Verify all tower zones have valid coordinates
        for (i, zone) in tower_zones.iter().enumerate() {
            let (world_min, world_max) = zone.world_bounds;
            
            // Convert to grid coordinates
            let grid_min = world_to_grid(world_min, &unified_grid);
            let grid_max = world_to_grid(world_max, &unified_grid);
            
            assert!(grid_min.is_some(), "Tower zone {} min bounds outside grid", i);
            assert!(grid_max.is_some(), "Tower zone {} max bounds outside grid", i);
            
            // Verify strategic value is reasonable
            assert!(zone.strategic_value >= 0.0 && zone.strategic_value <= 2.0,
                "Tower zone {} has invalid strategic value: {}", i, zone.strategic_value);
        }
    }
}