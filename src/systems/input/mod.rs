//! Centralized Input Mapping System
//! 
//! This module provides a comprehensive solution for managing input handlers
//! across the tower defense game, replacing the scattered F-key handling
//! approach with a centralized, conflict-aware system.
//! 
//! ## Features
//! 
//! * **Centralized Registry**: Single source of truth for all key bindings
//! * **Conflict Detection**: Automatic detection and logging of key conflicts
//! * **Priority System**: Handlers with higher priority take precedence
//! * **Context Awareness**: Handlers are organized by context (Game, Debug, UI, etc.)
//! * **Plugin Architecture**: Easy integration via Bevy plugin system
//! * **Runtime Monitoring**: Debug tools for monitoring input handling
//! * **Extensible Design**: Easy to add new handlers and key combinations
//! 
//! ## Quick Start
//! 
//! ```rust
//! use bevy::prelude::*;
//! use crate::systems::input::plugin::InputRegistryPlugin;
//! 
//! fn main() {
//!     App::new()
//!         .add_plugins((
//!             DefaultPlugins,
//!             // Add with default settings (registers standard F-key handlers)
//!             InputRegistryPlugin::default(),
//!         ))
//!         .run();
//! }
//! ```
//! 
//! ## Custom Configuration
//! 
//! ```rust
//! use crate::systems::input::plugin::{InputRegistryPlugin, InputRegistryPluginBuilder};
//! 
//! App::new()
//!     .add_plugins(
//!         InputRegistryPluginBuilder::new()
//!             .with_debug_logging()
//!             .with_combined_grid_handler()
//!             .build()
//!     )
//!     .run();
//! ```
//! 
//! ## Current Key Mappings
//! 
//! | Key | Handler | Description | Priority |
//! |-----|---------|-------------|----------|
//! | F1  | debug_visualization | Toggle debug visualization and switch to debug grid mode | 30 |
//! | F2  | debug_ui | Toggle debug UI panel visibility | 30 |  
//! | F3  | grid_mode | Cycle grid visualization mode (Normal -> Debug -> Placement) | 20 |
//! | F4  | grid_border | Toggle grid border visibility | 20 |
//! | F9  | cheat_menu | Toggle cheat menu visibility | 40 |
//! 
//! ## Adding Custom Handlers
//! 
//! ### Simple Handler
//! ```rust
//! use crate::systems::input::registry::{InputHandler, InputContext};
//! 
//! struct MyCustomHandler;
//! 
//! impl InputHandler for MyCustomHandler {
//!     fn handle_input(&self, world: &mut World, key: KeyCode) -> bool {
//!         if key == KeyCode::F5 {
//!             println!("F5 pressed!");
//!             true // Input consumed
//!         } else {
//!             false // Input not handled
//!         }
//!     }
//!     
//!     fn get_description(&self) -> &str { "My custom F5 handler" }
//!     fn get_id(&self) -> &str { "my_custom_handler" }
//!     fn get_priority(&self) -> u8 { 25 }
//!     fn handles_key(&self, key: KeyCode) -> bool { key == KeyCode::F5 }
//!     fn get_handled_keys(&self) -> Vec<KeyCode> { vec![KeyCode::F5] }
//!     fn get_context(&self) -> InputContext { InputContext::Game }
//! }
//! ```
//! 
//! ### Using the Macro
//! ```rust
//! use crate::create_input_handler;
//! 
//! create_input_handler!(
//!     MyF6Handler,
//!     "my_f6_handler",
//!     "Toggle something with F6",
//!     25,
//!     InputContext::Game,
//!     [KeyCode::F6],
//!     |world: &mut World, key: KeyCode| {
//!         println!("F6 handler called!");
//!         true
//!     }
//! );
//! ```
//! 
//! ## Conflict Resolution
//! 
//! When multiple handlers claim the same key:
//! 
//! 1. **Priority Order**: Handlers with higher priority values are processed first
//! 2. **First Consumption**: The first handler to return `true` consumes the input
//! 3. **Conflict Logging**: All conflicts are logged with details for debugging
//! 4. **Registration Order**: If priorities are equal, registration order determines precedence
//! 
//! ## Priority Guidelines
//! 
//! * **100+**: Critical system handlers (pause menu, emergency exits)
//! * **50-99**: UI and menu handlers (debug panels, cheat menus)  
//! * **10-49**: Game state handlers (debug visualization, grid modes)
//! * **1-9**: Low priority handlers (convenience shortcuts)
//! * **0**: Default priority (not recommended)
//! 
//! ## Migration from Old System
//! 
//! The centralized system maintains 100% backward compatibility with existing
//! F-key behaviors while providing the foundation for future enhancements.
//! 
//! ### Before (Scattered)
//! ```rust
//! // In debug_visualization.rs
//! fn debug_toggle_system(keyboard_input: Res<ButtonInput<KeyCode>>, ...) {
//!     if keyboard_input.just_pressed(KeyCode::F1) { ... }
//! }
//! 
//! // In debug_ui/interactions.rs  
//! fn debug_ui_toggle_system(keyboard_input: Res<ButtonInput<KeyCode>>, ...) {
//!     if keyboard_input.just_pressed(KeyCode::F2) { ... }
//! }
//! ```
//! 
//! ### After (Centralized)
//! ```rust
//! // All handled through InputRegistryPlugin automatically
//! App::new()
//!     .add_plugins(InputRegistryPlugin::default())
//!     .run();
//! ```
//! 
//! ## Development and Debugging
//! 
//! * **Ctrl+F12**: Show registry statistics in debug mode
//! * **Debug Logging**: Enable via `InputRegistryPluginBuilder::with_debug_logging()`
//! * **Conflict Detection**: Automatic logging of all key conflicts
//! * **Runtime Monitoring**: Live monitoring of input handler execution

pub mod registry;
pub mod handlers;
pub mod plugin;

// Re-export commonly used types and functions
pub use registry::{
    InputHandler, 
    InputContext, 
    InputMappingRegistry, 
    InputConflict,
    InputRegistryStats,
    process_centralized_input,
};

pub use handlers::{
    F1DebugVisualizationHandler,
    F2DebugUIHandler, 
    F3GridModeHandler,
    F4GridBorderHandler,
    F9CheatMenuHandler,
    GridSystemHandler,
    create_standard_fkey_handlers,
    create_combined_grid_handler,
};

pub use plugin::{
    InputRegistryPlugin,
    InputRegistryPluginBuilder,
    InputRegistryUpdateEvent,
    InputRegistryStatsEvent,
    RegistryAction,
    InputRegistryAppExt,
};

/// Convenience macro for creating simple input handlers
/// 
/// This macro simplifies the creation of input handlers that follow
/// common patterns. See module documentation for usage examples.
#[macro_export]
macro_rules! create_input_handler {
    ($name:ident, $id:expr, $description:expr, $priority:expr, $context:expr, $keys:expr, $handler_fn:expr) => {
        pub struct $name;
        
        impl $crate::systems::input::registry::InputHandler for $name {
            fn handle_input(&self, world: &mut World, key: bevy::prelude::KeyCode) -> bool {
                if self.handles_key(key) {
                    ($handler_fn)(world, key)
                } else {
                    false
                }
            }
            
            fn get_description(&self) -> &str {
                $description
            }
            
            fn get_priority(&self) -> u8 {
                $priority
            }
            
            fn get_id(&self) -> &str {
                $id
            }
            
            fn handles_key(&self, key: bevy::prelude::KeyCode) -> bool {
                $keys.contains(&key)
            }
            
            fn get_handled_keys(&self) -> Vec<bevy::prelude::KeyCode> {
                $keys.to_vec()
            }
            
            fn get_context(&self) -> $crate::systems::input::registry::InputContext {
                $context
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::prelude::*;
    use registry::InputMappingRegistry;
    use std::sync::Arc;
    
    #[test]
    fn test_registry_creation() {
        let registry = InputMappingRegistry::new();
        let stats = registry.get_stats();
        
        assert_eq!(stats.total_handlers, 0);
        assert_eq!(stats.total_keys, 0);
        assert_eq!(stats.total_conflicts, 0);
    }
    
    #[test]
    fn test_handler_registration() {
        let mut registry = InputMappingRegistry::new();
        let handler = Arc::new(handlers::F1DebugVisualizationHandler);
        
        let result = registry.register_handler(handler);
        assert!(result.is_ok());
        
        let stats = registry.get_stats();
        assert_eq!(stats.total_handlers, 1);
        assert_eq!(stats.total_keys, 1);
    }
    
    #[test]
    fn test_conflict_detection() {
        let mut registry = InputMappingRegistry::new();
        
        // Create two handlers for the same key
        let handler1 = Arc::new(handlers::F1DebugVisualizationHandler);
        let handler2 = Arc::new(handlers::F1DebugVisualizationHandler); // Same key!
        
        registry.register_handler(handler1).unwrap();
        let result = registry.register_handler(handler2);
        
        // Should still succeed but detect conflict
        assert!(result.is_err() || registry.get_conflicts().len() > 0);
    }
    
    #[test] 
    fn test_priority_ordering() {
        let mut registry = InputMappingRegistry::new();
        let handlers = handlers::create_standard_fkey_handlers();
        
        for handler in handlers {
            registry.register_handler(handler).unwrap();
        }
        
        // F9 (cheat menu) should have highest priority
        let f9_handler = registry.get_primary_handler(KeyCode::F9).unwrap();
        assert_eq!(f9_handler.get_id(), "cheat_menu");
        assert_eq!(f9_handler.get_priority(), 40);
    }
    
    #[test]
    fn test_macro_handler_creation() {
        create_input_handler!(
            TestHandler,
            "test_handler", 
            "Test handler for F5",
            25,
            InputContext::Game,
            [KeyCode::F5],
            |_world: &mut World, _key: KeyCode| true
        );
        
        let handler = TestHandler;
        assert_eq!(handler.get_id(), "test_handler");
        assert_eq!(handler.get_priority(), 25);
        assert!(handler.handles_key(KeyCode::F5));
        assert!(!handler.handles_key(KeyCode::F6));
    }
}