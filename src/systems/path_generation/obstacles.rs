use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use super::grid::{PathGrid, GridPos, CellType};

/// Generate a procedural map with obstacles and strategic layout
/// 
/// # Arguments
/// * `seed` - Random seed for reproducible generation
/// * `difficulty` - Difficulty factor (0.0 = easy, 1.0 = hard)
/// 
/// # Returns
/// * `PathGrid` - Generated map with obstacles and entry/exit points
pub fn generate_procedural_map(seed: u64, difficulty: f32) -> PathGrid {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut grid = PathGrid::new(20, 12);
    
    // Set entry and exit points (avoid corners)
    grid.entry_point = GridPos::new(0, rng.random_range(2..10));
    grid.exit_point = GridPos::new(19, rng.random_range(2..10));
    
    // Place strategic obstacles based on difficulty
    let obstacle_density = (difficulty * 0.3).min(0.25); // Max 25% obstacle coverage
    place_strategic_obstacles(&mut grid, &mut rng, obstacle_density);
    
    // Ensure path exists - if not, reduce obstacles and try again
    let mut attempts = 0;
    while crate::systems::path_generation::find_path(&grid, grid.entry_point, grid.exit_point).is_none() && attempts < 10 {
        reduce_obstacles(&mut grid, &mut rng, 0.1);
        attempts += 1;
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
    let mut grid = PathGrid::new(20, 12);
    
    // Set entry and exit points (avoid corners)
    grid.entry_point = GridPos::new(0, rng.random_range(2..10));
    grid.exit_point = GridPos::new(19, rng.random_range(2..10));
    
    // Place strategic obstacles with custom density
    let clamped_density = obstacle_density.clamp(0.0, 0.5); // Max 50% coverage
    place_strategic_obstacles(&mut grid, &mut rng, clamped_density);
    
    // Ensure path exists - if not, reduce obstacles and try again
    let mut attempts = 0;
    while crate::systems::path_generation::find_path(&grid, grid.entry_point, grid.exit_point).is_none() && attempts < 10 {
        reduce_obstacles(&mut grid, &mut rng, 0.1);
        attempts += 1;
    }
    
    grid
}

/// Place obstacles strategically to create interesting chokepoints
fn place_strategic_obstacles(grid: &mut PathGrid, rng: &mut StdRng, density: f32) {
    let total_cells = grid.width * grid.height;
    let target_obstacles = (total_cells as f32 * density) as usize;
    
    let mut placed = 0;
    
    // Strategy 1: Create obstacle clusters in center areas
    for _ in 0..(target_obstacles / 2) {
        let cluster_center = GridPos::new(
            rng.random_range(6..14),   // Center region
            rng.random_range(3..9),
        );
        
        if place_obstacle_cluster(grid, rng, cluster_center, 2, 3) {
            placed += 1;
        }
        
        if placed >= target_obstacles {
            break;
        }
    }
    
    // Strategy 2: Scatter remaining obstacles randomly
    while placed < target_obstacles {
        let pos = GridPos::new(
            rng.random_range(2..18),   // Avoid edges
            rng.random_range(1..11),
        );
        
        if grid.get_cell(pos) == Some(CellType::Empty) {
            grid.set_cell(pos, CellType::Blocked);
            placed += 1;
        }
    }
}

/// Place a small cluster of obstacles around a center point
fn place_obstacle_cluster(grid: &mut PathGrid, rng: &mut StdRng, center: GridPos, min_size: usize, max_size: usize) -> bool {
    let cluster_size = rng.random_range(min_size..=max_size);
    let mut positions = Vec::new();
    
    // Generate cluster positions
    for _ in 0..cluster_size {
        let offset_x = rng.random_range(-2..=2);
        let offset_y = rng.random_range(-2..=2);
        
        let x = (center.x as i32 + offset_x).max(0).min(grid.width as i32 - 1) as usize;
        let y = (center.y as i32 + offset_y).max(0).min(grid.height as i32 - 1) as usize;
        
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