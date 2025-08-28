use bevy::prelude::*;
use bevy_brp_extras::BrpExtrasPlugin;

mod components;
mod resources;
mod systems;

// Explicit imports to prevent namespace pollution
use resources::{Economy, GameState, Score, WaveManager, EnemyPath, AppState, GameSystemSet};
use systems::enemy_system::{enemy_spawning_system, enemy_movement_system, enemy_cleanup_system};
use systems::input_system::{mouse_input_system, tower_placement_system, tower_placement_preview_system, MouseInputState, auto_grid_mode_system};
use systems::ui_system::{update_ui_system};
use systems::combat_system::{tower_targeting_system, projectile_spawning_system, projectile_movement_system, collision_system, game_state_system, WaveStatus};
use systems::debug_visualization::{DebugVisualizationState, debug_visualization_system};
use systems::debug_ui::{DebugUIState, setup_debug_ui, DebugUIPlugin};
use systems::debug_ui::cheat_menu::CheatMenuState;
use systems::input::InputRegistryPlugin;
use systems::enemy_system::{manual_wave_system, path_generation_system, path_visualization_system, StartWaveEvent};
use systems::tower_ui::{
    TowerSelectionState, 
    TowerStatPopupState,
    setup_tower_placement_panel, 
    setup_tower_upgrade_panel,
    setup_tower_stat_popup,
    tower_selection_system,
    tower_type_button_system,
    upgrade_button_system,
    update_upgrade_panel_system,
    selected_tower_indicator_system,
    update_resource_status_system,
    tower_tooltip_system,
    tower_affordability_system,
    tower_stat_popup_system,
    hover_stat_popup_system,
    popup_close_button_system,
    popup_outside_click_system,
    start_wave_button_system,
    update_start_wave_button_system,
};
use systems::unified_grid::{
    UnifiedGridSystem,
    setup_unified_grid,
    update_grid_visualization,
};
use systems::obstacle_rendering::ObstacleRenderingPlugin;
use systems::tower_rendering::TowerRenderingPlugin;
use systems::path_generation::generate_level_path;
use systems::pause_system::{PauseSystemPlugin, pause_toggle_system};
use systems::settings_menu::SettingsSystemPlugin;

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
        // Add BRP Extras plugin (includes RemotePlugin for MCP server integration)
        .add_plugins(BrpExtrasPlugin)
        // Add custom plugins (ORDER MATTERS: SettingsSystemPlugin must come before SecurityPlugin)
        .add_plugins(SettingsSystemPlugin) // Must be first - loads GameSettings resource
        .add_plugins(tower_defense_bevy::systems::security::SecurityPlugin) // Security and authorization
        .add_plugins(InputRegistryPlugin::default()) // Centralized input handling
        .add_plugins(DebugUIPlugin)
        .add_plugins(ObstacleRenderingPlugin)
        .add_plugins(TowerRenderingPlugin)
        .add_plugins(PauseSystemPlugin)
        // Add events
        .add_event::<StartWaveEvent>()
        // Initialize state and resources
        .init_state::<AppState>()
        .init_resource::<Score>()
        .init_resource::<WaveManager>()
        .init_resource::<GameState>()
        .init_resource::<Economy>()
        .init_resource::<MouseInputState>()
        .init_resource::<WaveStatus>()
        .init_resource::<DebugVisualizationState>()
        .init_resource::<CheatMenuState>()
        .init_resource::<TowerSelectionState>()
        .init_resource::<TowerStatPopupState>()
        .init_resource::<UnifiedGridSystem>()
        .insert_resource(generate_level_path(1)) // Start with wave 1 generated path
        // Configure system sets
        .configure_sets(Update, (
            GameSystemSet::Input,
            GameSystemSet::UI,
            GameSystemSet::Gameplay,
        ).chain())
        // Setup systems - Stat popup last for proper Z-order (renders on top)
        .add_systems(Startup, (setup, setup_unified_grid, setup_tower_placement_panel, setup_tower_upgrade_panel, setup_tower_stat_popup).chain())
        // Input systems - run in all states
        .add_systems(Update, (
            mouse_input_system,
        ).in_set(GameSystemSet::Input))
        // UI systems - run in all states
        .add_systems(Update, (
            // UI interaction systems (consume UI clicks)
            tower_type_button_system,
            upgrade_button_system,
            tower_selection_system,
            popup_close_button_system,
            popup_outside_click_system,
            start_wave_button_system,
            
            // UI update systems
            update_upgrade_panel_system,
            selected_tower_indicator_system,
            update_resource_status_system,
            tower_tooltip_system,
            tower_affordability_system,
            tower_stat_popup_system,
            hover_stat_popup_system,
            update_start_wave_button_system,
            update_ui_system,
        ).chain().in_set(GameSystemSet::UI))
        // Gameplay systems - only run in Playing state
        .add_systems(Update, (
            // Tower placement systems
            tower_placement_system,
            tower_placement_preview_system,
            
            // Grid visualization systems
            auto_grid_mode_system,
            update_grid_visualization,
            
            // Debug visualization systems
            debug_visualization_system,
            
            // Combat systems (ORDER CRITICAL - dependency chain)
            tower_targeting_system,
            projectile_spawning_system,
            projectile_movement_system,
            collision_system,
            
            // Enemy and wave management (CRITICAL: path generation runs BEFORE spawning)
            manual_wave_system,
            path_generation_system, // Updates path when wave changes
            path_visualization_system, // Updates visual path representation
            enemy_spawning_system,
            enemy_movement_system,
            enemy_cleanup_system,
            
            // Game state management (runs last)
            game_state_system,
        ).in_set(GameSystemSet::Gameplay).run_if(in_state(AppState::Playing)))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
    
    commands.spawn((
        Text2d::new("Tower Defense Game - Phase 3 COMBAT!\nSTART WAVE button: spawn wave | ESC: pause menu\nLEFT CLICK tower button: select | RIGHT CLICK tower button: detailed stats\nLEFT CLICK: place tower | Click tower: upgrade mode\nF1: toggle debug visualization | F2: debug UI panel | F3: grid mode | F4: toggle grid | 1-9: select wave (debug mode)\nTowers auto-target and shoot enemies! Defend the base!"),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 1.0, 1.0)),
        Transform::from_translation(Vec3::new(0.0, 330.0, 0.0)),
    ));

    // Initial path visualization - will be updated dynamically by path_visualization_system
    // This creates placeholder entities that will be updated when the path changes
}

/// Create a simple straight-line path for Phase 1
fn create_default_path() -> EnemyPath {
    EnemyPath::new(vec![
        Vec2::new(-600.0, 0.0),  // Start left side of screen
        Vec2::new(600.0, 0.0),   // End right side of screen
    ])
}

// ESC key handling moved to pause_toggle_system in PauseSystemPlugin