/// Debug test to understand path validation requirements
use tower_defense_bevy::systems::path_generation::grid::{PathGrid, GridPos};
use tower_defense_bevy::systems::path_generation::pathfinding::validate_strategic_path_requirements;

#[cfg(test)]
mod validation_debug {
    use super::*;

    #[test]
    fn debug_path_validation() {
        let grid = PathGrid::new_unified();
        println!("Grid dimensions: {}x{}", grid.width, grid.height);
        
        // Create a step-by-step path with small jumps
        let test_path = vec![
            GridPos::new(0, 8),   // Start: Y=8 (in range 6-12)
            GridPos::new(1, 8),   // Step right
            GridPos::new(2, 8),   // Step right
            GridPos::new(3, 8),   // Step right
            GridPos::new(4, 8),   // Step right
            GridPos::new(5, 8),   // Step right
            GridPos::new(5, 9),   // Turn up (direction change 1)
            GridPos::new(5, 10),  // Step up
            GridPos::new(5, 11),  // Step up
            GridPos::new(6, 11),  // Turn right (direction change 2)
            GridPos::new(7, 11),  // Step right
            GridPos::new(8, 11),  // Step right
            GridPos::new(9, 11),  // Step right
            GridPos::new(10, 11), // Step right
            GridPos::new(10, 10), // Turn down (direction change 3)
            GridPos::new(10, 9),  // Step down
            GridPos::new(11, 9),  // Turn right (direction change 4)
            GridPos::new(12, 9),  // Step right
            GridPos::new(13, 9),  // Step right
            GridPos::new(14, 9),  // Step right
            GridPos::new(15, 9),  // Step right
            GridPos::new(16, 9),  // Step right
            GridPos::new(17, 9),  // Step right
            GridPos::new(18, 9),  // Step right
            GridPos::new(19, 9),  // Step right
            GridPos::new(20, 9),  // Step right
            GridPos::new(21, 9),  // Step right
            GridPos::new(22, 9),  // Step right
            GridPos::new(23, 9),  // Step right
            GridPos::new(24, 9),  // Step right
            GridPos::new(25, 9),  // Step right
            GridPos::new(26, 9),  // Step right
            GridPos::new(27, 9),  // Step right
            GridPos::new(28, 9),  // Step right
            GridPos::new(29, 9),  // Step right
            GridPos::new(30, 9),  // Step right
            GridPos::new(31, 9),  // End: Y=9 (in range 6-12)
        ];
        
        println!("Path length: {}", test_path.len());
        println!("Start: ({}, {}) End: ({}, {})", 
                 test_path[0].x, test_path[0].y, 
                 test_path.last().unwrap().x, test_path.last().unwrap().y);
        
        // Check each validation requirement individually
        
        // 1. Check start/end positions
        let start = test_path.first().unwrap();
        let end = test_path.last().unwrap();
        println!("Start edge check: x={} (should be 0), End edge check: x={} (should be {})", 
                 start.x, end.x, grid.width - 1);
        
        // 2. Check Y range
        let middle_range = 6..=12;
        println!("Start Y range: {} in {:?}: {}", start.y, middle_range, middle_range.contains(&start.y));
        println!("End Y range: {} in {:?}: {}", end.y, middle_range, middle_range.contains(&end.y));
        
        // 3. Check edge avoidance
        let mut edge_violations = Vec::new();
        for (i, &pos) in test_path.iter().enumerate() {
            if i == 0 || i == test_path.len() - 1 {
                continue; // Skip start and end
            }
            
            if pos.x == 0 || pos.x >= grid.width - 1 || pos.y == 0 || pos.y >= grid.height - 1 {
                edge_violations.push((i, pos));
            }
        }
        println!("Edge violations: {:?}", edge_violations);
        
        // 4. Check large jumps
        let mut large_jumps = Vec::new();
        for i in 0..test_path.len() - 1 {
            let dist = test_path[i].manhattan_distance(&test_path[i + 1]);
            if dist > 6.0 {
                large_jumps.push((i, dist));
            }
        }
        println!("Large jumps: {:?}", large_jumps);
        
        // 5. Count direction changes
        let mut direction_changes = 0;
        let mut last_direction = None;
        
        for i in 1..test_path.len() {
            let current_direction = (
                test_path[i].x as i32 - test_path[i-1].x as i32,
                test_path[i].y as i32 - test_path[i-1].y as i32,
            );
            
            if let Some(last_dir) = last_direction {
                if current_direction != last_dir {
                    direction_changes += 1;
                    println!("Direction change {} at step {}: {:?} -> {:?}", 
                             direction_changes, i, last_dir, current_direction);
                }
            }
            last_direction = Some(current_direction);
        }
        println!("Total direction changes: {} (should be 3-5)", direction_changes);
        
        // Final validation
        let is_valid = validate_strategic_path_requirements(&test_path, grid.width, grid.height);
        println!("Final validation result: {}", is_valid);
        
        assert!(is_valid, "Test path should be valid");
    }
}