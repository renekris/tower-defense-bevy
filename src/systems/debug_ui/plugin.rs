use bevy::prelude::*;
use super::components::{DebugUIState, SliderDragState, PerformanceMetrics};
use super::setup::setup_debug_ui;
use super::interactions::{
    debug_ui_toggle_system, update_debug_ui_visibility, handle_toggle_button_interactions,
    handle_slider_interactions, handle_action_buttons, handle_debug_keyboard_shortcuts,
    update_slider_values, update_enemy_path_from_ui, update_spawn_rate_from_ui,
    sync_ui_with_debug_state
};
use super::performance::{update_performance_metrics, update_performance_display};
use super::cheat_menu::{CheatMenuState, CheatMultipliers, CheatSliderDragState, setup_cheat_menu, cheat_menu_toggle_system, update_cheat_menu_visibility};
use super::cheat_interactions::{handle_cheat_button_interactions, handle_cheat_slider_interactions, update_cheat_slider_values, update_god_mode_button_text};
use super::cheat_multipliers::{apply_tower_multipliers_system, apply_enemy_multipliers_system, apply_god_mode_system, maintain_god_mode_system, validate_enemy_stats_system, validate_tower_stats_system, cheat_visual_feedback_system, reset_visual_effects_system, handle_extreme_fire_rates_system, handle_extreme_damage_system, enhanced_enemy_spawn_system};

/// Plugin for interactive debug UI controls
pub struct DebugUIPlugin;

impl Plugin for DebugUIPlugin {
    fn build(&self, app: &mut App) {
        app
            // Original debug UI resources
            .init_resource::<DebugUIState>()
            .init_resource::<SliderDragState>()
            .init_resource::<PerformanceMetrics>()
            
            // Cheat menu resources
            .init_resource::<CheatMenuState>()
            .init_resource::<CheatMultipliers>()
            .init_resource::<CheatSliderDragState>()
            
            // Setup systems
            .add_systems(Startup, (setup_debug_ui, setup_cheat_menu))
            
            // Original debug UI systems
            .add_systems(Update, debug_ui_toggle_system)
            .add_systems(Update, update_debug_ui_visibility)
            .add_systems(Update, handle_toggle_button_interactions)
            .add_systems(Update, handle_slider_interactions)
            .add_systems(Update, handle_action_buttons)
            .add_systems(Update, handle_debug_keyboard_shortcuts)
            .add_systems(Update, update_slider_values)
            .add_systems(Update, update_enemy_path_from_ui)
            .add_systems(Update, update_spawn_rate_from_ui)
            .add_systems(Update, update_performance_metrics)
            .add_systems(Update, update_performance_display)
            .add_systems(Update, sync_ui_with_debug_state)
            
            // Cheat menu systems
            .add_systems(Update, cheat_menu_toggle_system)
            .add_systems(Update, update_cheat_menu_visibility)
            .add_systems(Update, handle_cheat_button_interactions)
            .add_systems(Update, handle_cheat_slider_interactions)
            .add_systems(Update, update_cheat_slider_values)
            .add_systems(Update, update_god_mode_button_text)
            
            // Cheat multiplier application systems
            .add_systems(Update, apply_tower_multipliers_system)
            .add_systems(Update, apply_enemy_multipliers_system)
            .add_systems(Update, apply_god_mode_system)
            .add_systems(Update, maintain_god_mode_system)
            .add_systems(Update, validate_enemy_stats_system)
            .add_systems(Update, validate_tower_stats_system)
            .add_systems(Update, cheat_visual_feedback_system)
            .add_systems(Update, reset_visual_effects_system)
            .add_systems(Update, handle_extreme_fire_rates_system)
            .add_systems(Update, handle_extreme_damage_system)
            .add_systems(Update, enhanced_enemy_spawn_system);
    }
}