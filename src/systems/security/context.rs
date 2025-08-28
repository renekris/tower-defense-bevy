use bevy::prelude::*;

/// Security context resource managing authorization and access control
/// Controls who can access debug features and with what privileges
#[derive(Resource, Debug, Clone)]
pub struct SecurityContext {
    /// Whether this is a development build (debug_assertions enabled)
    pub development_build: bool,
    
    /// Whether debug mode is currently authorized for this session
    pub debug_mode_authorized: bool,
    
    /// Whether admin privileges are available (for cheat features)
    pub admin_privileges: bool,
    
    /// Session start time for timeout validation
    pub session_start: std::time::Instant,
    
    /// Maximum debug session duration (prevents indefinite debug access)
    pub max_debug_session_duration: std::time::Duration,
}

impl Default for SecurityContext {
    fn default() -> Self {
        Self {
            development_build: false,
            debug_mode_authorized: false,
            admin_privileges: false,
            session_start: std::time::Instant::now(),
            max_debug_session_duration: std::time::Duration::from_secs(3600), // 1 hour max
        }
    }
}

impl SecurityContext {
    /// Check if debug visualization features are authorized
    pub fn has_debug_visualization_permission(&self) -> bool {
        self.development_build && self.debug_mode_authorized && !self.is_session_expired()
    }
    
    /// Check if debug UI features are authorized
    pub fn has_debug_ui_permission(&self) -> bool {
        self.development_build && self.debug_mode_authorized && !self.is_session_expired()
    }
    
    /// Check if cheat/admin features are authorized
    pub fn has_cheat_permission(&self) -> bool {
        self.development_build 
            && self.debug_mode_authorized 
            && self.admin_privileges 
            && !self.is_session_expired()
    }
    
    /// Check if console output is permitted
    pub fn has_console_output_permission(&self) -> bool {
        self.development_build && !self.is_session_expired()
    }
    
    /// Check if the debug session has expired
    pub fn is_session_expired(&self) -> bool {
        self.session_start.elapsed() > self.max_debug_session_duration
    }
    
    /// Authorize debug access for this session
    pub fn authorize_debug_access(&mut self) {
        if self.development_build {
            self.debug_mode_authorized = true;
            self.session_start = std::time::Instant::now();
            info!("Debug access authorized for development session");
        } else {
            warn!("Debug access denied: Not a development build");
        }
    }
    
    /// Authorize admin privileges (requires debug access first)
    pub fn authorize_admin_privileges(&mut self) {
        if self.has_debug_ui_permission() {
            self.admin_privileges = true;
            info!("Admin privileges granted for current debug session");
        } else {
            warn!("Admin privileges denied: Debug access not authorized");
        }
    }
    
    /// Revoke all debug access
    pub fn revoke_debug_access(&mut self) {
        self.debug_mode_authorized = false;
        self.admin_privileges = false;
        info!("Debug access revoked");
    }
    
    /// Get security status summary for logging
    pub fn get_security_status(&self) -> String {
        format!(
            "SecurityContext: dev={}, debug_auth={}, admin={}, session_valid={}",
            self.development_build,
            self.debug_mode_authorized,
            self.admin_privileges,
            !self.is_session_expired()
        )
    }
}