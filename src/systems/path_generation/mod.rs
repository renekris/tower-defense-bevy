pub mod grid;
pub mod pathfinding;
pub mod obstacles;
pub mod zone_optimization;
pub mod cache;

pub use grid::*;
pub use pathfinding::*;
pub use obstacles::*;
pub use zone_optimization::*;
pub use cache::*;

use crate::resources::EnemyPath;

/// Main entry point for generating procedural level paths
/// 
/// # Arguments
/// * `wave_number` - Current wave number (affects difficulty and seed)
/// 
/// # Returns
/// * `EnemyPath` - Compatible with existing enemy movement system
pub fn generate_level_path(wave_number: u32) -> EnemyPath {
    let difficulty = (wave_number as f32 * 0.15).min(1.0); // Scales 0.0 to 1.0
    let seed = wave_number as u64 * 12345 + 67890; // Deterministic but varied
    
    // Generate the grid-based map
    let grid = generate_procedural_map(seed, difficulty);
    
    // Find optimal path through the generated obstacles
    let grid_path = find_path(&grid, grid.entry_point, grid.exit_point)
        .expect("Generated map must have valid path");
    
    // Convert to world coordinates for enemy movement
    grid.to_enemy_path(grid_path)
}

/// Generate level path with custom UI parameters
/// 
/// # Arguments
/// * `wave_number` - Current wave number for seed generation
/// * `custom_obstacle_density` - Override obstacle density (0.0-0.5)
/// 
/// # Returns
/// * `EnemyPath` - Compatible with existing enemy movement system
pub fn generate_level_path_with_params(wave_number: u32, custom_obstacle_density: f32) -> EnemyPath {
    let seed = wave_number as u64 * 12345 + 67890; // Deterministic but varied
    
    // Generate the grid-based map with custom obstacle density
    let grid = generate_procedural_map_with_density(seed, custom_obstacle_density);
    
    // Find optimal path through the generated obstacles
    let grid_path = find_path(&grid, grid.entry_point, grid.exit_point)
        .expect("Generated map must have valid path");
    
    // Convert to world coordinates for enemy movement
    grid.to_enemy_path(grid_path)
}

/// Generate placement zones optimized for the given wave
/// 
/// # Arguments  
/// * `wave_number` - Current wave number for consistency with path generation
///
/// # Returns
/// * `Vec<TowerZone>` - Optimized placement zones for strategic gameplay
pub fn generate_placement_zones(wave_number: u32) -> Vec<TowerZone> {
    let difficulty = (wave_number as f32 * 0.15).min(1.0);
    let seed = wave_number as u64 * 12345 + 67890;
    
    let grid = generate_procedural_map(seed, difficulty);
    let grid_path = find_path(&grid, grid.entry_point, grid.exit_point)
        .expect("Generated map must have valid path");
    
    calculate_optimal_tower_zones(&grid, &grid_path)
}