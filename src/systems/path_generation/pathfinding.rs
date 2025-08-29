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
    
    // Must have at least 3 turns, but allow more for complex paths
    let turn_count = count_direction_changes(path);
    if turn_count < 3 {
        return false;
    }
    
    // Check connectivity (no large jumps) - allow longer jumps for dense grid
    for i in 0..path.len() - 1 {
        let dist = path[i].manhattan_distance(&path[i + 1]);
        if dist > 10.0 {  // Allow jumps up to 10 steps for 32-wide grid
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

/// Analyze strategic positions along a path for optimal tower placement
/// Returns positions ranked by strategic value for defense planning
pub fn analyze_strategic_positions(grid: &PathGrid, path: &[GridPos]) -> Vec<(GridPos, f32)> {
    let mut strategic_positions = Vec::new();
    
    if path.is_empty() {
        return strategic_positions;
    }
    
    // Analyze positions around the path
    let search_range = 5; // How far from path to search for positions
    
    for &path_pos in path {
        // Check positions around each path point
        for dy in -(search_range as i32)..=(search_range as i32) {
            for dx in -(search_range as i32)..=(search_range as i32) {
                let check_x = (path_pos.x as i32 + dx).max(0).min(grid.width as i32 - 1) as usize;
                let check_y = (path_pos.y as i32 + dy).max(0).min(grid.height as i32 - 1) as usize;
                let pos = GridPos::new(check_x, check_y);
                
                // Only analyze empty positions
                if grid.is_traversable(pos) && grid.get_cell(pos) == Some(super::grid::CellType::Empty) {
                    let strategic_value = super::obstacles::calculate_strategic_value(grid, pos, path);
                    
                    if strategic_value > 0.3 { // Only include positions with reasonable strategic value
                        strategic_positions.push((pos, strategic_value));
                    }
                }
            }
        }
    }
    
    // Remove duplicates and sort by strategic value
    strategic_positions.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    strategic_positions.dedup_by(|a, b| a.0 == b.0);
    
    strategic_positions
}

/// Identify natural chokepoints along a smooth curved path
/// Returns positions where enemy flow can be most effectively controlled
pub fn identify_path_chokepoints(grid: &PathGrid, path: &[GridPos]) -> Vec<(GridPos, ChokePointType)> {
    let mut chokepoints = Vec::new();
    
    if path.len() < 3 {
        return chokepoints;
    }
    
    for (i, &pos) in path.iter().enumerate() {
        // Analyze each position for chokepoint characteristics
        let chokepoint_type = analyze_chokepoint_type(grid, pos, path, i);
        
        if chokepoint_type != ChokePointType::None {
            chokepoints.push((pos, chokepoint_type));
        }
    }
    
    chokepoints
}

/// Types of strategic chokepoints
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChokePointType {
    /// Not a significant chokepoint
    None,
    /// Narrow passage with limited alternative routes
    NarrowPassage,
    /// Sharp curve where movement is constrained
    CurveChokePoint,
    /// Junction where paths converge/diverge
    PathJunction,
    /// Position surrounded by obstacles
    ObstacleChokePoint,
}

/// Analyze what type of chokepoint a position represents
fn analyze_chokepoint_type(grid: &PathGrid, pos: GridPos, path: &[GridPos], path_index: usize) -> ChokePointType {
    let empty_neighbors = grid.count_empty_neighbors(pos);
    
    // Obstacle-based chokepoint
    if empty_neighbors <= 2 {
        return ChokePointType::ObstacleChokePoint;
    }
    
    // Narrow passage
    if empty_neighbors <= 3 {
        return ChokePointType::NarrowPassage;
    }
    
    // Curve-based chokepoint
    if path.len() >= 3 && path_index > 0 && path_index < path.len() - 1 {
        let prev = path[path_index - 1];
        let curr = pos;
        let next = path[path_index + 1];
        
        // Calculate curvature strength
        let curvature = calculate_path_curvature(prev, curr, next);
        if curvature > 0.7 && empty_neighbors <= 5 {
            return ChokePointType::CurveChokePoint;
        }
    }
    
    ChokePointType::None
}

/// Calculate the curvature strength at a path point (0.0 = straight, 1.0 = sharp turn)
fn calculate_path_curvature(prev: GridPos, curr: GridPos, next: GridPos) -> f32 {
    // Calculate direction vectors
    let dir1 = (curr.x as i32 - prev.x as i32, curr.y as i32 - prev.y as i32);
    let dir2 = (next.x as i32 - curr.x as i32, next.y as i32 - curr.y as i32);
    
    // Normalize directions
    let mag1 = ((dir1.0.pow(2) + dir1.1.pow(2)) as f32).sqrt();
    let mag2 = ((dir2.0.pow(2) + dir2.1.pow(2)) as f32).sqrt();
    
    if mag1 == 0.0 || mag2 == 0.0 {
        return 0.0;
    }
    
    let norm1 = (dir1.0 as f32 / mag1, dir1.1 as f32 / mag1);
    let norm2 = (dir2.0 as f32 / mag2, dir2.1 as f32 / mag2);
    
    // Dot product for angle calculation
    let dot_product = norm1.0 * norm2.0 + norm1.1 * norm2.1;
    let angle_cos = dot_product.clamp(-1.0, 1.0);
    
    // Convert to curvature (1.0 - cos gives us 0 for straight, 2 for opposite)
    (1.0 - angle_cos) / 2.0
}

/// Evaluate overall strategic value of a path layout
/// Returns scores for different defensive strategies
pub fn evaluate_path_strategic_value(grid: &PathGrid, path: &[GridPos]) -> PathStrategicAnalysis {
    let complexity = calculate_path_complexity(path);
    let chokepoints = identify_path_chokepoints(grid, path);
    let strategic_positions = analyze_strategic_positions(grid, path);
    
    // Calculate defensive opportunity score
    let defensive_score = chokepoints.len() as f32 * 0.3 + strategic_positions.len() as f32 * 0.1;
    
    // Calculate path coverage score (how much can be covered by towers)
    let coverage_score = calculate_path_coverage_potential(grid, path);
    
    // Calculate balance score (difficulty vs. fairness)
    let balance_score = calculate_path_balance(grid, path, &chokepoints);
    
    PathStrategicAnalysis {
        complexity_score: complexity,
        defensive_opportunities: defensive_score.min(5.0),
        path_coverage_potential: coverage_score,
        balance_score,
        chokepoint_count: chokepoints.len(),
        strategic_position_count: strategic_positions.len(),
        recommended_difficulty: calculate_recommended_difficulty(&chokepoints, defensive_score),
    }
}

/// Calculate how much of the path can potentially be covered by optimally placed towers
fn calculate_path_coverage_potential(grid: &PathGrid, path: &[GridPos]) -> f32 {
    let strategic_positions = analyze_strategic_positions(grid, path);
    let mut total_coverage = 0.0;
    
    for &(pos, _value) in &strategic_positions {
        // Calculate how much of the path this position can cover
        let coverage = path.iter()
            .filter(|&&path_pos| pos.manhattan_distance(&path_pos) <= 4.0)
            .count() as f32 / path.len() as f32;
        
        total_coverage += coverage;
    }
    
    // Average coverage potential
    if !strategic_positions.is_empty() {
        (total_coverage / strategic_positions.len() as f32).min(1.0)
    } else {
        0.0
    }
}

/// Calculate how balanced the path is for fair but challenging gameplay
fn calculate_path_balance(grid: &PathGrid, path: &[GridPos], chokepoints: &[(GridPos, ChokePointType)]) -> f32 {
    let mut balance_score: f32 = 0.5; // Start neutral
    
    // Penalty for too many chokepoints (unfair)
    if chokepoints.len() > 8 {
        balance_score -= 0.2;
    }
    
    // Penalty for too few chokepoints (boring)
    if chokepoints.len() < 2 {
        balance_score -= 0.3;
    }
    
    // Bonus for well-distributed chokepoints
    if chokepoints.len() >= 3 && chokepoints.len() <= 6 {
        balance_score += 0.2;
    }
    
    // Check path length balance
    let path_length: f32 = path.windows(2)
        .map(|w| w[0].manhattan_distance(&w[1]))
        .sum();
    
    let direct_distance = path[0].manhattan_distance(&path[path.len() - 1]);
    let detour_ratio = path_length / direct_distance.max(1.0);
    
    // Optimal detour ratio is between 2.0 and 4.0
    if detour_ratio >= 2.0 && detour_ratio <= 4.0 {
        balance_score += 0.2;
    } else if detour_ratio > 6.0 {
        balance_score -= 0.2; // Too winding
    }
    
    balance_score.clamp(0.0, 1.0)
}

/// Calculate recommended difficulty level based on path analysis
fn calculate_recommended_difficulty(chokepoints: &[(GridPos, ChokePointType)], defensive_score: f32) -> f32 {
    let mut difficulty = 0.5; // Base difficulty
    
    // More chokepoints = easier to defend = can handle higher difficulty
    difficulty += (chokepoints.len() as f32 * 0.05).min(0.3);
    
    // Higher defensive score = more strategic options = can handle higher difficulty
    difficulty += (defensive_score * 0.1).min(0.2);
    
    // Count dangerous chokepoint types
    let dangerous_chokepoints = chokepoints.iter()
        .filter(|(_, cp_type)| matches!(cp_type, ChokePointType::ObstacleChokePoint | ChokePointType::CurveChokePoint))
        .count();
    
    difficulty += (dangerous_chokepoints as f32 * 0.05).min(0.2);
    
    difficulty.clamp(0.1, 1.0)
}

/// Complete strategic analysis of a path for tower defense gameplay
#[derive(Debug, Clone)]
pub struct PathStrategicAnalysis {
    /// Path complexity score (0.0 = simple, 2.0+ = very complex)
    pub complexity_score: f32,
    /// Number of defensive opportunities available
    pub defensive_opportunities: f32,
    /// How much of the path can be covered by towers
    pub path_coverage_potential: f32,
    /// Balance score for fair gameplay (0.0 = unbalanced, 1.0 = perfect)
    pub balance_score: f32,
    /// Number of chokepoints identified
    pub chokepoint_count: usize,
    /// Number of strategic positions available
    pub strategic_position_count: usize,
    /// Recommended difficulty multiplier for this path
    pub recommended_difficulty: f32,
}