/// Comprehensive tests to verify the dense grid system (32x18 = 576 squares)
/// and path generation integration work correctly.

use bevy::math::Vec2;
use tower_defense_bevy::systems::path_generation::grid::{PathGrid, GridPos};
use tower_defense_bevy::systems::path_generation::pathfinding::{find_path, validate_strategic_path_requirements};
use tower_defense_bevy::systems::unified_grid::UnifiedGridSystem;

#[cfg(test)]
mod dense_grid_tests {
    use super::*;

    #[test]
    fn test_dense_grid_dimensions() {
        let grid = PathGrid::new_unified();
        
        // Verify dense grid dimensions: 32x18 = 576 squares
        assert_eq!(grid.width, 32, "Grid width should be 32");
        assert_eq!(grid.height, 18, "Grid height should be 18");
        assert_eq!(grid.width * grid.height, 576, "Total squares should be 576");
        
        // Verify cell size matches unified grid (40x40 pixels)
        assert_eq!(grid.cell_size, 40.0, "Cell size should be 40 pixels");
    }

    #[test]
    fn test_unified_grid_system_defaults() {
        let unified_grid = UnifiedGridSystem::default();
        
        // Verify unified grid matches PathGrid dimensions
        assert_eq!(unified_grid.grid_width, 32, "Unified grid width should be 32");
        assert_eq!(unified_grid.grid_height, 18, "Unified grid height should be 18");
        assert_eq!(unified_grid.cell_size, 40.0, "Unified cell size should be 40 pixels");
        assert_eq!(unified_grid.total_squares(), 576, "Total squares should be 576");
        
        // Verify full screen coverage (1280x720)
        let area_size = unified_grid.grid_area_size();
        assert_eq!(area_size.x, 1280.0, "Grid should cover full width (1280px)");
        assert_eq!(area_size.y, 720.0, "Grid should cover full height (720px)");
    }

    #[test]
    fn test_coordinate_conversions_40px_cells() {
        let grid = PathGrid::new_unified();
        
        // Test grid-to-world conversion for center cell (15.5, 8.5 would be exact center)
        // But we use (16, 9) as center cell since we use integer grid positions
        let center_grid = GridPos::new(15, 8); // Near-center of 32x18 grid
        let center_world = grid.grid_to_world(center_grid);
        
        // Center cell should be close to world origin, allowing for grid offset
        assert!(center_world.x.abs() < 50.0, "Center X should be near 0, got {}", center_world.x);
        assert!(center_world.y.abs() < 50.0, "Center Y should be near 0, got {}", center_world.y);
        
        // Test corner coordinates
        let corner_grid = GridPos::new(0, 0); // Bottom-left corner
        let corner_world = grid.grid_to_world(corner_grid);
        
        // Should be in the bottom-left quadrant of the grid
        assert!(corner_world.x < 0.0, "Corner X should be negative (left side)");
        assert!(corner_world.y < 0.0, "Corner Y should be negative (bottom side)");
        
        // Test that grid covers expected world area
        let top_right_grid = GridPos::new(31, 17);
        let top_right_world = grid.grid_to_world(top_right_grid);
        
        assert!(top_right_world.x > 0.0, "Top-right X should be positive");
        assert!(top_right_world.y > 0.0, "Top-right Y should be positive");
        
        // Verify grid spans approximately 1280x720 pixels
        let grid_width = top_right_world.x - corner_world.x;
        let grid_height = top_right_world.y - corner_world.y;
        
        // Should be close to expected dimensions (allowing for cell center positioning)
        assert!((grid_width - 1240.0).abs() < 100.0, "Grid width should be ~1240px, got {}", grid_width);
        assert!((grid_height - 680.0).abs() < 100.0, "Grid height should be ~680px, got {}", grid_height);
    }

    #[test]
    fn test_path_generation_uses_dense_grid_boundaries() {
        let grid = PathGrid::new_unified();
        
        // Test that entry/exit points respect dense grid boundaries
        assert_eq!(grid.entry_point.x, 0, "Entry should be at left edge");
        assert_eq!(grid.exit_point.x, 31, "Exit should be at right edge (width-1)");
        assert_eq!(grid.entry_point.y, 9, "Entry Y should be middle (height/2)");
        assert_eq!(grid.exit_point.y, 9, "Exit Y should be middle (height/2)");
        
        // Verify entry/exit points are within grid bounds
        assert!(grid.entry_point.x < grid.width, "Entry X within bounds");
        assert!(grid.entry_point.y < grid.height, "Entry Y within bounds");
        assert!(grid.exit_point.x < grid.width, "Exit X within bounds");
        assert!(grid.exit_point.y < grid.height, "Exit Y within bounds");
    }

    #[test]
    fn test_pathfinding_works_with_dense_grid() {
        let grid = PathGrid::new_unified();
        
        // Test pathfinding from entry to exit
        let path = find_path(&grid, grid.entry_point, grid.exit_point);
        
        assert!(path.is_some(), "Path should be found from entry to exit");
        
        if let Some(path) = path {
            assert!(!path.is_empty(), "Path should not be empty");
            assert_eq!(path[0], grid.entry_point, "Path should start at entry point");
            assert_eq!(path[path.len()-1], grid.exit_point, "Path should end at exit point");
            
            // Verify all path points are within dense grid bounds
            for &pos in &path {
                assert!(pos.x < grid.width, "Path point X {} should be within width {}", pos.x, grid.width);
                assert!(pos.y < grid.height, "Path point Y {} should be within height {}", pos.y, grid.height);
            }
        }
    }

    #[test]
    fn test_strategic_path_validation_with_dense_grid() {
        let grid = PathGrid::new_unified();
        
        // Create a valid strategic path that meets all requirements:
        // - Starts at x=0, ends at x=31
        // - Start/end Y in middle range (6-12 for 18-high grid)
        // - Avoids edges (no intermediate points on boundaries)
        // - Has 3-5 turns
        let strategic_path = vec![
            GridPos::new(0, 9),   // Start left edge (middle)
            GridPos::new(6, 9),   // Move right, away from edge
            GridPos::new(6, 12),  // Turn up, staying away from top edge (17)
            GridPos::new(12, 12), // Move right
            GridPos::new(12, 6),  // Turn down, staying away from bottom edge (0)
            GridPos::new(18, 6),  // Move right
            GridPos::new(18, 10), // Turn up
            GridPos::new(24, 10), // Move right  
            GridPos::new(24, 8),  // Turn down slightly
            GridPos::new(31, 8),  // End right edge
        ];
        
        // Test validation with dense grid dimensions
        let is_valid = validate_strategic_path_requirements(&strategic_path, grid.width, grid.height);
        assert!(is_valid, "Strategic path should be valid for dense grid");
        
        // Test invalid path (not enough turns)
        let simple_path = vec![
            GridPos::new(0, 9),
            GridPos::new(15, 9),
            GridPos::new(31, 9),
        ];
        
        let is_simple_valid = validate_strategic_path_requirements(&simple_path, grid.width, grid.height);
        assert!(!is_simple_valid, "Simple path should be invalid (not enough turns)");
    }

    #[test]
    fn test_edge_avoidance_with_dense_grid() {
        let grid = PathGrid::new_unified();
        
        // Test path that touches edges (should be invalid)
        let edge_path = vec![
            GridPos::new(0, 9),   // Start (valid)
            GridPos::new(5, 0),   // Bottom edge (invalid)
            GridPos::new(15, 0),  // Still on bottom edge
            GridPos::new(25, 5),  // Move up
            GridPos::new(31, 9),  // End (valid)
        ];
        
        let is_edge_valid = validate_strategic_path_requirements(&edge_path, grid.width, grid.height);
        assert!(!is_edge_valid, "Path touching edges should be invalid");
        
        // Test path avoiding edges (should be valid)
        // Requirements: start/end Y in range 6-12, no intermediate points on edges
        let safe_path = vec![
            GridPos::new(0, 8),   // Start left edge, Y in valid range
            GridPos::new(5, 8),   // Safe distance from edges  
            GridPos::new(5, 11),  // Turn up, staying away from top edge (17)
            GridPos::new(15, 11), // Move right
            GridPos::new(15, 5),  // Turn down, staying away from bottom edge (0)  
            GridPos::new(25, 5),  // Move right
            GridPos::new(25, 9),  // Turn up
            GridPos::new(31, 9),  // End right edge, Y in valid range
        ];
        
        let is_safe_valid = validate_strategic_path_requirements(&safe_path, grid.width, grid.height);
        assert!(is_safe_valid, "Path avoiding edges should be valid");
    }

    #[test]
    fn test_path_length_constraints_dense_grid() {
        let grid = PathGrid::new_unified();
        
        // With dense grid (32x18), minimum path would be 31 steps straight across
        // Strategic paths should be longer due to turns
        
        // Test very short path (should be invalid)
        let short_path = vec![
            GridPos::new(0, 9),
            GridPos::new(31, 9),
        ];
        
        let is_short_valid = validate_strategic_path_requirements(&short_path, grid.width, grid.height);
        assert!(!is_short_valid, "Very short path should be invalid");
        
        // Test reasonable length path with turns
        // Requirements: start Y 6-12, end Y 6-12, avoid edges, 3-5 turns
        let good_path = vec![
            GridPos::new(0, 8),   // Start, Y in valid range
            GridPos::new(6, 8),   // Move right, away from edges
            GridPos::new(6, 11),  // Turn up, staying in bounds
            GridPos::new(14, 11), // Move right
            GridPos::new(14, 5),  // Turn down, staying away from edges
            GridPos::new(22, 5),  // Move right
            GridPos::new(22, 9),  // Turn up
            GridPos::new(31, 9),  // End right edge, Y in valid range
        ];
        
        let is_good_valid = validate_strategic_path_requirements(&good_path, grid.width, grid.height);
        assert!(is_good_valid, "Good length path with turns should be valid");
    }

    #[test]
    fn test_world_to_grid_coordinate_conversion() {
        let grid = PathGrid::new_unified();
        
        // Test world coordinate (0, 0) maps to center grid cell
        let center_world = Vec2::new(0.0, 0.0);
        let center_grid = grid.world_to_grid(center_world);
        
        assert!(center_grid.is_some(), "Center world should map to valid grid");
        if let Some(grid_pos) = center_grid {
            // Should map to center of grid (16, 9)
            assert_eq!(grid_pos.x, 16, "Center world X should map to grid X 16");
            assert_eq!(grid_pos.y, 9, "Center world Y should map to grid Y 9");
        }
        
        // Test corner coordinate
        let corner_world = Vec2::new(-620.0, -340.0); // Bottom-left area
        let corner_grid = grid.world_to_grid(corner_world);
        
        assert!(corner_grid.is_some(), "Corner world should map to valid grid");
        if let Some(grid_pos) = corner_grid {
            assert!(grid_pos.x < grid.width, "Corner X should be within bounds");
            assert!(grid_pos.y < grid.height, "Corner Y should be within bounds");
        }
    }
}