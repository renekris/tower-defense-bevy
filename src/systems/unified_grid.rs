use bevy::prelude::*;
use crate::systems::path_generation::grid::{PathGrid, GridPos, CellType};

/// Different visualization modes for the unified grid system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GridVisualizationMode {
    /// Normal mode: Subtle grid lines for tower placement guidance
    #[default]
    Normal,
    /// Debug mode: Shows pathfinding data (obstacles, paths, tower zones)
    Debug,
    /// Placement mode: Highlights valid/invalid tower placement areas
    Placement,
}

/// Resource to manage the unified grid system state
#[derive(Resource, Debug)]
pub struct UnifiedGridSystem {
    pub mode: GridVisualizationMode,
    pub grid_entities: Vec<Entity>,
    pub grid_width: usize,
    pub grid_height: usize,
    pub cell_size: f32,
    pub show_grid: bool,
    pub show_path: bool,
    pub show_zones: bool,
    pub show_obstacles: bool,
}

impl UnifiedGridSystem {
    /// Get the total number of grid squares
    pub fn total_squares(&self) -> usize {
        self.grid_width * self.grid_height
    }

    /// Get grid area dimensions in pixels
    pub fn grid_area_size(&self) -> Vec2 {
        Vec2::new(
            self.grid_width as f32 * self.cell_size,
            self.grid_height as f32 * self.cell_size,
        )
    }
}

impl Default for UnifiedGridSystem {
    fn default() -> Self {
        Self {
            mode: GridVisualizationMode::Normal,
            grid_entities: Vec::new(),
            // Dense grid covering full game area - 1280x720 screen
            // Optimized for maximum density with 1:1 aspect ratio cells
            grid_width: 32,  // 1280 pixels / 40 = 32 cells
            grid_height: 18, // 720 pixels / 40 = 18 cells
            cell_size: 40.0, // 40x40 pixel squares (1:1 ratio)
            show_grid: true, // Always visible for dense grid
            show_path: true,
            show_zones: true,
            show_obstacles: true,
        }
    }
}

/// Component to identify grid tiles
#[derive(Component, Debug)]
pub struct GridTile {
    pub grid_pos: GridPos,
    pub cell_type: CellType,
}

/// System to setup the unified grid using simple sprites
pub fn setup_unified_grid(
    mut commands: Commands,
    mut unified_grid: ResMut<UnifiedGridSystem>,
) {
    // Calculate grid offset to center it on screen
    let grid_world_size = Vec2::new(
        unified_grid.grid_width as f32 * unified_grid.cell_size,
        unified_grid.grid_height as f32 * unified_grid.cell_size,
    );
    let grid_offset = -grid_world_size / 2.0;

    // Clear any existing grid entities
    unified_grid.grid_entities.clear();
    
    // Log dense grid information
    info!(
        "Dense Grid System Initialized: {}x{} = {} total squares ({}x{} pixels each)",
        unified_grid.grid_width,
        unified_grid.grid_height,
        unified_grid.total_squares(),
        unified_grid.cell_size as u32,
        unified_grid.cell_size as u32
    );
    info!(
        "Grid Coverage: {:.0}x{:.0} pixels (Full Screen: 1280x720)",
        unified_grid.grid_area_size().x,
        unified_grid.grid_area_size().y
    );

    // Spawn sprite entities for the entire grid
    for y in 0..unified_grid.grid_height {
        for x in 0..unified_grid.grid_width {
            let grid_pos = GridPos::new(x, y);
            
            // Calculate world position for this grid cell
            let world_pos = grid_offset + Vec2::new(
                x as f32 * unified_grid.cell_size + unified_grid.cell_size / 2.0,
                y as f32 * unified_grid.cell_size + unified_grid.cell_size / 2.0,
            );
            
            let entity = commands.spawn((
                Sprite {
                    color: Color::NONE, // Start invisible
                    custom_size: Some(Vec2::splat(unified_grid.cell_size)),
                    ..default()
                },
                Transform::from_translation(world_pos.extend(-0.1)),
                GridTile {
                    grid_pos,
                    cell_type: CellType::Empty,
                },
            )).id();
            
            unified_grid.grid_entities.push(entity);
        }
    }
}

/// System to update grid visualization based on current mode
pub fn update_grid_visualization(
    unified_grid: Res<UnifiedGridSystem>,
    path_grid: Option<Res<PathGrid>>,
    mut sprite_query: Query<(&GridTile, &mut Sprite)>,
) {
    if !unified_grid.is_changed() && path_grid.as_ref().map_or(false, |pg| !pg.is_changed()) {
        return;
    }

    for (grid_tile, mut sprite) in sprite_query.iter_mut() {
        let color = match unified_grid.mode {
            GridVisualizationMode::Normal => {
                // Always show grid lines for dense grid system
                // Bright enough to be clearly visible but not dominating
                Color::srgba(0.7, 0.7, 0.7, 0.4)
            },
            GridVisualizationMode::Debug => {
                if let Some(path_grid) = &path_grid {
                    let cell_type = path_grid.get_cell(grid_tile.grid_pos)
                        .unwrap_or(CellType::Empty);
                    
                    match cell_type {
                        CellType::Empty => {
                            if unified_grid.show_grid {
                                Color::srgba(0.0, 1.0, 0.0, 0.3) // Light green - valid placement
                            } else {
                                Color::NONE
                            }
                        },
                        CellType::Blocked => {
                            if unified_grid.show_obstacles {
                                Color::srgba(1.0, 0.0, 0.0, 0.7) // Bright red obstacles - invalid placement
                            } else {
                                Color::srgba(0.3, 0.3, 0.3, 0.3) // Gray outline when hidden
                            }
                        },
                        CellType::Path => {
                            if unified_grid.show_path {
                                Color::srgba(1.0, 0.0, 0.0, 0.6) // Red path - invalid placement
                            } else {
                                Color::srgba(0.3, 0.3, 0.3, 0.3) // Gray outline when hidden
                            }
                        },
                        CellType::TowerZone => {
                            if unified_grid.show_zones {
                                Color::srgba(0.2, 0.2, 0.8, 0.8) // Blue tower zones
                            } else {
                                Color::srgba(0.3, 0.3, 0.3, 0.3) // Gray outline when hidden
                            }
                        }
                    }
                } else {
                    Color::srgba(0.3, 0.3, 0.3, 0.3) // Default gray if no PathGrid
                }
            },
            GridVisualizationMode::Placement => {
                // Determine if this cell is a valid placement location using the correct logic
                let is_valid_placement = is_valid_placement_cell(
                    grid_tile.grid_pos,
                    &path_grid,
                    unified_grid.grid_width,
                    unified_grid.grid_height,
                );
                
                if is_valid_placement {
                    Color::srgba(0.0, 1.0, 0.0, 0.3) // Green for valid placement
                } else {
                    Color::srgba(1.0, 0.0, 0.0, 0.2) // Red for invalid placement (more subtle)
                }
            },
        };
        
        sprite.color = color;
    }
}

/// Utility function to snap world coordinates to grid
pub fn snap_to_grid(world_pos: Vec2, unified_grid: &UnifiedGridSystem) -> Vec2 {
    // Convert to grid coordinates and back to get snapped position
    let grid_offset = Vec2::new(
        -(unified_grid.grid_width as f32 * unified_grid.cell_size) / 2.0,
        -(unified_grid.grid_height as f32 * unified_grid.cell_size) / 2.0,
    );
    
    let relative_pos = world_pos - grid_offset;
    let grid_x = (relative_pos.x / unified_grid.cell_size).round();
    let grid_y = (relative_pos.y / unified_grid.cell_size).round();
    
    grid_offset + Vec2::new(
        grid_x * unified_grid.cell_size + unified_grid.cell_size / 2.0,
        grid_y * unified_grid.cell_size + unified_grid.cell_size / 2.0,
    )
}

/// Utility function to convert world coordinates to grid coordinates
pub fn world_to_grid(world_pos: Vec2, unified_grid: &UnifiedGridSystem) -> Option<GridPos> {
    let grid_offset = Vec2::new(
        -(unified_grid.grid_width as f32 * unified_grid.cell_size) / 2.0,
        -(unified_grid.grid_height as f32 * unified_grid.cell_size) / 2.0,
    );
    
    let relative_pos = world_pos - grid_offset;
    let grid_x = (relative_pos.x / unified_grid.cell_size).floor() as i32;
    let grid_y = (relative_pos.y / unified_grid.cell_size).floor() as i32;
    
    if grid_x >= 0 && grid_x < unified_grid.grid_width as i32 && 
       grid_y >= 0 && grid_y < unified_grid.grid_height as i32 {
        Some(GridPos::new(grid_x as usize, grid_y as usize))
    } else {
        None
    }
}

/// Utility function to convert grid coordinates to world coordinates
pub fn grid_to_world(grid_pos: GridPos, unified_grid: &UnifiedGridSystem) -> Vec2 {
    let grid_offset = Vec2::new(
        -(unified_grid.grid_width as f32 * unified_grid.cell_size) / 2.0,
        -(unified_grid.grid_height as f32 * unified_grid.cell_size) / 2.0,
    );
    
    grid_offset + Vec2::new(
        grid_pos.x as f32 * unified_grid.cell_size + unified_grid.cell_size / 2.0,
        grid_pos.y as f32 * unified_grid.cell_size + unified_grid.cell_size / 2.0,
    )
}

/// System to handle grid mode switching with F3 key
pub fn grid_mode_toggle_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut unified_grid: ResMut<UnifiedGridSystem>,
) {
    if keyboard_input.just_pressed(KeyCode::F3) {
        unified_grid.mode = match unified_grid.mode {
            GridVisualizationMode::Normal => GridVisualizationMode::Debug,
            GridVisualizationMode::Debug => GridVisualizationMode::Placement,
            GridVisualizationMode::Placement => GridVisualizationMode::Normal,
        };
        
        info!("Grid visualization mode changed to: {:?}", unified_grid.mode);
    }
    
    if keyboard_input.just_pressed(KeyCode::F4) {
        unified_grid.show_grid = !unified_grid.show_grid;
        info!("Grid visibility toggled: {}", unified_grid.show_grid);
    }
}

/// Helper function to determine if a grid cell is valid for tower placement
/// This uses the same logic as the tower placement system for consistency
fn is_valid_placement_cell(
    grid_pos: GridPos,
    path_grid: &Option<Res<PathGrid>>,
    grid_width: usize,
    grid_height: usize,
) -> bool {
    // Check bounds
    if grid_pos.x >= grid_width || grid_pos.y >= grid_height {
        return false;
    }
    
    // Check against PathGrid if available (this includes obstacles and path)
    if let Some(path_grid) = path_grid {
        match path_grid.get_cell(grid_pos) {
            Some(CellType::Empty) => true, // Allow placement on any empty cell
            Some(CellType::TowerZone) => true, // Explicitly designated tower zones
            Some(CellType::Path) => false, // Can't place on path
            Some(CellType::Blocked) => false, // Can't place on obstacles
            None => true, // Allow placement outside PathGrid bounds
        }
    } else {
        // Without PathGrid, allow placement everywhere (let other systems handle restrictions)
        true
    }
}