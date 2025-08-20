use bevy::prelude::*;

mod components;
mod resources;
mod systems;

use resources::*;
use systems::enemy_system::*;

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
        // Initialize game resources
        .init_resource::<Score>()
        .init_resource::<WaveManager>()
        .init_resource::<GameState>()
        .insert_resource(create_default_path())
        // Setup systems
        .add_systems(Startup, setup)
        // Game systems
        .add_systems(Update, (
            manual_wave_system,
            enemy_spawning_system,
            enemy_movement_system,
            enemy_cleanup_system,
            close_on_esc,
        ))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    
    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Tower Defense Game - Phase 1\nPress SPACE to spawn wave\nPress ESC to exit",
            TextStyle {
                font_size: 30.0,
                color: Color::srgb(1.0, 1.0, 1.0),
                ..default()
            },
        ),
        transform: Transform::from_translation(Vec3::new(0.0, 300.0, 0.0)),
        ..default()
    });

    // Draw the path line so players can see where enemies will move
    let path = create_default_path();
    for i in 0..path.waypoints.len() - 1 {
        let start = path.waypoints[i];
        let end = path.waypoints[i + 1];
        let midpoint = (start + end) / 2.0;
        let length = start.distance(end);
        
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.5, 0.5, 0.5),
                custom_size: Some(Vec2::new(length, 5.0)),
                ..default()
            },
            transform: Transform::from_translation(midpoint.extend(-1.0)),
            ..default()
        });
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