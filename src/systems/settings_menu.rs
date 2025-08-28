use bevy::prelude::*;
use crate::resources::{AppState, GameSystemSet};

// ============================================================================
// SETTINGS MENU COMPONENTS
// ============================================================================

#[derive(Component)]
pub struct SettingsMenu;

#[derive(Component)]
pub struct SettingsMenuOverlay;

#[derive(Component)]
pub struct SettingsButton {
    pub action: SettingsMenuAction,
}

#[derive(Component)]
pub struct ResolutionButton {
    pub resolution: ResolutionOption,
}

#[derive(Component)]
pub struct FullscreenToggle;

#[derive(Component)]
pub struct VSyncToggle;

#[derive(Component)]
pub struct ResolutionText;

#[derive(Component)]
pub struct FullscreenText;

#[derive(Component)]
pub struct VSyncText;

#[derive(Component)]
pub struct SettingsSlider {
    pub setting_type: SettingsType,
    pub value: f32,
}

#[derive(Clone, Debug)]
pub enum SettingsMenuAction {
    Back,
    ResetToDefaults,
    ToggleFullscreen,
    ToggleVSync,
    ChangeResolution(ResolutionOption),
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ResolutionOption {
    Res1920x1080,
    Res1600x900,
    Res1366x768,
    Res1280x720,
    Res1024x768,
}

impl ResolutionOption {
    pub fn to_vec2(&self) -> Vec2 {
        match self {
            ResolutionOption::Res1920x1080 => Vec2::new(1920.0, 1080.0),
            ResolutionOption::Res1600x900 => Vec2::new(1600.0, 900.0),
            ResolutionOption::Res1366x768 => Vec2::new(1366.0, 768.0),
            ResolutionOption::Res1280x720 => Vec2::new(1280.0, 720.0),
            ResolutionOption::Res1024x768 => Vec2::new(1024.0, 768.0),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            ResolutionOption::Res1920x1080 => "1920x1080".to_string(),
            ResolutionOption::Res1600x900 => "1600x900".to_string(),
            ResolutionOption::Res1366x768 => "1366x768".to_string(),
            ResolutionOption::Res1280x720 => "1280x720".to_string(),
            ResolutionOption::Res1024x768 => "1024x768".to_string(),
        }
    }
    
    pub fn all() -> Vec<Self> {
        vec![
            ResolutionOption::Res1920x1080,
            ResolutionOption::Res1600x900,
            ResolutionOption::Res1366x768,
            ResolutionOption::Res1280x720,
            ResolutionOption::Res1024x768,
        ]
    }
}

#[derive(Clone, Debug)]
pub enum SettingsType {
    MasterVolume,
    SFXVolume,
    MusicVolume,
    Resolution,
    Fullscreen,
    VSync,
}

// ============================================================================
// UI COLOR CONSTANTS (matching pause menu)
// ============================================================================

struct UIColors;

impl UIColors {
    // Panel colors
    const PANEL_BG: Color = Color::srgb(0.08, 0.12, 0.18);
    const PANEL_BORDER: Color = Color::srgb(0.22, 0.28, 0.38);
    const HEADER_BG: Color = Color::srgb(0.06, 0.10, 0.16);
    
    // Button states
    const BUTTON_DEFAULT: Color = Color::srgb(0.15, 0.20, 0.28);
    const BUTTON_HOVER: Color = Color::srgb(0.20, 0.28, 0.38);
    
    // Border colors
    const BORDER_DEFAULT: Color = Color::srgb(0.32, 0.38, 0.48);
    const BORDER_HOVER: Color = Color::srgb(0.48, 0.58, 0.70);
    
    // Text colors
    const TEXT_PRIMARY: Color = Color::srgb(0.96, 0.96, 0.98);
    const TEXT_SECONDARY: Color = Color::srgb(0.78, 0.82, 0.88);
    const TEXT_MUTED: Color = Color::srgb(0.58, 0.62, 0.68);
    const TEXT_INFO: Color = Color::srgb(0.58, 0.78, 1.0);
    const TEXT_ERROR: Color = Color::srgb(1.0, 0.58, 0.58);
    
    // Overlay colors
    const OVERLAY_BG: Color = Color::srgba(0.0, 0.0, 0.0, 0.6);
    
    // Slider colors
    const SLIDER_TRACK: Color = Color::srgb(0.12, 0.16, 0.22);
    const SLIDER_HANDLE: Color = Color::srgb(0.35, 0.60, 0.85);
    const SLIDER_FILL: Color = Color::srgb(0.25, 0.45, 0.65);
}

// ============================================================================
// SETTINGS RESOURCES
// ============================================================================

/// Resource to store current game settings
#[derive(Resource, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GameSettings {
    pub current_resolution: ResolutionOption,
    pub fullscreen_enabled: bool,
    pub vsync_enabled: bool,
    pub master_volume: f32,
    pub sfx_volume: f32,
    pub music_volume: f32,
    pub debug_admin_enabled: bool,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            current_resolution: ResolutionOption::Res1920x1080, // Default to 1080p
            fullscreen_enabled: false, // Disabled by default
            vsync_enabled: true, // Enabled by default
            master_volume: 1.0,
            sfx_volume: 0.8,
            music_volume: 0.6,
            debug_admin_enabled: false, // Secure default
        }
    }
}

impl GameSettings {
    const SETTINGS_FILE: &'static str = "settings.json";
    
    /// Load settings from file, or create default settings if file doesn't exist
    pub fn load() -> Self {
        match std::fs::read_to_string(Self::SETTINGS_FILE) {
            Ok(contents) => {
                match serde_json::from_str::<GameSettings>(&contents) {
                    Ok(settings) => {
                        println!("Loaded settings from {}", Self::SETTINGS_FILE);
                        settings
                    }
                    Err(e) => {
                        println!("Failed to parse settings file: {}. Using defaults.", e);
                        Self::default()
                    }
                }
            }
            Err(_) => {
                println!("Settings file not found. Creating default settings.");
                let default_settings = Self::default();
                default_settings.save(); // Save default settings to file
                default_settings
            }
        }
    }
    
    /// Save current settings to file
    pub fn save(&self) {
        match serde_json::to_string_pretty(self) {
            Ok(json) => {
                if let Err(e) = std::fs::write(Self::SETTINGS_FILE, json) {
                    println!("Failed to save settings: {}", e);
                } else {
                    println!("Settings saved to {}", Self::SETTINGS_FILE);
                }
            }
            Err(e) => {
                println!("Failed to serialize settings: {}", e);
            }
        }
    }
}

// ============================================================================
// SETTINGS MENU SETUP SYSTEM
// ============================================================================

pub fn setup_settings_menu(mut commands: Commands) {
    // Create settings menu overlay (initially hidden)
    let settings_menu_entity = commands.spawn((
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
        ZIndex(1001), // Higher than pause menu
        SettingsMenuOverlay,
    )).with_children(|parent| {
        // Settings menu panel
        parent.spawn((
            Node {
                width: Val::Px(500.0),
                height: Val::Px(500.0),  // More compact height
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(20.0)),  // Reduced padding
                row_gap: Val::Px(15.0),  // Reduced gap
                border: UiRect::all(Val::Px(3.0)),
                ..default()
            },
            BackgroundColor(UIColors::PANEL_BG),
            BorderColor(UIColors::PANEL_BORDER),
            BorderRadius::all(Val::Px(15.0)),
            SettingsMenu,
        )).with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("SETTINGS"),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
                TextColor(UIColors::TEXT_PRIMARY),
                Node {
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                },
            ));
            
            // Graphics Section Header
            create_section_header(parent, "GRAPHICS");
            
            // Resolution setting
            create_resolution_setting(parent);
            
            // Fullscreen toggle
            create_fullscreen_toggle(parent);
            
            // VSync toggle
            create_vsync_toggle(parent);
            
            // Audio Section Header
            create_section_header(parent, "AUDIO");
            
            // Volume sliders (more compact)
            create_compact_volume_slider(parent, "Master", SettingsType::MasterVolume, 1.0);
            create_compact_volume_slider(parent, "SFX", SettingsType::SFXVolume, 0.8);
            create_compact_volume_slider(parent, "Music", SettingsType::MusicVolume, 0.6);
            
            // Spacer to push buttons to bottom
            parent.spawn(Node {
                flex_grow: 1.0,
                ..default()
            });
            
            // Button container
            parent.spawn(Node {
                width: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                column_gap: Val::Px(20.0),
                ..default()
            }).with_children(|parent| {
                // Back button
                create_settings_button(parent, "BACK", SettingsMenuAction::Back, UIColors::TEXT_INFO);
                
                // Reset button
                create_settings_button(parent, "RESET TO DEFAULTS", SettingsMenuAction::ResetToDefaults, UIColors::TEXT_ERROR);
            });
            
            // Keyboard shortcut hint
            parent.spawn((
                Text::new("Press ESC to go back"),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(UIColors::TEXT_MUTED),
                Node {
                    margin: UiRect::top(Val::Px(10.0)),
                    ..default()
                },
            ));
        });
    }).id();

    // Store the settings menu entity ID as a resource for easy access
    commands.insert_resource(SettingsMenuEntity(settings_menu_entity));
}

fn create_section_header(parent: &mut ChildSpawnerCommands, text: &str) {
    parent.spawn((
        Text::new(text),
        TextFont {
            font_size: 18.0,  // Slightly smaller
            ..default()
        },
        TextColor(UIColors::TEXT_SECONDARY),
        Node {
            margin: UiRect {
                top: Val::Px(8.0),  // Reduced margins
                bottom: Val::Px(4.0),
                ..default()
            },
            ..default()
        },
    ));
}

fn create_compact_volume_slider(
    parent: &mut ChildSpawnerCommands,
    label: &str,
    setting_type: SettingsType,
    initial_value: f32,
) {
    parent.spawn(Node {
        width: Val::Percent(100.0),
        flex_direction: FlexDirection::Row,
        justify_content: JustifyContent::SpaceBetween,
        align_items: AlignItems::Center,
        ..default()
    }).with_children(|parent| {
        // Label (more compact)
        parent.spawn((
            Text::new(format!("{}:", label)),
            TextFont {
                font_size: 14.0,  // Smaller text
                ..default()
            },
            TextColor(UIColors::TEXT_PRIMARY),
        ));
        
        // Slider container (more compact)
        parent.spawn(Node {
            width: Val::Px(150.0),  // Smaller width
            height: Val::Px(16.0),  // Smaller height
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Center,
            ..default()
        }).with_children(|parent| {
            // Slider track
            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(4.0),
                    ..default()
                },
                BackgroundColor(UIColors::SLIDER_TRACK),
                BorderRadius::all(Val::Px(2.0)),
            ));
            
            // Slider fill (shows current value)
            parent.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(initial_value * 100.0),
                    height: Val::Px(4.0),
                    ..default()
                },
                BackgroundColor(UIColors::SLIDER_FILL),
                BorderRadius::all(Val::Px(2.0)),
                SettingsSlider {
                    setting_type,
                    value: initial_value,
                },
            ));
            
            // Slider handle
            parent.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(initial_value * 100.0 - 1.0), // Center on value
                    width: Val::Px(12.0),
                    height: Val::Px(12.0),
                    ..default()
                },
                BackgroundColor(UIColors::SLIDER_HANDLE),
                BorderRadius::all(Val::Px(6.0)),
            ));
        });
        
        // Value display
        parent.spawn((
            Text::new(format!("{:.0}%", initial_value * 100.0)),
            TextFont {
                font_size: 14.0,
                ..default()
            },
            TextColor(UIColors::TEXT_SECONDARY),
        ));
    });
}

fn create_resolution_setting(parent: &mut ChildSpawnerCommands) {
    parent.spawn(Node {
        width: Val::Percent(100.0),
        flex_direction: FlexDirection::Row,
        justify_content: JustifyContent::SpaceBetween,
        align_items: AlignItems::Center,
        ..default()
    }).with_children(|parent| {
        // Label
        parent.spawn((
            Text::new("Resolution:"),
            TextFont {
                font_size: 14.0,
                ..default()
            },
            TextColor(UIColors::TEXT_PRIMARY),
        ));
        
        // Resolution dropdown (simplified as button for now)
        parent.spawn((
            Button,
            Node {
                width: Val::Px(120.0),
                height: Val::Px(28.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            BackgroundColor(UIColors::BUTTON_DEFAULT),
            BorderColor(UIColors::BORDER_DEFAULT),
            ResolutionButton { resolution: ResolutionOption::Res1920x1080 },
        )).with_children(|button| {
            button.spawn((
                Text::new("1920x1080"),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(UIColors::TEXT_PRIMARY),
                ResolutionText,
            ));
        });
    });
}

fn create_fullscreen_toggle(parent: &mut ChildSpawnerCommands) {
    parent.spawn(Node {
        width: Val::Percent(100.0),
        flex_direction: FlexDirection::Row,
        justify_content: JustifyContent::SpaceBetween,
        align_items: AlignItems::Center,
        ..default()
    }).with_children(|parent| {
        // Label
        parent.spawn((
            Text::new("Fullscreen:"),
            TextFont {
                font_size: 14.0,
                ..default()
            },
            TextColor(UIColors::TEXT_PRIMARY),
        ));
        
        // Toggle button
        parent.spawn((
            Button,
            Node {
                width: Val::Px(80.0),
                height: Val::Px(28.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            BackgroundColor(UIColors::BUTTON_DEFAULT),
            BorderColor(UIColors::BORDER_DEFAULT),
            FullscreenToggle,
        )).with_children(|button| {
            button.spawn((
                Text::new("OFF"),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(UIColors::TEXT_PRIMARY),
                FullscreenText,
            ));
        });
    });
}

fn create_vsync_toggle(parent: &mut ChildSpawnerCommands) {
    parent.spawn(Node {
        width: Val::Percent(100.0),
        flex_direction: FlexDirection::Row,
        justify_content: JustifyContent::SpaceBetween,
        align_items: AlignItems::Center,
        ..default()
    }).with_children(|parent| {
        // Label
        parent.spawn((
            Text::new("VSync:"),
            TextFont {
                font_size: 14.0,
                ..default()
            },
            TextColor(UIColors::TEXT_PRIMARY),
        ));
        
        // Toggle button
        parent.spawn((
            Button,
            Node {
                width: Val::Px(80.0),
                height: Val::Px(28.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            BackgroundColor(UIColors::BUTTON_DEFAULT),
            BorderColor(UIColors::BORDER_DEFAULT),
            VSyncToggle,
        )).with_children(|button| {
            button.spawn((
                Text::new("ON"),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(UIColors::TEXT_PRIMARY),
                VSyncText,
            ));
        });
    });
}

fn create_settings_button(
    parent: &mut ChildSpawnerCommands,
    text: &str,
    action: SettingsMenuAction,
    text_color: Color,
) {
    parent.spawn((
        Button,
        Node {
            width: Val::Px(160.0),
            height: Val::Px(45.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect::all(Val::Px(2.0)),
            ..default()
        },
        BackgroundColor(UIColors::BUTTON_DEFAULT),
        BorderColor(UIColors::BORDER_DEFAULT),
        BorderRadius::all(Val::Px(6.0)),
        SettingsButton { action },
    )).with_children(|parent| {
        parent.spawn((
            Text::new(text),
            TextFont {
                font_size: 16.0,
                ..default()
            },
            TextColor(text_color),
        ));
    });
}

// ============================================================================
// SETTINGS MENU SYSTEMS
// ============================================================================

#[derive(Resource)]
pub struct SettingsMenuEntity(pub Entity);

/// System to show/hide settings menu based on app state
pub fn settings_menu_visibility_system(
    app_state: Res<State<AppState>>,
    settings_menu_entity: Res<SettingsMenuEntity>,
    mut visibility_query: Query<&mut Visibility>,
) {
    if app_state.is_changed() {
        if let Ok(mut visibility) = visibility_query.get_mut(settings_menu_entity.0) {
            *visibility = match app_state.get() {
                AppState::Settings => Visibility::Visible,
                _ => Visibility::Hidden,
            };
        }
    }
}

/// System to handle settings menu button interactions
pub fn settings_menu_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor, &SettingsButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, mut bg_color, mut border_color, settings_button) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                match &settings_button.action {
                    SettingsMenuAction::Back => {
                        next_state.set(AppState::Paused);
                        info!("Back button pressed - returning to pause menu");
                    }
                    SettingsMenuAction::ResetToDefaults => {
                        info!("Reset to defaults button pressed");
                    }
                    SettingsMenuAction::ToggleFullscreen => {
                        info!("Fullscreen toggle pressed");
                    }
                    SettingsMenuAction::ToggleVSync => {
                        info!("VSync toggle pressed");
                    }
                    SettingsMenuAction::ChangeResolution(_resolution) => {
                        info!("Resolution change pressed");
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

/// System to handle fullscreen toggle button
pub fn fullscreen_toggle_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<FullscreenToggle>),
    >,
    mut windows: Query<&mut Window>,
    mut game_settings: ResMut<GameSettings>,
) {
    for (interaction, mut bg_color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                game_settings.fullscreen_enabled = !game_settings.fullscreen_enabled;
                
                if let Ok(mut window) = windows.single_mut() {
                    window.mode = if game_settings.fullscreen_enabled {
                        bevy::window::WindowMode::Fullscreen(bevy::window::MonitorSelection::Current, bevy::window::VideoModeSelection::Current)
                    } else {
                        bevy::window::WindowMode::Windowed
                    };
                }
                
                info!("Fullscreen toggled: {}", game_settings.fullscreen_enabled);
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

/// System to handle VSync toggle button
pub fn vsync_toggle_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<VSyncToggle>),
    >,
    mut windows: Query<&mut Window>,
    mut game_settings: ResMut<GameSettings>,
) {
    for (interaction, mut bg_color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                game_settings.vsync_enabled = !game_settings.vsync_enabled;
                
                if let Ok(mut window) = windows.single_mut() {
                    window.present_mode = if game_settings.vsync_enabled {
                        bevy::window::PresentMode::AutoVsync
                    } else {
                        bevy::window::PresentMode::AutoNoVsync
                    };
                }
                
                info!("VSync toggled: {}", game_settings.vsync_enabled);
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

/// System to handle resolution button (cycles through available resolutions)
pub fn resolution_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor, &mut ResolutionButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut windows: Query<&mut Window>,
    mut game_settings: ResMut<GameSettings>,
) {
    for (interaction, mut bg_color, mut border_color, mut resolution_button) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                // Cycle to next resolution
                let resolutions = ResolutionOption::all();
                let current_index = resolutions.iter().position(|r| *r == resolution_button.resolution).unwrap_or(0);
                let next_index = (current_index + 1) % resolutions.len();
                let new_resolution = resolutions[next_index].clone();
                
                resolution_button.resolution = new_resolution.clone();
                game_settings.current_resolution = new_resolution.clone();
                
                if let Ok(mut window) = windows.single_mut() {
                    let size = new_resolution.to_vec2();
                    window.resolution = (size.x, size.y).into();
                }
                
                info!("Resolution changed to: {}", new_resolution.to_string());
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

/// System to update settings UI text based on current settings
pub fn update_settings_ui_system(
    game_settings: Res<GameSettings>,
    mut resolution_text_query: Query<&mut Text, (With<ResolutionText>, Without<FullscreenText>, Without<VSyncText>)>,
    mut fullscreen_text_query: Query<&mut Text, (With<FullscreenText>, Without<ResolutionText>, Without<VSyncText>)>,
    mut vsync_text_query: Query<&mut Text, (With<VSyncText>, Without<ResolutionText>, Without<FullscreenText>)>,
    mut resolution_button_query: Query<&mut ResolutionButton>,
) {
    if game_settings.is_changed() {
        // Update resolution text
        if let Ok(mut text) = resolution_text_query.single_mut() {
            **text = game_settings.current_resolution.to_string();
        }
        
        // Update fullscreen text
        if let Ok(mut text) = fullscreen_text_query.single_mut() {
            **text = if game_settings.fullscreen_enabled { "ON" } else { "OFF" }.to_string();
        }
        
        // Update VSync text
        if let Ok(mut text) = vsync_text_query.single_mut() {
            **text = if game_settings.vsync_enabled { "ON" } else { "OFF" }.to_string();
        }
        
        // Update resolution button state
        if let Ok(mut resolution_button) = resolution_button_query.single_mut() {
            resolution_button.resolution = game_settings.current_resolution.clone();
        }
    }
}

// ============================================================================
// SETTINGS PERSISTENCE SYSTEMS
// ============================================================================

/// System to automatically save settings when they change
pub fn save_settings_on_change(
    settings: Res<GameSettings>,
) {
    if settings.is_changed() {
        settings.save();
    }
}

/// System to apply loaded settings to the window on startup
pub fn apply_loaded_settings_to_window(
    settings: Res<GameSettings>,
    mut windows: Query<&mut Window>,
) {
    if let Ok(mut window) = windows.single_mut() {
        // Apply resolution from loaded settings
        let resolution = settings.current_resolution.to_vec2();
        window.resolution.set(resolution.x, resolution.y);
        
        // Apply fullscreen setting
        window.mode = if settings.fullscreen_enabled {
            bevy::window::WindowMode::Fullscreen(
                bevy::window::MonitorSelection::Current,
                bevy::window::VideoModeSelection::Current
            )
        } else {
            bevy::window::WindowMode::Windowed
        };
        
        // Apply VSync setting
        window.present_mode = if settings.vsync_enabled {
            bevy::window::PresentMode::AutoVsync
        } else {
            bevy::window::PresentMode::AutoNoVsync
        };
        
        println!("Applied settings: {}x{}, Fullscreen: {}, VSync: {}", 
                 resolution.x, resolution.y, 
                 settings.fullscreen_enabled, 
                 settings.vsync_enabled);
    }
}

// ============================================================================
// SETTINGS SYSTEM PLUGIN
// ============================================================================

pub struct SettingsSystemPlugin;

impl Plugin for SettingsSystemPlugin {
    fn build(&self, app: &mut App) {
        app
            // GameSettings resource is now loaded earlier in main.rs to ensure availability
            .add_systems(Startup, (setup_settings_menu, apply_loaded_settings_to_window))
            .add_systems(
                Update,
                (settings_menu_visibility_system, save_settings_on_change).in_set(GameSystemSet::UI)
            )
            .add_systems(
                Update,
                (
                    settings_menu_button_system,
                    fullscreen_toggle_system,
                    vsync_toggle_system,
                    resolution_button_system,
                    update_settings_ui_system,
                )
                    .in_set(GameSystemSet::Settings)
                    .run_if(in_state(AppState::Settings))
            );
    }
}