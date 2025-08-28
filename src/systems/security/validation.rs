use bevy::prelude::*;
use super::{SecurityContext, DebugFeatureFlags};

/// Authorization validator for debug feature access
/// Provides centralized security checks for all debug functionality
pub struct DebugAuthorization;

impl DebugAuthorization {
    /// Validate F1 (debug visualization) access
    pub fn validate_debug_visualization_access(
        security_context: &SecurityContext,
        feature_flags: &DebugFeatureFlags,
    ) -> bool {
        if !feature_flags.debug_visualization_enabled {
            warn!("F1 debug visualization denied: Feature disabled");
            return false;
        }
        
        if !security_context.has_debug_visualization_permission() {
            warn!("F1 debug visualization denied: Insufficient permissions");
            return false;
        }
        
        true
    }
    
    /// Validate F2 (debug UI) access
    pub fn validate_debug_ui_access(
        security_context: &SecurityContext,
        feature_flags: &DebugFeatureFlags,
    ) -> bool {
        if !feature_flags.debug_ui_enabled {
            warn!("F2 debug UI denied: Feature disabled");
            return false;
        }
        
        if !security_context.has_debug_ui_permission() {
            warn!("F2 debug UI denied: Insufficient permissions");
            return false;
        }
        
        true
    }
    
    /// Validate F9 (cheat menu) access
    pub fn validate_cheat_menu_access(
        security_context: &SecurityContext,
        feature_flags: &DebugFeatureFlags,
    ) -> bool {
        if !feature_flags.cheat_menu_enabled {
            warn!("F9 cheat menu denied: Feature disabled");
            return false;
        }
        
        if !security_context.has_cheat_permission() {
            warn!("F9 cheat menu denied: Admin privileges required");
            return false;
        }
        
        true
    }
    
    /// Validate grid controls (F3, F4) access
    pub fn validate_grid_controls_access(
        security_context: &SecurityContext,
        feature_flags: &DebugFeatureFlags,
    ) -> bool {
        if !feature_flags.grid_controls_enabled {
            warn!("Grid controls denied: Feature disabled");
            return false;
        }
        
        if !security_context.has_debug_ui_permission() {
            warn!("Grid controls denied: Debug UI permission required");
            return false;
        }
        
        true
    }
    
    /// Validate wave selection (Ctrl+1-9) access
    pub fn validate_wave_selection_access(
        security_context: &SecurityContext,
        feature_flags: &DebugFeatureFlags,
    ) -> bool {
        if !feature_flags.wave_selection_enabled {
            warn!("Wave selection denied: Feature disabled");
            return false;
        }
        
        if !security_context.has_debug_ui_permission() {
            warn!("Wave selection denied: Debug UI permission required");
            return false;
        }
        
        true
    }
    
    /// Validate spawn rate controls (1-5 keys) access
    pub fn validate_spawn_rate_controls_access(
        security_context: &SecurityContext,
        feature_flags: &DebugFeatureFlags,
    ) -> bool {
        if !feature_flags.spawn_rate_controls_enabled {
            warn!("Spawn rate controls denied: Feature disabled");
            return false;
        }
        
        if !security_context.has_debug_ui_permission() {
            warn!("Spawn rate controls denied: Debug UI permission required");
            return false;
        }
        
        true
    }
    
    /// Validate console output access
    pub fn validate_console_output_access(
        security_context: &SecurityContext,
        feature_flags: &DebugFeatureFlags,
    ) -> bool {
        feature_flags.console_output_enabled && security_context.has_console_output_permission()
    }
    
    /// Comprehensive security audit of current configuration
    pub fn audit_security_configuration(
        security_context: &SecurityContext,
        feature_flags: &DebugFeatureFlags,
    ) -> SecurityAuditResult {
        let mut audit = SecurityAuditResult::default();
        
        // Check build type consistency
        #[cfg(not(debug_assertions))]
        {
            if feature_flags.has_any_debug_features_enabled() {
                audit.violations.push("Debug features enabled in release build".to_string());
            }
        }
        
        // Check session validity
        if security_context.debug_mode_authorized && security_context.is_session_expired() {
            audit.violations.push("Debug session expired but still authorized".to_string());
        }
        
        // Check privilege escalation
        if security_context.admin_privileges && !security_context.debug_mode_authorized {
            audit.violations.push("Admin privileges without debug authorization".to_string());
        }
        
        // Check feature flag consistency
        if feature_flags.cheat_menu_enabled && !security_context.admin_privileges {
            audit.warnings.push("Cheat menu enabled without admin privileges".to_string());
        }
        
        audit.features_enabled = feature_flags.has_any_debug_features_enabled();
        audit.session_valid = !security_context.is_session_expired();
        
        info!("Security audit completed: {} violations, {} warnings", 
              audit.violations.len(), audit.warnings.len());
        
        audit
    }
}

/// Result of security configuration audit
#[derive(Debug, Default)]
pub struct SecurityAuditResult {
    /// Critical security violations that must be addressed
    pub violations: Vec<String>,
    
    /// Security warnings that should be reviewed
    pub warnings: Vec<String>,
    
    /// Whether any debug features are currently enabled
    pub features_enabled: bool,
    
    /// Whether the current session is still valid
    pub session_valid: bool,
}

impl SecurityAuditResult {
    /// Check if the configuration passes security audit
    pub fn is_secure(&self) -> bool {
        self.violations.is_empty()
    }
    
    /// Get summary of audit results
    pub fn get_summary(&self) -> String {
        if self.is_secure() {
            format!("Security audit PASSED: {} warnings, features={}, session_valid={}", 
                   self.warnings.len(), self.features_enabled, self.session_valid)
        } else {
            format!("Security audit FAILED: {} violations, {} warnings", 
                   self.violations.len(), self.warnings.len())
        }
    }
}