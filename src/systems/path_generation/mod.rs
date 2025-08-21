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
use std::time::{SystemTime, UNIX_EPOCH};

/// Main entry point for generating procedural level paths with time-based variety
/// Enhanced with obstacles, A* pathfinding, random start/end positioning, and 2x path length requirement
/// 
/// # Arguments
/// * `wave_number` - Current wave number (affects difficulty, not seed)
/// 
/// # Returns
/// * `EnemyPath` - Compatible with existing enemy movement system with varied layouts
pub fn generate_level_path(wave_number: u32) -> EnemyPath {
    // Generate time-based seed for map variety each startup
    let time_seed = generate_startup_seed();
    let seed = time_seed;
    
    // Generate procedural map with obstacles based on wave difficulty
    let difficulty = (wave_number as f32 / 20.0).min(1.0); // Scales up to wave 20
    let grid = obstacles::generate_procedural_map_with_random_sides(seed, difficulty);
    
    // Generate strategic path using A* pathfinding around obstacles
    let grid_path = obstacles::generate_random_strategic_path(seed + 1000, &grid);
    
    // Convert to world coordinates for enemy movement
    grid.to_enemy_path(grid_path)
}

use std::sync::OnceLock;

/// Global startup seed that's generated once per application run
static STARTUP_SEED: OnceLock<u64> = OnceLock::new();

/// Generate a startup-based seed for map variety
/// Uses system time to ensure different maps each game session, but consistent within session
fn generate_startup_seed() -> u64 {
    *STARTUP_SEED.get_or_init(|| {
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(duration) => {
                // Use milliseconds for more granular seeding
                let millis = duration.as_millis() as u64;
                // Mix in some additional entropy
                millis.wrapping_mul(1103515245).wrapping_add(12345)
            }
            Err(_) => {
                // Fallback if system time fails (e.g., in tests)
                42_u64.wrapping_mul(1103515245).wrapping_add(12345)
            }
        }
    })
}

/// Generate level path with custom UI parameters
/// Enhanced with obstacle density and A* pathfinding
/// 
/// # Arguments
/// * `wave_number` - Current wave number for seed generation
/// * `turn_complexity` - Custom complexity factor (0.0-1.0, affects obstacle density)
/// 
/// # Returns
/// * `EnemyPath` - Compatible with existing enemy movement system
pub fn generate_level_path_with_params(wave_number: u32, turn_complexity: f32) -> EnemyPath {
    let seed = wave_number as u64 * 12345 + 67890; // Deterministic but varied
    
    // Use turn_complexity to control obstacle density
    let obstacle_density = turn_complexity * 0.15; // Scale to reasonable range
    let grid = obstacles::generate_procedural_map_with_density(seed, obstacle_density);
    
    // Generate strategic path using A* pathfinding around obstacles
    let modified_seed = if turn_complexity > 0.5 {
        seed + 1000 // More complex paths with different seed offset
    } else {
        seed
    };
    
    let grid_path = obstacles::generate_random_strategic_path(modified_seed, &grid);
    
    // Convert to world coordinates for enemy movement
    grid.to_enemy_path(grid_path)
}

/// Generate placement zones optimized for the given wave
/// Creates strategic zones based on the generated path
/// 
/// # Arguments  
/// * `wave_number` - Current wave number for consistency with path generation
///
/// # Returns
/// * `Vec<TowerZone>` - Optimized placement zones for strategic gameplay
pub fn generate_placement_zones(wave_number: u32) -> Vec<TowerZone> {
    let seed = wave_number as u64 * 12345 + 67890;
    
    // Create unified grid and generate strategic path
    let mut grid = PathGrid::new_unified();
    let grid_path = obstacles::generate_random_strategic_path(seed, &grid);
    
    // Mark path cells for zone calculation
    grid.apply_path(&grid_path);
    
    let mut zones = calculate_optimal_tower_zones(&grid, &grid_path);
    
    // Fallback: If no zones generated, create strategic zones around the path
    if zones.is_empty() {
        use crate::systems::input_system::PlacementZoneType;
        use crate::systems::path_generation::grid::{GridPos, TowerZone};
        
        // Create zones in corners and along path for strategic placement
        let fallback_zones = vec![
            // Corner zones (safe from most paths)
            (GridPos::new(2, 1), GridPos::new(4, 3)),    // Bottom-left
            (GridPos::new(13, 1), GridPos::new(15, 3)),  // Bottom-right  
            (GridPos::new(2, 6), GridPos::new(4, 8)),    // Top-left
            (GridPos::new(13, 6), GridPos::new(15, 8)),  // Top-right
            
            // Central strategic zones
            (GridPos::new(7, 1), GridPos::new(10, 3)),   // Bottom-center
            (GridPos::new(7, 6), GridPos::new(10, 8)),   // Top-center
        ];
        
        for (start, end) in fallback_zones {
            // Only add if within bounds
            if start.x < grid.width && start.y < grid.height && 
               end.x < grid.width && end.y < grid.height {
                zones.push(TowerZone::new(
                    PlacementZoneType::FreeZone,
                    (start, end),
                    &grid,
                    0.6, // Good strategic value
                ));
            }
        }
    }
    
    zones
}