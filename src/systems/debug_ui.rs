use bevy::prelude::*;
use crate::systems::debug_visualization::DebugVisualizationState;

/// Plugin for interactive debug UI controls
pub struct DebugUIPlugin;

impl Plugin for DebugUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<DebugUIState>()
            .add_systems(Startup, setup_debug_ui)
            .add_systems(Update, (
                debug_ui_toggle_system,
                update_debug_ui_visibility,
                handle_toggle_button_interactions,
                sync_ui_with_debug_state,
            ));
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
        });
}

/// Helper function to create UI sections
fn create_ui_section(parent: &mut ChildBuilder, section_type: UISectionType) {
    let section_title = match section_type {
        UISectionType::Controls => "Visualization Controls",
        UISectionType::Metrics => "Performance Metrics",
        UISectionType::Parameters => "Generation Parameters",
        UISectionType::Actions => "Actions",
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
                _ => {
                    // Placeholder for other sections
                    section.spawn(TextBundle::from_section(
                        match section_type {
                            UISectionType::Metrics => "Real-time metrics coming soon...",
                            UISectionType::Parameters => "Parameter sliders coming soon...",
                            UISectionType::Actions => "Action buttons coming soon...",
                            _ => "",
                        },
                        TextStyle {
                            font_size: 12.0,
                            color: Color::srgb(0.7, 0.7, 0.7),
                            ..default()
                        },
                    ));
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