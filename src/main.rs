use bevy::prelude::*;

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
        .add_systems(Startup, setup)
        .add_systems(Update, close_on_esc)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    
    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Tower Defense Game\nPress ESC to exit",
            TextStyle {
                font_size: 40.0,
                color: Color::srgb(1.0, 1.0, 1.0),
                ..default()
            },
        ),
        ..default()
    });
}

fn close_on_esc(
    mut exit: EventWriter<AppExit>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
}