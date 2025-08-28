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

// Re-export key functions with standardized names
pub use interactions::f2_debug_ui_panel_toggle;
pub use setup::setup_debug_ui;
pub use cheat_menu::{f9_cheat_menu_toggle, setup_cheat_menu};

// DEPRECATED: Legacy function names for backward compatibility (will be removed)
pub use interactions::f2_debug_ui_panel_toggle as debug_ui_toggle_system;
pub use cheat_menu::f9_cheat_menu_toggle as cheat_menu_toggle_system;