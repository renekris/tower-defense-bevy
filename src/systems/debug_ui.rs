use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;
use crate::systems::debug_visualization::DebugVisualizationState;

/// Plugin for interactive debug UI controls
pub struct DebugUIPlugin;

impl Plugin for DebugUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<DebugUIState>()
            .init_resource::<SliderDragState>()
            .init_resource::<PerformanceMetrics>()
            .add_systems(Startup, setup_debug_ui)
            // TODO: Fix type alias issues with complex function signatures
            // .add_systems(Update, debug_ui_toggle_system)
            // .add_systems(Update, update_debug_ui_visibility)
            // .add_systems(Update, handle_toggle_button_interactions)
            // .add_systems(Update, handle_slider_interactions)
            // .add_systems(Update, handle_action_buttons)
            // .add_systems(Update, handle_debug_keyboard_shortcuts)
            // .add_systems(Update, update_slider_values)
            // .add_systems(Update, update_enemy_path_from_ui)
            // .add_systems(Update, update_spawn_rate_from_ui)
            // .add_systems(Update, update_performance_metrics)
            // .add_systems(Update, update_performance_display)
            // .add_systems(Update, sync_ui_with_debug_state);
            ;
    }
}

/// Resource to manage debug UI state
#[derive(Resource, Debug)]
pub struct DebugUIState {
    pub panel_visible: bool,
    pub grid_visible: bool,
    pub path_visible: bool,
    pub zones_visible: bool,
    pub performance_visible: bool,
    pub current_difficulty: f32,
    pub current_obstacle_density: f32,
    pub enemy_spawn_rate: f32,
    pub tower_damage_multiplier: f32,
    pub current_wave: u32,
}

impl Default for DebugUIState {
    fn default() -> Self {
        Self {
            panel_visible: false,
            grid_visible: true,
            path_visible: true,
            zones_visible: true,
            performance_visible: true,
            current_difficulty: 0.15,
            current_obstacle_density: 0.15,
            enemy_spawn_rate: 1.0,
            tower_damage_multiplier: 1.0,
            current_wave: 1,
        }
    }
}

/// Component marker for the debug UI panel
#[derive(Component)]
pub struct DebugUIPanel;

/// Component marker for UI sections
#[derive(Component)]
pub struct DebugUISection {
    pub section_type: UISectionType,
}

#[derive(Debug, Clone, Copy)]
pub enum UISectionType {
    Controls,
    Metrics,
    Parameters,
    Actions,
    Help,
}

/// Component for toggle buttons
#[derive(Component)]
pub struct ToggleButton {
    pub toggle_type: ToggleType,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ToggleType {
    Grid,
    Path,
    Zones,
    Performance,
}

/// Component for parameter sliders
#[derive(Component)]
pub struct ParameterSlider {
    pub slider_type: SliderType,
    pub min_value: f32,
    pub max_value: f32,
    pub current_value: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SliderType {
    ObstacleDensity,
    EnemySpawnRate,
    TowerDamageMultiplier,
}

/// Component for slider handle (draggable part)
#[derive(Component)]
pub struct SliderHandle {
    pub slider_type: SliderType,
}

/// Component for slider track (background bar)
#[derive(Component)]
pub struct SliderTrack {
    pub slider_type: SliderType,
}

/// System to handle F2 key toggle for debug UI
pub fn debug_ui_toggle_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut ui_state: ResMut<DebugUIState>,
) {
    if keyboard_input.just_pressed(KeyCode::F2) {
        ui_state.panel_visible = !ui_state.panel_visible;
        println!("Debug UI panel: {}", if ui_state.panel_visible { "enabled" } else { "disabled" });
    }
}

/// System to update UI panel visibility based on state
pub fn update_debug_ui_visibility(
    ui_state: Res<DebugUIState>,
    mut panel_query: Query<&mut Style, With<DebugUIPanel>>,
) {
    if ui_state.is_changed() {
        for mut style in &mut panel_query {
            style.display = if ui_state.panel_visible {
                Display::Flex
            } else {
                Display::None
            };
        }
    }
}

/// Setup the debug UI panel structure
pub fn setup_debug_ui(mut commands: Commands) {
    // Main debug panel container - positioned on the right side
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    right: Val::Px(0.0),
                    top: Val::Px(0.0),
                    width: Val::Px(300.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(10.0)),
                    display: Display::None, // Hidden by default
                    ..default()
                },
                background_color: Color::srgba(0.0, 0.0, 0.0, 0.8).into(),
                z_index: ZIndex::Global(1000),
                ..default()
            },
            DebugUIPanel,
        ))
        .with_children(|parent| {
            // Header
            parent.spawn(TextBundle::from_section(
                "Debug Controls (F2)",
                TextStyle {
                    font_size: 18.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));

            // Spacer
            parent.spawn(NodeBundle {
                style: Style {
                    height: Val::Px(10.0),
                    ..default()
                },
                ..default()
            });

            // Controls Section
            create_ui_section(parent, UISectionType::Controls);
            
            // Metrics Section
            create_ui_section(parent, UISectionType::Metrics);
            
            // Parameters Section
            create_ui_section(parent, UISectionType::Parameters);
            
            // Actions Section
            create_ui_section(parent, UISectionType::Actions);
            
            // Help Section
            create_ui_section(parent, UISectionType::Help);
        });
}

/// Helper function to create UI sections
fn create_ui_section(parent: &mut ChildBuilder, section_type: UISectionType) {
    let section_title = match section_type {
        UISectionType::Controls => "Visualization Controls",
        UISectionType::Metrics => "Performance Metrics",
        UISectionType::Parameters => "Generation Parameters",
        UISectionType::Actions => "Actions",
        UISectionType::Help => "Keyboard Shortcuts",
    };

    parent
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    margin: UiRect::vertical(Val::Px(5.0)),
                    padding: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
                background_color: Color::srgba(0.2, 0.2, 0.2, 0.5).into(),
                ..default()
            },
            DebugUISection { section_type },
        ))
        .with_children(|section| {
            // Section header
            section.spawn(TextBundle::from_section(
                section_title,
                TextStyle {
                    font_size: 14.0,
                    color: Color::srgb(0.8, 0.8, 1.0),
                    ..default()
                },
            ));

            // Section content
            match section_type {
                UISectionType::Controls => {
                    create_toggle_buttons(section);
                }
                UISectionType::Parameters => {
                    create_parameter_sliders(section);
                }
                UISectionType::Metrics => {
                    create_performance_metrics_display(section);
                }
                UISectionType::Actions => {
                    // Create a flex container for the action buttons in a 2x2 grid
                    section
                        .spawn(NodeBundle {
                            style: Style {
                                display: Display::Flex,
                                flex_direction: FlexDirection::Row,
                                flex_wrap: FlexWrap::Wrap,
                                justify_content: JustifyContent::SpaceBetween,
                                align_items: AlignItems::Center,
                                width: Val::Percent(100.0),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|buttons_container| {
                            create_action_buttons(buttons_container);
                        });
                }
                UISectionType::Help => {
                    create_help_section(section);
                }
            }
        });
}

/// Helper function to create toggle buttons for the Controls section
fn create_toggle_buttons(parent: &mut ChildBuilder) {
    let toggle_types = [
        (ToggleType::Grid, "Grid"),
        (ToggleType::Path, "Path"),
        (ToggleType::Zones, "Zones"),
        (ToggleType::Performance, "Performance"),
    ];

    for (toggle_type, label) in toggle_types {
        parent
            .spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(30.0),
                        margin: UiRect::vertical(Val::Px(2.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::srgba(0.2, 0.7, 0.2, 0.8).into(), // Start green (active)
                    ..default()
                },
                ToggleButton { toggle_type },
            ))
            .with_children(|button| {
                button.spawn(TextBundle::from_section(
                    label,
                    TextStyle {
                        font_size: 14.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ));
            });
    }
}

/// System to handle toggle button interactions
pub fn handle_toggle_button_interactions(
    mut interaction_query: Query<
        (&Interaction, &ToggleButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut debug_state: ResMut<DebugVisualizationState>,
) {
    for (interaction, toggle_button, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                // Toggle the corresponding debug state
                match toggle_button.toggle_type {
                    ToggleType::Grid => {
                        debug_state.show_grid = !debug_state.show_grid;
                        println!("Grid visualization: {}", debug_state.show_grid);
                    }
                    ToggleType::Path => {
                        debug_state.show_path = !debug_state.show_path;
                        println!("Path visualization: {}", debug_state.show_path);
                    }
                    ToggleType::Zones => {
                        debug_state.show_tower_zones = !debug_state.show_tower_zones;
                        println!("Zone visualization: {}", debug_state.show_tower_zones);
                    }
                    ToggleType::Performance => {
                        // Toggle performance metrics (placeholder for now)
                        println!("Performance metrics toggled");
                    }
                }
            }
            Interaction::Hovered => {
                *color = Color::srgba(0.3, 0.3, 0.7, 0.8).into();
            }
            Interaction::None => {
                // Return to normal color based on toggle state
                let is_active = match toggle_button.toggle_type {
                    ToggleType::Grid => debug_state.show_grid,
                    ToggleType::Path => debug_state.show_path,
                    ToggleType::Zones => debug_state.show_tower_zones,
                    ToggleType::Performance => true, // Always true for now
                };
                
                *color = if is_active {
                    Color::srgba(0.2, 0.7, 0.2, 0.8).into() // Green when active
                } else {
                    Color::srgba(0.7, 0.2, 0.2, 0.8).into() // Red when inactive
                };
            }
        }
    }
}

/// System to sync UI state with debug visualization state
pub fn sync_ui_with_debug_state(
    debug_state: Res<DebugVisualizationState>,
    mut toggle_query: Query<(&ToggleButton, &mut BackgroundColor), With<Button>>,
) {
    if debug_state.is_changed() {
        for (toggle_button, mut color) in &mut toggle_query {
            let is_active = match toggle_button.toggle_type {
                ToggleType::Grid => debug_state.show_grid,
                ToggleType::Path => debug_state.show_path,
                ToggleType::Zones => debug_state.show_tower_zones,
                ToggleType::Performance => true, // Always true for now
            };
            
            *color = if is_active {
                Color::srgba(0.2, 0.7, 0.2, 0.8).into() // Green when active
            } else {
                Color::srgba(0.7, 0.2, 0.2, 0.8).into() // Red when inactive
            };
        }
    }
}

/// Helper function to create parameter sliders for the Parameters section
fn create_parameter_sliders(parent: &mut ChildBuilder) {
    let slider_configs = [
        (SliderType::ObstacleDensity, "Obstacle Density", 0.0, 0.5, 0.15, "Map complexity (0.0=simple, 0.5=complex). Press M to randomize."),
        (SliderType::EnemySpawnRate, "Enemy Spawn Rate", 0.5, 3.0, 1.0, "Spawn frequency (0.5=slow, 3.0=fast). Press 1-5 for presets."),
        (SliderType::TowerDamageMultiplier, "Tower Damage", 0.5, 2.0, 1.0, "Damage scaling (1.0=normal, 2.0=double). Press +/- to adjust."),
    ];

    for (slider_type, label, min_val, max_val, default_val, tooltip) in slider_configs {
        // Slider container with label
        parent
            .spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    margin: UiRect::vertical(Val::Px(3.0)),
                    ..default()
                },
                ..default()
            })
            .with_children(|slider_container| {
                // Label with current value
                slider_container.spawn((
                    TextBundle::from_section(
                        format!("{}: {:.2}", label, default_val),
                        TextStyle {
                            font_size: 11.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ),
                    SliderValueText { slider_type },
                ));

                // Tooltip text
                slider_container.spawn(TextBundle::from_section(
                    tooltip,
                    TextStyle {
                        font_size: 9.0,
                        color: Color::srgb(0.7, 0.7, 0.8), // Light blue-gray for tooltips
                        ..default()
                    },
                ));

                // Slider track and handle container
                slider_container
                    .spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Px(20.0),
                                margin: UiRect::vertical(Val::Px(2.0)),
                                position_type: PositionType::Relative,
                                ..default()
                            },
                            background_color: Color::srgba(0.3, 0.3, 0.3, 0.8).into(),
                            ..default()
                        },
                        SliderTrack { slider_type },
                    ))
                    .with_children(|track| {
                        // Slider handle (draggable)
                        let handle_position = ((default_val - min_val) / (max_val - min_val)) * 100.0;
                        track.spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(16.0),
                                    height: Val::Px(16.0),
                                    position_type: PositionType::Absolute,
                                    left: Val::Percent(handle_position - 2.0), // Center handle
                                    top: Val::Px(2.0),
                                    ..default()
                                },
                                background_color: Color::srgba(0.8, 0.8, 0.8, 1.0).into(),
                                ..default()
                            },
                            SliderHandle { slider_type },
                            ParameterSlider {
                                slider_type,
                                min_value: min_val,
                                max_value: max_val,
                                current_value: default_val,
                            },
                        ));
                    });
            });
    }
}

/// Component for slider value text display
#[derive(Component)]
pub struct SliderValueText {
    pub slider_type: SliderType,
}

/// Resource to track which slider is being dragged
#[derive(Resource, Default)]
pub struct SliderDragState {
    pub dragging: Option<SliderType>,
}

/// Resource to track performance metrics
#[derive(Resource)]
pub struct PerformanceMetrics {
    pub fps: f32,
    pub frame_time_ms: f32,
    pub entity_count: usize,
    pub path_generation_time_ms: f32,
    pub last_update_time: f32,
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            fps: 60.0,
            frame_time_ms: 16.67,
            entity_count: 0,
            path_generation_time_ms: 0.0,
            last_update_time: 0.0,
        }
    }
}

/// System to handle slider interactions (clicking and basic feedback)
pub fn handle_slider_interactions(
    mut interaction_query: Query<
        (&Interaction, &ParameterSlider, &mut BackgroundColor),
        (Changed<Interaction>, With<SliderHandle>),
    >,
    mut drag_state: ResMut<SliderDragState>,
) {
    for (interaction, slider, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                drag_state.dragging = Some(slider.slider_type);
                *color = Color::srgba(0.6, 0.6, 1.0, 1.0).into(); // Blue when dragging
                println!("Started dragging slider: {:?}", slider.slider_type);
            }
            Interaction::Hovered => {
                *color = Color::srgba(1.0, 1.0, 1.0, 1.0).into(); // White when hovered
            }
            Interaction::None => {
                *color = Color::srgba(0.8, 0.8, 0.8, 1.0).into(); // Gray when normal
            }
        }
    }
}

/// System to handle slider value changes and update UI state
pub fn update_slider_values(
    mut slider_query: Query<(&mut ParameterSlider, &mut Style), With<SliderHandle>>,
    mut ui_state: ResMut<DebugUIState>,
    mut text_query: Query<(&SliderValueText, &mut Text)>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut drag_state: ResMut<SliderDragState>,
    windows: Query<&Window>,
    _camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    // Stop dragging when mouse is released
    if !mouse_input.pressed(MouseButton::Left)
        && drag_state.dragging.is_some() {
            println!("Stopped dragging slider");
            drag_state.dragging = None;
        }

    // Update slider values based on mouse position when dragging
    if let Some(dragging_type) = drag_state.dragging {
        if let Ok(window) = windows.get_single() {
            if let Some(mouse_pos) = window.cursor_position() {
                // Find the dragging slider and update its position
                for (mut slider, mut style) in &mut slider_query {
                    if slider.slider_type == dragging_type {
                        // Simple horizontal drag logic - convert mouse position to slider value
                        // This is a simplified version - in practice you'd need proper track bounds
                        let window_width = window.width();
                        let slider_width = 250.0; // Approximate slider track width
                        let slider_right = window_width - 50.0; // Account for panel positioning
                        let slider_left = slider_right - slider_width;
                        
                        // Calculate relative position within slider bounds
                        let mouse_x = mouse_pos.x;
                        let relative_pos = if mouse_x >= slider_left && mouse_x <= slider_right {
                            (mouse_x - slider_left) / slider_width
                        } else {
                            // Clamp to slider bounds
                            if mouse_x < slider_left { 0.0 } else { 1.0 }
                        };
                        
                        // Convert to slider value
                        let new_value = slider.min_value + (slider.max_value - slider.min_value) * relative_pos;
                        slider.current_value = new_value.clamp(slider.min_value, slider.max_value);
                        
                        // Update handle position
                        let handle_pos = ((slider.current_value - slider.min_value) / 
                            (slider.max_value - slider.min_value)) * 100.0;
                        style.left = Val::Percent(handle_pos - 2.0); // Center the handle
                        
                        println!("Slider {:?}: {:.2}", slider.slider_type, slider.current_value);
                        break;
                    }
                }
            }
        }
    }

    // Update UI state based on slider values
    for (slider, _) in &slider_query {
        match slider.slider_type {
            SliderType::ObstacleDensity => {
                ui_state.current_obstacle_density = slider.current_value;
            }
            SliderType::EnemySpawnRate => {
                ui_state.enemy_spawn_rate = slider.current_value;
            }
            SliderType::TowerDamageMultiplier => {
                ui_state.tower_damage_multiplier = slider.current_value;
            }
        }
    }

    // Update text displays
    for (text_info, mut text) in &mut text_query {
        if let Some((slider, _)) = slider_query.iter().find(|(s, _)| s.slider_type == text_info.slider_type) {
            let label = match slider.slider_type {
                SliderType::ObstacleDensity => "Obstacle Density",
                SliderType::EnemySpawnRate => "Enemy Spawn Rate",
                SliderType::TowerDamageMultiplier => "Tower Damage",
            };
            text.sections[0].value = format!("{}: {:.2}", label, slider.current_value);
        }
    }
}

/// Component marker for path line segments in the main game view
#[derive(Component)]
pub struct GamePathLine;

/// System to update the enemy path resource when UI parameters change
pub fn update_enemy_path_from_ui(
    mut commands: Commands,
    ui_state: Res<DebugUIState>,
    debug_state: Res<crate::systems::debug_visualization::DebugVisualizationState>,
    mut enemy_path: ResMut<EnemyPath>,
    path_line_query: Query<Entity, With<GamePathLine>>,
) {
    // Only update if UI state has changed and debug is enabled
    if ui_state.is_changed() && debug_state.enabled {
        // Generate new path with current UI parameters
        let new_path = crate::systems::path_generation::generate_level_path_with_params(
            debug_state.current_wave,
            ui_state.current_obstacle_density
        );
        
        // Remove old path line visualization
        for entity in path_line_query.iter() {
            commands.entity(entity).despawn();
        }
        
        // Create new path line visualization
        for i in 0..new_path.waypoints.len() - 1 {
            let start = new_path.waypoints[i];
            let end = new_path.waypoints[i + 1];
            let midpoint = (start + end) / 2.0;
            let length = start.distance(end);
            let angle = (end - start).angle_between(Vec2::X);

            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgb(0.3, 0.3, 0.3),
                        custom_size: Some(Vec2::new(length, 4.0)),
                        ..default()
                    },
                    transform: Transform::from_translation(midpoint.extend(-1.0))
                        .with_rotation(Quat::from_rotation_z(angle)),
                    ..default()
                },
                GamePathLine,
            ));
        }
        
        // Update the enemy path resource that the game systems use
        *enemy_path = new_path;
        println!("Updated enemy path with obstacle density: {:.2}", ui_state.current_obstacle_density);
    }
}

/// System to update enemy spawn rate when UI parameters change
pub fn update_spawn_rate_from_ui(
    ui_state: Res<DebugUIState>,
    debug_state: Res<crate::systems::debug_visualization::DebugVisualizationState>,
    mut wave_manager: ResMut<crate::resources::WaveManager>,
) {
    // Only update if UI state has changed and debug is enabled
    if ui_state.is_changed() && debug_state.enabled {
        // Update the wave manager spawn rate based on the slider value
        wave_manager.set_spawn_rate(ui_state.enemy_spawn_rate);
        println!("Updated enemy spawn rate: {:.2} (interval: {:.2}s)", 
            ui_state.enemy_spawn_rate, 
            1.0 / ui_state.enemy_spawn_rate.max(0.1));
    }
}

/// Component for performance metric text displays
#[derive(Component)]
pub struct PerformanceMetricText {
    pub metric_type: MetricType,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MetricType {
    FPS,
    FrameTime,
    EntityCount,
    PathGenTime,
}

/// Helper function to create performance metrics display for the Metrics section
fn create_performance_metrics_display(parent: &mut ChildBuilder) {
    let metrics = [
        (MetricType::FPS, "FPS: 60.0"),
        (MetricType::FrameTime, "Frame Time: 16.7ms"),
        (MetricType::EntityCount, "Entities: 0"),
        (MetricType::PathGenTime, "Path Gen: 0.0ms"),
    ];

    for (metric_type, default_text) in metrics {
        parent.spawn((
            TextBundle::from_section(
                default_text,
                TextStyle {
                    font_size: 11.0,
                    color: Color::srgb(0.8, 1.0, 0.8), // Light green for metrics
                    ..default()
                },
            ),
            PerformanceMetricText { metric_type },
        ));
    }
}

/// System to update performance metrics
pub fn update_performance_metrics(
    mut metrics: ResMut<PerformanceMetrics>,
    time: Res<Time>,
    entities: Query<Entity>,
) {
    // Calculate FPS and frame time
    let delta_time = time.delta_seconds();
    if delta_time > 0.0 {
        // Simple moving average for smoother display
        let new_fps = 1.0 / delta_time;
        metrics.fps = metrics.fps * 0.9 + new_fps * 0.1;
        metrics.frame_time_ms = delta_time * 1000.0;
    }
    
    // Count entities
    metrics.entity_count = entities.iter().count();
    
    // Update timestamp
    metrics.last_update_time = time.elapsed_seconds();
}

/// System to update performance metrics display
pub fn update_performance_display(
    metrics: Res<PerformanceMetrics>,
    mut text_query: Query<(&PerformanceMetricText, &mut Text)>,
) {
    // Only update display every few frames to avoid flickering
    if metrics.last_update_time % 0.1 < 0.016 { // Update ~10 times per second
        for (metric_info, mut text) in &mut text_query {
            let display_text = match metric_info.metric_type {
                MetricType::FPS => format!("FPS: {:.1}", metrics.fps),
                MetricType::FrameTime => format!("Frame Time: {:.1}ms", metrics.frame_time_ms),
                MetricType::EntityCount => format!("Entities: {}", metrics.entity_count),
                MetricType::PathGenTime => format!("Path Gen: {:.1}ms", metrics.path_generation_time_ms),
            };
            text.sections[0].value = display_text;
        }
    }
}

/// Component marker for action buttons
#[derive(Component)]
pub struct ActionButton {
    pub action_type: ActionType,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ActionType {
    ResetGame,
    RandomizeMap,
    SaveState,
    LoadState,
}

/// Helper function to create help text for the Help section
fn create_help_section(parent: &mut ChildBuilder) {
    let help_text = [
        "F1 - Toggle debug visualization",
        "F2 - Toggle this debug panel",
        "R - Reset game state",
        "M - Randomize map",
        "1-5 - Set spawn rate (1=slow, 5=fast)",
        "+/- - Adjust tower damage",
        "SPACE - Start wave",
        "ESC - Exit game"
    ];

    for line in help_text {
        parent.spawn(TextBundle::from_section(
            line,
            TextStyle {
                font_size: 10.0,
                color: Color::srgb(0.8, 0.9, 0.8), // Light green for help text
                ..default()
            },
        ));
    }
}

/// Helper function to create action buttons for the Actions section
fn create_action_buttons(parent: &mut ChildBuilder) {
    let buttons = [
        (ActionType::ResetGame, "Reset Game", Color::srgb(0.8, 0.3, 0.3)),
        (ActionType::RandomizeMap, "Randomize Map", Color::srgb(0.3, 0.6, 0.8)),
        (ActionType::SaveState, "Save State", Color::srgb(0.3, 0.8, 0.3)),
        (ActionType::LoadState, "Load State", Color::srgb(0.8, 0.8, 0.3)),
    ];

    for (action_type, label, color) in buttons {
        parent
            .spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Percent(48.0),
                        height: Val::Px(25.0),
                        margin: UiRect::all(Val::Px(2.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: color.into(),
                    ..default()
                },
                ActionButton { action_type },
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    label,
                    TextStyle {
                        font_size: 11.0,
                        color: Color::srgb(1.0, 1.0, 1.0),
                        ..default()
                    },
                ));
            });
    }
}

/// System to handle keyboard shortcuts for debug UI
// Note: Removed complex type aliases due to lifetime issues

pub fn handle_debug_keyboard_shortcuts(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut ui_state: ResMut<DebugUIState>,
    mut wave_manager: ResMut<WaveManager>,
    mut economy: ResMut<Economy>,
    mut wave_status: ResMut<crate::systems::combat_system::WaveStatus>,
    mut game_state: ResMut<GameState>,
    mut commands: Commands,
    enemy_query: Query<Entity, With<Enemy>>,
    projectile_query: Query<Entity, With<Projectile>>,
    tower_query: Query<Entity, With<TowerStats>>,
) {
    // R key - Reset game
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        println!("Keyboard shortcut: Resetting game (R key)");
        
        // Reset all game entities
        for entity in enemy_query.iter() {
            commands.entity(entity).despawn();
        }
        for entity in projectile_query.iter() {
            commands.entity(entity).despawn();
        }
        for entity in tower_query.iter() {
            commands.entity(entity).despawn();
        }
        
        // Reset resources
        wave_manager.current_wave = 0;
        wave_manager.enemies_in_wave = 0;
        wave_manager.enemies_spawned = 0;
        
        economy.money = 100;
        economy.research_points = 0;
        
        wave_status.enemies_remaining = 0;
        wave_status.enemies_killed = 0;
        wave_status.enemies_escaped = 0;
        wave_status.wave_complete = false;
        
        *game_state = GameState::Playing;
    }
    
    // M key - Randomize map
    if keyboard_input.just_pressed(KeyCode::KeyM) {
        println!("Keyboard shortcut: Randomizing map (M key)");
        
        use rand::Rng;
        let mut rng = rand::rng();
        ui_state.current_obstacle_density = rng.random_range(0.1..=0.8);
        ui_state.set_changed(); // Trigger path regeneration
    }
    
    // Number keys 1-5 - Quick adjust spawn rate (1=slow, 5=fast)
    if keyboard_input.just_pressed(KeyCode::Digit1) {
        ui_state.enemy_spawn_rate = 0.5; // Slow
        ui_state.set_changed();
        println!("Keyboard shortcut: Spawn rate set to SLOW (1 key)");
    }
    if keyboard_input.just_pressed(KeyCode::Digit2) {
        ui_state.enemy_spawn_rate = 1.0; // Normal
        ui_state.set_changed();
        println!("Keyboard shortcut: Spawn rate set to NORMAL (2 key)");
    }
    if keyboard_input.just_pressed(KeyCode::Digit3) {
        ui_state.enemy_spawn_rate = 2.0; // Fast
        ui_state.set_changed();
        println!("Keyboard shortcut: Spawn rate set to FAST (3 key)");
    }
    if keyboard_input.just_pressed(KeyCode::Digit4) {
        ui_state.enemy_spawn_rate = 3.0; // Very Fast
        ui_state.set_changed();
        println!("Keyboard shortcut: Spawn rate set to VERY FAST (4 key)");
    }
    if keyboard_input.just_pressed(KeyCode::Digit5) {
        ui_state.enemy_spawn_rate = 5.0; // Ultra Fast
        ui_state.set_changed();
        println!("Keyboard shortcut: Spawn rate set to ULTRA FAST (5 key)");
    }
    
    // Plus/Minus keys - Adjust tower damage multiplier
    if keyboard_input.just_pressed(KeyCode::Equal) { // Plus key (without shift)
        ui_state.tower_damage_multiplier = (ui_state.tower_damage_multiplier + 0.5).clamp(0.1, 10.0);
        println!("Keyboard shortcut: Tower damage increased to {:.1}x (+ key)", ui_state.tower_damage_multiplier);
    }
    if keyboard_input.just_pressed(KeyCode::Minus) {
        ui_state.tower_damage_multiplier = (ui_state.tower_damage_multiplier - 0.5).clamp(0.1, 10.0);
        println!("Keyboard shortcut: Tower damage decreased to {:.1}x (- key)", ui_state.tower_damage_multiplier);
    }
}

// Note: Removed complex type aliases due to lifetime issues

/// System to handle action button clicks
pub fn handle_action_buttons(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &ActionButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut ui_state: ResMut<DebugUIState>,
    mut wave_manager: ResMut<WaveManager>,
    mut economy: ResMut<Economy>,
    mut wave_status: ResMut<crate::systems::combat_system::WaveStatus>,
    mut game_state: ResMut<GameState>,
    enemy_query: Query<Entity, With<Enemy>>,
    projectile_query: Query<Entity, With<Projectile>>,
    tower_query: Query<Entity, With<TowerStats>>,
    _path_line_query: Query<Entity, With<GamePathLine>>,
    _enemy_path: ResMut<EnemyPath>,
) {
    for (interaction, action_button, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                // Button press visual feedback
                *color = Color::srgb(1.0, 1.0, 1.0).into();
                
                match action_button.action_type {
                    ActionType::ResetGame => {
                        // Reset all game state
                        println!("Resetting game state...");
                        
                        // Despawn all game entities
                        for entity in enemy_query.iter() {
                            commands.entity(entity).despawn();
                        }
                        for entity in projectile_query.iter() {
                            commands.entity(entity).despawn();
                        }
                        for entity in tower_query.iter() {
                            commands.entity(entity).despawn();
                        }
                        
                        // Reset resources
                        wave_manager.current_wave = 0;
                        wave_manager.enemies_in_wave = 0;
                        wave_manager.enemies_spawned = 0;
                        
                        economy.money = 100;
                        economy.research_points = 0;
                        
                        wave_status.enemies_remaining = 0;
                        wave_status.enemies_killed = 0;
                        wave_status.enemies_escaped = 0;
                        wave_status.wave_complete = false;
                        
                        *game_state = GameState::Playing;
                        
                        println!("Game reset complete!");
                    },
                    ActionType::RandomizeMap => {
                        println!("Randomizing map...");
                        
                        // Generate new random path parameters
                        use rand::Rng;
                        let mut rng = rand::rng();
                        ui_state.current_obstacle_density = rng.random_range(0.1..=0.8);
                        
                        // Trigger path regeneration by changing the parameter
                        ui_state.set_changed();
                        
                        println!("Map randomized with obstacle density: {:.2}", ui_state.current_obstacle_density);
                    },
                    ActionType::SaveState => {
                        println!("Saving game state... (Feature not yet implemented)");
                        // TODO: Implement save functionality
                        // Could save to file: wave progress, economy, tower positions, etc.
                    },
                    ActionType::LoadState => {
                        println!("Loading game state... (Feature not yet implemented)");
                        // TODO: Implement load functionality
                        // Could load from file: restore game state
                    },
                }
            },
            Interaction::Hovered => {
                // Button hover visual feedback
                let hover_color = match action_button.action_type {
                    ActionType::ResetGame => Color::srgb(1.0, 0.4, 0.4),
                    ActionType::RandomizeMap => Color::srgb(0.4, 0.7, 1.0),
                    ActionType::SaveState => Color::srgb(0.4, 1.0, 0.4),
                    ActionType::LoadState => Color::srgb(1.0, 1.0, 0.4),
                };
                *color = hover_color.into();
            },
            Interaction::None => {
                // Button normal visual state
                let normal_color = match action_button.action_type {
                    ActionType::ResetGame => Color::srgb(0.8, 0.3, 0.3),
                    ActionType::RandomizeMap => Color::srgb(0.3, 0.6, 0.8),
                    ActionType::SaveState => Color::srgb(0.3, 0.8, 0.3),
                    ActionType::LoadState => Color::srgb(0.8, 0.8, 0.3),
                };
                *color = normal_color.into();
            },
        }
    }
}