use bevy::prelude::*;

/// Debug feature flags controlling granular access to development features
/// Each flag can be individually controlled for fine-grained security
#[derive(Resource, Debug, Clone)]
pub struct DebugFeatureFlags {
    /// F1 - Debug visualization (pathfinding, performance metrics)
    pub debug_visualization_enabled: bool,
    
    /// F2 - Debug UI panel (parameter controls, toggles)
    pub debug_ui_enabled: bool,
    
    /// F9 - Cheat menu (economy manipulation, entity spawning)
    pub cheat_menu_enabled: bool,
    
    /// Console debug output (system state changes, logging)
    pub console_output_enabled: bool,
    
    /// Wave selection controls (Ctrl+1-9)
    pub wave_selection_enabled: bool,
    
    /// Spawn rate modification (1-5 keys)
    pub spawn_rate_controls_enabled: bool,
    
    /// Grid visualization controls (F3, F4)
    pub grid_controls_enabled: bool,
}

impl Default for DebugFeatureFlags {
    fn default() -> Self {
        // Secure defaults: All debug features disabled
        Self {
            debug_visualization_enabled: false,
            debug_ui_enabled: false,
            cheat_menu_enabled: false,
            console_output_enabled: false,
            wave_selection_enabled: false,
            spawn_rate_controls_enabled: false,
            grid_controls_enabled: false,
        }
    }
}

impl DebugFeatureFlags {
    /// Enable all development features (for development builds only)
    pub fn enable_development_features(&mut self) {
        self.debug_visualization_enabled = true;
        self.debug_ui_enabled = true;
        self.console_output_enabled = true;
        self.wave_selection_enabled = true;
        self.spawn_rate_controls_enabled = true;
        self.grid_controls_enabled = true;
        // Note: cheat_menu_enabled requires explicit authorization
        
        info!("Development debug features enabled (cheat menu requires admin privileges)");
    }
    
    /// Enable admin features (cheat menu and advanced controls)
    pub fn enable_admin_features(&mut self) {
        self.cheat_menu_enabled = true;
        info!("Admin debug features enabled");
    }
    
    /// Disable all debug features (for production safety)
    pub fn disable_all_debug_features(&mut self) {
        self.debug_visualization_enabled = false;
        self.debug_ui_enabled = false;
        self.cheat_menu_enabled = false;
        self.console_output_enabled = false;
        self.wave_selection_enabled = false;
        self.spawn_rate_controls_enabled = false;
        self.grid_controls_enabled = false;
        
        info!("All debug features disabled for production safety");
    }
    
    /// Check if any debug features are currently enabled
    pub fn has_any_debug_features_enabled(&self) -> bool {
        self.debug_visualization_enabled
            || self.debug_ui_enabled
            || self.cheat_menu_enabled
            || self.wave_selection_enabled
            || self.spawn_rate_controls_enabled
            || self.grid_controls_enabled
    }
    
    /// Get feature status summary for logging
    pub fn get_feature_summary(&self) -> String {
        format!(
            "DebugFeatures: vis={}, ui={}, cheat={}, console={}, wave={}, spawn={}, grid={}",
            self.debug_visualization_enabled,
            self.debug_ui_enabled,
            self.cheat_menu_enabled,
            self.console_output_enabled,
            self.wave_selection_enabled,
            self.spawn_rate_controls_enabled,
            self.grid_controls_enabled
        )
    }
    
    /// Validate feature configuration for security compliance
    pub fn validate_security_compliance(&self) -> Result<(), String> {
        // In release builds, no debug features should be enabled
        #[cfg(not(debug_assertions))]
        {
            if self.has_any_debug_features_enabled() {
                return Err("Debug features enabled in release build - security violation".to_string());
            }
        }
        
        Ok(())
    }
}

/// Compile-time feature flag macros for conditional compilation
#[macro_export]
macro_rules! debug_feature_check {
    ($feature_flags:expr, $feature:ident, $action:expr) => {
        if $feature_flags.$feature {
            $action
        } else {
            warn!("Debug feature '{}' access denied: Feature not enabled", stringify!($feature));
        }
    };
}

/// Secure console output macro that respects feature flags
#[macro_export]
macro_rules! secure_debug_println {
    ($feature_flags:expr, $($arg:tt)*) => {
        if $feature_flags.console_output_enabled {
            println!($($arg)*);
        }
    };
}

/// Secure info logging that respects feature flags
#[macro_export]
macro_rules! secure_debug_info {
    ($feature_flags:expr, $($arg:tt)*) => {
        if $feature_flags.console_output_enabled {
            info!($($arg)*);
        }
    };
}