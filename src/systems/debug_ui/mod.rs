// Debug UI module with split responsibilities for maintainability

pub mod components;
pub mod interactions;
pub mod performance;
pub mod plugin;
pub mod setup;
pub mod cheat_menu;
pub mod cheat_interactions;
pub mod cheat_multipliers;

// Re-export the main plugin for external use
pub use plugin::DebugUIPlugin;

// Re-export key components that other systems might need
pub use components::{DebugUIState, DebugUIPanel};
pub use cheat_menu::{CheatMenuState, CheatMultipliers, CheatMenuPanel};

// Re-export key functions for backward compatibility
pub use interactions::debug_ui_toggle_system;
pub use setup::setup_debug_ui;
pub use cheat_menu::{cheat_menu_toggle_system, setup_cheat_menu};