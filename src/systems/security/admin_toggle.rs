use bevy::prelude::*;
use crate::systems::settings_menu::GameSettings;
use super::{SecurityContext, DebugFeatureFlags};

/// Event for admin privilege changes
#[derive(Event)]
pub struct AdminToggleEvent {
    pub enabled: bool,
}

/// System to handle backtick (`) key toggle for admin privileges
/// Toggles admin mode and sends event for settings persistence
pub fn admin_toggle_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut security_context: ResMut<SecurityContext>,
    mut feature_flags: ResMut<DebugFeatureFlags>,
    mut admin_events: EventWriter<AdminToggleEvent>,
) {
    // Only process in development builds for security
    if !security_context.development_build {
        return;
    }

    if keyboard_input.just_pressed(KeyCode::Backquote) {
        // Toggle admin privileges
        let new_admin_state = !security_context.admin_privileges;
        
        if new_admin_state {
            // Grant admin privileges
            security_context.authorize_debug_access(); // Ensure debug access first
            security_context.authorize_admin_privileges();
            feature_flags.enable_admin_features();
            
            info!("🔑 Admin privileges ENABLED - Full F-key access granted (` to toggle)");
            info!("   F1: Debug Visualization | F2: Debug UI | F3: Grid Mode | F4: Grid Borders");
            info!("   F9: Cheat Menu | 1-5: Spawn Rate | Ctrl+1-9: Wave Selection");
        } else {
            // Revoke admin privileges but keep debug access
            security_context.admin_privileges = false;
            feature_flags.cheat_menu_enabled = false;
            
            info!("🔒 Admin privileges DISABLED - Cheat menu (F9) locked (` to toggle)");
            info!("   F1-F4 and debug features still available");
        }
        
        // Send event to update settings - no direct resource access conflict
        admin_events.write(AdminToggleEvent {
            enabled: new_admin_state,
        });
        info!("💾 Admin preference queued for auto-save");
    }
}

/// System to handle admin settings persistence
pub fn admin_settings_persistence_system(
    mut admin_events: EventReader<AdminToggleEvent>,
    mut settings: Option<ResMut<GameSettings>>,
) {
    for event in admin_events.read() {
        if let Some(ref mut settings) = settings {
            settings.debug_admin_enabled = event.enabled;
            info!("💾 Admin preference persisted via event system");
        } else {
            info!("⚠️ Settings temporarily unavailable for event persistence");
        }
    }
}

/// System to initialize admin privileges from settings on startup
pub fn initialize_admin_from_settings(
    mut security_context: ResMut<SecurityContext>,
    mut feature_flags: ResMut<DebugFeatureFlags>,
    settings: Option<Res<GameSettings>>,
) {
    // Only in development builds
    if !security_context.development_build {
        return;
    }
    
    // Try to load settings, but gracefully handle if they're not available
    match settings {
        Some(ref settings) => {
            if settings.debug_admin_enabled {
                security_context.authorize_debug_access();
                security_context.authorize_admin_privileges();
                feature_flags.enable_admin_features();
                
                info!("🔑 Admin privileges restored from settings - Full F-key access available");
                info!("   Press ` (backtick) to toggle admin mode");
            } else {
                info!("🔒 Admin mode disabled - Press ` (backtick) to enable full F-key access");
            }
        }
        None => {
            // Settings not available yet - use safe defaults and show admin toggle instructions
            info!("🔒 Settings not yet loaded - Using defaults. Press ` (backtick) to enable admin mode");
        }
    }
}

/// Deferred system to load admin settings once GameSettings is available
/// This runs after the main initialization to properly load admin preferences
pub fn deferred_admin_settings_load(
    mut security_context: ResMut<SecurityContext>,
    mut feature_flags: ResMut<DebugFeatureFlags>, 
    settings: Option<Res<GameSettings>>,
) {
    // Only in development builds
    if !security_context.development_build {
        return;
    }
    
    // Only run if we haven't already loaded admin settings
    static mut ADMIN_SETTINGS_LOADED: bool = false;
    unsafe {
        if ADMIN_SETTINGS_LOADED {
            return;
        }
        ADMIN_SETTINGS_LOADED = true;
    }
    
    // Try to load admin preferences if settings are available
    match settings {
        Some(ref settings) => {
            if settings.debug_admin_enabled && !security_context.admin_privileges {
                security_context.authorize_debug_access();
                security_context.authorize_admin_privileges();
                feature_flags.enable_admin_features();
                
                info!("🔑 Deferred admin privileges loaded from settings - Full F-key access available");
            }
        }
        None => {
            // Settings still not available - this is OK, admin toggle will still work
            debug!("Deferred admin settings load: GameSettings not yet available");
        }
    }
}

/// System to show admin status information
pub fn admin_status_display_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    security_context: Res<SecurityContext>,
    feature_flags: Res<DebugFeatureFlags>,
) {
    // Show status on F12 key press (alongside existing Ctrl+F12 registry inspector)
    if keyboard_input.just_pressed(KeyCode::F12) && 
       !keyboard_input.pressed(KeyCode::ControlLeft) && 
       !keyboard_input.pressed(KeyCode::ControlRight) {
        
        if security_context.development_build {
            info!("🔍 DEBUG STATUS:");
            info!("   Development Build: ✅");
            info!("   Debug Authorized: {}", if security_context.debug_mode_authorized { "✅" } else { "❌" });
            info!("   Admin Privileges: {}", if security_context.admin_privileges { "✅" } else { "❌" });
            info!("   Session Valid: {}", if !security_context.is_session_expired() { "✅" } else { "❌" });
            
            info!("🎮 AVAILABLE F-KEYS:");
            info!("   F1 (Debug Viz): {}", if feature_flags.debug_visualization_enabled { "✅" } else { "❌" });
            info!("   F2 (Debug UI): {}", if feature_flags.debug_ui_enabled { "✅" } else { "❌" });
            info!("   F3 (Grid Mode): ✅ (Always available)");
            info!("   F4 (Grid Borders): ✅ (Always available)");
            info!("   F9 (Cheat Menu): {}", if feature_flags.cheat_menu_enabled { "✅" } else { "❌" });
            
            info!("💡 TIPS:");
            info!("   Press ` (backtick) to toggle admin mode");
            info!("   Press Ctrl+F12 for input registry inspector");
        } else {
            info!("ℹ️  Production build - Debug features disabled for security");
        }
    }
}