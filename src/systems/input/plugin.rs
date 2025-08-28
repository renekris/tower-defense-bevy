use bevy::prelude::*;
use std::sync::Arc;

use crate::systems::input::registry::{InputMappingRegistry, InputHandler, process_centralized_input};
use crate::systems::input::handlers::{create_standard_fkey_handlers, create_combined_grid_handler};

/// Plugin for managing centralized input mapping
/// 
/// This plugin provides:
/// * Centralized input handler registration
/// * Conflict detection and resolution
/// * Priority-based input processing
/// * Debug logging and monitoring
pub struct InputRegistryPlugin {
    /// Whether to enable debug logging for input events
    pub debug_logging: bool,
    /// Whether to use individual F3/F4 handlers or combined grid handler
    pub use_combined_grid_handler: bool,
    /// Whether to automatically register standard F-key handlers
    pub auto_register_fkeys: bool,
}

impl Default for InputRegistryPlugin {
    fn default() -> Self {
        Self {
            debug_logging: false,
            use_combined_grid_handler: false,
            auto_register_fkeys: true,
        }
    }
}

impl Plugin for InputRegistryPlugin {
    fn build(&self, app: &mut App) {
        // Add the input mapping registry resource
        let mut registry = InputMappingRegistry::new();
        registry.set_debug_logging(self.debug_logging);
        app.insert_resource(registry);
        
        // Add the centralized input processing system
        // This system runs early in the input processing pipeline
        app.add_systems(PreUpdate, process_centralized_input);
        
        // Add systems for monitoring and debugging
        app.add_systems(PostUpdate, (
            log_input_conflicts_system,
            log_registry_stats_system,
        ));
        
        // Add events for registry monitoring
        app.add_event::<InputRegistryUpdateEvent>();
        app.add_event::<InputRegistryStatsEvent>();
        
        // Register standard F-key handlers if requested
        let use_combined = self.use_combined_grid_handler;
        if self.auto_register_fkeys {
            app.add_systems(Startup, move |mut registry: ResMut<InputMappingRegistry>| {
                register_standard_handlers(&mut registry, use_combined);
            });
        }
        
        // Add development systems in debug mode
        #[cfg(debug_assertions)]
        {
            app.add_systems(Update, (
                debug_input_monitoring_system,
                input_registry_commands_system,
            ));
        }
    }
}

/// Event sent when the input registry is updated
#[derive(Event)]
pub struct InputRegistryUpdateEvent {
    pub handler_id: String,
    pub action: RegistryAction,
}

/// Event sent to request registry statistics logging
#[derive(Event)]
pub struct InputRegistryStatsEvent;

/// Type of registry action that occurred
#[derive(Debug, Clone)]
pub enum RegistryAction {
    HandlerRegistered,
    HandlerUnregistered,
    ConflictDetected,
}

/// System to register all standard F-key handlers
fn register_standard_handlers(registry: &mut InputMappingRegistry, use_combined_grid: bool) {
    info!("Registering standard F-key handlers...");
    
    if use_combined_grid {
        // Use combined F3/F4 handler
        let combined_handler = create_combined_grid_handler();
        if let Err(e) = registry.register_handler(combined_handler) {
            error!("Failed to register combined grid handler: {}", e);
        }
        
        // Register other individual handlers
        let handlers = vec![
            create_standard_fkey_handlers()[0].clone(), // F1
            create_standard_fkey_handlers()[1].clone(), // F2
            create_standard_fkey_handlers()[4].clone(), // F9
        ];
        
        for handler in handlers {
            if let Err(e) = registry.register_handler(handler.clone()) {
                error!("Failed to register handler '{}': {}", handler.get_id(), e);
            }
        }
    } else {
        // Use individual handlers for all F-keys
        let handlers = create_standard_fkey_handlers();
        for handler in handlers {
            if let Err(e) = registry.register_handler(handler.clone()) {
                error!("Failed to register handler '{}': {}", handler.get_id(), e);
            }
        }
    }
    
    // Log final registration status
    let stats = registry.get_stats();
    info!("Input registry initialized with {} handlers across {} keys", 
          stats.total_handlers, stats.total_keys);
    
    if stats.total_conflicts > 0 {
        warn!("Detected {} input conflicts - check logs for details", stats.total_conflicts);
    }
}

/// System to log input conflicts when they occur
fn log_input_conflicts_system(
    registry: Res<InputMappingRegistry>,
    mut events: EventReader<InputRegistryUpdateEvent>,
) {
    for event in events.read() {
        if matches!(event.action, RegistryAction::ConflictDetected) {
            let conflicts = registry.get_conflicts();
            for conflict in conflicts.iter().rev().take(1) { // Log most recent conflict
                warn!(
                    "INPUT CONFLICT: Key {:?} claimed by '{}' (priority {}) and '{}' (priority {})",
                    conflict.key,
                    conflict.handler1, conflict.priority1,
                    conflict.handler2, conflict.priority2
                );
                
                if conflict.priority1 == conflict.priority2 {
                    warn!("  -> Both handlers have equal priority! Consider adjusting priorities.");
                } else {
                    info!("  -> Handler '{}' will take precedence due to higher priority.", 
                          if conflict.priority1 > conflict.priority2 { &conflict.handler1 } else { &conflict.handler2 });
                }
            }
        }
    }
}

/// System to log registry statistics on demand
fn log_registry_stats_system(
    registry: Res<InputMappingRegistry>,
    mut events: EventReader<InputRegistryStatsEvent>,
) {
    for _event in events.read() {
        let stats = registry.get_stats();
        let bindings = registry.get_binding_summary();
        
        info!("=== INPUT REGISTRY STATISTICS ===");
        info!("Total Handlers: {}", stats.total_handlers);
        info!("Total Keys: {}", stats.total_keys);
        info!("Total Conflicts: {}", stats.total_conflicts);
        
        info!("Handlers by Context:");
        for (context, count) in stats.handlers_by_context {
            info!("  {:?}: {}", context, count);
        }
        
        info!("Key Bindings:");
        for (key, handler_info) in bindings {
            info!("  {:?}: {}", key, handler_info.join(", "));
        }
        
        if stats.total_conflicts > 0 {
            warn!("Registry has {} unresolved conflicts", stats.total_conflicts);
        }
    }
}

/// Development system for monitoring input in debug mode
#[cfg(debug_assertions)]
fn debug_input_monitoring_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    registry: Res<InputMappingRegistry>,
) {
    // Monitor F-keys specifically
    for key in [KeyCode::F1, KeyCode::F2, KeyCode::F3, KeyCode::F4, KeyCode::F9] {
        if keyboard_input.just_pressed(key) {
            if let Some(handler) = registry.get_primary_handler(key) {
                debug!("F-key {:?} pressed -> Primary handler: '{}'", key, handler.get_id());
            } else {
                debug!("F-key {:?} pressed -> No handler registered", key);
            }
        }
    }
}

/// Development system for runtime registry commands
#[cfg(debug_assertions)]
fn input_registry_commands_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut stats_events: EventWriter<InputRegistryStatsEvent>,
) {
    // Ctrl+F12 - Show registry statistics
    if keyboard_input.just_pressed(KeyCode::F12) && 
       (keyboard_input.pressed(KeyCode::ControlLeft) || keyboard_input.pressed(KeyCode::ControlRight)) {
        stats_events.write(InputRegistryStatsEvent);
    }
}

/// Builder for creating custom InputRegistryPlugin configurations
pub struct InputRegistryPluginBuilder {
    debug_logging: bool,
    use_combined_grid_handler: bool,
    auto_register_fkeys: bool,
}

impl InputRegistryPluginBuilder {
    /// Create a new builder with default settings
    pub fn new() -> Self {
        Self {
            debug_logging: false,
            use_combined_grid_handler: false,
            auto_register_fkeys: true,
        }
    }
    
    /// Enable debug logging for input events
    pub fn with_debug_logging(mut self) -> Self {
        self.debug_logging = true;
        self
    }
    
    /// Use combined F3/F4 handler instead of separate handlers
    pub fn with_combined_grid_handler(mut self) -> Self {
        self.use_combined_grid_handler = true;
        self
    }
    
    /// Disable automatic registration of standard F-key handlers
    pub fn without_auto_registration(mut self) -> Self {
        self.auto_register_fkeys = false;
        self
    }
    
    /// Build the plugin with current configuration
    pub fn build(self) -> InputRegistryPlugin {
        InputRegistryPlugin {
            debug_logging: self.debug_logging,
            use_combined_grid_handler: self.use_combined_grid_handler,
            auto_register_fkeys: self.auto_register_fkeys,
        }
    }
}

impl Default for InputRegistryPluginBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Extension trait for App to easily add input handlers
pub trait InputRegistryAppExt {
    /// Register a custom input handler
    fn register_input_handler<T: InputHandler>(&mut self, handler: T) -> &mut Self;
    
    /// Register multiple input handlers
    fn register_input_handlers(&mut self, handlers: Vec<Arc<dyn InputHandler>>) -> &mut Self;
}

impl InputRegistryAppExt for App {
    fn register_input_handler<T: InputHandler>(&mut self, handler: T) -> &mut Self {
        let handler_arc = Arc::new(handler);
        self.add_systems(Startup, move |mut registry: ResMut<InputMappingRegistry>| {
            if let Err(e) = registry.register_handler(handler_arc.clone()) {
                error!("Failed to register input handler '{}': {}", handler_arc.get_id(), e);
            }
        });
        self
    }
    
    fn register_input_handlers(&mut self, handlers: Vec<Arc<dyn InputHandler>>) -> &mut Self {
        self.add_systems(Startup, move |mut registry: ResMut<InputMappingRegistry>| {
            for handler in &handlers {
                if let Err(e) = registry.register_handler(handler.clone()) {
                    error!("Failed to register input handler '{}': {}", handler.get_id(), e);
                }
            }
        });
        self
    }
}