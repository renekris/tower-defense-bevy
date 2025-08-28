use bevy::prelude::*;
use crate::systems::settings_menu::GameSettings;

/// Simple debug feature toggle resource
/// Replaces the complex security system with a straightforward on/off switch
#[derive(Resource, Debug, Clone)]
pub struct DebugToggle {
    /// Whether debug features (F2, F9) are enabled
    pub enabled: bool,
}

impl Default for DebugToggle {
    fn default() -> Self {
        Self { enabled: false }
    }
}

impl DebugToggle {
    /// Create a new debug toggle in the disabled state
    pub fn new() -> Self {
        Self { enabled: false }
    }
    
    /// Enable debug features
    pub fn enable(&mut self) {
        self.enabled = true;
    }
    
    /// Disable debug features
    pub fn disable(&mut self) {
        self.enabled = false;
    }
    
    /// Toggle debug features on/off
    pub fn toggle(&mut self) {
        self.enabled = !self.enabled;
    }
    
    /// Check if debug features are enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

/// Event for debug toggle changes
#[derive(Event)]
pub struct DebugToggleEvent {
    pub enabled: bool,
}

/// System to handle backtick (`) key toggle for debug features
/// Toggles F2 and F9 access and sends event for settings persistence
pub fn debug_toggle_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut debug_toggle: ResMut<DebugToggle>,
    mut toggle_events: EventWriter<DebugToggleEvent>,
) {
    if keyboard_input.just_pressed(KeyCode::Backquote) {
        // Toggle debug features
        debug_toggle.toggle();
        
        if debug_toggle.is_enabled() {
            info!("🔓 Debug features ENABLED - F2: Debug UI | F9: Cheat Menu (` to toggle)");
            info!("   F1, F3, F4 are always available");
        } else {
            info!("🔒 Debug features DISABLED - F2 and F9 locked (` to toggle)");
            info!("   F1, F3, F4 remain available");
        }
        
        // Send event to update settings
        toggle_events.write(DebugToggleEvent {
            enabled: debug_toggle.is_enabled(),
        });
        info!("💾 Debug preference queued for auto-save");
    }
}

/// System to handle debug settings persistence
pub fn debug_settings_persistence_system(
    mut toggle_events: EventReader<DebugToggleEvent>,
    mut settings: Option<ResMut<GameSettings>>,
) {
    for event in toggle_events.read() {
        if let Some(ref mut settings) = settings {
            settings.debug_admin_enabled = event.enabled;
            info!("💾 Debug preference persisted via event system");
        } else {
            info!("⚠️ Settings temporarily unavailable for event persistence");
        }
    }
}

/// System to initialize debug toggle from settings on startup
pub fn initialize_debug_from_settings(
    mut debug_toggle: ResMut<DebugToggle>,
    settings: Option<Res<GameSettings>>,
) {
    // Try to load settings, but gracefully handle if they're not available
    match settings {
        Some(ref settings) => {
            debug_toggle.enabled = settings.debug_admin_enabled;
            
            if debug_toggle.is_enabled() {
                info!("🔓 Debug features restored from settings - F2 and F9 available");
                info!("   Press ` (backtick) to toggle debug mode");
            } else {
                info!("🔒 Debug features disabled - Press ` (backtick) to enable F2 and F9");
            }
        }
        None => {
            // Settings not available yet - use safe defaults
            debug_toggle.enabled = false;
            info!("🔒 Settings not yet loaded - Using defaults. Press ` (backtick) to enable debug features");
        }
    }
}

/// System to show debug status information
pub fn debug_status_display_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    debug_toggle: Res<DebugToggle>,
) {
    // Show status on F12 key press
    if keyboard_input.just_pressed(KeyCode::F12) && 
       !keyboard_input.pressed(KeyCode::ControlLeft) && 
       !keyboard_input.pressed(KeyCode::ControlRight) {
        
        info!("🔍 DEBUG STATUS:");
        info!("   Debug Features: {}", if debug_toggle.is_enabled() { "✅ ENABLED" } else { "❌ DISABLED" });
        
        info!("🎮 AVAILABLE F-KEYS:");
        info!("   F1 (Debug Viz): ✅ (Always available)");
        info!("   F2 (Debug UI): {}", if debug_toggle.is_enabled() { "✅" } else { "❌" });
        info!("   F3 (Grid Mode): ✅ (Always available)");
        info!("   F4 (Grid Borders): ✅ (Always available)");
        info!("   F9 (Cheat Menu): {}", if debug_toggle.is_enabled() { "✅" } else { "❌" });
        
        info!("💡 TIPS:");
        info!("   Press ` (backtick) to toggle debug features");
        info!("   Press Ctrl+F12 for input registry inspector");
    }
}

/// Plugin to initialize simple debug toggle system
pub struct DebugTogglePlugin;

impl Plugin for DebugTogglePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DebugToggle>()
           .add_event::<DebugToggleEvent>()
           .add_systems(Startup, initialize_debug_from_settings)
           .add_systems(Update, (
               debug_toggle_system,
               debug_status_display_system,
           ))
           .add_systems(Update, debug_settings_persistence_system.in_set(crate::resources::GameSystemSet::Settings));

        info!("Simple debug toggle system initialized");
        info!("💡 Press ` (backtick) to toggle F2 and F9 access");
        info!("💡 Press F12 to check debug status");
    }
}