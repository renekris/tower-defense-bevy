use bevy::prelude::*;
use crate::resources::*;
use crate::components::*;
use super::components::DebugUIState;
use crate::systems::combat_system::{WaveStatus, Target};

/// Resource to manage cheat menu state
#[derive(Resource, Debug)]
pub struct CheatMenuState {
    pub visible: bool,
    pub god_mode: bool,
}

impl Default for CheatMenuState {
    fn default() -> Self {
        Self {
            visible: false,
            god_mode: false,
        }
    }
}

/// Resource containing all cheat multipliers
#[derive(Resource, Debug)]
pub struct CheatMultipliers {
    pub tower_damage: f32,
    pub tower_range: f32,
    pub tower_fire_rate: f32,
    pub enemy_health: f32,
    pub enemy_speed: f32,
}

impl Default for CheatMultipliers {
    fn default() -> Self {
        Self {
            tower_damage: 1.0,
            tower_range: 1.0,
            tower_fire_rate: 1.0,
            enemy_health: 1.0,
            enemy_speed: 1.0,
        }
    }
}

/// Component marker for the cheat menu panel
#[derive(Component)]
pub struct CheatMenuPanel;

/// Component marker for cheat menu sections
#[derive(Component)]
pub struct CheatSection {
    pub section_type: CheatSectionType,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CheatSectionType {
    Currency,
    Stats,
    GameState,
}

/// Component for cheat buttons
#[derive(Component)]
pub struct CheatButton {
    pub button_type: CheatButtonType,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CheatButtonType {
    // Currency buttons
    AddMoney100,
    AddMoney1K,
    AddMoney10K,
    SetMoneyMax,
    AddResearch10,
    AddResearch100,
    SetResearchMax,
    AddMaterials10,
    AddMaterials100,
    SetMaterialsMax,
    AddEnergy10,
    AddEnergy100,
    SetEnergyMax,
    ResetAllResources,
    
    // Game state buttons
    NextWave,
    InstantWin,
    ResetGame,
    ToggleGodMode,
}

/// Component for cheat sliders
#[derive(Component)]
pub struct CheatSlider {
    pub slider_type: CheatSliderType,
    pub min_value: f32,
    pub max_value: f32,
    pub current_value: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CheatSliderType {
    TowerDamage,
    TowerRange,
    TowerFireRate,
    EnemyHealth,
    EnemySpeed,
}

/// Component for cheat slider handles
#[derive(Component)]
pub struct CheatSliderHandle {
    pub slider_type: CheatSliderType,
}

/// Component for cheat slider tracks
#[derive(Component)]
pub struct CheatSliderTrack {
    pub slider_type: CheatSliderType,
}

/// Component for cheat slider value text
#[derive(Component)]
pub struct CheatSliderValueText {
    pub slider_type: CheatSliderType,
}

/// Resource to track which cheat slider is being dragged
#[derive(Resource, Default)]
pub struct CheatSliderDragState {
    pub dragging: Option<CheatSliderType>,
}

// System functions

/// System to handle F9 key toggle for cheat menu
pub fn cheat_menu_toggle_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut cheat_state: ResMut<CheatMenuState>,
) {
    if keyboard_input.just_pressed(KeyCode::F9) {
        cheat_state.visible = !cheat_state.visible;
        println!("Cheat menu: {}", if cheat_state.visible { "enabled" } else { "disabled" });
    }
}

/// System to update cheat menu visibility
pub fn update_cheat_menu_visibility(
    cheat_state: Res<CheatMenuState>,
    mut panel_query: Query<&mut Node, With<CheatMenuPanel>>,
) {
    if cheat_state.is_changed() {
        for mut node in &mut panel_query {
            node.display = if cheat_state.visible {
                Display::Flex
            } else {
                Display::None
            };
        }
    }
}

/// Main setup system for cheat menu UI
pub fn setup_cheat_menu(mut commands: Commands) {
    println!("DEBUG: Creating cheat menu panel");
    
    // Create the main cheat menu panel
    let panel_entity = commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Percent(50.0),
                top: Val::Percent(50.0),
                width: Val::Px(400.0),
                height: Val::Px(600.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(15.0)),
                display: Display::None, // Hidden by default
                margin: UiRect {
                    left: Val::Px(-200.0), // Center horizontally
                    top: Val::Px(-300.0),  // Center vertically
                    ..default()
                },
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.95)),
            BorderColor(Color::srgb(0.3, 0.3, 0.3)),
            CheatMenuPanel,
        ))
        .id();
    
    // Add dark overlay behind menu
    let overlay_entity = commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::None,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
            CheatMenuPanel, // Same component so it shows/hides together
        ))
        .id();
        
    println!("DEBUG: Cheat menu panel created");
    
    // Add all UI sections to the panel
    commands.entity(panel_entity).with_children(|parent| {
        // Panel title
        parent.spawn((
            Text::new("CHEAT MENU (F9 to close)"),
            TextFont {
                font_size: 18.0,
                ..default()
            },
            TextColor(Color::srgb(1.0, 0.3, 0.3)),
            Node {
                margin: UiRect::bottom(Val::Px(15.0)),
                justify_content: JustifyContent::Center,
                ..default()
            },
        ));
        
        // Create all cheat sections
        create_cheat_section(parent, CheatSectionType::Currency);
        create_cheat_section(parent, CheatSectionType::Stats);
        create_cheat_section(parent, CheatSectionType::GameState);
    });
    
    println!("DEBUG: All cheat sections added to panel");
}

/// Helper function to create cheat sections
fn create_cheat_section(parent: &mut ChildSpawnerCommands, section_type: CheatSectionType) {
    let section_title = match section_type {
        CheatSectionType::Currency => "CURRENCY",
        CheatSectionType::Stats => "STAT MULTIPLIERS", 
        CheatSectionType::GameState => "GAME STATE",
    };

    parent.spawn((
        Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            margin: UiRect::bottom(Val::Px(12.0)),
            padding: UiRect::all(Val::Px(8.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.8)),
        CheatSection { section_type },
    )).with_children(|section| {
        // Section header
        section.spawn((
            Text::new(section_title),
            TextFont {
                font_size: 14.0,
                ..default()
            },
            TextColor(Color::srgb(1.0, 1.0, 0.6)), // Light yellow for headers
            Node {
                margin: UiRect::bottom(Val::Px(8.0)),
                ..default()
            },
        ));

        // Create specific content based on section type
        match section_type {
            CheatSectionType::Currency => create_currency_section(section),
            CheatSectionType::Stats => create_stats_section(section),
            CheatSectionType::GameState => create_game_state_section(section),
        }
    });
}

/// Create currency manipulation buttons
fn create_currency_section(parent: &mut ChildSpawnerCommands) {
    // Money row
    create_currency_row(parent, "Money:", &[
        (CheatButtonType::AddMoney100, "+100"),
        (CheatButtonType::AddMoney1K, "+1K"), 
        (CheatButtonType::AddMoney10K, "+10K"),
        (CheatButtonType::SetMoneyMax, "MAX"),
    ]);
    
    // Research row
    create_currency_row(parent, "Research:", &[
        (CheatButtonType::AddResearch10, "+10"),
        (CheatButtonType::AddResearch100, "+100"),
        (CheatButtonType::SetResearchMax, "MAX"),
    ]);
    
    // Materials row
    create_currency_row(parent, "Materials:", &[
        (CheatButtonType::AddMaterials10, "+10"),
        (CheatButtonType::AddMaterials100, "+100"),
        (CheatButtonType::SetMaterialsMax, "MAX"),
    ]);
    
    // Energy row
    create_currency_row(parent, "Energy:", &[
        (CheatButtonType::AddEnergy10, "+10"),
        (CheatButtonType::AddEnergy100, "+100"),
        (CheatButtonType::SetEnergyMax, "MAX"),
    ]);
    
    // Reset button
    parent.spawn((
        Button,
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(30.0),
            margin: UiRect::top(Val::Px(8.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgb(0.6, 0.2, 0.2)),
        CheatButton { button_type: CheatButtonType::ResetAllResources },
    )).with_children(|button| {
        button.spawn((
            Text::new("RESET ALL RESOURCES"),
            TextFont {
                font_size: 11.0,
                ..default()
            },
            TextColor(Color::WHITE),
        ));
    });
}

/// Helper to create a row of currency buttons
fn create_currency_row(parent: &mut ChildSpawnerCommands, label: &str, buttons: &[(CheatButtonType, &str)]) {
    parent.spawn((
        Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            margin: UiRect::bottom(Val::Px(4.0)),
            ..default()
        },
    )).with_children(|row| {
        // Label
        row.spawn((
            Text::new(label),
            TextFont {
                font_size: 10.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
            Node {
                width: Val::Px(70.0),
                ..default()
            },
        ));
        
        // Buttons
        for (button_type, text) in buttons {
            row.spawn((
                Button,
                Node {
                    width: Val::Px(50.0),
                    height: Val::Px(25.0),
                    margin: UiRect::right(Val::Px(4.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.3, 0.5, 0.3)),
                CheatButton { button_type: *button_type },
            )).with_children(|button| {
                button.spawn((
                    Text::new(*text),
                    TextFont {
                        font_size: 9.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
            });
        }
    });
}

/// Create stat multiplier sliders
fn create_stats_section(parent: &mut ChildSpawnerCommands) {
    let sliders = [
        (CheatSliderType::TowerDamage, "Tower Damage", 0.1, 10.0, 1.0),
        (CheatSliderType::TowerRange, "Tower Range", 0.5, 3.0, 1.0),
        (CheatSliderType::TowerFireRate, "Fire Rate", 0.1, 5.0, 1.0),
        (CheatSliderType::EnemyHealth, "Enemy Health", 0.1, 10.0, 1.0),
        (CheatSliderType::EnemySpeed, "Enemy Speed", 0.1, 5.0, 1.0),
    ];

    for (slider_type, label, min_val, max_val, default_val) in sliders {
        create_cheat_slider(parent, slider_type, label, min_val, max_val, default_val);
    }
}

/// Helper to create individual cheat sliders
fn create_cheat_slider(
    parent: &mut ChildSpawnerCommands,
    slider_type: CheatSliderType,
    label: &str,
    min_val: f32,
    max_val: f32,
    default_val: f32,
) {
    parent.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(35.0),
            flex_direction: FlexDirection::Column,
            margin: UiRect::bottom(Val::Px(6.0)),
            ..default()
        },
    )).with_children(|slider_container| {
        // Label with value
        slider_container.spawn((
            Text::new(format!("{}: {:.2}x", label, default_val)),
            TextFont {
                font_size: 10.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
            CheatSliderValueText { slider_type },
        ));
        
        // Slider track
        slider_container.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(12.0),
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                margin: UiRect::top(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
            CheatSliderTrack { slider_type },
        )).with_children(|track| {
            // Slider handle
            track.spawn((
                Button,
                Node {
                    width: Val::Px(16.0),
                    height: Val::Px(16.0),
                    position_type: PositionType::Absolute,
                    left: Val::Percent(50.0 * (default_val - min_val) / (max_val - min_val)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.8, 0.8, 0.8)),
                CheatSliderHandle { slider_type },
                CheatSlider {
                    slider_type,
                    min_value: min_val,
                    max_value: max_val,
                    current_value: default_val,
                },
            ));
        });
    });
}

/// Create game state manipulation buttons
fn create_game_state_section(parent: &mut ChildSpawnerCommands) {
    let buttons = [
        (CheatButtonType::NextWave, "NEXT WAVE", Color::srgb(0.3, 0.3, 0.7)),
        (CheatButtonType::InstantWin, "INSTANT WIN", Color::srgb(0.3, 0.7, 0.3)),
        (CheatButtonType::ResetGame, "RESET GAME", Color::srgb(0.7, 0.3, 0.3)),
        (CheatButtonType::ToggleGodMode, "GOD MODE: OFF", Color::srgb(0.7, 0.7, 0.3)),
    ];

    parent.spawn((
        Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Row,
            flex_wrap: FlexWrap::Wrap,
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        },
    )).with_children(|button_row| {
        for (button_type, text, color) in buttons {
            button_row.spawn((
                Button,
                Node {
                    width: Val::Percent(48.0),
                    height: Val::Px(30.0),
                    margin: UiRect::bottom(Val::Px(4.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(color),
                CheatButton { button_type },
            )).with_children(|button| {
                button.spawn((
                    Text::new(text),
                    TextFont {
                        font_size: 9.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
            });
        }
    });
}