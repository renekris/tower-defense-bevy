use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use bevy::prelude::*;
use super::grid::{PathGrid, GridPos, CellType};
use super::pathfinding::find_path;

/// Represents the four sides of the grid for start/end point placement
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GridSide {
    Top,    // y = 0
    Bottom, // y = height - 1
    Left,   // x = 0
    Right,  // x = width - 1
}

/// Obstacle component for visual rendering
#[derive(Component, Debug, Clone)]
pub struct Obstacle {
    pub position: GridPos,
    pub obstacle_type: ObstacleType,
}

/// Types of obstacles with different visual styles
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObstacleType {
    Rock,      // Large impassable terrain features
    Building,  // Structural obstacles with square appearance
    Debris,    // Small scattered obstacles
    Crystal,   // Special decorative obstacles
}

/// Generate random start and end points on opposite sides of the grid
/// Ensures start and end are on different sides for interesting paths
/// 
/// # Arguments
/// * `rng` - Random number generator
/// * `width` - Grid width
/// * `height` - Grid height
/// 
/// # Returns
/// * `(GridPos, GridPos)` - Entry point and exit point on opposite sides
fn generate_random_opposite_points(rng: &mut StdRng, width: usize, height: usize) -> (GridPos, GridPos) {
    let sides = [GridSide::Top, GridSide::Bottom, GridSide::Left, GridSide::Right];
    
    // Choose random starting side
    let start_side = sides[rng.random_range(0..4)];
    
    // Choose opposite side for end point
    let end_side = match start_side {
        GridSide::Top => GridSide::Bottom,
        GridSide::Bottom => GridSide::Top,
        GridSide::Left => GridSide::Right,
        GridSide::Right => GridSide::Left,
    };
    
    // Generate random positions along the chosen sides
    let entry_point = generate_point_on_side(rng, start_side, width, height);
    let exit_point = generate_point_on_side(rng, end_side, width, height);
    
    (entry_point, exit_point)
}

/// Generate a random point along a specific side of the grid
/// Avoids corners to ensure better pathfinding and tower placement opportunities
/// 
/// # Arguments
/// * `rng` - Random number generator
/// * `side` - Which side of the grid to place the point
/// * `width` - Grid width
/// * `height` - Grid height
/// 
/// # Returns
/// * `GridPos` - Random position along the specified side
fn generate_point_on_side(rng: &mut StdRng, side: GridSide, width: usize, height: usize) -> GridPos {
    // Define margins to avoid placing points too close to corners
    let margin_horizontal = (width / 6).max(2).min(4); // About 16% margin, min 2, max 4
    let margin_vertical = (height / 4).max(2).min(3); // About 25% margin, min 2, max 3
    
    match side {
        GridSide::Top => {
            let x = rng.random_range(margin_horizontal..(width - margin_horizontal));
            GridPos::new(x, 0)
        }
        GridSide::Bottom => {
            let x = rng.random_range(margin_horizontal..(width - margin_horizontal));
            GridPos::new(x, height - 1)
        }
        GridSide::Left => {
            let y = rng.random_range(margin_vertical..(height - margin_vertical));
            GridPos::new(0, y)
        }
        GridSide::Right => {
            let y = rng.random_range(margin_vertical..(height - margin_vertical));
            GridPos::new(width - 1, y)
        }
    }
}

/// Generate a procedural map with obstacles and strategic layout
/// Enhanced with A* pathfinding validation and 2x path length requirement
/// 
/// # Arguments
/// * `seed` - Random seed for reproducible generation
/// * `difficulty` - Difficulty factor (0.0 = easy, 1.0 = hard)
/// 
/// # Returns
/// * `PathGrid` - Generated map with obstacles and entry/exit points
pub fn generate_procedural_map(seed: u64, difficulty: f32) -> PathGrid {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut grid = PathGrid::new_unified(); // Use dense unified 32x18 grid
    
    // Set entry and exit points (avoid corners) - adjusted for 32x18 grid
    grid.entry_point = GridPos::new(0, rng.random_range(6..12)); // More centered
    grid.exit_point = GridPos::new(31, rng.random_range(6..12)); // More centered
    
    // Place strategic obstacles based on difficulty with path length validation
    let obstacle_density = (difficulty * 0.2).min(0.15); // Reduced to ensure paths exist
    place_strategic_obstacles_with_validation(&mut grid, &mut rng, obstacle_density);
    
    // Validate path exists and meets 2x length requirement
    let mut attempts = 0;
    let max_attempts = 20;
    
    loop {
        if let Some(path) = find_path(&grid, grid.entry_point, grid.exit_point) {
            if validate_path_length_requirement(&path, &grid) {
                // Path exists and meets length requirement
                break;
            }
        }
        
        attempts += 1;
        if attempts >= max_attempts {
            // Fallback: create simpler obstacle layout
            create_fallback_obstacle_layout(&mut grid, &mut rng);
            break;
        }
        
        // Adjust obstacle placement and try again
        if attempts < max_attempts / 2 {
            reduce_obstacles(&mut grid, &mut rng, 0.1);
        } else {
            // More aggressive reduction
            reduce_obstacles(&mut grid, &mut rng, 0.2);
        }
    }
    
    grid
}

/// Generate a procedural map with random start/end positions on opposite sides
/// Enhanced with time-based variety, A* pathfinding validation and 2x path length requirement
/// 
/// # Arguments
/// * `seed` - Random seed for reproducible generation
/// * `difficulty` - Difficulty factor (0.0 = easy, 1.0 = hard)
/// 
/// # Returns
/// * `PathGrid` - Generated map with obstacles and randomized entry/exit points
pub fn generate_procedural_map_with_random_sides(seed: u64, difficulty: f32) -> PathGrid {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut grid = PathGrid::new_unified(); // Use dense unified 32x18 grid
    
    // Generate random start and end positions on opposite sides
    let (entry_point, exit_point) = generate_random_opposite_points(&mut rng, grid.width, grid.height);
    grid.entry_point = entry_point;
    grid.exit_point = exit_point;
    
    // Place strategic obstacles based on difficulty with path length validation
    let obstacle_density = (difficulty * 0.2).min(0.15); // Reduced to ensure paths exist
    place_strategic_obstacles_with_validation(&mut grid, &mut rng, obstacle_density);
    
    // Validate path exists and meets 2x length requirement
    let mut attempts = 0;
    let max_attempts = 20;
    
    loop {
        if let Some(path) = find_path(&grid, grid.entry_point, grid.exit_point) {
            if validate_path_length_requirement(&path, &grid) {
                // Path exists and meets length requirement
                break;
            }
        }
        
        attempts += 1;
        if attempts >= max_attempts {
            // Fallback: create simpler obstacle layout
            create_fallback_obstacle_layout(&mut grid, &mut rng);
            break;
        }
        
        // Adjust obstacle placement and try again
        if attempts < max_attempts / 2 {
            reduce_obstacles(&mut grid, &mut rng, 0.1);
        } else {
            // More aggressive reduction
            reduce_obstacles(&mut grid, &mut rng, 0.2);
        }
    }
    
    grid
}

/// Generate a procedural map with custom obstacle density
/// 
/// # Arguments
/// * `seed` - Random seed for reproducible generation
/// * `obstacle_density` - Direct obstacle density override (0.0-0.5)
/// 
/// # Returns
/// * `PathGrid` - Generated map with obstacles and entry/exit points
pub fn generate_procedural_map_with_density(seed: u64, obstacle_density: f32) -> PathGrid {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut grid = PathGrid::new_unified(); // Use dense unified 32x18 grid
    
    // Set entry and exit points (avoid corners) - adjusted for 32x18 grid
    grid.entry_point = GridPos::new(0, rng.random_range(4..14));
    grid.exit_point = GridPos::new(31, rng.random_range(4..14));
    
    // Place strategic obstacles with custom density
    let clamped_density = obstacle_density.clamp(0.0, 0.5); // Max 50% coverage
    place_strategic_obstacles_with_validation(&mut grid, &mut rng, clamped_density);
    
    // Ensure path exists - if not, reduce obstacles and try again
    let mut attempts = 0;
    while find_path(&grid, grid.entry_point, grid.exit_point).is_none() && attempts < 10 {
        reduce_obstacles(&mut grid, &mut rng, 0.1);
        attempts += 1;
    }
    
    grid
}

/// Place obstacles strategically to create interesting chokepoints with A* validation
fn place_strategic_obstacles_with_validation(grid: &mut PathGrid, rng: &mut StdRng, density: f32) {
    let total_cells = grid.width * grid.height;
    let target_obstacles = (total_cells as f32 * density) as usize;
    
    let mut placed = 0;
    let mut placement_attempts = 0;
    let max_placement_attempts = target_obstacles * 3; // Allow multiple attempts per obstacle
    
    // Strategy 1: Create strategic obstacle clusters
    while placed < target_obstacles && placement_attempts < max_placement_attempts {
        placement_attempts += 1;
        
        // Create clusters in strategic areas
        let cluster_center = if placed < target_obstacles / 3 {
            // Early obstacles: central clusters for chokepoints
            GridPos::new(
                rng.random_range(10..22),  // Central region
                rng.random_range(6..12),   // Middle vertical area
            )
        } else if placed < (2 * target_obstacles) / 3 {
            // Mid obstacles: side clusters for path variety
            GridPos::new(
                rng.random_range(6..26),   // Wider range
                rng.random_range(3..15),   // Wider vertical range
            )
        } else {
            // Late obstacles: scattered for fine-tuning
            GridPos::new(
                rng.random_range(4..28),   // Near-full range
                rng.random_range(2..16),   // Near-full vertical
            )
        };
        
        // Try to place obstacle cluster
        let old_grid = grid.clone();
        if place_strategic_obstacle_cluster(grid, rng, cluster_center, 1, 2) {
            // Validate that path still exists after placing obstacles
            if find_path(grid, grid.entry_point, grid.exit_point).is_some() {
                placed += 1;
            } else {
                // Revert if path blocked
                *grid = old_grid;
            }
        }
    }
}

/// Enhanced obstacle cluster placement with strategic positioning
fn place_strategic_obstacle_cluster(grid: &mut PathGrid, rng: &mut StdRng, center: GridPos, min_size: usize, max_size: usize) -> bool {
    let cluster_size = rng.random_range(min_size..=max_size);
    let mut positions = Vec::new();
    
    // Generate strategic cluster patterns
    for i in 0..cluster_size {
        let pattern = rng.random_range(0..3);
        let (offset_x, offset_y) = match pattern {
            0 => {
                // Linear pattern (creates corridors)
                let direction = rng.random_range(0..4);
                match direction {
                    0 => (i as i32, 0),           // Horizontal
                    1 => (0, i as i32),           // Vertical
                    2 => (i as i32, i as i32),    // Diagonal
                    _ => (-(i as i32), i as i32), // Anti-diagonal
                }
            },
            1 => {
                // Compact cluster (creates chokepoints)
                let offset = rng.random_range(-1..=1);
                (offset, rng.random_range(-1..=1))
            },
            _ => {
                // Random scatter
                (rng.random_range(-2..=2), rng.random_range(-2..=2))
            }
        };
        
        let x = (center.x as i32 + offset_x).max(1).min(grid.width as i32 - 2) as usize;
        let y = (center.y as i32 + offset_y).max(1).min(grid.height as i32 - 2) as usize;
        
        positions.push(GridPos::new(x, y));
    }
    
    // Place obstacles if cells are empty
    let mut placed_any = false;
    for pos in positions {
        if grid.get_cell(pos) == Some(CellType::Empty) {
            grid.set_cell(pos, CellType::Blocked);
            placed_any = true;
        }
    }
    
    placed_any
}

/// Legacy obstacle cluster placement (kept for compatibility)
fn place_obstacle_cluster(grid: &mut PathGrid, rng: &mut StdRng, center: GridPos, min_size: usize, max_size: usize) -> bool {
    place_strategic_obstacle_cluster(grid, rng, center, min_size, max_size)
}

/// Reduce obstacle density by randomly removing some obstacles
fn reduce_obstacles(grid: &mut PathGrid, rng: &mut StdRng, reduction_factor: f32) {
    for y in 0..grid.height {
        for x in 0..grid.width {
            let pos = GridPos::new(x, y);
            if grid.get_cell(pos) == Some(CellType::Blocked) && rng.random::<f32>() < reduction_factor {
                grid.set_cell(pos, CellType::Empty);
            }
        }
    }
}

/// Create chokepoints by identifying narrow passages and enhancing them
pub fn enhance_chokepoints(grid: &mut PathGrid, path: &[GridPos]) {
    for &pos in path {
        let empty_neighbors = grid.count_empty_neighbors(pos);
        
        // If this path position has few empty neighbors, it's naturally a chokepoint
        if empty_neighbors <= 2 {
            // Mark surrounding area as having strategic value
            mark_strategic_area(grid, pos);
        }
    }
}

/// Mark an area around a chokepoint as strategically valuable
fn mark_strategic_area(_grid: &mut PathGrid, _center: GridPos) {
    // This could be used for tower zone optimization
    // For now, just a placeholder that could store strategic values
    let _strategic_radius = 2;
    
    // Future implementation: store strategic values for tower zone calculation
    // This would help optimize placement zones around important chokepoints
}

/// Calculate the strategic value of a position based on nearby obstacles and chokepoints
pub fn calculate_strategic_value(grid: &PathGrid, pos: GridPos, path: &[GridPos]) -> f32 {
    let mut value = 0.0;
    
    // Higher value for positions near the path
    let min_path_distance = path.iter()
        .map(|&path_pos| pos.manhattan_distance(&path_pos))
        .fold(f32::INFINITY, f32::min);
    
    if min_path_distance <= 3.0 {
        value += (4.0 - min_path_distance) * 0.25; // Up to 1.0 points for being close to path
    }
    
    // Higher value for positions near chokepoints  
    let empty_neighbors = grid.count_empty_neighbors(pos);
    if empty_neighbors <= 4 {
        value += (4 - empty_neighbors) as f32 * 0.1; // Up to 0.4 points for being in tight areas
    }
    
    // Higher value for positions with good coverage of the path
    let path_coverage = calculate_path_coverage(grid, pos, path);
    value += path_coverage * 0.5; // Up to 0.5 points for good path coverage
    
    value.min(2.0) // Cap at 2.0 maximum strategic value
}

/// Calculate how much of the path this position can "see" or cover
fn calculate_path_coverage(_grid: &PathGrid, pos: GridPos, path: &[GridPos]) -> f32 {
    let max_range = 4.0; // Typical tower range in grid cells
    
    let covered_path_segments = path.iter()
        .filter(|&&path_pos| pos.manhattan_distance(&path_pos) <= max_range)
        .count();
    
    (covered_path_segments as f32 / path.len() as f32).min(1.0)
}

/// Generate a random path with strategic obstacles and A* pathfinding
/// Enhanced to use A* pathfinding around obstacles with 2x length requirement
/// 
/// # Arguments
/// * `seed` - Random seed for reproducible generation
/// * `grid` - The grid with obstacles already placed
/// 
/// # Returns
/// * `Vec<GridPos>` - A* calculated path around obstacles meeting length requirement
pub fn generate_random_strategic_path(seed: u64, grid: &PathGrid) -> Vec<GridPos> {
    let mut rng = StdRng::seed_from_u64(seed);
    
    // Ensure we're working with proper dense unified grid dimensions
    assert_eq!(grid.width, 32, "Expected dense unified grid width of 32");
    assert_eq!(grid.height, 18, "Expected dense unified grid height of 18");
    
    // First, try to find A* path with existing obstacles
    if let Some(path) = find_path(grid, grid.entry_point, grid.exit_point) {
        if validate_path_length_requirement(&path, grid) {
            return path;
        }
    }
    
    // If A* path doesn't meet length requirement, create strategic waypoint path
    let num_turns = rng.random_range(3..=5);
    
    // Generate strategic waypoints that force longer paths based on start/end positions
    let mut waypoints = vec![grid.entry_point];
    
    // Create adaptive waypoint strategy based on entry/exit relationship
    let waypoint_positions = generate_adaptive_waypoints(
        &mut rng, 
        grid.entry_point, 
        grid.exit_point, 
        num_turns, 
        grid.width, 
        grid.height
    );
    
    waypoints.extend(waypoint_positions);
    waypoints.push(grid.exit_point);
    
    // Connect waypoints with A* pathfinding
    let mut final_path = Vec::new();
    
    for i in 0..waypoints.len() - 1 {
        let start = waypoints[i];
        let end = waypoints[i + 1];
        
        if let Some(segment) = find_path(grid, start, end) {
            if i == 0 {
                final_path.extend(segment);
            } else {
                // Skip first point to avoid duplication
                final_path.extend(&segment[1..]);
            }
        } else {
            // Fallback: direct connection if A* fails
            if i > 0 {
                final_path.push(end);
            }
        }
    }
    
    // Validate final path
    if !final_path.is_empty() && validate_path_length_requirement(&final_path, grid) {
        final_path
    } else {
        // Ultimate fallback
        generate_fallback_path(grid.entry_point, grid.exit_point, grid)
    }
}

/// Validate that a strategic path meets all requirements
fn validate_strategic_path(path: &[GridPos], grid: &PathGrid) -> bool {
    if path.len() < 5 {  // Start + 3-5 turns + End = at least 5 points
        return false;
    }
    
    // Check edge avoidance (except for start/end points)
    for (i, &pos) in path.iter().enumerate() {
        if i == 0 || i == path.len() - 1 {
            continue; // Skip start and end points
        }
        
        if pos.x == 0 || pos.x >= grid.width - 1 || pos.y == 0 || pos.y >= grid.height - 1 {
            return false; // Too close to edges
        }
    }
    
    // Count actual direction changes
    let turn_count = count_direction_changes_in_path(path);
    if turn_count < 3 || turn_count > 5 {
        return false;
    }
    
    // Ensure path is traversable (basic connectivity check)
    for i in 0..path.len() - 1 {
        let dist = path[i].manhattan_distance(&path[i + 1]);
        if dist > 5.0 {  // No jumps too large
            return false;
        }
    }
    
    true
}

/// Count direction changes in a path for validation
fn count_direction_changes_in_path(path: &[GridPos]) -> usize {
    if path.len() < 3 {
        return 0;
    }
    
    let mut changes = 0;
    let mut last_direction: Option<(i32, i32)> = None;
    
    for i in 1..path.len() {
        let current_direction = (
            path[i].x as i32 - path[i - 1].x as i32,
            path[i].y as i32 - path[i - 1].y as i32,
        );
        
        if let Some(last_dir) = last_direction {
            if current_direction != last_dir {
                changes += 1;
            }
        }
        
        last_direction = Some(current_direction);
    }
    
    changes
}

/// Generate a fallback path if the main algorithm fails
/// Enhanced to work with variable start/end positions on any side
fn generate_fallback_path(start: GridPos, end: GridPos, grid: &PathGrid) -> Vec<GridPos> {
    // First try A* without waypoints
    if let Some(direct_path) = find_path(grid, start, end) {
        if direct_path.len() >= 8 { // Minimum reasonable path length
            return direct_path;
        }
    }
    
    // Create adaptive zigzag path based on start/end positions
    let mut path = vec![start];
    
    // Determine if we're moving more horizontally or vertically
    let dx = (end.x as i32 - start.x as i32).abs();
    let dy = (end.y as i32 - start.y as i32).abs();
    
    let waypoints = if dx > dy {
        // More horizontal movement - create vertical detours
        generate_horizontal_waypoints(start, end, grid)
    } else {
        // More vertical movement - create horizontal detours
        generate_vertical_waypoints(start, end, grid)
    };
    
    let mut current = start;
    for waypoint in waypoints {
        if let Some(segment) = find_path(grid, current, waypoint) {
            if path.len() == 1 {
                path.extend(segment);
            } else {
                path.extend(&segment[1..]);
            }
            current = waypoint;
        } else {
            // Direct connection as last resort
            path.push(waypoint);
            current = waypoint;
        }
    }
    
    // Ensure we end at the correct point
    if current != end {
        if let Some(final_segment) = find_path(grid, current, end) {
            path.extend(&final_segment[1..]);
        } else {
            path.push(end);
        }
    }
    
    path
}

/// Generate waypoints for primarily horizontal movement
fn generate_horizontal_waypoints(start: GridPos, end: GridPos, grid: &PathGrid) -> Vec<GridPos> {
    let mid_x = (start.x + end.x) / 2;
    let quarter_x = (start.x + mid_x) / 2;
    let three_quarter_x = (mid_x + end.x) / 2;
    
    // Create vertical detours to increase path length
    let detour_y1 = if start.y > grid.height / 2 { 3 } else { grid.height - 4 };
    let detour_y2 = if end.y > grid.height / 2 { 3 } else { grid.height - 4 };
    
    vec![
        GridPos::new(quarter_x, detour_y1),
        GridPos::new(mid_x, grid.height / 2),
        GridPos::new(three_quarter_x, detour_y2),
        end,
    ]
}

/// Generate waypoints for primarily vertical movement
fn generate_vertical_waypoints(start: GridPos, end: GridPos, grid: &PathGrid) -> Vec<GridPos> {
    let mid_y = (start.y + end.y) / 2;
    let quarter_y = (start.y + mid_y) / 2;
    let three_quarter_y = (mid_y + end.y) / 2;
    
    // Create horizontal detours to increase path length
    let detour_x1 = if start.x > grid.width / 2 { 3 } else { grid.width - 4 };
    let detour_x2 = if end.x > grid.width / 2 { 3 } else { grid.width - 4 };
    
    vec![
        GridPos::new(detour_x1, quarter_y),
        GridPos::new(grid.width / 2, mid_y),
        GridPos::new(detour_x2, three_quarter_y),
        end,
    ]
}

/// Generate adaptive waypoints based on start/end positions
/// Creates strategic paths that work regardless of which sides the points are on
fn generate_adaptive_waypoints(
    rng: &mut StdRng,
    start: GridPos,
    end: GridPos,
    num_turns: usize,
    width: usize,
    height: usize,
) -> Vec<GridPos> {
    let mut waypoints = Vec::new();
    
    // Determine the primary direction of travel
    let dx = end.x as i32 - start.x as i32;
    let dy = end.y as i32 - start.y as i32;
    
    // Create waypoints that force the path to take detours
    for i in 1..=num_turns {
        let progress = i as f32 / (num_turns + 1) as f32; // 0.0 to 1.0
        
        // Interpolate between start and end
        let base_x = start.x as f32 + dx as f32 * progress;
        let base_y = start.y as f32 + dy as f32 * progress;
        
        // Add strategic detours based on position along the path
        let (detour_x, detour_y) = if i % 2 == 1 {
            // Odd waypoints: detour perpendicular to main direction
            if dx.abs() > dy.abs() {
                // Horizontal travel - detour vertically
                let detour_magnitude = rng.random_range(5..12);
                let detour_direction = if rng.random() { 1 } else { -1 };
                (0, detour_direction * detour_magnitude)
            } else {
                // Vertical travel - detour horizontally
                let detour_magnitude = rng.random_range(6..15);
                let detour_direction = if rng.random() { 1 } else { -1 };
                (detour_direction * detour_magnitude, 0)
            }
        } else {
            // Even waypoints: smaller corrections toward center
            let center_x = width as i32 / 2;
            let center_y = height as i32 / 2;
            let to_center_x = (center_x - base_x as i32) / 3;
            let to_center_y = (center_y - base_y as i32) / 3;
            (to_center_x, to_center_y)
        };
        
        // Apply detours with bounds checking
        let final_x = ((base_x as i32 + detour_x).max(2).min(width as i32 - 3)) as usize;
        let final_y = ((base_y as i32 + detour_y).max(2).min(height as i32 - 3)) as usize;
        
        waypoints.push(GridPos::new(final_x, final_y));
    }
    
    waypoints
}

/// Validate that path meets 2x minimum length requirement
fn validate_path_length_requirement(path: &[GridPos], grid: &PathGrid) -> bool {
    if path.is_empty() {
        return false;
    }
    
    // Calculate actual path length (Manhattan distance)
    let actual_length: f32 = path.windows(2)
        .map(|window| window[0].manhattan_distance(&window[1]))
        .sum();
    
    // Calculate direct distance (straight line)
    let direct_distance = grid.entry_point.manhattan_distance(&grid.exit_point);
    
    // Path must be at least 2x the direct distance
    let min_required_length = direct_distance * 2.0;
    
    actual_length >= min_required_length
}

/// Create a simplified obstacle layout as fallback
fn create_fallback_obstacle_layout(grid: &mut PathGrid, rng: &mut StdRng) {
    // Clear all obstacles first
    for y in 0..grid.height {
        for x in 0..grid.width {
            let pos = GridPos::new(x, y);
            if grid.get_cell(pos) == Some(CellType::Blocked) {
                grid.set_cell(pos, CellType::Empty);
            }
        }
    }
    
    // Place minimal strategic obstacles that guarantee path existence
    let safe_obstacles = [
        GridPos::new(8, 3),   // Bottom area
        GridPos::new(12, 14), // Top area
        GridPos::new(16, 6),  // Mid-left
        GridPos::new(20, 12), // Mid-right
        GridPos::new(24, 8),  // Near end
    ];
    
    for &pos in &safe_obstacles {
        if pos.x < grid.width && pos.y < grid.height {
            grid.set_cell(pos, CellType::Blocked);
        }
    }
    
    // Add some randomness while keeping it safe
    for _ in 0..5 {
        let pos = GridPos::new(
            rng.random_range(6..26),
            rng.random_range(3..15),
        );
        
        // Only place if it doesn't block the basic path
        let old_grid = grid.clone();
        grid.set_cell(pos, CellType::Blocked);
        
        if find_path(grid, grid.entry_point, grid.exit_point).is_none() {
            *grid = old_grid; // Revert if path blocked
        }
    }
}

/// Create obstacle entities for visual rendering
pub fn create_obstacle_entities(
    commands: &mut Commands,
    grid: &PathGrid,
    obstacle_type_seed: u64,
) {
    let mut rng = StdRng::seed_from_u64(obstacle_type_seed);
    
    for y in 0..grid.height {
        for x in 0..grid.width {
            let pos = GridPos::new(x, y);
            if grid.get_cell(pos) == Some(CellType::Blocked) {
                let world_pos = grid.grid_to_world(pos);
                let obstacle_type = match rng.random_range(0..4) {
                    0 => ObstacleType::Rock,
                    1 => ObstacleType::Building, 
                    2 => ObstacleType::Debris,
                    _ => ObstacleType::Crystal,
                };
                
                spawn_obstacle_sprite(commands, world_pos, pos, obstacle_type);
            }
        }
    }
}

/// Spawn visual obstacle sprite
fn spawn_obstacle_sprite(
    commands: &mut Commands,
    world_pos: Vec2,
    grid_pos: GridPos,
    obstacle_type: ObstacleType,
) {
    let (color, size_factor) = match obstacle_type {
        ObstacleType::Rock => (Color::srgb(0.4, 0.3, 0.2), 0.9),      // Brown, large
        ObstacleType::Building => (Color::srgb(0.6, 0.6, 0.7), 0.95), // Gray, full size
        ObstacleType::Debris => (Color::srgb(0.5, 0.4, 0.3), 0.7),    // Dark brown, small
        ObstacleType::Crystal => (Color::srgb(0.3, 0.5, 0.8), 0.8),   // Blue, medium
    };
    
    let sprite_size = 40.0 * size_factor; // Scale based on grid cell size
    
    commands.spawn((
        Sprite {
            color,
            custom_size: Some(Vec2::new(sprite_size, sprite_size)),
            ..default()
        },
        Transform::from_translation(world_pos.extend(0.2)), // Slightly above background
        Obstacle {
            position: grid_pos,
            obstacle_type,
        },
    ));
}