use super::grid::{PathGrid, GridPos, TowerZone};
use super::obstacles::calculate_strategic_value;
use crate::systems::input_system::PlacementZoneType;

/// Calculate optimal tower placement zones based on generated map and path
/// 
/// # Arguments
/// * `grid` - The generated pathfinding grid
/// * `path` - The calculated path through the grid
/// 
/// # Returns
/// * `Vec<TowerZone>` - Optimized placement zones for strategic gameplay
pub fn calculate_optimal_tower_zones(grid: &PathGrid, path: &[GridPos]) -> Vec<TowerZone> {
    let mut zones = Vec::new();
    
    // Strategy 1: Create zones near path chokepoints
    let chokepoint_zones = find_chokepoint_zones(grid, path);
    zones.extend(chokepoint_zones);
    
    // Strategy 2: Create zones in large empty areas
    let area_zones = find_large_area_zones(grid, path, &zones);
    zones.extend(area_zones);
    
    // Strategy 3: Ensure minimum zone coverage
    ensure_minimum_zones(grid, &mut zones);
    
    // Sort zones by strategic value (highest first)
    zones.sort_by(|a, b| b.strategic_value.partial_cmp(&a.strategic_value).unwrap());
    
    zones
}

/// Find zones near chokepoints in the path
fn find_chokepoint_zones(grid: &PathGrid, path: &[GridPos]) -> Vec<TowerZone> {
    let mut zones = Vec::new();
    
    for (i, &path_pos) in path.iter().enumerate() {
        if is_chokepoint(grid, path_pos, path, i) {
            if let Some(zone) = create_zone_near_chokepoint(grid, path_pos, path) {
                zones.push(zone);
            }
        }
    }
    
    zones
}

/// Determine if a path position represents a chokepoint
fn is_chokepoint(grid: &PathGrid, pos: GridPos, _path: &[GridPos], _path_index: usize) -> bool {
    // A chokepoint has limited empty space around it
    let empty_neighbors = grid.count_empty_neighbors(pos);
    empty_neighbors <= 3
}

/// Create a tower zone near a chokepoint
fn create_zone_near_chokepoint(grid: &PathGrid, chokepoint: GridPos, path: &[GridPos]) -> Option<TowerZone> {
    // Look for empty areas adjacent to the chokepoint
    let search_radius = 3;
    
    for dy in -search_radius..=search_radius {
        for dx in -search_radius..=search_radius {
            let x = (chokepoint.x as i32 + dx).max(0).min(grid.width as i32 - 1) as usize;
            let y = (chokepoint.y as i32 + dy).max(0).min(grid.height as i32 - 1) as usize;
            let candidate = GridPos::new(x, y);
            
            if let Some(zone_bounds) = find_zone_bounds_from_position(grid, candidate, 2, 3) {
                let strategic_value = calculate_strategic_value(grid, candidate, path);
                
                if strategic_value > 0.5 { // Only create zones with decent strategic value
                    return Some(TowerZone::new(
                        PlacementZoneType::GridZone,
                        zone_bounds,
                        grid,
                        strategic_value,
                    ));
                }
            }
        }
    }
    
    None
}

/// Find zones in large empty areas away from chokepoints
fn find_large_area_zones(grid: &PathGrid, path: &[GridPos], existing_zones: &[TowerZone]) -> Vec<TowerZone> {
    let mut zones = Vec::new();
    
    // Scan for large empty rectangular areas
    for y in 0..grid.height - 2 {
        for x in 0..grid.width - 2 {
            let candidate = GridPos::new(x, y);
            
            // Skip if this area overlaps with existing zones
            if overlaps_existing_zones(grid, candidate, existing_zones) {
                continue;
            }
            
            if let Some(zone_bounds) = find_zone_bounds_from_position(grid, candidate, 3, 4) {
                let strategic_value = calculate_strategic_value(grid, candidate, path);
                
                zones.push(TowerZone::new(
                    PlacementZoneType::FreeZone,
                    zone_bounds,
                    grid,
                    strategic_value,
                ));
            }
        }
    }
    
    zones
}

/// Find the bounds of a tower zone starting from a position
fn find_zone_bounds_from_position(
    grid: &PathGrid,
    start: GridPos,
    min_size: usize,
    max_size: usize,
) -> Option<(GridPos, GridPos)> {
    // Try different rectangular sizes starting from max and working down
    for size in (min_size..=max_size).rev() {
        for height in min_size..=size {
            let width = size / height;
            if width < min_size {
                continue;
            }
            
            let end_x = (start.x + width - 1).min(grid.width - 1);
            let end_y = (start.y + height - 1).min(grid.height - 1);
            let end = GridPos::new(end_x, end_y);
            
            if is_area_suitable_for_zone(grid, start, end) {
                return Some((start, end));
            }
        }
    }
    
    None
}

/// Check if a rectangular area is suitable for a tower zone
fn is_area_suitable_for_zone(grid: &PathGrid, top_left: GridPos, bottom_right: GridPos) -> bool {
    let mut empty_count = 0;
    let mut total_count = 0;
    
    for y in top_left.y..=bottom_right.y {
        for x in top_left.x..=bottom_right.x {
            if x >= grid.width || y >= grid.height {
                return false;
            }
            
            let pos = GridPos::new(x, y);
            match grid.get_cell(pos) {
                Some(super::grid::CellType::Empty) => empty_count += 1,
                Some(super::grid::CellType::Path) => return false, // Can't build on path
                Some(super::grid::CellType::Blocked) => return false, // Can't build on obstacles
                Some(super::grid::CellType::TowerZone) => {}, // Already designated as zone
                None => return false,
            }
            total_count += 1;
        }
    }
    
    // Require at least 70% empty cells for a good zone
    (empty_count as f32 / total_count as f32) >= 0.7
}

/// Check if a position overlaps with existing zones
fn overlaps_existing_zones(grid: &PathGrid, pos: GridPos, existing_zones: &[TowerZone]) -> bool {
    let world_pos = grid.grid_to_world(pos);
    
    for zone in existing_zones {
        if zone.contains_world_pos(world_pos) {
            return true;
        }
    }
    
    false
}

/// Ensure we have a minimum number of viable zones for gameplay
fn ensure_minimum_zones(grid: &PathGrid, zones: &mut Vec<TowerZone>) {
    const MIN_ZONES: usize = 4;
    
    if zones.len() >= MIN_ZONES {
        return;
    }
    
    // Add fallback zones in corners or edges if we don't have enough
    let fallback_positions = vec![
        GridPos::new(1, 1),                           // Top-left
        GridPos::new(grid.width - 3, 1),              // Top-right  
        GridPos::new(1, grid.height - 3),             // Bottom-left
        GridPos::new(grid.width - 3, grid.height - 3), // Bottom-right
    ];
    
    for pos in fallback_positions {
        if zones.len() >= MIN_ZONES {
            break;
        }
        
        if let Some(zone_bounds) = find_zone_bounds_from_position(grid, pos, 2, 2) {
            zones.push(TowerZone::new(
                PlacementZoneType::FreeZone,
                zone_bounds,
                grid,
                0.1, // Low strategic value for fallback zones
            ));
        }
    }
}