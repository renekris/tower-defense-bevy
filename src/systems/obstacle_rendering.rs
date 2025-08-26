use bevy::prelude::*;
use crate::systems::path_generation::{
    obstacles::{Obstacle, ObstacleType, create_obstacle_entities},
    PathGrid,
};
use crate::resources::{EnemyPath, WaveManager};

/// Resource to store the current obstacle grid for rendering
#[derive(Resource, Clone)]
pub struct ObstacleGrid {
    pub grid: PathGrid,
    pub wave_number: u32,
}

impl Default for ObstacleGrid {
    fn default() -> Self {
        Self {
            grid: PathGrid::new_unified(),
            wave_number: 0,
        }
    }
}

/// Component to mark entities that should be cleared when obstacles are regenerated
#[derive(Component)]
pub struct ObstacleEntity;

/// System to initialize obstacles for the first wave
pub fn setup_initial_obstacles(
    mut commands: Commands,
    mut obstacle_grid: ResMut<ObstacleGrid>,
    wave_manager: Res<WaveManager>,
) {
    // Generate initial obstacle grid for wave 1
    let seed = 1u64 * 12345 + 67890;
    let difficulty = (1.0_f32 / 20.0).min(1.0);
    
    // Generate procedural map with obstacles
    let grid = crate::systems::path_generation::obstacles::generate_procedural_map(seed, difficulty);
    
    // Store the grid
    obstacle_grid.grid = grid.clone();
    obstacle_grid.wave_number = 1;
    
    // Spawn obstacle entities
    create_obstacle_entities(&mut commands, &grid, seed + 5000);
    
    info!("Initialized obstacles for wave 1 with {} obstacles", count_obstacles(&grid));
}

/// System to update obstacles when wave changes
/// DISABLED: Obstacles should be static and not regenerate per wave
/// This system is kept but made inactive to prevent obstacles from changing
pub fn update_obstacles_on_wave_change(
    _commands: Commands,
    _obstacle_grid: ResMut<ObstacleGrid>,
    _wave_manager: Res<WaveManager>,
    _existing_obstacles: Query<Entity, With<Obstacle>>,
) {
    // This system has been disabled because obstacles should persist across all waves
    // Obstacles are static terrain features that paths navigate around
    // They are only generated once during initial setup
}

/// System to handle obstacle-tower collision detection
pub fn obstacle_tower_collision_system(
    obstacle_grid: Res<ObstacleGrid>,
    mut tower_placement_attempts: Local<Vec<Vec2>>, // Track recent placement attempts
    time: Res<Time>,
) {
    // This system could be extended to provide feedback when players try to place towers on obstacles
    // For now, it's a placeholder for future collision feedback
    
    // Clear old placement attempts (older than 1 second)
    tower_placement_attempts.retain(|_| false); // Simple clear for now
    
    // In the future, this could:
    // - Show red indicators when hovering towers over obstacles
    // - Provide audio/visual feedback for invalid placements
    // - Calculate optimal placement suggestions near obstacles
}

/// System to debug render obstacle information
pub fn debug_obstacle_info_system(
    obstacle_grid: Res<ObstacleGrid>,
    obstacles: Query<&Obstacle>,
    mut gizmos: Gizmos,
) {
    // Only render debug info if there are obstacles
    if obstacles.is_empty() {
        return;
    }
    
    // Optional: Draw grid lines to show obstacle placement
    let grid = &obstacle_grid.grid;
    let cell_size = grid.cell_size;
    let grid_width = grid.width as f32 * cell_size;
    let grid_height = grid.height as f32 * cell_size;
    
    // Calculate grid offset (same as in PathGrid::grid_to_world)
    let offset = Vec2::new(-grid_width / 2.0, -grid_height / 2.0);
    
    // Draw obstacle boundaries (subtle)
    for obstacle in obstacles.iter() {
        let world_pos = grid.grid_to_world(obstacle.position);
        let color = match obstacle.obstacle_type {
            ObstacleType::Rock => Color::srgba(0.4, 0.3, 0.2, 0.3),
            ObstacleType::Building => Color::srgba(0.6, 0.6, 0.7, 0.3),
            ObstacleType::Debris => Color::srgba(0.5, 0.4, 0.3, 0.3),
            ObstacleType::Crystal => Color::srgba(0.3, 0.5, 0.8, 0.3),
        };
        
        // Draw a subtle outline around obstacles
        let half_size = cell_size * 0.5;
        gizmos.rect_2d(world_pos, Vec2::new(cell_size, cell_size), color);
    }
}

/// Count obstacles in a grid for debugging
fn count_obstacles(grid: &PathGrid) -> usize {
    let mut count = 0;
    for y in 0..grid.height {
        for x in 0..grid.width {
            if let Some(crate::systems::path_generation::grid::CellType::Blocked) = 
                grid.get_cell(crate::systems::path_generation::grid::GridPos::new(x, y)) {
                count += 1;
            }
        }
    }
    count
}

/// System to validate obstacle placement doesn't break pathfinding
pub fn validate_obstacle_pathfinding(
    obstacle_grid: Res<ObstacleGrid>,
    enemy_path: Res<EnemyPath>,
) {
    // Validate that the current path is still valid with current obstacles
    // This is mainly for debugging and ensuring consistency
    
    if enemy_path.waypoints.len() < 2 {
        return;
    }
    
    // Check that path endpoints are not blocked
    let grid = &obstacle_grid.grid;
    let start_pos = enemy_path.waypoints.first().unwrap();
    let end_pos = enemy_path.waypoints.last().unwrap();
    
    // Convert world positions back to grid positions for validation
    if let (Some(start_grid), Some(end_grid)) = (
        grid.world_to_grid(*start_pos),
        grid.world_to_grid(*end_pos)
    ) {
        if !grid.is_traversable(start_grid) || !grid.is_traversable(end_grid) {
            warn!("Path endpoints are blocked by obstacles!");
        }
    }
}

/// Plugin to add obstacle rendering systems
pub struct ObstacleRenderingPlugin;

impl Plugin for ObstacleRenderingPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ObstacleGrid>()
            .add_systems(Startup, setup_initial_obstacles)
            .add_systems(Update, (
                update_obstacles_on_wave_change,
                obstacle_tower_collision_system,
                validate_obstacle_pathfinding,
            ))
            .add_systems(PostUpdate, debug_obstacle_info_system);
    }
}