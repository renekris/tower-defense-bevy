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

/// Plugin for interactive debug UI controls
pub struct DebugUIPlugin;

impl Plugin for DebugUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<DebugUIState>()
            .init_resource::<SliderDragState>()
            .init_resource::<PerformanceMetrics>()
            .add_systems(Startup, setup_debug_ui)
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
            .add_systems(Update, sync_ui_with_debug_state);
    }
}