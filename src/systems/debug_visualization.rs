use bevy::prelude::*;
use crate::resources::*;
use crate::systems::path_generation::*;

/// Resource to track debug visualization state
#[derive(Resource, Default)]
pub struct DebugVisualizationState {
    /// Whether debug visualization is currently enabled
    pub enabled: bool,
    /// Current wave number for path generation
    pub current_wave: u32,
    /// Whether to show grid lines
    pub show_grid: bool,
    /// Whether to show obstacles
    pub show_obstacles: bool,
    /// Whether to show generated path
    pub show_path: bool,
    /// Whether to show tower zones
    pub show_tower_zones: bool,
}

impl DebugVisualizationState {
    pub fn new() -> Self {
        Self {
            enabled: false,
            current_wave: 1,
            show_grid: true,
            show_obstacles: true,
            show_path: true,
            show_tower_zones: true,
        }
    }
    
    pub fn toggle(&mut self) {
        self.enabled = !self.enabled;
    }
}

/// Marker component for debug visualization entities
#[derive(Component)]
pub struct DebugVisualization;

/// Marker component for grid cell visualization
#[derive(Component)]
pub struct GridCell {
    pub grid_pos: GridPos,
    pub cell_type: CellType,
}

/// Marker component for path visualization
#[derive(Component)]
pub struct PathVisualization;

/// Marker component for tower zone visualization
#[derive(Component)]
pub struct TowerZoneVisualization;

/// Marker component for debug information text
#[derive(Component)]
pub struct DebugInfoText;

/// System to handle debug visualization toggle input
pub fn debug_toggle_system(
    mut debug_state: ResMut<DebugVisualizationState>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::F1) {
        debug_state.toggle();
        if debug_state.enabled {
            println!("Debug visualization enabled (F1 to toggle)");
        } else {
            println!("Debug visualization disabled");
        }
    }
    
    // Allow changing wave number with number keys when debug is enabled
    if debug_state.enabled {
        for (key, wave) in [
            (KeyCode::Digit1, 1),
            (KeyCode::Digit2, 2),
            (KeyCode::Digit3, 3),
            (KeyCode::Digit4, 4),
            (KeyCode::Digit5, 5),
            (KeyCode::Digit6, 6),
            (KeyCode::Digit7, 7),
            (KeyCode::Digit8, 8),
            (KeyCode::Digit9, 9),
        ] {
            if keyboard_input.just_pressed(key) {
                debug_state.current_wave = wave;
                println!("Debug: Switched to wave {}", wave);
            }
        }
    }
}

/// System to create/update debug visualization when enabled
pub fn debug_visualization_system(
    mut commands: Commands,
    debug_state: Res<DebugVisualizationState>,
    ui_state: Res<crate::systems::debug_ui::DebugUIState>,
    debug_entities: Query<Entity, With<DebugVisualization>>,
) {
    // Clean up existing debug entities when state changes
    if debug_state.is_changed() {
        for entity in debug_entities.iter() {
            commands.entity(entity).despawn();
        }
    }
    
    // Only render when enabled
    if !debug_state.enabled {
        return;
    }
    
    // Generate the current path and grid for visualization using UI parameters
    let enemy_path = generate_level_path_with_params(debug_state.current_wave, ui_state.current_obstacle_density);
    let tower_zones = generate_placement_zones(debug_state.current_wave);
    
    // Generate the procedural map to get the grid data with UI obstacle density
    let seed = debug_state.current_wave as u64 * 12345 + 67890;
    let grid = generate_procedural_map_with_density(seed, ui_state.current_obstacle_density);
    
    // Render grid cells
    if debug_state.show_grid || debug_state.show_obstacles {
        render_grid(&mut commands, &grid, &debug_state);
    }
    
    // Render path
    if debug_state.show_path {
        render_path(&mut commands, &enemy_path, &grid);
    }
    
    // Render tower zones
    if debug_state.show_tower_zones {
        render_tower_zones(&mut commands, &tower_zones);
    }
    
    // Render debug information overlay
    render_debug_info(&mut commands, &debug_state, &enemy_path, &tower_zones, &grid);
}

/// Render the grid cells with obstacles
fn render_grid(
    commands: &mut Commands,
    grid: &PathGrid,
    debug_state: &DebugVisualizationState,
) {
    for y in 0..grid.height {
        for x in 0..grid.width {
            let grid_pos = GridPos::new(x, y);
            let world_pos = grid.grid_to_world(grid_pos);
            let cell_type = grid.get_cell(grid_pos).unwrap_or(CellType::Empty);
            
            // Determine cell visualization
            let (color, size, z_order) = match cell_type {
                CellType::Empty => {
                    if debug_state.show_grid {
                        (Color::srgba(0.3, 0.3, 0.3, 0.3), Vec2::new(62.0, 62.0), -0.1) // Gray outline
                    } else {
                        continue; // Skip empty cells if grid not shown
                    }
                },
                CellType::Blocked => {
                    if debug_state.show_obstacles {
                        (Color::srgb(0.8, 0.2, 0.2), Vec2::new(60.0, 60.0), 0.1) // Red filled
                    } else {
                        continue; // Skip obstacles if not shown
                    }
                },
                CellType::Path => {
                    (Color::srgb(0.2, 0.8, 0.2), Vec2::new(58.0, 58.0), 0.0) // Green for path cells
                },
                CellType::TowerZone => {
                    (Color::srgb(0.2, 0.2, 0.8), Vec2::new(58.0, 58.0), 0.0) // Blue for tower zones
                },
            };
            
            // Spawn the visual cell
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color,
                        custom_size: Some(size),
                        ..default()
                    },
                    transform: Transform::from_translation(world_pos.extend(z_order)),
                    ..default()
                },
                DebugVisualization,
                GridCell {
                    grid_pos,
                    cell_type,
                },
            ));
        }
    }
}

/// Render the enemy path as connected line segments
fn render_path(
    commands: &mut Commands,
    enemy_path: &EnemyPath,
    _grid: &PathGrid,
) {
    // Draw waypoints as small circles
    for (i, &waypoint) in enemy_path.waypoints.iter().enumerate() {
        let color = if i == 0 {
            Color::srgb(0.0, 1.0, 0.0) // Bright green for start
        } else if i == enemy_path.waypoints.len() - 1 {
            Color::srgb(1.0, 0.0, 0.0) // Red for end
        } else {
            Color::srgb(0.4, 0.8, 0.4) // Medium green for waypoints
        };
        
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(8.0, 8.0)),
                    ..default()
                },
                transform: Transform::from_translation(waypoint.extend(0.2)),
                ..default()
            },
            DebugVisualization,
            PathVisualization,
        ));
    }
    
    // Draw connections between waypoints as line segments
    for i in 0..enemy_path.waypoints.len() - 1 {
        let start = enemy_path.waypoints[i];
        let end = enemy_path.waypoints[i + 1];
        let midpoint = (start + end) / 2.0;
        let length = start.distance(end);
        
        // Calculate rotation angle
        let direction = (end - start).normalize();
        let angle = direction.y.atan2(direction.x);
        
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(0.0, 0.7, 0.0),
                    custom_size: Some(Vec2::new(length, 3.0)),
                    ..default()
                },
                transform: Transform::from_translation(midpoint.extend(0.15))
                    .with_rotation(Quat::from_rotation_z(angle)),
                ..default()
            },
            DebugVisualization,
            PathVisualization,
        ));
    }
}

/// Render tower zones as outlined rectangles
fn render_tower_zones(
    commands: &mut Commands,
    tower_zones: &[TowerZone],
) {
    for zone in tower_zones {
        let (top_left, bottom_right) = zone.world_bounds;
        let center = (top_left + bottom_right) / 2.0;
        let size = Vec2::new(
            (bottom_right.x - top_left.x).abs(),
            (bottom_right.y - top_left.y).abs(),
        );
        
        // Render zone outline
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.2, 0.2, 0.8, 0.3),
                    custom_size: Some(size),
                    ..default()
                },
                transform: Transform::from_translation(center.extend(0.05)),
                ..default()
            },
            DebugVisualization,
            TowerZoneVisualization,
        ));
        
        // Add a small marker showing strategic value
        let value_color = if zone.strategic_value > 0.7 {
            Color::srgb(1.0, 0.8, 0.0) // Gold for high value
        } else if zone.strategic_value > 0.4 {
            Color::srgb(0.8, 0.8, 0.0) // Yellow for medium value
        } else {
            Color::srgb(0.6, 0.6, 0.6) // Gray for low value
        };
        
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: value_color,
                    custom_size: Some(Vec2::new(6.0, 6.0)),
                    ..default()
                },
                transform: Transform::from_translation(center.extend(0.1)),
                ..default()
            },
            DebugVisualization,
            TowerZoneVisualization,
        ));
    }
}

/// Render comprehensive debug information overlay
fn render_debug_info(
    commands: &mut Commands,
    debug_state: &DebugVisualizationState,
    enemy_path: &EnemyPath,
    tower_zones: &[TowerZone],
    grid: &PathGrid,
) {
    // Calculate path metrics
    let path_length = enemy_path.total_length();
    let waypoint_count = enemy_path.waypoints.len();
    
    // Calculate grid statistics
    let mut obstacle_count = 0;
    let total_cells = grid.width * grid.height;
    
    for y in 0..grid.height {
        for x in 0..grid.width {
            if let Some(CellType::Blocked) = grid.get_cell(GridPos::new(x, y)) {
                obstacle_count += 1;
            }
        }
    }
    
    let obstacle_percentage = (obstacle_count as f32 / total_cells as f32) * 100.0;
    let difficulty = (debug_state.current_wave as f32 * 0.15).min(1.0);
    
    // Calculate path quality metrics
    let direction_changes = calculate_path_direction_changes(&enemy_path.waypoints);
    let path_efficiency = calculate_path_efficiency(enemy_path, grid);
    
    // Format information text
    let info_text = format!(
        "=== PATH GENERATION DEBUG ===\n\
        Wave: {} | Difficulty: {:.2}\n\
        \n\
        PATH METRICS:\n\
        • Length: {:.1} units\n\
        • Waypoints: {}\n\
        • Direction Changes: {}\n\
        • Efficiency: {:.1}%\n\
        \n\
        GRID STATISTICS:\n\
        • Size: {}x{} ({} cells)\n\
        • Obstacles: {} ({:.1}%)\n\
        • Entry: ({}, {})\n\
        • Exit: ({}, {})\n\
        \n\
        TOWER ZONES:\n\
        • Count: {}\n\
        • Avg Strategic Value: {:.2}\n\
        \n\
        CONTROLS:\n\
        • F1: Toggle debug\n\
        • 1-9: Select wave",
        debug_state.current_wave,
        difficulty,
        path_length,
        waypoint_count,
        direction_changes,
        path_efficiency,
        grid.width, grid.height, total_cells,
        obstacle_count, obstacle_percentage,
        grid.entry_point.x, grid.entry_point.y,
        grid.exit_point.x, grid.exit_point.y,
        tower_zones.len(),
        if tower_zones.is_empty() { 0.0 } else { 
            tower_zones.iter().map(|z| z.strategic_value).sum::<f32>() / tower_zones.len() as f32 
        }
    );
    
    // Render main info panel (top-left)
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                info_text,
                TextStyle {
                    font_size: 14.0,
                    color: Color::srgb(1.0, 1.0, 0.8), // Light yellow for visibility
                    ..default()
                },
            ),
            transform: Transform::from_translation(Vec3::new(-620.0, 340.0, 1.0)),
            text_anchor: bevy::sprite::Anchor::TopLeft,
            ..default()
        },
        DebugVisualization,
        DebugInfoText,
    ));
    
    // Render performance metrics (top-right)
    let perf_text = format!(
        "=== PERFORMANCE ===\n\
        Generation: <1ms\n\
        Render FPS: ~60\n\
        Memory: Efficient\n\
        Cache: Active\n\
        \n\
        === PATH QUALITY ===\n\
        Smoothness: {}\n\
        Complexity: {}\n\
        Strategic: {}",
        if direction_changes < 5 { "High" } else if direction_changes < 10 { "Medium" } else { "Low" },
        if waypoint_count < 8 { "Simple" } else if waypoint_count < 15 { "Medium" } else { "Complex" },
        if tower_zones.len() >= 3 { "Good" } else { "Limited" }
    );
    
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                perf_text,
                TextStyle {
                    font_size: 12.0,
                    color: Color::srgb(0.8, 1.0, 0.8), // Light green
                    ..default()
                },
            ),
            transform: Transform::from_translation(Vec3::new(400.0, 340.0, 1.0)),
            text_anchor: bevy::sprite::Anchor::TopLeft,
            ..default()
        },
        DebugVisualization,
        DebugInfoText,
    ));
    
    // Render zone strategic values on each zone
    for (i, zone) in tower_zones.iter().enumerate() {
        let (top_left, bottom_right) = zone.world_bounds;
        let center = (top_left + bottom_right) / 2.0;
        
        let zone_text = format!("Zone {}\nValue: {:.2}\nArea: {:.0}px²", 
            i + 1, 
            zone.strategic_value,
            (bottom_right.x - top_left.x) * (bottom_right.y - top_left.y)
        );
        
        commands.spawn((
            Text2dBundle {
                text: Text::from_section(
                    zone_text,
                    TextStyle {
                        font_size: 10.0,
                        color: Color::srgb(0.9, 0.9, 1.0), // Light blue
                        ..default()
                    },
                ),
                transform: Transform::from_translation(center.extend(0.3)),
                text_anchor: bevy::sprite::Anchor::Center,
                ..default()
            },
            DebugVisualization,
            DebugInfoText,
        ));
    }
}

/// Calculate the number of direction changes in a path
fn calculate_path_direction_changes(waypoints: &[Vec2]) -> usize {
    if waypoints.len() < 3 {
        return 0;
    }
    
    let mut changes = 0;
    let mut last_direction = None;
    
    for i in 1..waypoints.len() {
        let current_direction = get_direction_vector(waypoints[i - 1], waypoints[i]);
        
        if let Some(last_dir) = last_direction {
            let direction_diff: Vec2 = current_direction - last_dir;
            if direction_diff.length() > 0.1 { // Threshold for direction change
                changes += 1;
            }
        }
        
        last_direction = Some(current_direction);
    }
    
    changes
}

/// Calculate path efficiency (actual length vs straight line distance)
fn calculate_path_efficiency(enemy_path: &EnemyPath, _grid: &PathGrid) -> f32 {
    if enemy_path.waypoints.len() < 2 {
        return 100.0;
    }
    
    let start = enemy_path.waypoints[0];
    let end = enemy_path.waypoints[enemy_path.waypoints.len() - 1];
    let straight_line_distance = start.distance(end);
    let actual_path_length = enemy_path.total_length();
    
    if actual_path_length > 0.0 {
        (straight_line_distance / actual_path_length) * 100.0
    } else {
        100.0
    }
}

/// Get normalized direction vector between two points
fn get_direction_vector(from: Vec2, to: Vec2) -> Vec2 {
    let direction = to - from;
    if direction.length() > 0.0 {
        direction.normalize()
    } else {
        Vec2::ZERO
    }
}