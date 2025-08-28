use bevy::prelude::*;
use tower_defense_bevy::systems::security::*;

/// Test security context initialization with secure defaults
#[test]
fn test_security_context_secure_defaults() {
    let security_context = SecurityContext::default();
    
    // Should start with secure defaults
    assert_eq!(security_context.development_build, false);
    assert_eq!(security_context.debug_mode_authorized, false);
    assert_eq!(security_context.admin_privileges, false);
    assert!(!security_context.is_session_expired()); // New session should be valid
}

/// Test debug feature flags secure defaults
#[test]
fn test_debug_feature_flags_secure_defaults() {
    let feature_flags = DebugFeatureFlags::default();
    
    // All debug features should be disabled by default
    assert_eq!(feature_flags.debug_visualization_enabled, false);
    assert_eq!(feature_flags.debug_ui_enabled, false);
    assert_eq!(feature_flags.cheat_menu_enabled, false);
    assert_eq!(feature_flags.console_output_enabled, false);
    assert_eq!(feature_flags.wave_selection_enabled, false);
    assert_eq!(feature_flags.spawn_rate_controls_enabled, false);
    assert_eq!(feature_flags.grid_controls_enabled, false);
    assert!(!feature_flags.has_any_debug_features_enabled());
}

/// Test authorization validation for F1 debug visualization
#[test]
fn test_f1_debug_visualization_authorization() {
    let security_context = SecurityContext::default();
    let feature_flags = DebugFeatureFlags::default();
    
    // Should be denied with default (secure) settings
    assert!(!DebugAuthorization::validate_debug_visualization_access(
        &security_context, &feature_flags
    ));
    
    // Enable development features
    let mut dev_feature_flags = DebugFeatureFlags::default();
    dev_feature_flags.enable_development_features();
    
    let mut dev_context = SecurityContext::default();
    dev_context.development_build = true;
    dev_context.authorize_debug_access();
    
    // Should be allowed with proper authorization
    assert!(DebugAuthorization::validate_debug_visualization_access(
        &dev_context, &dev_feature_flags
    ));
}

/// Test authorization validation for F9 cheat menu (requires admin privileges)
#[test]
fn test_f9_cheat_menu_authorization() {
    let mut security_context = SecurityContext::default();
    let mut feature_flags = DebugFeatureFlags::default();
    
    // Should be denied without admin privileges
    assert!(!DebugAuthorization::validate_cheat_menu_access(
        &security_context, &feature_flags
    ));
    
    // Enable development mode but not admin privileges
    security_context.development_build = true;
    security_context.authorize_debug_access();
    feature_flags.enable_development_features();
    
    // Still denied without admin privileges
    assert!(!DebugAuthorization::validate_cheat_menu_access(
        &security_context, &feature_flags
    ));
    
    // Grant admin privileges
    security_context.authorize_admin_privileges();
    feature_flags.enable_admin_features();
    
    // Should now be allowed
    assert!(DebugAuthorization::validate_cheat_menu_access(
        &security_context, &feature_flags
    ));
}

/// Test security audit functionality
#[test]
fn test_security_audit() {
    let security_context = SecurityContext::default();
    let feature_flags = DebugFeatureFlags::default();
    
    // Secure configuration should pass audit
    let audit = DebugAuthorization::audit_security_configuration(
        &security_context, &feature_flags
    );
    
    assert!(audit.is_secure());
    assert_eq!(audit.violations.len(), 0);
    assert!(!audit.features_enabled);
    assert!(audit.session_valid);
}

/// Test production build security compliance
#[cfg(not(debug_assertions))]
#[test]
fn test_production_security_compliance() {
    let mut feature_flags = DebugFeatureFlags::default();
    
    // Should pass validation with secure defaults
    assert!(feature_flags.validate_security_compliance().is_ok());
    
    // Enable debug features (simulating security violation)
    feature_flags.debug_visualization_enabled = true;
    
    // Should fail validation in release build
    assert!(feature_flags.validate_security_compliance().is_err());
}

/// Test session timeout functionality
#[test]
fn test_session_timeout() {
    let mut security_context = SecurityContext::default();
    
    // Set very short timeout for testing
    security_context.max_debug_session_duration = std::time::Duration::from_nanos(1);
    
    // Session should expire immediately
    std::thread::sleep(std::time::Duration::from_millis(1));
    assert!(security_context.is_session_expired());
    
    // Should not have debug permissions with expired session
    security_context.development_build = true;
    security_context.debug_mode_authorized = true;
    assert!(!security_context.has_debug_visualization_permission());
}