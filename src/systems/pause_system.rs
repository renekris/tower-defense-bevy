use bevy::prelude::*;
use crate::resources::{AppState, GameSystemSet};

// ============================================================================
// PAUSE MENU COMPONENTS
// ============================================================================

#[derive(Component)]
pub struct PauseMenu;

#[derive(Component)]
pub struct PauseMenuOverlay;

#[derive(Component)]
pub struct PauseButton {
    pub action: PauseMenuAction,
}

#[derive(Clone, Debug)]
pub enum PauseMenuAction {
    Resume,
    Settings,
    Exit,
}

// ============================================================================
// UI COLOR CONSTANTS (matching tower UI)
// ============================================================================

struct UIColors;

impl UIColors {
    // Panel colors - Enhanced with subtle gradients
    const PANEL_BG: Color = Color::srgb(0.08, 0.12, 0.18);           // Darker, more premium feel
    const PANEL_BORDER: Color = Color::srgb(0.22, 0.28, 0.38);       // Softer border
    const PANEL_SHADOW: Color = Color::srgb(0.02, 0.03, 0.08);       // Shadow/depth
    const HEADER_BG: Color = Color::srgb(0.06, 0.10, 0.16);          // Even darker header
    
    // Button states - Improved contrast and visual feedback
    const BUTTON_DEFAULT: Color = Color::srgb(0.15, 0.20, 0.28);     // Slightly darker default
    const BUTTON_HOVER: Color = Color::srgb(0.20, 0.28, 0.38);       // More pronounced hover
    const BUTTON_SELECTED: Color = Color::srgb(0.12, 0.40, 0.70);    // Refined blue selection
    const BUTTON_SELECTED_HOVER: Color = Color::srgb(0.18, 0.48, 0.78); // Enhanced selected hover
    const BUTTON_DISABLED: Color = Color::srgb(0.10, 0.12, 0.16);    // Clearly disabled state
    
    // Border colors - Better visual hierarchy
    const BORDER_DEFAULT: Color = Color::srgb(0.32, 0.38, 0.48);     // Subtle default
    const BORDER_HOVER: Color = Color::srgb(0.48, 0.58, 0.70);       // Clear hover indication
    const BORDER_SELECTED: Color = Color::srgb(0.35, 0.60, 0.85);    // Strong selection
    const BORDER_SELECTED_HOVER: Color = Color::srgb(0.45, 0.70, 0.95); // Bright selected hover
    const BORDER_DISABLED: Color = Color::srgb(0.18, 0.22, 0.28);    // Muted disabled
    
    // Text colors - Enhanced readability and hierarchy
    const TEXT_PRIMARY: Color = Color::srgb(0.96, 0.96, 0.98);       // Crisp white text
    const TEXT_SECONDARY: Color = Color::srgb(0.78, 0.82, 0.88);     // Clear secondary
    const TEXT_MUTED: Color = Color::srgb(0.58, 0.62, 0.68);         // Subtle muted text
    const TEXT_ACCENT: Color = Color::srgb(0.88, 0.92, 0.62);        // Warmer yellow
    const TEXT_SUCCESS: Color = Color::srgb(0.58, 0.88, 0.68);       // Clear green
    const TEXT_WARNING: Color = Color::srgb(1.0, 0.78, 0.58);        // Warm orange
    const TEXT_ERROR: Color = Color::srgb(1.0, 0.58, 0.58);          // Clear red
    const TEXT_INFO: Color = Color::srgb(0.58, 0.78, 1.0);           // Cool blue info
    
    // Overlay colors
    const OVERLAY_BG: Color = Color::srgba(0.0, 0.0, 0.0, 0.6);      // Semi-transparent dark overlay
}

// ============================================================================
// PAUSE MENU SETUP SYSTEM
// ============================================================================

pub fn setup_pause_menu(mut commands: Commands) {
    // Create pause menu overlay (initially hidden)
    let pause_menu_entity = commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(0.0),
            top: Val::Px(0.0),
            width: Val::Vw(100.0),
            height: Val::Vh(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(UIColors::OVERLAY_BG),
        Visibility::Hidden, // Start hidden
        ZIndex(1000), // High z-index to appear above game
        PauseMenuOverlay,
    )).with_children(|parent| {
        // Pause menu panel
        parent.spawn((
            Node {
                width: Val::Px(400.0),
                height: Val::Px(500.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(30.0)),
                row_gap: Val::Px(20.0),
                border: UiRect::all(Val::Px(3.0)),
                ..default()
            },
            BackgroundColor(UIColors::PANEL_BG),
            BorderColor(UIColors::PANEL_BORDER),
            BorderRadius::all(Val::Px(15.0)),
            PauseMenu,
        )).with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("GAME PAUSED"),
                TextFont {
                    font_size: 36.0,
                    ..default()
                },
                TextColor(UIColors::TEXT_PRIMARY),
                Node {
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                },
            ));
            
            // Resume button
            create_pause_button(parent, "RESUME GAME", PauseMenuAction::Resume, UIColors::TEXT_SUCCESS);
            
            // Settings button
            create_pause_button(parent, "SETTINGS", PauseMenuAction::Settings, UIColors::TEXT_INFO);
            
            // Exit button
            create_pause_button(parent, "EXIT GAME", PauseMenuAction::Exit, UIColors::TEXT_ERROR);
            
            // Keyboard shortcut hint
            parent.spawn((
                Text::new("Press ESC to resume"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(UIColors::TEXT_MUTED),
                Node {
                    margin: UiRect::top(Val::Px(20.0)),
                    ..default()
                },
            ));
        });
    }).id();

    // Store the pause menu entity ID as a resource for easy access
    commands.insert_resource(PauseMenuEntity(pause_menu_entity));
}

fn create_pause_button(
    parent: &mut ChildSpawnerCommands,
    text: &str,
    action: PauseMenuAction,
    text_color: Color,
) {
    parent.spawn((
        Button,
        Node {
            width: Val::Px(280.0),
            height: Val::Px(60.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect::all(Val::Px(2.0)),
            margin: UiRect::all(Val::Px(5.0)),
            ..default()
        },
        BackgroundColor(UIColors::BUTTON_DEFAULT),
        BorderColor(UIColors::BORDER_DEFAULT),
        BorderRadius::all(Val::Px(8.0)),
        PauseButton { action },
    )).with_children(|parent| {
        parent.spawn((
            Text::new(text),
            TextFont {
                font_size: 20.0,
                ..default()
            },
            TextColor(text_color),
        ));
    });
}

// ============================================================================
// PAUSE MENU SYSTEMS
// ============================================================================

#[derive(Resource)]
pub struct PauseMenuEntity(pub Entity);

/// System to handle ESC key for pause toggle
pub fn pause_toggle_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        match current_state.get() {
            AppState::Playing => {
                next_state.set(AppState::Paused);
                info!("Game paused");
            }
            AppState::Paused => {
                next_state.set(AppState::Playing);
                info!("Game resumed");
            }
            AppState::Settings => {
                // From settings, go back to pause menu
                next_state.set(AppState::Paused);
                info!("Returned to pause menu from settings");
            }
        }
    }
}

/// System to show/hide pause menu based on app state
pub fn pause_menu_visibility_system(
    app_state: Res<State<AppState>>,
    pause_menu_entity: Res<PauseMenuEntity>,
    mut visibility_query: Query<&mut Visibility>,
) {
    if app_state.is_changed() {
        if let Ok(mut visibility) = visibility_query.get_mut(pause_menu_entity.0) {
            *visibility = match app_state.get() {
                AppState::Paused => Visibility::Visible,
                _ => Visibility::Hidden,
            };
        }
    }
}

/// System to handle pause menu button interactions
pub fn pause_menu_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor, &PauseButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, mut bg_color, mut border_color, pause_button) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                match pause_button.action {
                    PauseMenuAction::Resume => {
                        next_state.set(AppState::Playing);
                        info!("Resume button pressed");
                    }
                    PauseMenuAction::Settings => {
                        next_state.set(AppState::Settings);
                        info!("Settings button pressed");
                    }
                    PauseMenuAction::Exit => {
                        info!("Exit button pressed");
                        exit.write(AppExit::Success);
                    }
                }
            }
            Interaction::Hovered => {
                *bg_color = BackgroundColor(UIColors::BUTTON_HOVER);
                *border_color = BorderColor(UIColors::BORDER_HOVER);
            }
            Interaction::None => {
                *bg_color = BackgroundColor(UIColors::BUTTON_DEFAULT);
                *border_color = BorderColor(UIColors::BORDER_DEFAULT);
            }
        }
    }
}

/// System to handle time scaling - pause game time in Paused state
pub fn time_scale_system(
    app_state: Res<State<AppState>>,
    mut time: ResMut<Time<Virtual>>,
) {
    if app_state.is_changed() {
        match app_state.get() {
            AppState::Playing => {
                time.unpause();
                info!("Game time resumed");
            }
            AppState::Paused | AppState::Settings => {
                time.pause();
                info!("Game time paused");
            }
        }
    }
}

// ============================================================================
// PAUSE SYSTEM PLUGIN
// ============================================================================

pub struct PauseSystemPlugin;

impl Plugin for PauseSystemPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<AppState>()
            .add_systems(Startup, setup_pause_menu)
            .add_systems(
                Update,
                (
                    pause_toggle_system,
                    pause_menu_visibility_system,
                    time_scale_system,
                ).in_set(GameSystemSet::Input)
            )
            .add_systems(
                Update,
                pause_menu_button_system
                    .in_set(GameSystemSet::Pause)
                    .run_if(in_state(AppState::Paused))
            );
    }
}