// Security module for managing debug feature access and authorization
// Provides compile-time and runtime controls for debug functionality

pub mod context;
pub mod features;
pub mod validation;
pub mod admin_toggle;

pub use context::*;
pub use features::*;
pub use validation::*;
pub use admin_toggle::*;

use bevy::prelude::*;

/// Plugin to initialize security systems and resources
pub struct SecurityPlugin;

impl Plugin for SecurityPlugin {
    fn build(&self, app: &mut App) {
        // Initialize security resources with secure defaults
        app.init_resource::<SecurityContext>()
           .init_resource::<DebugFeatureFlags>()
           .add_systems(Startup, (initialize_security_context, initialize_admin_from_settings).chain())
           .add_systems(Update, (
               validate_security_context,
               admin_toggle_system,
               admin_status_display_system,
           ));

        // Log security initialization
        info!("Security system initialized with secure defaults");
        info!("💡 Press ` (backtick) in development builds to toggle admin mode");
        info!("💡 Press F12 to check debug status and available F-keys");
    }
}

/// Initialize security context with appropriate defaults based on build type
fn initialize_security_context(
    mut security_context: ResMut<SecurityContext>,
    mut feature_flags: ResMut<DebugFeatureFlags>,
) {
    // Configure based on build type and environment
    #[cfg(debug_assertions)]
    {
        security_context.development_build = true;
        security_context.debug_mode_authorized = true;
        feature_flags.enable_development_features();
        info!("Development build detected: Debug features enabled with authorization checks");
    }
    
    #[cfg(not(debug_assertions))]
    {
        security_context.development_build = false;
        security_context.debug_mode_authorized = false;
        feature_flags.disable_all_debug_features();
        info!("Release build detected: All debug features disabled for production safety");
    }
}

/// Periodic validation of security context integrity
fn validate_security_context(
    security_context: Res<SecurityContext>,
    feature_flags: Res<DebugFeatureFlags>,
) {
    // Only validate every 60 frames to avoid performance impact
    static mut VALIDATION_COUNTER: u32 = 0;
    unsafe {
        VALIDATION_COUNTER += 1;
        if VALIDATION_COUNTER % 60 != 0 {
            return;
        }
    }

    // Validate security context consistency
    if !security_context.development_build {
        // In production builds, ensure no debug features are enabled
        if feature_flags.has_any_debug_features_enabled() {
            error!("SECURITY VIOLATION: Debug features enabled in production build!");
        }
    }
}