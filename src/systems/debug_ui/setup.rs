use bevy::prelude::*;
use super::components::*;

/// Main setup system for debug UI
pub fn setup_debug_ui(mut commands: Commands) {
    println!("DEBUG: Creating simple test debug UI panel");
    
    // Create a simple test panel first to verify basic functionality
    let panel_entity = commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(10.0),
                top: Val::Px(50.0),
                width: Val::Px(250.0),
                height: Val::Px(200.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(10.0)),
                display: Display::Flex, // Visible by default for testing
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.9)),
            DebugUIPanel,
        ))
        .id();
        
    println!("DEBUG: Simple debug panel created with entity ID: {:?}", panel_entity);
    
    // Add a simple text child to verify the panel works
    commands.entity(panel_entity).with_children(|parent| {
        parent.spawn((
            Text::new("Debug Panel (F2)\nPress F2 to toggle"),
            TextFont {
                font_size: 14.0,
                ..default()
            },
            TextColor(Color::WHITE),
        ));
        println!("DEBUG: Added text child to debug panel");
    });
}

/// Helper function to create UI sections
fn create_ui_section(parent: &mut ChildSpawnerCommands, section_type: UISectionType) {
    let section_title = match section_type {
        UISectionType::Controls => "DEBUG CONTROLS",
        UISectionType::Metrics => "PERFORMANCE METRICS", 
        UISectionType::Parameters => "GAME PARAMETERS",
        UISectionType::Actions => "ACTIONS",
        UISectionType::Help => "HELP & CONTROLS",
    };

    parent.spawn((
        Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            margin: UiRect::bottom(Val::Px(8.0)),
            ..default()
        },
        DebugUISection { section_type },
    )).with_children(|section| {
        // Section header
        section.spawn((
            Text::new(section_title),
            TextFont {
                font_size: 11.0,
                ..default()
            },
            TextColor(Color::srgb(1.0, 1.0, 0.6)), // Light yellow for headers
        ));

        // Create specific content based on section type
        match section_type {
            UISectionType::Controls => create_toggle_buttons(section),
            UISectionType::Metrics => create_performance_metrics_display(section),
            UISectionType::Parameters => create_parameter_sliders(section),
            UISectionType::Actions => create_action_buttons(section),
            UISectionType::Help => create_help_section(section),
        }
    });
}

/// Create toggle buttons for the Controls section
fn create_toggle_buttons(parent: &mut ChildSpawnerCommands) {
    let toggles = [
        (ToggleType::Grid, "Toggle Grid"),
        (ToggleType::Path, "Toggle Path"),
        (ToggleType::Zones, "Toggle Zones"),
        (ToggleType::Performance, "Toggle Performance"),
    ];

    for (toggle_type, label) in toggles {
        parent.spawn((
            Button,
            Node {
                width: Val::Percent(90.0),
                height: Val::Px(25.0),
                margin: UiRect::bottom(Val::Px(3.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
            ToggleButton { toggle_type },
        )).with_children(|button| {
            button.spawn((
                Text::new(label),
                TextFont {
                    font_size: 10.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
    }
}

/// Create parameter sliders for the Parameters section
fn create_parameter_sliders(parent: &mut ChildSpawnerCommands) {
    let sliders = [
        (SliderType::ObstacleDensity, "Obstacle Density", 0.0, 1.0, 0.15),
        (SliderType::EnemySpawnRate, "Enemy Spawn Rate", 0.1, 5.0, 1.0),
        (SliderType::TowerDamageMultiplier, "Tower Damage", 0.1, 3.0, 1.0),
    ];

    for (slider_type, label, min_val, max_val, default_val) in sliders {
        // Slider container
        parent.spawn((
            Node {
                width: Val::Percent(90.0),
                height: Val::Px(40.0),
                flex_direction: FlexDirection::Column,
                margin: UiRect::bottom(Val::Px(5.0)),
                ..default()
            },
        )).with_children(|slider_container| {
            // Label
            slider_container.spawn((
                Text::new(format!("{}: {:.2}", label, default_val)),
                TextFont {
                    font_size: 9.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                SliderValueText { slider_type },
            ));
            
            // Slider track
            slider_container.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(12.0),
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                SliderTrack { slider_type },
            )).with_children(|track| {
                // Slider handle
                track.spawn((
                    Button,
                    Node {
                        width: Val::Px(16.0),
                        height: Val::Px(16.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.8, 0.8, 0.8)),
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

/// Helper function to create performance metrics display for the Metrics section
fn create_performance_metrics_display(parent: &mut ChildSpawnerCommands) {
    let metrics = [
        (MetricType::FPS, "FPS: 60.0"),
        (MetricType::FrameTime, "Frame Time: 16.7ms"),
        (MetricType::EntityCount, "Entities: 0"),
        (MetricType::PathGenTime, "Path Gen: 0.0ms"),
    ];

    for (metric_type, default_text) in metrics {
        parent.spawn((
            Text::new(default_text),
            TextFont {
                font_size: 11.0,
                ..default()
            },
            TextColor(Color::srgb(0.8, 1.0, 0.8)), // Light green for metrics
            PerformanceMetricText { metric_type },
        ));
    }
}

/// Helper function to create help text for the Help section
fn create_help_section(parent: &mut ChildSpawnerCommands) {
    let help_text = [
        "F1 - Toggle debug visualization",
        "F2 - Toggle this debug panel",
        "R - Reset game state",
        "G - Toggle grid display",
        "P - Toggle path display",
        "SPACE - Spawn enemy wave",
    ];

    for text in help_text {
        parent.spawn((
            Text::new(text),
            TextFont {
                font_size: 9.0,
                ..default()
            },
            TextColor(Color::srgb(0.8, 0.8, 1.0)), // Light blue for help text
        ));
    }
}

/// Helper function to create action buttons for the Actions section
fn create_action_buttons(parent: &mut ChildSpawnerCommands) {
    let actions = [
        (ActionType::ResetGame, "Reset Game"),
        (ActionType::RandomizeMap, "Randomize Map"),
        (ActionType::SaveState, "Save State"),
        (ActionType::LoadState, "Load State"),
    ];

    for (action_type, label) in actions {
        parent.spawn((
            Button,
            Node {
                width: Val::Percent(45.0),
                height: Val::Px(25.0),
                margin: UiRect::all(Val::Px(2.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.4, 0.2, 0.2)), // Dark red for action buttons
            ActionButton { action_type },
        )).with_children(|button| {
            button.spawn((
                Text::new(label),
                TextFont {
                    font_size: 9.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
    }
}