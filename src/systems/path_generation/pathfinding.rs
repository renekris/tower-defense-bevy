use std::collections::{HashMap, BinaryHeap};
use std::cmp::Ordering;
use super::grid::{PathGrid, GridPos};

/// A* pathfinding node for priority queue
#[derive(Debug, Clone)]
struct PathNode {
    pos: GridPos,
    g_cost: f32,    // Distance from start
    h_cost: f32,    // Heuristic distance to goal  
    f_cost: f32,    // g_cost + h_cost
    parent: Option<GridPos>,
}

impl PathNode {
    fn new(pos: GridPos, g_cost: f32, h_cost: f32, parent: Option<GridPos>) -> Self {
        Self {
            pos,
            g_cost,
            h_cost,
            f_cost: g_cost + h_cost,
            parent,
        }
    }
}

impl PartialEq for PathNode {
    fn eq(&self, other: &Self) -> bool {
        self.f_cost == other.f_cost
    }
}

impl Eq for PathNode {}

impl PartialOrd for PathNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PathNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for min-heap behavior
        // HOTFIX: Handle NaN values safely to prevent crashes
        match other.f_cost.partial_cmp(&self.f_cost) {
            Some(ord) => ord,
            None => {
                // Handle NaN cases: NaN values are considered "worse" than any number
                if self.f_cost.is_nan() && other.f_cost.is_nan() {
                    Ordering::Equal
                } else if self.f_cost.is_nan() {
                    Ordering::Greater // self is worse
                } else {
                    Ordering::Less // other is worse
                }
            }
        }
    }
}

/// Find optimal path using A* algorithm
/// 
/// # Arguments
/// * `grid` - The pathfinding grid
/// * `start` - Starting grid position
/// * `goal` - Goal grid position
/// 
/// # Returns
/// * `Some(Vec<GridPos>)` - Path from start to goal if found
/// * `None` - No path exists
pub fn find_path(grid: &PathGrid, start: GridPos, goal: GridPos) -> Option<Vec<GridPos>> {
    // Early validation
    if !grid.is_traversable(start) || !grid.is_traversable(goal) {
        return None;
    }
    
    if start == goal {
        return Some(vec![start]);
    }
    
    // A* data structures
    let mut open_set = BinaryHeap::new();
    let mut came_from: HashMap<GridPos, GridPos> = HashMap::new();
    let mut g_score: HashMap<GridPos, f32> = HashMap::new();
    
    // Initialize start node
    g_score.insert(start, 0.0);
    let h_cost = start.manhattan_distance(&goal);
    open_set.push(PathNode::new(start, 0.0, h_cost, None));
    
    while let Some(current_node) = open_set.pop() {
        let current = current_node.pos;
        
        // Goal reached
        if current == goal {
            return Some(reconstruct_path(&came_from, current));
        }
        
        // Check neighbors
        for neighbor in current.neighbors(grid.width, grid.height) {
            if !grid.is_traversable(neighbor) {
                continue;
            }
            
            let tentative_g_score = g_score[&current] + 1.0; // All moves cost 1
            
            let neighbor_g_score = g_score.get(&neighbor).copied().unwrap_or(f32::INFINITY);
            
            if tentative_g_score < neighbor_g_score {
                // This path to neighbor is better
                came_from.insert(neighbor, current);
                g_score.insert(neighbor, tentative_g_score);
                
                let h_cost = neighbor.manhattan_distance(&goal);
                open_set.push(PathNode::new(neighbor, tentative_g_score, h_cost, Some(current)));
            }
        }
    }
    
    None // No path found
}

/// Reconstruct the path by following parent pointers
fn reconstruct_path(came_from: &HashMap<GridPos, GridPos>, mut current: GridPos) -> Vec<GridPos> {
    let mut path = vec![current];
    
    while let Some(&parent) = came_from.get(&current) {
        current = parent;
        path.push(current);
    }
    
    path.reverse();
    path
}

/// Validate that a path meets quality requirements
pub fn validate_path_quality(path: &[GridPos], min_length: usize, max_length: usize) -> bool {
    if path.len() < min_length || path.len() > max_length {
        return false;
    }
    
    // Check for excessive backtracking or loops
    if has_loops(path) {
        return false;
    }
    
    // Path should have some turns for strategic interest
    if count_direction_changes(path) < 2 {
        return false;
    }
    
    true
}

/// Enhanced validation for strategic paths with unified grid requirements
/// 
/// # Arguments
/// * `path` - The path to validate
/// * `grid_width` - Grid width (should be 32 for dense unified grid)
/// * `grid_height` - Grid height (should be 18 for dense unified grid)
/// 
/// # Returns
/// * `bool` - True if path meets all strategic requirements
pub fn validate_strategic_path_requirements(path: &[GridPos], grid_width: usize, grid_height: usize) -> bool {
    if path.is_empty() {
        return false;
    }
    
    // Must start on left edge and end on right edge
    let start = path.first().unwrap();
    let end = path.last().unwrap();
    
    if start.x != 0 || end.x != grid_width - 1 {
        return false;
    }
    
    // Check edge avoidance for intermediate points (allow touching edges at x=1 and y=1 for strategic paths)
    for (i, &pos) in path.iter().enumerate() {
        if i == 0 || i == path.len() - 1 {
            continue; // Skip start and end
        }
        
        // Must not be ON the actual edges (x=0, y=0, etc.), but 1 cell away is okay
        if pos.x == 0 || pos.x >= grid_width - 1 || pos.y == 0 || pos.y >= grid_height - 1 {
            return false;
        }
    }
    
    // Must have between 3-5 turns
    let turn_count = count_direction_changes(path);
    if turn_count < 3 || turn_count > 5 {
        return false;
    }
    
    // Check connectivity (no large jumps)
    for i in 0..path.len() - 1 {
        let dist = path[i].manhattan_distance(&path[i + 1]);
        if dist > 6.0 {  // Allow reasonable jumps for strategic paths
            return false;
        }
    }
    
    // Path should use middle range for start/end points (strategic gameplay)
    let middle_range = 6..=12; // For 18-tall grid: 6-12 (middle area)
    if !middle_range.contains(&start.y) || !middle_range.contains(&end.y) {
        return false;
    }
    
    true
}

/// Calculate path complexity score (higher = more interesting strategically)
/// 
/// # Arguments
/// * `path` - The path to analyze
/// 
/// # Returns
/// * `f32` - Complexity score (0.0 = boring straight line, 1.0+ = highly complex)
pub fn calculate_path_complexity(path: &[GridPos]) -> f32 {
    if path.len() < 3 {
        return 0.0;
    }
    
    let mut complexity = 0.0;
    
    // Turn complexity
    let turn_count = count_direction_changes(path) as f32;
    complexity += turn_count * 0.2; // Each turn adds complexity
    
    // Path length complexity (longer paths are more complex)
    let total_distance: f32 = path.windows(2)
        .map(|window| window[0].manhattan_distance(&window[1]))
        .sum();
    let straight_distance = path[0].manhattan_distance(&path[path.len() - 1]);
    
    if straight_distance > 0.0 {
        let detour_ratio = total_distance / straight_distance;
        complexity += (detour_ratio - 1.0) * 0.3; // Detours add complexity
    }
    
    // Vertical variance complexity (up/down movement creates opportunities)
    let y_positions: Vec<usize> = path.iter().map(|p| p.y).collect();
    let y_min = *y_positions.iter().min().unwrap();
    let y_max = *y_positions.iter().max().unwrap();
    let y_variance = (y_max - y_min) as f32;
    complexity += y_variance * 0.1;
    
    complexity.max(0.0)
}

/// Check if path contains loops (visits same position twice)
fn has_loops(path: &[GridPos]) -> bool {
    for (i, &pos1) in path.iter().enumerate() {
        for &pos2 in path.iter().skip(i + 1) {
            if pos1 == pos2 {
                return true;
            }
        }
    }
    false
}

/// Count the number of direction changes in a path
fn count_direction_changes(path: &[GridPos]) -> usize {
    if path.len() < 3 {
        return 0;
    }
    
    let mut changes = 0;
    let mut last_direction = None;
    
    for i in 1..path.len() {
        let current_direction = get_direction(path[i - 1], path[i]);
        if let Some(last_dir) = last_direction {
            if current_direction != last_dir {
                changes += 1;
            }
        }
        last_direction = Some(current_direction);
    }
    
    changes
}

/// Get direction between two adjacent grid positions
fn get_direction(from: GridPos, to: GridPos) -> (i32, i32) {
    (
        to.x as i32 - from.x as i32,
        to.y as i32 - from.y as i32,
    )
}