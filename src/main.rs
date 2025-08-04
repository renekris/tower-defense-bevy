use bevy::prelude::*;

mod components;
mod resources;
mod systems;

use resources::*;
use systems::enemy_system::*;
use systems::input_system::*;
use systems::ui_system::*;
use systems::combat_system::*;
use systems::debug_visualization::*;
use systems::debug_ui::GamePathLine;
use systems::debug_ui::*;
use systems::tower_ui::{
    TowerSelectionState, 
    setup_tower_placement_panel, 
    setup_tower_upgrade_panel,
    tower_selection_system,
    tower_type_button_system,
    upgrade_button_system,
    update_upgrade_panel_system,
    selected_tower_indicator_system,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Tower Defense - Bevy".to_string(),
                resolution: (1280.0, 720.0).into(),
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        // TODO: Add Remote Protocol plugin for MCP server integration when upgrading to Bevy 0.15+
        // .add_plugins(bevy::remote::RemotePlugin::default())
        // .add_plugins(BrpExtrasPlugin)
        // Add custom plugins
        .add_plugins(DebugUIPlugin)
        // Initialize game resources
        .init_resource::<Score>()
        .init_resource::<WaveManager>()
        .init_resource::<GameState>()
        .init_resource::<Economy>()
        .init_resource::<MouseInputState>()
        .init_resource::<WaveStatus>()
        .init_resource::<DebugVisualizationState>()
        .init_resource::<TowerSelectionState>()
        .insert_resource(create_default_path())
        // Setup systems
        .add_systems(Startup, (setup, setup_placement_zones, setup_tower_placement_panel, setup_tower_upgrade_panel))
        // Game systems - Split into groups to avoid tuple size limits
        .add_systems(Update, (
            // Input and UI systems
            mouse_input_system,
            tower_placement_system,
            tower_placement_preview_system,
            update_ui_system,
        ))
        .add_systems(Update, (
            // Tower UI systems
            tower_selection_system,
            tower_type_button_system,
            upgrade_button_system,
            update_upgrade_panel_system,
            selected_tower_indicator_system,
        ))
        .add_systems(Update, (
            // Debug visualization systems
            debug_toggle_system,
            debug_visualization_system,
            
            // Combat systems (ORDER CRITICAL - dependency chain)
            tower_targeting_system,
            projectile_spawning_system,
            projectile_movement_system,
            collision_system,
        ))
        .add_systems(Update, (
            // Enemy and wave management
            manual_wave_system,
            enemy_spawning_system,
            enemy_movement_system,
            enemy_cleanup_system,
            
            // Game state management (runs last)
            game_state_system,
            
            close_on_esc,
        ))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    
    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Tower Defense Game - Phase 3 COMBAT!\nSPACE: spawn wave | ESC: exit\n1-5: select tower type | LEFT CLICK: place tower\nF1: toggle debug visualization | F2: debug UI panel | 1-9: select wave (debug mode)\nTowers auto-target and shoot enemies! Defend the base!",
            TextStyle {
                font_size: 20.0,
                color: Color::srgb(1.0, 1.0, 1.0),
                ..default()
            },
        ),
        transform: Transform::from_translation(Vec3::new(0.0, 330.0, 0.0)),
        ..default()
    });

    // Draw the path line so players can see where enemies will move
    let path = create_default_path();
    for i in 0..path.waypoints.len() - 1 {
        let start = path.waypoints[i];
        let end = path.waypoints[i + 1];
        let midpoint = (start + end) / 2.0;
        let length = start.distance(end);
        
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(0.5, 0.5, 0.5),
                    custom_size: Some(Vec2::new(length, 5.0)),
                    ..default()
                },
                transform: Transform::from_translation(midpoint.extend(-1.0)),
                ..default()
            },
            GamePathLine,
        ));
    }
}

/// Create a simple straight-line path for Phase 1
fn create_default_path() -> EnemyPath {
    EnemyPath::new(vec![
        Vec2::new(-600.0, 0.0),  // Start left side of screen
        Vec2::new(600.0, 0.0),   // End right side of screen
    ])
}

fn close_on_esc(
    mut exit: EventWriter<AppExit>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
}