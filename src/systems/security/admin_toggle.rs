use bevy::prelude::*;
use crate::systems::settings_menu::GameSettings;
use super::{SecurityContext, DebugFeatureFlags};

/// System to handle backtick (`) key toggle for admin privileges
/// Toggles admin mode and persists the setting to settings.json
pub fn admin_toggle_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut security_context: ResMut<SecurityContext>,
    mut feature_flags: ResMut<DebugFeatureFlags>,
    mut settings: Option<ResMut<GameSettings>>,
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
            
            // Save to settings if available
            if let Some(ref mut settings) = settings {
                settings.debug_admin_enabled = true;
                info!("💾 Admin preference saved to settings.json");
            } else {
                info!("⚠️ Settings not available - admin state not persisted");
            }
            
            info!("🔑 Admin privileges ENABLED - Full F-key access granted (` to toggle)");
            info!("   F1: Debug Visualization | F2: Debug UI | F3: Grid Mode | F4: Grid Borders");
            info!("   F9: Cheat Menu | 1-5: Spawn Rate | Ctrl+1-9: Wave Selection");
        } else {
            // Revoke admin privileges but keep debug access
            security_context.admin_privileges = false;
            feature_flags.cheat_menu_enabled = false;
            
            // Save to settings if available
            if let Some(ref mut settings) = settings {
                settings.debug_admin_enabled = false;
                info!("💾 Admin preference saved to settings.json");
            } else {
                info!("⚠️ Settings not available - admin state not persisted");
            }
            
            info!("🔒 Admin privileges DISABLED - Cheat menu (F9) locked (` to toggle)");
            info!("   F1-F4 and debug features still available");
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
    
    // Check if settings are available yet (they might not be loaded during early startup)
    if let Some(settings) = settings {
        if settings.debug_admin_enabled {
            security_context.authorize_debug_access();
            security_context.authorize_admin_privileges();
            feature_flags.enable_admin_features();
            
            info!("🔑 Admin privileges restored from settings - Full F-key access available");
            info!("   Press ` (backtick) to toggle admin mode");
        } else {
            info!("🔒 Admin mode disabled - Press ` (backtick) to enable full F-key access");
        }
    } else {
        // Settings not available yet - just show admin toggle instructions
        info!("🔒 Settings loading... Press ` (backtick) to toggle admin mode when ready");
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
    
    // Check if settings are available and load admin preferences
    if let Some(settings) = settings {
        if settings.debug_admin_enabled && !security_context.admin_privileges {
            security_context.authorize_debug_access();
            security_context.authorize_admin_privileges();
            feature_flags.enable_admin_features();
            
            info!("🔑 Deferred admin privileges loaded from settings - Full F-key access available");
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