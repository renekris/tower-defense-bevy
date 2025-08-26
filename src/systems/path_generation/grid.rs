use bevy::prelude::*;
use crate::resources::EnemyPath;
use crate::systems::input_system::PlacementZoneType;

/// Represents the type of content in each grid cell
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CellType {
    /// Empty cell - available for path routing or tower placement
    #[default]
    Empty,
    /// Active path cell - enemies will traverse this cell
    Path,
    /// Designated tower placement zone
    TowerZone,
    /// Blocked cell - impassable obstacle
    Blocked,
}

/// Grid position using integer coordinates
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GridPos {
    pub x: usize,
    pub y: usize,
}

impl GridPos {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
    
    /// Get 4-directional neighbors (no diagonals)
    pub fn neighbors(&self, width: usize, height: usize) -> Vec<GridPos> {
        let mut neighbors = Vec::new();
        
        // North
        if self.y > 0 {
            neighbors.push(GridPos::new(self.x, self.y - 1));
        }
        
        // South  
        if self.y + 1 < height {
            neighbors.push(GridPos::new(self.x, self.y + 1));
        }
        
        // West
        if self.x > 0 {
            neighbors.push(GridPos::new(self.x - 1, self.y));
        }
        
        // East
        if self.x + 1 < width {
            neighbors.push(GridPos::new(self.x + 1, self.y));
        }
        
        neighbors
    }
    
    /// Calculate Manhattan distance to another position
    pub fn manhattan_distance(&self, other: &GridPos) -> f32 {
        ((self.x as i32 - other.x as i32).abs() + (self.y as i32 - other.y as i32).abs()) as f32
    }
}

/// Grid-based representation of the game map for pathfinding
#[derive(Debug, Clone, Resource)]
pub struct PathGrid {
    /// Grid width in cells
    pub width: usize,
    /// Grid height in cells  
    pub height: usize,
    /// Size of each cell in world units (pixels)
    pub cell_size: f32,
    /// 2D grid of cell types [y][x] indexing
    pub cells: Vec<Vec<CellType>>,
    /// Entry point for enemies (grid coordinates)
    pub entry_point: GridPos,
    /// Exit point for enemies (grid coordinates)
    pub exit_point: GridPos,
}

impl PathGrid {
    /// Create a new empty grid with specified dimensions
    pub fn new(width: usize, height: usize) -> Self {
        let cells = vec![vec![CellType::Empty; width]; height];
        
        Self {
            width,
            height,
            cell_size: 40.0, // Matches dense unified grid cell size
            cells,
            entry_point: GridPos::new(0, height / 2),
            exit_point: GridPos::new(width - 1, height / 2),
        }
    }
    
    /// Create a new grid using dense unified grid system dimensions (32x18)
    pub fn new_unified() -> Self {
        Self::new(32, 18)
    }
    
    /// Get cell type at grid position (bounds-checked)
    pub fn get_cell(&self, pos: GridPos) -> Option<CellType> {
        if pos.x < self.width && pos.y < self.height {
            Some(self.cells[pos.y][pos.x])
        } else {
            None
        }
    }
    
    /// Set cell type at grid position (bounds-checked)
    pub fn set_cell(&mut self, pos: GridPos, cell_type: CellType) -> bool {
        if pos.x < self.width && pos.y < self.height {
            self.cells[pos.y][pos.x] = cell_type;
            true
        } else {
            false
        }
    }
    
    /// Check if a position is traversable for pathfinding
    pub fn is_traversable(&self, pos: GridPos) -> bool {
        match self.get_cell(pos) {
            Some(CellType::Empty) | Some(CellType::Path) => true,
            Some(CellType::Blocked) | Some(CellType::TowerZone) => false,
            None => false,
        }
    }
    
    /// Convert grid coordinates to world coordinates (center of cell)
    /// Uses unified grid coordinate system for consistency
    pub fn grid_to_world(&self, grid_pos: GridPos) -> Vec2 {
        let grid_offset = Vec2::new(
            -(self.width as f32 * self.cell_size) / 2.0,
            -(self.height as f32 * self.cell_size) / 2.0,
        );
        
        grid_offset + Vec2::new(
            grid_pos.x as f32 * self.cell_size + self.cell_size / 2.0,
            grid_pos.y as f32 * self.cell_size + self.cell_size / 2.0,
        )
    }
    
    /// Convert world coordinates to grid coordinates
    /// Uses unified grid coordinate system for consistency
    pub fn world_to_grid(&self, world_pos: Vec2) -> Option<GridPos> {
        let grid_offset = Vec2::new(
            -(self.width as f32 * self.cell_size) / 2.0,
            -(self.height as f32 * self.cell_size) / 2.0,
        );
        
        let relative_pos = world_pos - grid_offset;
        let grid_x = (relative_pos.x / self.cell_size).floor() as i32;
        let grid_y = (relative_pos.y / self.cell_size).floor() as i32;
        
        if grid_x >= 0 && grid_x < self.width as i32 && 
           grid_y >= 0 && grid_y < self.height as i32 {
            Some(GridPos::new(grid_x as usize, grid_y as usize))
        } else {
            None
        }
    }
    
    /// Convert a path of grid positions to EnemyPath with world coordinates
    pub fn to_enemy_path(&self, grid_path: Vec<GridPos>) -> EnemyPath {
        let waypoints: Vec<Vec2> = grid_path.iter()
            .map(|&pos| self.grid_to_world(pos))
            .collect();
            
        EnemyPath::new(waypoints)
    }
    
    /// Apply a path to the grid, marking cells as Path type
    pub fn apply_path(&mut self, path: &[GridPos]) {
        for &pos in path {
            self.set_cell(pos, CellType::Path);
        }
    }
    
    /// Count empty cells adjacent to a position
    pub fn count_empty_neighbors(&self, pos: GridPos) -> usize {
        pos.neighbors(self.width, self.height)
            .iter()
            .filter(|&&neighbor| self.get_cell(neighbor) == Some(CellType::Empty))
            .count()
    }
    
    /// Find the largest empty rectangular area (for tower zone optimization)
    pub fn find_largest_empty_rect(&self) -> Option<(GridPos, GridPos)> {
        let mut max_area = 0;
        let mut best_rect = None;
        
        // Simple algorithm - could be optimized with more sophisticated approaches
        for y in 0..self.height {
            for x in 0..self.width {
                if self.cells[y][x] == CellType::Empty {
                    for h in 1..=(self.height - y) {
                        for w in 1..=(self.width - x) {
                            if self.is_rect_empty(GridPos::new(x, y), w, h) {
                                let area = w * h;
                                if area > max_area {
                                    max_area = area;
                                    best_rect = Some((
                                        GridPos::new(x, y),
                                        GridPos::new(x + w - 1, y + h - 1)
                                    ));
                                }
                            } else {
                                break; // Can't extend width further
                            }
                        }
                    }
                }
            }
        }
        
        best_rect
    }
    
    /// Check if a rectangular area is entirely empty
    fn is_rect_empty(&self, top_left: GridPos, width: usize, height: usize) -> bool {
        for dy in 0..height {
            for dx in 0..width {
                let pos = GridPos::new(top_left.x + dx, top_left.y + dy);
                if pos.x >= self.width || pos.y >= self.height || self.cells[pos.y][pos.x] != CellType::Empty {
                    return false;
                }
            }
        }
        true
    }
}

/// Represents an optimized tower placement zone
#[derive(Debug, Clone)]
pub struct TowerZone {
    /// Type of placement zone (grid-based or free-form)
    pub zone_type: PlacementZoneType,
    /// Grid boundaries (top-left, bottom-right)
    pub grid_bounds: (GridPos, GridPos),
    /// World coordinate boundaries  
    pub world_bounds: (Vec2, Vec2),
    /// Strategic value (higher = more important for defense)
    pub strategic_value: f32,
}

impl TowerZone {
    /// Create a new tower zone
    pub fn new(
        zone_type: PlacementZoneType,
        grid_bounds: (GridPos, GridPos),
        grid: &PathGrid,
        strategic_value: f32,
    ) -> Self {
        let world_top_left = grid.grid_to_world(grid_bounds.0);
        let world_bottom_right = grid.grid_to_world(grid_bounds.1);
        
        Self {
            zone_type,
            grid_bounds,
            world_bounds: (world_top_left, world_bottom_right),
            strategic_value,
        }
    }
    
    /// Calculate the area of this zone in grid cells
    pub fn area(&self) -> usize {
        let width = self.grid_bounds.1.x - self.grid_bounds.0.x + 1;
        let height = self.grid_bounds.1.y - self.grid_bounds.0.y + 1;
        width * height
    }
    
    /// Check if a world position is within this zone
    pub fn contains_world_pos(&self, world_pos: Vec2) -> bool {
        world_pos.x >= self.world_bounds.0.x.min(self.world_bounds.1.x) &&
        world_pos.x <= self.world_bounds.0.x.max(self.world_bounds.1.x) &&
        world_pos.y >= self.world_bounds.0.y.min(self.world_bounds.1.y) &&
        world_pos.y <= self.world_bounds.0.y.max(self.world_bounds.1.y)
    }
}