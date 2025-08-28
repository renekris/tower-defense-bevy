use bevy::prelude::*;
use crate::systems::input::registry::{InputHandler, InputContext};
use crate::systems::debug_visualization::DebugVisualizationState;
use crate::systems::debug_ui::components::DebugUIState;
use crate::systems::unified_grid::{UnifiedGridSystem, GridVisualizationMode};
use crate::systems::debug_ui::cheat_menu::CheatMenuState;

/// F1 Key Handler - Debug Visualization Toggle
/// 
/// Toggles debug visualization mode and switches grid to debug mode
/// This is one of the most commonly used debug features
pub struct F1DebugVisualizationHandler;

impl InputHandler for F1DebugVisualizationHandler {
    fn handle_input(&self, world: &mut World, key: KeyCode) -> bool {
        if key != KeyCode::F1 {
            return false;
        }
        
        // Get required resources
        let debug_state_exists = world.contains_resource::<DebugVisualizationState>();
        let unified_grid_exists = world.contains_resource::<UnifiedGridSystem>();
        
        if !debug_state_exists || !unified_grid_exists {
            warn!("F1 handler: Required resources not found (DebugVisualizationState: {}, UnifiedGridSystem: {})", 
                  debug_state_exists, unified_grid_exists);
            return false;
        }
        
        // Toggle debug visualization state
        world.resource_scope(|world, mut debug_state: Mut<DebugVisualizationState>| {
            debug_state.toggle();
            
            // Also update the unified grid system
            world.resource_scope(|_world, mut unified_grid: Mut<UnifiedGridSystem>| {
                if debug_state.enabled {
                    info!("Debug visualization enabled (F1 to toggle, Ctrl+1-9 for wave selection)");
                    // Switch to debug mode when debug visualization is enabled
                    // unless we're currently in placement mode
                    if unified_grid.mode != GridVisualizationMode::Placement {
                        unified_grid.mode = GridVisualizationMode::Debug;
                    }
                    // Enable all visualization features in debug mode
                    unified_grid.show_path = true;
                    unified_grid.show_zones = true;
                    unified_grid.show_obstacles = true;
                } else {
                    info!("Debug visualization disabled");
                    // Switch back to normal mode when debug visualization is disabled
                    // unless we're currently in placement mode
                    if unified_grid.mode == GridVisualizationMode::Debug {
                        unified_grid.mode = GridVisualizationMode::Normal;
                    }
                }
            });
        });
        
        true // Input consumed
    }
    
    fn get_description(&self) -> &str {
        "Toggle debug visualization and switch to debug grid mode"
    }
    
    fn get_priority(&self) -> u8 {
        30 // High priority for debug features
    }
    
    fn get_id(&self) -> &str {
        "debug_visualization"
    }
    
    fn handles_key(&self, key: KeyCode) -> bool {
        key == KeyCode::F1
    }
    
    fn get_handled_keys(&self) -> Vec<KeyCode> {
        vec![KeyCode::F1]
    }
    
    fn get_context(&self) -> InputContext {
        InputContext::Debug
    }
}

/// F2 Key Handler - Debug UI Panel Toggle
/// 
/// Toggles the debug UI panel visibility for development tools
pub struct F2DebugUIHandler;

impl InputHandler for F2DebugUIHandler {
    fn handle_input(&self, world: &mut World, key: KeyCode) -> bool {
        if key != KeyCode::F2 {
            return false;
        }
        
        // Check if DebugUIState resource exists
        if !world.contains_resource::<DebugUIState>() {
            warn!("F2 handler: DebugUIState resource not found");
            return false;
        }
        
        // Toggle debug UI panel visibility
        world.resource_scope(|_world, mut ui_state: Mut<DebugUIState>| {
            ui_state.panel_visible = !ui_state.panel_visible;
            info!("Debug UI panel: {}", if ui_state.panel_visible { "enabled" } else { "disabled" });
        });
        
        true // Input consumed
    }
    
    fn get_description(&self) -> &str {
        "Toggle debug UI panel visibility"
    }
    
    fn get_priority(&self) -> u8 {
        30 // High priority for debug features
    }
    
    fn get_id(&self) -> &str {
        "debug_ui"
    }
    
    fn handles_key(&self, key: KeyCode) -> bool {
        key == KeyCode::F2
    }
    
    fn get_handled_keys(&self) -> Vec<KeyCode> {
        vec![KeyCode::F2]
    }
    
    fn get_context(&self) -> InputContext {
        InputContext::Debug
    }
}

/// F3 Key Handler - Grid Mode Cycling
/// 
/// Cycles through grid visualization modes: Normal -> Debug -> Placement -> Normal
pub struct F3GridModeHandler;

impl InputHandler for F3GridModeHandler {
    fn handle_input(&self, world: &mut World, key: KeyCode) -> bool {
        if key != KeyCode::F3 {
            return false;
        }
        
        // Check if UnifiedGridSystem resource exists
        if !world.contains_resource::<UnifiedGridSystem>() {
            warn!("F3 handler: UnifiedGridSystem resource not found");
            return false;
        }
        
        // Cycle grid visualization mode
        world.resource_scope(|_world, mut unified_grid: Mut<UnifiedGridSystem>| {
            unified_grid.mode = match unified_grid.mode {
                GridVisualizationMode::Normal => GridVisualizationMode::Debug,
                GridVisualizationMode::Debug => GridVisualizationMode::Placement,
                GridVisualizationMode::Placement => GridVisualizationMode::Normal,
            };
            
            info!("Grid visualization mode changed to: {:?}", unified_grid.mode);
        });
        
        true // Input consumed
    }
    
    fn get_description(&self) -> &str {
        "Cycle grid visualization mode (Normal -> Debug -> Placement)"
    }
    
    fn get_priority(&self) -> u8 {
        20 // Medium priority for grid controls
    }
    
    fn get_id(&self) -> &str {
        "grid_mode"
    }
    
    fn handles_key(&self, key: KeyCode) -> bool {
        key == KeyCode::F3
    }
    
    fn get_handled_keys(&self) -> Vec<KeyCode> {
        vec![KeyCode::F3]
    }
    
    fn get_context(&self) -> InputContext {
        InputContext::Game
    }
}

/// F4 Key Handler - Grid Border Toggle
/// 
/// Toggles grid border visibility completely on/off
pub struct F4GridBorderHandler;

impl InputHandler for F4GridBorderHandler {
    fn handle_input(&self, world: &mut World, key: KeyCode) -> bool {
        if key != KeyCode::F4 {
            return false;
        }
        
        // Check if UnifiedGridSystem resource exists
        if !world.contains_resource::<UnifiedGridSystem>() {
            warn!("F4 handler: UnifiedGridSystem resource not found");
            return false;
        }
        
        // Toggle grid border visibility
        world.resource_scope(|_world, mut unified_grid: Mut<UnifiedGridSystem>| {
            unified_grid.hide_grid_borders = !unified_grid.hide_grid_borders;
            info!("Grid borders visibility toggled: {}", 
                  if unified_grid.hide_grid_borders { "hidden" } else { "visible" });
        });
        
        true // Input consumed
    }
    
    fn get_description(&self) -> &str {
        "Toggle grid border visibility"
    }
    
    fn get_priority(&self) -> u8 {
        20 // Medium priority for grid controls
    }
    
    fn get_id(&self) -> &str {
        "grid_border"
    }
    
    fn handles_key(&self, key: KeyCode) -> bool {
        key == KeyCode::F4
    }
    
    fn get_handled_keys(&self) -> Vec<KeyCode> {
        vec![KeyCode::F4]
    }
    
    fn get_context(&self) -> InputContext {
        InputContext::Game
    }
}

/// F9 Key Handler - Cheat Menu Toggle
/// 
/// Toggles the cheat menu visibility for development and testing
pub struct F9CheatMenuHandler;

impl InputHandler for F9CheatMenuHandler {
    fn handle_input(&self, world: &mut World, key: KeyCode) -> bool {
        if key != KeyCode::F9 {
            return false;
        }
        
        // Check if CheatMenuState resource exists
        if !world.contains_resource::<CheatMenuState>() {
            warn!("F9 handler: CheatMenuState resource not found");
            return false;
        }
        
        // Toggle cheat menu visibility
        world.resource_scope(|_world, mut cheat_state: Mut<CheatMenuState>| {
            cheat_state.visible = !cheat_state.visible;
            info!("Cheat menu: {}", if cheat_state.visible { "enabled" } else { "disabled" });
        });
        
        true // Input consumed
    }
    
    fn get_description(&self) -> &str {
        "Toggle cheat menu visibility"
    }
    
    fn get_priority(&self) -> u8 {
        40 // Higher priority for admin features
    }
    
    fn get_id(&self) -> &str {
        "cheat_menu"
    }
    
    fn handles_key(&self, key: KeyCode) -> bool {
        key == KeyCode::F9
    }
    
    fn get_handled_keys(&self) -> Vec<KeyCode> {
        vec![KeyCode::F9]
    }
    
    fn get_context(&self) -> InputContext {
        InputContext::Admin
    }
}

/// Multi-key handler that demonstrates handling multiple keys in one handler
/// This could be used for F3/F4 combined grid system if desired
pub struct GridSystemHandler;

impl InputHandler for GridSystemHandler {
    fn handle_input(&self, world: &mut World, key: KeyCode) -> bool {
        match key {
            KeyCode::F3 => {
                // Delegate to F3 handler logic
                let f3_handler = F3GridModeHandler;
                f3_handler.handle_input(world, key)
            }
            KeyCode::F4 => {
                // Delegate to F4 handler logic  
                let f4_handler = F4GridBorderHandler;
                f4_handler.handle_input(world, key)
            }
            _ => false,
        }
    }
    
    fn get_description(&self) -> &str {
        "Combined grid mode cycling (F3) and border toggle (F4) handler"
    }
    
    fn get_priority(&self) -> u8 {
        20
    }
    
    fn get_id(&self) -> &str {
        "grid_system_combined"
    }
    
    fn handles_key(&self, key: KeyCode) -> bool {
        matches!(key, KeyCode::F3 | KeyCode::F4)
    }
    
    fn get_handled_keys(&self) -> Vec<KeyCode> {
        vec![KeyCode::F3, KeyCode::F4]
    }
    
    fn get_context(&self) -> InputContext {
        InputContext::Game
    }
}

/// Helper function to create all standard F-key handlers
pub fn create_standard_fkey_handlers() -> Vec<std::sync::Arc<dyn InputHandler>> {
    vec![
        std::sync::Arc::new(F1DebugVisualizationHandler),
        std::sync::Arc::new(F2DebugUIHandler),
        std::sync::Arc::new(F3GridModeHandler),
        std::sync::Arc::new(F4GridBorderHandler),
        std::sync::Arc::new(F9CheatMenuHandler),
    ]
}

/// Helper function to create combined grid handler (alternative approach)
pub fn create_combined_grid_handler() -> std::sync::Arc<dyn InputHandler> {
    std::sync::Arc::new(GridSystemHandler)
}