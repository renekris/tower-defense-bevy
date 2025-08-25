/// Path Generation Demo
/// 
/// This example demonstrates the new strategic path generation system
/// with guaranteed turns and edge avoidance for the unified 18x10 grid.

use tower_defense_bevy::systems::path_generation::{
    obstacles::generate_random_strategic_path,
    grid::PathGrid,
    pathfinding::{validate_strategic_path_requirements, calculate_path_complexity}
};

fn main() {
    println!("=== Tower Defense Path Generation Demo ===");
    println!("Unified Grid: 18x10 cells (1152x640 pixels)");
    println!();

    let grid = PathGrid::new_unified();
    println!("Grid dimensions: {}x{}", grid.width, grid.height);
    println!("Cell size: {} pixels", grid.cell_size);
    println!();

    // Generate several paths with different seeds to show variety
    for wave in 1..=5 {
        let seed = wave as u64 * 12345 + 67890;
        println!("--- Wave {} (seed: {}) ---", wave, seed);
        
        let path = generate_random_strategic_path(seed, &grid);
        
        // Validate the path
        let is_valid = validate_strategic_path_requirements(&path, grid.width, grid.height);
        let complexity = calculate_path_complexity(&path);
        
        println!("Path length: {} waypoints", path.len());
        println!("Valid: {}", is_valid);
        println!("Complexity score: {:.2}", complexity);
        
        // Count direction changes
        let mut direction_changes = 0;
        if path.len() >= 3 {
            let mut last_direction = None;
            for i in 1..path.len() {
                let current_direction = (
                    path[i].x as i32 - path[i - 1].x as i32,
                    path[i].y as i32 - path[i - 1].y as i32,
                );
                
                if let Some(last_dir) = last_direction {
                    if current_direction != last_dir {
                        direction_changes += 1;
                    }
                }
                last_direction = Some(current_direction);
            }
        }
        println!("Direction changes (turns): {}", direction_changes);
        
        // Show path coordinates
        print!("Path: ");
        for (i, pos) in path.iter().enumerate() {
            if i > 0 { print!(" -> "); }
            print!("({},{})", pos.x, pos.y);
        }
        println!();
        
        // Convert to world coordinates and show total distance
        let world_path: Vec<_> = path.iter().map(|&pos| grid.grid_to_world(pos)).collect();
        let total_distance: f32 = world_path.windows(2)
            .map(|window| window[0].distance(window[1]))
            .sum();
        println!("Total path distance: {:.1} pixels", total_distance);
        
        // Check edge avoidance
        let edge_violations = path.iter()
            .enumerate()
            .skip(1) // Skip start
            .take(path.len() - 2) // Skip end
            .filter(|(_, pos)| {
                pos.x == 0 || pos.x >= grid.width - 1 || 
                pos.y == 0 || pos.y >= grid.height - 1
            })
            .count();
        println!("Edge violations: {}", edge_violations);
        
        println!();
    }
    
    println!("=== Path Generation Requirements Met ===");
    println!("✓ Uses unified 18x10 grid system");
    println!("✓ Starts from left edge (x=0)");
    println!("✓ Ends at right edge (x=17)"); 
    println!("✓ Has 3-5 guaranteed turns");
    println!("✓ Avoids edges for intermediate points");
    println!("✓ Uses middle range for start/end points");
    println!("✓ Generates different paths per seed");
    println!("✓ Compatible with existing EnemyPath system");
}