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
    // HOTFIX: Handle NaN values safely to prevent crashes during zone sorting
    zones.sort_by(|a, b| {
        match b.strategic_value.partial_cmp(&a.strategic_value) {
            Some(ord) => ord,
            None => {
                // Handle NaN cases: NaN strategic values are considered "worst"
                if a.strategic_value.is_nan() && b.strategic_value.is_nan() {
                    std::cmp::Ordering::Equal
                } else if a.strategic_value.is_nan() {
                    std::cmp::Ordering::Greater // a is worse
                } else {
                    std::cmp::Ordering::Less // b is worse
                }
            }
        }
    });
    
    zones
}

/// Find zones near chokepoints in the path with enhanced curve analysis
/// Optimized for Catmull-Rom splined paths to identify strategic positions
fn find_chokepoint_zones(grid: &PathGrid, path: &[GridPos]) -> Vec<TowerZone> {
    let mut zones = Vec::new();
    
    for (i, &path_pos) in path.iter().enumerate() {
        if is_enhanced_chokepoint(grid, path_pos, path, i) {
            if let Some(zone) = create_zone_near_chokepoint(grid, path_pos, path) {
                zones.push(zone);
            }
        }
    }
    
    // Add zones specifically for curve positions
    let curve_zones = find_curve_strategic_zones(grid, path);
    zones.extend(curve_zones);
    
    zones
}

/// Determine if a path position represents a chokepoint
fn is_chokepoint(grid: &PathGrid, pos: GridPos, _path: &[GridPos], _path_index: usize) -> bool {
    // A chokepoint has limited empty space around it
    let empty_neighbors = grid.count_empty_neighbors(pos);
    empty_neighbors <= 3
}

/// Enhanced chokepoint detection for smooth curved paths
/// Considers path flow direction and curve geometry for strategic positioning
fn is_enhanced_chokepoint(grid: &PathGrid, pos: GridPos, path: &[GridPos], path_index: usize) -> bool {
    // Traditional chokepoint detection
    let empty_neighbors = grid.count_empty_neighbors(pos);
    if empty_neighbors <= 3 {
        return true;
    }
    
    // Enhanced detection for curved sections
    if path.len() >= 3 && path_index > 0 && path_index < path.len() - 1 {
        let prev = path[path_index - 1];
        let curr = pos;
        let next = path[path_index + 1];
        
        // Check if this is a significant curve
        let dir1 = (curr.x as i32 - prev.x as i32, curr.y as i32 - prev.y as i32);
        let dir2 = (next.x as i32 - curr.x as i32, next.y as i32 - curr.y as i32);
        
        let is_curve = dir1.0 != dir2.0 || dir1.1 != dir2.1;
        if is_curve {
            // Curves with moderate space around them are strategic positions
            return empty_neighbors <= 5;
        }
    }
    
    // Check for narrow passages (even if not at obstacles)
    let search_radius = 2;
    let mut narrow_directions = 0;
    
    for dy in -search_radius..=search_radius {
        for dx in -search_radius..=search_radius {
            if dx == 0 && dy == 0 { continue; }
            
            let check_x = (pos.x as i32 + dx).max(0).min(grid.width as i32 - 1) as usize;
            let check_y = (pos.y as i32 + dy).max(0).min(grid.height as i32 - 1) as usize;
            let check_pos = GridPos::new(check_x, check_y);
            
            if !grid.is_traversable(check_pos) {
                narrow_directions += 1;
            }
        }
    }
    
    // Position is a chokepoint if surrounded by many blocked cells
    narrow_directions >= 8 // More than half of the surrounding area is blocked
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

/// Find strategic zones specifically positioned for curved path sections
/// These zones take advantage of the natural chokepoints created by Catmull-Rom curves
fn find_curve_strategic_zones(grid: &PathGrid, path: &[GridPos]) -> Vec<TowerZone> {
    let mut zones = Vec::new();
    
    if path.len() < 3 {
        return zones;
    }
    
    // Analyze each path segment for curvature
    for i in 1..path.len() - 1 {
        let prev = path[i - 1];
        let curr = path[i];
        let next = path[i + 1];
        
        // Calculate direction vectors
        let dir1 = (curr.x as i32 - prev.x as i32, curr.y as i32 - prev.y as i32);
        let dir2 = (next.x as i32 - curr.x as i32, next.y as i32 - curr.y as i32);
        
        // Detect significant direction changes
        let is_curve = dir1.0 != dir2.0 || dir1.1 != dir2.1;
        if is_curve {
            // Try to create zones on the inside and outside of the curve
            if let Some(inside_zone) = create_curve_inside_zone(grid, prev, curr, next, path) {
                zones.push(inside_zone);
            }
            
            if let Some(outside_zone) = create_curve_outside_zone(grid, prev, curr, next, path) {
                zones.push(outside_zone);
            }
        }
    }
    
    zones
}

/// Create a tower zone on the inside of a curve (tighter coverage)
fn create_curve_inside_zone(
    grid: &PathGrid, 
    prev: GridPos, 
    curr: GridPos, 
    next: GridPos, 
    path: &[GridPos]
) -> Option<TowerZone> {
    // Calculate the direction of the curve
    let turn_vector = calculate_curve_inside_direction(prev, curr, next);
    
    // Position zone on the inside of the curve
    let zone_x = (curr.x as i32 + turn_vector.0 * 2).max(1).min(grid.width as i32 - 2) as usize;
    let zone_y = (curr.y as i32 + turn_vector.1 * 2).max(1).min(grid.height as i32 - 2) as usize;
    let zone_center = GridPos::new(zone_x, zone_y);
    
    // Create zone bounds
    if let Some(zone_bounds) = find_zone_bounds_from_position(grid, zone_center, 2, 3) {
        let strategic_value = calculate_strategic_value(grid, zone_center, path);
        
        // Inside curve zones get a bonus for coverage
        let enhanced_value = strategic_value + 0.3;
        
        return Some(TowerZone::new(
            PlacementZoneType::GridZone,
            zone_bounds,
            grid,
            enhanced_value,
        ));
    }
    
    None
}

/// Create a tower zone on the outside of a curve (wider coverage, defensive positioning)
fn create_curve_outside_zone(
    grid: &PathGrid, 
    prev: GridPos, 
    curr: GridPos, 
    next: GridPos, 
    path: &[GridPos]
) -> Option<TowerZone> {
    // Calculate the direction opposite to the inside curve
    let inside_vector = calculate_curve_inside_direction(prev, curr, next);
    let outside_vector = (-inside_vector.0, -inside_vector.1);
    
    // Position zone on the outside of the curve (farther but better coverage)
    let zone_x = (curr.x as i32 + outside_vector.0 * 3).max(2).min(grid.width as i32 - 3) as usize;
    let zone_y = (curr.y as i32 + outside_vector.1 * 3).max(2).min(grid.height as i32 - 3) as usize;
    let zone_center = GridPos::new(zone_x, zone_y);
    
    // Create larger zone bounds for outside positioning
    if let Some(zone_bounds) = find_zone_bounds_from_position(grid, zone_center, 2, 4) {
        let strategic_value = calculate_strategic_value(grid, zone_center, path);
        
        // Outside curve zones get bonus for multi-segment coverage
        let enhanced_value = strategic_value + 0.2;
        
        return Some(TowerZone::new(
            PlacementZoneType::FreeZone,
            zone_bounds,
            grid,
            enhanced_value,
        ));
    }
    
    None
}

/// Calculate the direction vector pointing to the inside of a curve
/// This helps position towers optimally around curved sections
fn calculate_curve_inside_direction(prev: GridPos, curr: GridPos, next: GridPos) -> (i32, i32) {
    // Calculate direction vectors
    let dir1 = (curr.x as i32 - prev.x as i32, curr.y as i32 - prev.y as i32);
    let dir2 = (next.x as i32 - curr.x as i32, next.y as i32 - curr.y as i32);
    
    // Calculate the cross product to determine turn direction
    let cross_product = dir1.0 * dir2.1 - dir1.1 * dir2.0;
    
    // Calculate the average direction for the curve center
    let avg_dir_x = dir1.0 + dir2.0;
    let avg_dir_y = dir1.1 + dir2.1;
    
    // Perpendicular vector pointing toward the inside of the curve
    let perpendicular = if cross_product > 0 {
        // Right turn - inside is to the left
        (-avg_dir_y, avg_dir_x)
    } else if cross_product < 0 {
        // Left turn - inside is to the right  
        (avg_dir_y, -avg_dir_x)
    } else {
        // Straight line - no preferred inside
        (0, 0)
    };
    
    // Normalize to unit direction
    if perpendicular.0 != 0 || perpendicular.1 != 0 {
        let magnitude = ((perpendicular.0.pow(2) + perpendicular.1.pow(2)) as f32).sqrt() as i32;
        if magnitude > 0 {
            (perpendicular.0 / magnitude.max(1), perpendicular.1 / magnitude.max(1))
        } else {
            (0, 0)
        }
    } else {
        (0, 0)
    }
}