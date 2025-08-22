use bevy::prelude::*;
use bevy::input::mouse::MouseButtonInput;
use bevy::window::PrimaryWindow;
use crate::resources::*;
use crate::components::*;
use crate::systems::combat_system::Target;
use crate::systems::tower_ui::TowerSelectionState;
use crate::systems::tower_rendering::spawn_tower_with_pattern;
use crate::systems::unified_grid::{UnifiedGridSystem, GridVisualizationMode, snap_to_grid, world_to_grid};
use crate::systems::obstacle_rendering::ObstacleGrid;

#[derive(Resource, Debug)]
pub struct MouseInputState {
    pub current_position: Vec2,
    pub world_position: Vec2,
    pub left_clicked: bool,
    pub right_clicked: bool,
    pub placement_mode: PlacementMode,
    pub preview_position: Option<Vec2>,
}

impl Default for MouseInputState {
    fn default() -> Self {
        Self {
            current_position: Vec2::ZERO,
            world_position: Vec2::ZERO,
            left_clicked: false,
            right_clicked: false,
            placement_mode: PlacementMode::Hybrid,
            preview_position: None,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum PlacementMode {
    #[default]
    None,
    GridBased,
    FreeForm,
    Hybrid,
}

#[derive(Component)]
pub struct PlacementPreview;

#[derive(Component)]
pub struct PlacementZoneMarker {
    pub zone_type: PlacementZoneType,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlacementZoneType {
    GridZone,
    FreeZone,
    RestrictedZone,
}

// Mouse input tracking system
pub fn mouse_input_system(
    mut mouse_state: ResMut<MouseInputState>,
    mut mouse_button_events: EventReader<MouseButtonInput>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    // Update mouse position
    if let Ok(window) = window_query.single() {
        if let Some(screen_pos) = window.cursor_position() {
            mouse_state.current_position = screen_pos;
            
            // Convert to world coordinates
            if let Ok((camera, camera_transform)) = camera_query.single() {
                mouse_state.world_position = screen_to_world_position(
                    screen_pos, 
                    camera_transform, 
                    camera,
                    window,
                );
            }
        }
    }

    // Handle mouse clicks
    mouse_state.left_clicked = false;
    mouse_state.right_clicked = false;

    for event in mouse_button_events.read() {
        if event.state.is_pressed() {
            match event.button {
                MouseButton::Left => mouse_state.left_clicked = true,
                MouseButton::Right => mouse_state.right_clicked = true,
                _ => {}
            }
        }
    }
}

/// Tower placement system - Enhanced with obstacle collision detection
pub fn tower_placement_system(
    mut commands: Commands,
    mouse_state: Res<MouseInputState>,
    tower_selection_state: Res<TowerSelectionState>,
    mut economy: ResMut<Economy>,
    existing_towers: Query<&Transform, With<TowerStats>>,
    enemy_path: Res<EnemyPath>,
    ui_interaction_query: Query<&Interaction, With<Button>>,
    unified_grid: Res<UnifiedGridSystem>,
    obstacle_grid: Res<ObstacleGrid>,
) {
    // CRITICAL SAFETY CHECK: Don't place towers if any UI button is being interacted with
    let ui_is_active = ui_interaction_query.iter().any(|interaction| {
        matches!(*interaction, Interaction::Pressed | Interaction::Hovered)
    });
    
    // Only attempt placement if we have a tower type selected and left click
    // AND we're in placement mode (not upgrade mode) AND no UI is being interacted with
    if tower_selection_state.is_placement_mode() && !ui_is_active {
        if let Some(tower_type) = tower_selection_state.selected_placement_type {
            if mouse_state.left_clicked {
                println!("Attempting to place {:?} at {:?}", tower_type, mouse_state.world_position);
                let placement_pos = get_placement_position(
                    mouse_state.world_position,
                    mouse_state.placement_mode,
                    &unified_grid,
                );

                // Validate placement using unified system (ensures consistency with red areas)
                if is_valid_tower_placement_unified(
                    placement_pos,
                    &existing_towers,
                    &enemy_path.waypoints,
                    &unified_grid,
                    Some(&obstacle_grid.grid),
                    40.0, // Tower size - exactly one grid cell
                ) {
                    let cost = tower_type.get_cost();
                    if economy.can_afford(&cost) {
                        // Place the tower
                        spawn_tower(&mut commands, placement_pos, tower_type);
                        economy.spend(&cost);
                        println!("Placed {:?} tower at {:?}", tower_type, placement_pos);
                    } else {
                        println!("Cannot afford {:?} tower", tower_type);
                    }
                } else {
                    println!("Invalid tower placement position");
                }
            }
        }
    }
}

/// Preview system for tower placement - Enhanced with obstacle collision detection
pub fn tower_placement_preview_system(
    mut commands: Commands,
    mouse_state: Res<MouseInputState>,
    tower_selection_state: Res<TowerSelectionState>,
    existing_previews: Query<Entity, With<PlacementPreview>>,
    economy: Res<Economy>,
    existing_towers: Query<&Transform, With<TowerStats>>,
    enemy_path: Res<EnemyPath>,
    unified_grid: Res<UnifiedGridSystem>,
    obstacle_grid: Res<ObstacleGrid>,
) {
    // Clear existing previews
    for entity in existing_previews.iter() {
        commands.entity(entity).despawn();
    }

    // Show preview if tower type is selected and we're in placement mode
    if tower_selection_state.is_placement_mode() {
        if let Some(tower_type) = tower_selection_state.selected_placement_type {
            let placement_pos = get_placement_position(
                mouse_state.world_position,
                mouse_state.placement_mode,
                &unified_grid,
            );

            let is_valid = is_valid_tower_placement_unified(
                placement_pos,
                &existing_towers,
                &enemy_path.waypoints,
                &unified_grid,
                Some(&obstacle_grid.grid),
                40.0, // Tower size - exactly one grid cell
            );

            let cost = tower_type.get_cost();
            let can_afford = economy.can_afford(&cost);
            let color = if is_valid && can_afford {
                Color::srgba(0.0, 1.0, 0.0, 0.5) // Green
            } else {
                Color::srgba(1.0, 0.0, 0.0, 0.5) // Red
            };

            // Spawn preview sprite
            commands.spawn((
                Sprite {
                    color,
                    custom_size: Some(Vec2::new(40.0, 40.0)), // Exactly one grid cell
                    ..default()
                },
                Transform::from_translation(placement_pos.extend(1.0)),
                PlacementPreview,
            ));

            // Show range indicator
            spawn_range_preview(&mut commands, placement_pos, tower_type);
        }
    }
}

// Utility functions
pub fn screen_to_world_position(
    screen_pos: Vec2,
    camera_transform: &GlobalTransform,
    _camera: &Camera,
    window: &Window,
) -> Vec2 {
    let window_size = Vec2::new(window.width(), window.height());
    
    // Convert screen coordinates to normalized device coordinates (NDC)
    let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
    
    // Flip Y coordinate (screen Y increases downward, world Y increases upward)
    let ndc = Vec2::new(ndc.x, -ndc.y);
    
    // Apply camera transform
    camera_transform.translation().truncate() + ndc * window_size * 0.5
}

pub fn get_placement_position(
    world_pos: Vec2,
    mode: PlacementMode,
    unified_grid: &UnifiedGridSystem,
) -> Vec2 {
    match mode {
        PlacementMode::GridBased => snap_to_grid(world_pos, unified_grid),
        PlacementMode::FreeForm => world_pos,
        PlacementMode::Hybrid => {
            // Use unified grid to determine if we're in a valid placement area
            if let Some(grid_pos) = world_to_grid(world_pos, unified_grid) {
                if is_valid_grid_placement(grid_pos, unified_grid) {
                    snap_to_grid(world_pos, unified_grid)
                } else {
                    world_pos // Free zone or fallback
                }
            } else {
                world_pos
            }
        }
        PlacementMode::None => world_pos,
    }
}

pub fn is_in_grid_zone(position: Vec2) -> bool {
    // Left or right grid zones 
    (position.x >= -400.0 && position.x <= -200.0 && position.y.abs() <= 200.0) ||
    (position.x >= 200.0 && position.x <= 400.0 && position.y.abs() <= 200.0)
}

/// Check if a grid position is valid for tower placement using unified grid logic
/// This function now delegates to the path grid system for consistent validation
pub fn is_valid_grid_placement(grid_pos: crate::systems::path_generation::grid::GridPos, unified_grid: &UnifiedGridSystem) -> bool {
    // Check bounds
    if grid_pos.x >= unified_grid.grid_width || grid_pos.y >= unified_grid.grid_height {
        return false;
    }
    
    // Always allow placement - let the PathGrid system handle the actual restrictions
    // This ensures consistency with the unified validation system
    true
}

pub fn is_in_free_zone(position: Vec2) -> bool {
    // Top or bottom free zones
    (position.y >= 125.0 && position.y <= 275.0 && position.x.abs() <= 300.0) ||
    (position.y >= -275.0 && position.y <= -125.0 && position.x.abs() <= 300.0)
}

pub fn is_valid_placement_position(position: Vec2, path_points: &[Vec2], min_distance: f32) -> bool {
    // Check distance to path
    for i in 0..path_points.len().saturating_sub(1) {
        let start = path_points[i];
        let end = path_points[i + 1];
        let distance = distance_to_line_segment(position, start, end);
        if distance < min_distance {
            return false;
        }
    }
    true
}

pub fn is_valid_tower_placement(
    position: Vec2,
    existing_towers: &Query<&Transform, With<TowerStats>>,
    path_points: &[Vec2],
    tower_size: f32,
) -> bool {
    // Check path collision
    if !is_valid_placement_position(position, path_points, tower_size) {
        return false;
    }

    // Check overlap with existing towers - use full tower size for collision
    for transform in existing_towers.iter() {
        let distance = position.distance(transform.translation.truncate());
        if distance < tower_size {
            return false;
        }
    }

    true
}

/// Enhanced tower placement validation that includes obstacle collision
pub fn is_valid_tower_placement_with_obstacles(
    position: Vec2,
    existing_towers: &Query<&Transform, With<TowerStats>>,
    path_points: &[Vec2],
    obstacle_grid: &crate::systems::path_generation::PathGrid,
    tower_size: f32,
) -> bool {
    // Check basic placement validity (path and existing towers)
    if !is_valid_tower_placement(position, existing_towers, path_points, tower_size) {
        return false;
    }

    // Check obstacle collision
    if let Some(grid_pos) = obstacle_grid.world_to_grid(position) {
        // Check if the tower position overlaps with any obstacle
        if !obstacle_grid.is_traversable(grid_pos) {
            return false;
        }
        
        // Check surrounding cells for obstacle collision (tower occupies full cell)
        let neighbors = grid_pos.neighbors(obstacle_grid.width, obstacle_grid.height);
        for neighbor in neighbors {
            let neighbor_world = obstacle_grid.grid_to_world(neighbor);
            let distance = position.distance(neighbor_world);
            
            // If neighbor obstacle is too close to tower center
            if distance < tower_size * 0.7 && !obstacle_grid.is_traversable(neighbor) {
                return false;
            }
        }
    }

    true
}

/// Unified tower placement validation that uses the same logic as grid visualization
/// This ensures consistency between red areas and actual placement blocking
pub fn is_valid_tower_placement_unified(
    position: Vec2,
    existing_towers: &Query<&Transform, With<TowerStats>>,
    path_points: &[Vec2],
    unified_grid: &UnifiedGridSystem,
    obstacle_grid: Option<&crate::systems::path_generation::PathGrid>,
    tower_size: f32,
) -> bool {
    // First check if this is within unified grid bounds
    if let Some(grid_pos) = crate::systems::unified_grid::world_to_grid(position, unified_grid) {
        // Use the PathGrid system for placement validation if available
        if let Some(path_grid) = obstacle_grid {
            match path_grid.get_cell(grid_pos) {
                Some(crate::systems::path_generation::grid::CellType::Empty) => {
                    // Allow placement on empty cells
                },
                Some(crate::systems::path_generation::grid::CellType::TowerZone) => {
                    // Allow placement on designated tower zones
                },
                Some(crate::systems::path_generation::grid::CellType::Path) => {
                    // Prevent placement on path cells
                    return false;
                },
                Some(crate::systems::path_generation::grid::CellType::Blocked) => {
                    // Prevent placement on blocked cells (obstacles)
                    return false;
                },
                None => {
                    // Allow placement outside PathGrid bounds (fallback behavior)
                }
            }
        }
    } else {
        // Outside unified grid bounds
        return false;
    }
    
    // Check existing tower overlaps - use full tower size for collision
    for transform in existing_towers.iter() {
        let distance = position.distance(transform.translation.truncate());
        if distance < tower_size {
            return false;
        }
    }
    
    // Check distance to path segments (additional safety check)
    for i in 0..path_points.len().saturating_sub(1) {
        let start = path_points[i];
        let end = path_points[i + 1];
        let distance = distance_to_line_segment(position, start, end);
        if distance < tower_size / 2.0 {
            return false;
        }
    }

    true
}

pub fn distance_to_line_segment(point: Vec2, line_start: Vec2, line_end: Vec2) -> f32 {
    let line_vec = line_end - line_start;
    let point_vec = point - line_start;
    
    if line_vec.length_squared() == 0.0 {
        return point_vec.length();
    }
    
    let t = (point_vec.dot(line_vec) / line_vec.length_squared()).clamp(0.0, 1.0);
    let projection = line_start + line_vec * t;
    point.distance(projection)
}

pub fn spawn_tower(commands: &mut Commands, position: Vec2, tower_type: TowerType) {
    // Use the new pattern-based tower spawning system
    spawn_tower_with_pattern(commands, position, tower_type);
}

pub fn spawn_range_preview(commands: &mut Commands, position: Vec2, tower_type: TowerType) {
    let tower_stats = TowerStats::new(tower_type);
    let range = tower_stats.range;
    
    commands.spawn((
        Sprite {
            color: Color::srgba(1.0, 1.0, 1.0, 0.1),
            custom_size: Some(Vec2::new(range * 2.0, range * 2.0)),
            ..default()
        },
        Transform::from_translation(position.extend(0.5)),
        PlacementPreview,
    ));
}

// Legacy placement zones removed - now handled by unified grid system

// Legacy grid line helper removed - now handled by unified grid system

/// System to automatically switch grid visualization mode based on tower selection
pub fn auto_grid_mode_system(
    tower_selection_state: Res<TowerSelectionState>,
    mut unified_grid: ResMut<UnifiedGridSystem>,
) {
    // Switch to placement mode when a tower type is selected for placement
    let desired_mode = if tower_selection_state.is_placement_mode() && tower_selection_state.selected_placement_type.is_some() {
        GridVisualizationMode::Placement
    } else {
        GridVisualizationMode::Normal
    };
    
    // Only update if mode needs to change to avoid triggering change detection unnecessarily
    if unified_grid.mode != desired_mode {
        unified_grid.mode = desired_mode;
    }
}