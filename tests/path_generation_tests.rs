use tower_defense_bevy::systems::path_generation::*;
use bevy::prelude::Vec2;

#[test]
fn test_grid_creation() {
    let grid = PathGrid::new(20, 12);
    
    assert_eq!(grid.width, 20);
    assert_eq!(grid.height, 12);
    assert_eq!(grid.cell_size, 40.0);
    assert_eq!(grid.cells.len(), 12); // Height
    assert_eq!(grid.cells[0].len(), 20); // Width
    
    // Check that all cells start as empty
    for row in &grid.cells {
        for &cell in row {
            assert_eq!(cell, CellType::Empty);
        }
    }
}

#[test]  
fn test_grid_coordinate_conversion() {
    let grid = PathGrid::new(20, 12);
    
    // Test center position
    let center_grid = GridPos::new(10, 6);
    let center_world = grid.grid_to_world(center_grid);
    
    // For a 20x12 grid with 40.0 cell size, center should be at (20, 20)
    assert!((center_world.x - 20.0).abs() < 1.0, "Center X should be near 20, got {}", center_world.x);
    assert!((center_world.y - 20.0).abs() < 1.0, "Center Y should be near 20, got {}", center_world.y);
    
    // Test round-trip conversion
    if let Some(converted_back) = grid.world_to_grid(center_world) {
        assert_eq!(converted_back, center_grid);
    } else {
        panic!("World to grid conversion failed for center position");
    }
}

#[test]
fn test_grid_bounds_checking() {
    let mut grid = PathGrid::new(20, 12);
    
    // Valid position
    assert!(grid.set_cell(GridPos::new(10, 6), CellType::Blocked));
    assert_eq!(grid.get_cell(GridPos::new(10, 6)), Some(CellType::Blocked));
    
    // Invalid positions
    assert!(!grid.set_cell(GridPos::new(20, 6), CellType::Blocked)); // X out of bounds
    assert!(!grid.set_cell(GridPos::new(10, 12), CellType::Blocked)); // Y out of bounds
    assert_eq!(grid.get_cell(GridPos::new(20, 6)), None);
    assert_eq!(grid.get_cell(GridPos::new(10, 12)), None);
}

#[test]
fn test_grid_pos_neighbors() {
    let pos = GridPos::new(5, 5);
    let neighbors = pos.neighbors(20, 12);
    
    assert_eq!(neighbors.len(), 4); // Should have 4 neighbors (no diagonals)
    
    let expected_neighbors = vec![
        GridPos::new(5, 4), // North
        GridPos::new(5, 6), // South  
        GridPos::new(4, 5), // West
        GridPos::new(6, 5), // East
    ];
    
    for expected in expected_neighbors {
        assert!(neighbors.contains(&expected), "Missing neighbor {:?}", expected);
    }
}

#[test]
fn test_grid_pos_neighbors_at_edges() {
    // Test corner position
    let corner = GridPos::new(0, 0);
    let corner_neighbors = corner.neighbors(20, 12);
    assert_eq!(corner_neighbors.len(), 2); // Only South and East
    
    // Test edge position
    let edge = GridPos::new(0, 5);
    let edge_neighbors = edge.neighbors(20, 12);
    assert_eq!(edge_neighbors.len(), 3); // North, South, East (no West)
}

#[test]
fn test_manhattan_distance() {
    let pos1 = GridPos::new(0, 0);
    let pos2 = GridPos::new(3, 4);
    
    assert_eq!(pos1.manhattan_distance(&pos2), 7.0);
    assert_eq!(pos2.manhattan_distance(&pos1), 7.0); // Symmetric
    assert_eq!(pos1.manhattan_distance(&pos1), 0.0); // Self distance
}

#[test]
fn test_is_traversable() {
    let mut grid = PathGrid::new(20, 12);
    
    let pos = GridPos::new(10, 6);
    
    // Empty cell should be traversable
    assert!(grid.is_traversable(pos));
    
    // Path cell should be traversable
    grid.set_cell(pos, CellType::Path);
    assert!(grid.is_traversable(pos));
    
    // Blocked cell should not be traversable
    grid.set_cell(pos, CellType::Blocked);
    assert!(!grid.is_traversable(pos));
    
    // Tower zone should not be traversable
    grid.set_cell(pos, CellType::TowerZone);
    assert!(!grid.is_traversable(pos));
}

#[test]
fn test_pathfinding_simple_case() {
    let grid = PathGrid::new(20, 12);
    let start = GridPos::new(0, 5);
    let goal = GridPos::new(5, 5);
    
    let path = find_path(&grid, start, goal);
    
    assert!(path.is_some(), "Should find path in empty grid");
    let path = path.unwrap();
    
    assert_eq!(path[0], start);
    assert_eq!(path[path.len() - 1], goal);
    assert!(path.len() >= 6, "Path should be at least Manhattan distance (6 steps)");
}

#[test]
fn test_pathfinding_with_obstacles() {
    let mut grid = PathGrid::new(20, 12);
    
    // Create a wall blocking direct path
    for y in 2..10 {
        grid.set_cell(GridPos::new(3, y), CellType::Blocked);
    }
    
    let start = GridPos::new(0, 5);
    let goal = GridPos::new(6, 5);
    
    let path = find_path(&grid, start, goal);
    
    assert!(path.is_some(), "Should find path around obstacles");
    let path = path.unwrap();
    
    assert_eq!(path[0], start);
    assert_eq!(path[path.len() - 1], goal);
    
    // Path should not go through blocked cells
    for &pos in &path {
        assert!(grid.is_traversable(pos), "Path should not go through blocked cells");
    }
}

#[test]
fn test_pathfinding_no_path() {
    let mut grid = PathGrid::new(20, 12);
    
    // Create complete wall blocking any path
    for y in 0..12 {
        grid.set_cell(GridPos::new(10, y), CellType::Blocked);
    }
    
    let start = GridPos::new(5, 5);
    let goal = GridPos::new(15, 5);
    
    let path = find_path(&grid, start, goal);
    assert!(path.is_none(), "Should not find path when completely blocked");
}

#[test]
fn test_path_quality_validation() {
    // Valid path
    let valid_path = vec![
        GridPos::new(0, 0),
        GridPos::new(1, 0),
        GridPos::new(2, 0),
        GridPos::new(2, 1),
        GridPos::new(2, 2),
        GridPos::new(3, 2),
    ];
    
    assert!(validate_path_quality(&valid_path, 4, 10));
    
    // Too short path
    let short_path = vec![
        GridPos::new(0, 0),
        GridPos::new(1, 0),
    ];
    
    assert!(!validate_path_quality(&short_path, 4, 10));
    
    // Path with loop
    let loop_path = vec![
        GridPos::new(0, 0),
        GridPos::new(1, 0),
        GridPos::new(1, 1),
        GridPos::new(0, 1),
        GridPos::new(0, 0), // Back to start - loop
    ];
    
    assert!(!validate_path_quality(&loop_path, 4, 10));
}

#[test]
fn test_enemy_path_conversion() {
    let grid = PathGrid::new(20, 12);
    let grid_path = vec![
        GridPos::new(0, 6),
        GridPos::new(5, 6),
        GridPos::new(10, 6),
    ];
    
    let enemy_path = grid.to_enemy_path(grid_path.clone());
    
    assert_eq!(enemy_path.waypoints.len(), grid_path.len());
    
    // Check that waypoints are properly converted
    for (i, &grid_pos) in grid_path.iter().enumerate() {
        let expected_world = grid.grid_to_world(grid_pos);
        let actual_world = enemy_path.waypoints[i];
        
        assert!((expected_world.x - actual_world.x).abs() < 0.1, "X coordinate mismatch");
        assert!((expected_world.y - actual_world.y).abs() < 0.1, "Y coordinate mismatch");
    }
}

#[test]
fn test_procedural_map_generation() {
    let grid = generate_procedural_map(12345, 0.3);
    
    // Basic validation - updated for dense unified grid dimensions
    assert_eq!(grid.width, 32);
    assert_eq!(grid.height, 18);
    
    // Entry and exit should be set
    assert!(grid.entry_point.x < grid.width);
    assert!(grid.entry_point.y < grid.height);
    assert!(grid.exit_point.x < grid.width);
    assert!(grid.exit_point.y < grid.height);
    
    // Should be able to find a path
    let path = find_path(&grid, grid.entry_point, grid.exit_point);
    assert!(path.is_some(), "Generated map should always have a valid path");
    
    // Should have some obstacles for interesting gameplay
    let mut obstacle_count = 0;
    for row in &grid.cells {
        for &cell in row {
            if cell == CellType::Blocked {
                obstacle_count += 1;
            }
        }
    }
    
    assert!(obstacle_count > 0, "Generated map should have some obstacles");
    assert!(obstacle_count < grid.width * grid.height / 2, "Map should not be mostly obstacles");
}

#[test]
fn test_integration_with_existing_system() {
    // Test that generated paths work with existing enemy movement system
    let enemy_path = generate_level_path(1);
    
    assert!(enemy_path.waypoints.len() >= 2, "Path should have at least start and end");
    
    // Test path properties
    let total_length = enemy_path.total_length();
    assert!(total_length > 0.0, "Path should have positive length");
    
    // Test position calculation
    let start_pos = enemy_path.get_position_at_progress(0.0);
    let end_pos = enemy_path.get_position_at_progress(1.0);
    let mid_pos = enemy_path.get_position_at_progress(0.5);
    
    assert_eq!(start_pos, enemy_path.waypoints[0]);
    assert_eq!(end_pos, enemy_path.waypoints[enemy_path.waypoints.len() - 1]);
    
    // Mid position should be different from start/end for non-trivial paths
    assert!(start_pos.distance(mid_pos) > 0.0);
    assert!(end_pos.distance(mid_pos) > 0.0);
}

#[test]
fn test_deterministic_generation() {
    // Same seed should produce same path
    let path1 = generate_level_path(1);
    let path2 = generate_level_path(1);
    
    assert_eq!(path1.waypoints.len(), path2.waypoints.len());
    
    for (i, (wp1, wp2)) in path1.waypoints.iter().zip(path2.waypoints.iter()).enumerate() {
        assert!((wp1.x - wp2.x).abs() < 0.1, "Waypoint {} X mismatch: {} vs {}", i, wp1.x, wp2.x);
        assert!((wp1.y - wp2.y).abs() < 0.1, "Waypoint {} Y mismatch: {} vs {}", i, wp1.y, wp2.y);
    }
}

#[test]
fn test_difficulty_scaling() {
    let easy_path = generate_level_path(1);   // Low difficulty
    let hard_path = generate_level_path(10);  // Higher difficulty
    
    // Both should be valid paths
    assert!(easy_path.waypoints.len() >= 2);
    assert!(hard_path.waypoints.len() >= 2);
    
    // Paths should be different (different difficulty/seed)
    assert_ne!(easy_path.waypoints, hard_path.waypoints);
}