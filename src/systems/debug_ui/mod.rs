// Debug UI module with split responsibilities for maintainability

pub mod components;
pub mod interactions;
pub mod performance;
pub mod plugin;
pub mod setup;

// Re-export the main plugin for external use
pub use plugin::DebugUIPlugin;

// Re-export key components that other systems might need
pub use components::{DebugUIState, DebugUIPanel};

// Re-export key functions for backward compatibility
pub use interactions::debug_ui_toggle_system;
pub use setup::setup_debug_ui;