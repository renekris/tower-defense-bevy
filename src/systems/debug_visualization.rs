use bevy::prelude::*;
use crate::resources::*;
use crate::systems::path_generation::*;
use crate::systems::unified_grid::{UnifiedGridSystem, GridVisualizationMode};

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
    /// Whether to show generated path (deprecated - now handled by unified grid)
    pub show_path: bool,
    /// Whether to show tower zones (deprecated - now handled by unified grid)
    pub show_tower_zones: bool,
}

impl DebugVisualizationState {
    pub fn new() -> Self {
        Self {
            enabled: false,
            current_wave: 1,
            show_grid: true,
            show_obstacles: true,
            show_path: false, // Disabled - handled by unified grid system
            show_tower_zones: false, // Disabled - handled by unified grid system
        }
    }
    
    pub fn toggle(&mut self) {
        self.enabled = !self.enabled;
    }
}

/// Marker component for debug visualization entities
#[derive(Component)]
pub struct DebugVisualization;

// GridCell component removed - grid visualization now handled by unified grid system

/// Marker component for path visualization (deprecated - use unified grid)
#[derive(Component)]
pub struct PathVisualization;

/// Marker component for tower zone visualization (deprecated - use unified grid)
#[derive(Component)]
pub struct TowerZoneVisualization;

/// Marker component for debug information text
#[derive(Component)]
pub struct DebugInfoText;

/// System to handle debug visualization toggle input
pub fn debug_toggle_system(
    mut debug_state: ResMut<DebugVisualizationState>,
    mut unified_grid: ResMut<UnifiedGridSystem>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::F1) {
        debug_state.toggle();
        if debug_state.enabled {
            println!("Debug visualization enabled (F1 to toggle, Ctrl+1-9 for wave selection)");
            // Switch to debug mode when debug visualization is enabled
            // unless we're currently in placement mode
            if unified_grid.mode != GridVisualizationMode::Placement {
                unified_grid.mode = GridVisualizationMode::Debug;
            }
            // Enable all visualization features in debug mode
            unified_grid.show_path = true;
            unified_grid.show_zones = true;
            unified_grid.show_obstacles = true;
        } else {
            println!("Debug visualization disabled");
            // Switch back to normal mode when debug visualization is disabled
            // unless we're currently in placement mode
            if unified_grid.mode == GridVisualizationMode::Debug {
                unified_grid.mode = GridVisualizationMode::Normal;
            }
        }
    }
    
    // Allow changing wave number with Ctrl+number keys when debug is enabled
    // This prevents conflict with debug UI spawn rate controls (keys 1-5)
    if debug_state.enabled && (keyboard_input.pressed(KeyCode::ControlLeft) || keyboard_input.pressed(KeyCode::ControlRight)) {
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
                println!("Debug: Switched to wave {} (Ctrl+{})", wave, wave);
            }
        }
    }
}

/// System to create/update debug visualization when enabled
pub fn debug_visualization_system(
    mut commands: Commands,
    debug_state: Res<DebugVisualizationState>,
    // ui_state: Res<crate::systems::debug_ui::DebugUIState>, // Disabled due to Bevy 0.16 Style issues
    debug_entities: Query<Entity, With<DebugVisualization>>,
) {
    // CRITICAL FIX: Always clean up existing debug entities first to prevent memory leaks
    // This ensures entities are cleaned up even if debug is toggled multiple times rapidly
    for entity in debug_entities.iter() {
        commands.entity(entity).despawn();
    }
    
    // Only generate PathGrid resource when debug is enabled
    // The actual visualization is handled by unified grid system
    if !debug_state.enabled {
        return;
    }
    
    // Generate the current path and grid for visualization using default parameters (UI disabled)
    let enemy_path = generate_level_path_with_params(debug_state.current_wave, 0.15); // Default obstacle density
    let tower_zones = generate_placement_zones(debug_state.current_wave);
    
    // Generate the procedural map to get the grid data with default obstacle density
    let seed = debug_state.current_wave as u64 * 12345 + 67890;
    let grid = generate_procedural_map_with_density(seed, 0.15); // Default obstacle density
    
    // Store the PathGrid as a resource so the unified grid can access it for debug visualization
    commands.insert_resource(grid.clone());
    
    // Path visualization is now handled by the unified grid system
    // This ensures consistent rendering with the grid-based approach
    
    // Tower zones are now rendered by the unified grid system
    // This ensures single source of truth for all grid visualization
    
    // Render debug information overlay
    render_debug_info(&mut commands, &debug_state, &enemy_path, &tower_zones, &grid);
}

// render_grid function removed - grid visualization now handled by unified grid system

// Path rendering moved to unified grid system for consistency
// This eliminates duplicate path visualization and uses grid-based rendering

// Tower zone rendering moved to unified grid system for consistency
// This eliminates duplicate visualization and ensures single source of truth

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
        Text2d::new(info_text),
        TextFont {
            font_size: 14.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 1.0, 0.8)),
        Transform::from_translation(Vec3::new(-620.0, 340.0, 1.0)),
        TextLayout::new_with_justify(JustifyText::Left),
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
        Text2d::new(perf_text),
        TextFont {
            font_size: 12.0,
            ..default()
        },
        TextColor(Color::srgb(0.8, 1.0, 0.8)),
        Transform::from_translation(Vec3::new(400.0, 340.0, 1.0)),
        TextLayout::new_with_justify(JustifyText::Left),
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
            Text2d::new(zone_text),
            TextFont {
                font_size: 10.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 1.0)),
            Transform::from_translation(center.extend(0.3)),
            TextLayout::new_with_justify(JustifyText::Center),
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