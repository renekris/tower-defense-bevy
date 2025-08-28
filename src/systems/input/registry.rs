use bevy::prelude::*;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

/// Core trait for all input handlers in the system
/// 
/// This trait provides a standardized interface for processing input events
/// with priority-based conflict resolution and context awareness.
pub trait InputHandler: Send + Sync + 'static {
    /// Handle the input event and return whether it was consumed
    /// 
    /// # Arguments
    /// * `world` - Mutable reference to the Bevy world for system access
    /// * `key` - The key code that was pressed
    /// 
    /// # Returns
    /// * `true` if the input was consumed and should not be passed to other handlers
    /// * `false` if the input was not consumed and can be passed to other handlers
    fn handle_input(&self, world: &mut World, key: KeyCode) -> bool;
    
    /// Get a human-readable description of what this handler does
    fn get_description(&self) -> &str;
    
    /// Get the priority level of this handler (higher = more priority)
    /// 
    /// Priority levels:
    /// * 100+ = Critical system handlers (pause menu, emergency exits)
    /// * 50-99 = UI and menu handlers (debug panels, cheat menus)
    /// * 10-49 = Game state handlers (debug visualization, grid modes)
    /// * 1-9 = Low priority handlers (convenience shortcuts)
    /// * 0 = Default priority
    fn get_priority(&self) -> u8 { 50 }
    
    /// Get the unique identifier for this handler (used for conflict detection)
    fn get_id(&self) -> &str;
    
    /// Check if this handler can process the given key code
    fn handles_key(&self, key: KeyCode) -> bool;
    
    /// Get all key codes this handler is responsible for
    fn get_handled_keys(&self) -> Vec<KeyCode>;
    
    /// Get the context category for this handler (used for organization)
    fn get_context(&self) -> InputContext { InputContext::Game }
}

/// Input context categories for organizational purposes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputContext {
    /// Core game functionality (movement, actions)
    Game,
    /// Debug and development tools
    Debug,
    /// User interface and menus
    UI,
    /// Administrative and cheat functions
    Admin,
    /// System-level functions (pause, exit)
    System,
}

/// Information about an input conflict
#[derive(Debug, Clone)]
pub struct InputConflict {
    pub key: KeyCode,
    pub handler1: String,
    pub handler2: String,
    pub priority1: u8,
    pub priority2: u8,
    pub context1: InputContext,
    pub context2: InputContext,
}

/// Registry for managing all input mappings and handlers
#[derive(Resource)]
pub struct InputMappingRegistry {
    /// Map of key codes to their registered handlers (sorted by priority)
    bindings: HashMap<KeyCode, Vec<Arc<dyn InputHandler>>>,
    /// Set of all registered handler IDs for conflict detection
    registered_handlers: HashSet<String>,
    /// List of detected conflicts
    conflicts: Vec<InputConflict>,
    /// Whether to log input events for debugging
    debug_logging: bool,
}

impl Default for InputMappingRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl InputMappingRegistry {
    /// Create a new empty input mapping registry
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
            registered_handlers: HashSet::new(),
            conflicts: Vec::new(),
            debug_logging: false,
        }
    }
    
    /// Enable or disable debug logging for input events
    pub fn set_debug_logging(&mut self, enabled: bool) {
        self.debug_logging = enabled;
        if enabled {
            info!("Input debug logging enabled");
        }
    }
    
    /// Register a new input handler
    /// 
    /// # Arguments
    /// * `handler` - The input handler to register
    /// 
    /// # Returns
    /// * `Ok(())` if registration succeeded
    /// * `Err(String)` if there was a conflict or error
    pub fn register_handler(&mut self, handler: Arc<dyn InputHandler>) -> Result<(), String> {
        let handler_id = handler.get_id();
        
        // Check for duplicate handler IDs
        if self.registered_handlers.contains(handler_id) {
            return Err(format!("Handler with ID '{}' is already registered", handler_id));
        }
        
        // Register the handler for each of its keys
        let handled_keys = handler.get_handled_keys();
        for key in handled_keys {
            self.register_key_handler(key, handler.clone())?;
        }
        
        // Add to registered handlers set
        self.registered_handlers.insert(handler_id.to_string());
        
        info!("Registered input handler '{}' for keys: {:?}", 
              handler_id, handler.get_handled_keys());
        
        Ok(())
    }
    
    /// Register a handler for a specific key (internal method)
    fn register_key_handler(&mut self, key: KeyCode, handler: Arc<dyn InputHandler>) -> Result<(), String> {
        // Check for conflicts with existing handlers
        if let Some(existing_handlers) = self.bindings.get(&key) {
            for existing_handler in existing_handlers {
                let conflict = InputConflict {
                    key,
                    handler1: existing_handler.get_id().to_string(),
                    handler2: handler.get_id().to_string(),
                    priority1: existing_handler.get_priority(),
                    priority2: handler.get_priority(),
                    context1: existing_handler.get_context(),
                    context2: handler.get_context(),
                };
                
                // Log the conflict
                warn!("Input conflict detected for key {:?}: '{}' (priority {}) vs '{}' (priority {})",
                      key, conflict.handler1, conflict.priority1, conflict.handler2, conflict.priority2);
                
                self.conflicts.push(conflict);
                
                // Allow registration but warn about conflict
                // Higher priority handlers will be processed first
            }
        }
        
        // Add the handler to the bindings
        let handlers = self.bindings.entry(key).or_insert_with(Vec::new);
        handlers.push(handler);
        
        // Sort handlers by priority (highest first)
        handlers.sort_by(|a, b| b.get_priority().cmp(&a.get_priority()));
        
        Ok(())
    }
    
    /// Process input for a specific key
    /// 
    /// Returns true if any handler consumed the input
    pub fn process_input(&self, world: &mut World, key: KeyCode) -> bool {
        if self.debug_logging {
            debug!("Processing input for key: {:?}", key);
        }
        
        if let Some(handlers) = self.bindings.get(&key) {
            for handler in handlers {
                if handler.handles_key(key) {
                    if self.debug_logging {
                        debug!("Attempting to handle key {:?} with handler '{}'", key, handler.get_id());
                    }
                    
                    if handler.handle_input(world, key) {
                        if self.debug_logging {
                            debug!("Key {:?} consumed by handler '{}'", key, handler.get_id());
                        }
                        return true; // Input was consumed
                    }
                }
            }
        }
        
        if self.debug_logging {
            debug!("Key {:?} not handled by any registered handler", key);
        }
        
        false // Input was not consumed
    }
    
    /// Get the highest priority handler for a specific key
    pub fn get_primary_handler(&self, key: KeyCode) -> Option<&Arc<dyn InputHandler>> {
        self.bindings.get(&key)?.first()
    }
    
    /// Get all handlers for a specific key (ordered by priority)
    pub fn get_handlers_for_key(&self, key: KeyCode) -> Option<&Vec<Arc<dyn InputHandler>>> {
        self.bindings.get(&key)
    }
    
    /// Get all registered conflicts
    pub fn get_conflicts(&self) -> &[InputConflict] {
        &self.conflicts
    }
    
    /// Get a summary of all registered bindings
    pub fn get_binding_summary(&self) -> HashMap<KeyCode, Vec<String>> {
        self.bindings
            .iter()
            .map(|(key, handlers)| {
                let handler_info: Vec<String> = handlers
                    .iter()
                    .map(|h| format!("{} (p:{})", h.get_id(), h.get_priority()))
                    .collect();
                (*key, handler_info)
            })
            .collect()
    }
    
    /// Remove a handler by ID
    pub fn unregister_handler(&mut self, handler_id: &str) -> Result<(), String> {
        if !self.registered_handlers.contains(handler_id) {
            return Err(format!("Handler '{}' is not registered", handler_id));
        }
        
        // Remove from all key bindings
        for handlers in self.bindings.values_mut() {
            handlers.retain(|h| h.get_id() != handler_id);
        }
        
        // Remove empty bindings
        self.bindings.retain(|_, handlers| !handlers.is_empty());
        
        // Remove from registered set
        self.registered_handlers.remove(handler_id);
        
        // Remove related conflicts
        self.conflicts.retain(|c| c.handler1 != handler_id && c.handler2 != handler_id);
        
        info!("Unregistered input handler '{}'", handler_id);
        Ok(())
    }
    
    /// Clear all handlers and conflicts
    pub fn clear(&mut self) {
        self.bindings.clear();
        self.registered_handlers.clear();
        self.conflicts.clear();
        info!("Cleared all input handlers");
    }
    
    /// Get statistics about the registry
    pub fn get_stats(&self) -> InputRegistryStats {
        InputRegistryStats {
            total_handlers: self.registered_handlers.len(),
            total_keys: self.bindings.len(),
            total_conflicts: self.conflicts.len(),
            handlers_by_context: self.get_handlers_by_context(),
        }
    }
    
    /// Group handlers by context for analysis
    fn get_handlers_by_context(&self) -> HashMap<InputContext, usize> {
        let mut context_counts = HashMap::new();
        
        for handlers in self.bindings.values() {
            for handler in handlers {
                let context = handler.get_context();
                *context_counts.entry(context).or_insert(0) += 1;
            }
        }
        
        context_counts
    }
}

/// Statistics about the input registry
#[derive(Debug)]
pub struct InputRegistryStats {
    pub total_handlers: usize,
    pub total_keys: usize,
    pub total_conflicts: usize,
    pub handlers_by_context: HashMap<InputContext, usize>,
}

/// System to process keyboard input through the centralized registry
pub fn process_centralized_input(
    world: &mut World,
) {
    // Get keyboard input resource
    let keyboard_input = world.get_resource::<ButtonInput<KeyCode>>();
    if keyboard_input.is_none() {
        return;
    }
    
    // Clone the pressed keys to avoid borrowing issues
    let pressed_keys: Vec<KeyCode> = keyboard_input
        .unwrap()
        .get_just_pressed()
        .cloned()
        .collect();
    
    // Process each pressed key through the registry
    for key in pressed_keys {
        // Temporarily remove the registry to avoid borrowing conflicts
        if let Some(mut registry) = world.remove_resource::<InputMappingRegistry>() {
            let consumed = registry.process_input(world, key);
            
            // Re-insert the registry
            world.insert_resource(registry);
            
            if consumed {
                // Input was handled, no need to process further for this key
                continue;
            }
        }
    }
}

