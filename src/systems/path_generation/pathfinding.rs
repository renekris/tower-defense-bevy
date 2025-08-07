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