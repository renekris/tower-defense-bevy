use bevy::prelude::*;

/// Defines the path that enemies follow from spawn to goal
#[derive(Debug, Clone, Resource)]
pub struct EnemyPath {
    /// Waypoints that define the path enemies follow
    pub waypoints: Vec<Vec2>,
}

impl EnemyPath {
    /// Create a new enemy path with the given waypoints
    pub fn new(waypoints: Vec<Vec2>) -> Self {
        assert!(!waypoints.is_empty(), "Enemy path must have at least one waypoint");
        Self { waypoints }
    }

    /// Get the position along the path at the given progress (0.0 to 1.0)
    /// 0.0 = start of path, 1.0 = end of path
    pub fn get_position_at_progress(&self, progress: f32) -> Vec2 {
        let progress = progress.clamp(0.0, 1.0);
        
        if progress <= 0.0 {
            return self.waypoints[0];
        }
        if progress >= 1.0 {
            return self.waypoints[self.waypoints.len() - 1];
        }

        // Calculate which segment we're in
        let total_segments = self.waypoints.len() - 1;
        let segment_progress = progress * total_segments as f32;
        let segment_index = segment_progress.floor() as usize;
        let local_progress = segment_progress - segment_index as f32;

        // Handle edge case where we're exactly at the last segment
        if segment_index >= total_segments {
            return self.waypoints[self.waypoints.len() - 1];
        }

        // Linear interpolation between waypoints
        let start = self.waypoints[segment_index];
        let end = self.waypoints[segment_index + 1];
        start.lerp(end, local_progress)
    }

    /// Get the total length of the path (sum of distances between waypoints)
    pub fn total_length(&self) -> f32 {
        let mut total = 0.0;
        for i in 0..self.waypoints.len() - 1 {
            total += self.waypoints[i].distance(self.waypoints[i + 1]);
        }
        total
    }

    /// Get the smooth position along the path using Catmull-Rom spline interpolation
    /// This creates natural curves between waypoints while ensuring the path passes through all waypoints
    /// 0.0 = start of path, 1.0 = end of path
    pub fn get_smooth_position_at_progress(&self, progress: f32) -> Vec2 {
        let progress = progress.clamp(0.0, 1.0);
        
        // Handle edge cases
        if self.waypoints.len() < 2 {
            return self.waypoints.get(0).copied().unwrap_or(Vec2::ZERO);
        }
        
        if progress <= 0.0 {
            return self.waypoints[0];
        }
        if progress >= 1.0 {
            return self.waypoints[self.waypoints.len() - 1];
        }

        // Calculate which segment we're in
        let total_segments = self.waypoints.len() - 1;
        let segment_progress = progress * total_segments as f32;
        let segment_index = segment_progress.floor() as usize;
        let local_progress = segment_progress - segment_index as f32;

        // Handle edge case where we're exactly at the last segment
        if segment_index >= total_segments {
            return self.waypoints[self.waypoints.len() - 1];
        }

        // Get the four control points for Catmull-Rom spline
        let p0 = self.get_control_point(segment_index as i32 - 1);
        let p1 = self.waypoints[segment_index];
        let p2 = self.waypoints[segment_index + 1];
        let p3 = self.get_control_point(segment_index as i32 + 2);

        // Apply Catmull-Rom interpolation
        self.catmull_rom_interpolation(p0, p1, p2, p3, local_progress)
    }

    /// Performs Catmull-Rom spline interpolation between four control points
    /// p1 and p2 are the actual waypoints, p0 and p3 are control points
    /// t is the interpolation parameter (0.0 to 1.0)
    fn catmull_rom_interpolation(&self, p0: Vec2, p1: Vec2, p2: Vec2, p3: Vec2, t: f32) -> Vec2 {
        let t2 = t * t;
        let t3 = t2 * t;

        // Catmull-Rom basis functions
        let a0 = -0.5 * t3 + t2 - 0.5 * t;
        let a1 = 1.5 * t3 - 2.5 * t2 + 1.0;
        let a2 = -1.5 * t3 + 2.0 * t2 + 0.5 * t;
        let a3 = 0.5 * t3 - 0.5 * t2;

        // Apply the basis functions to each control point
        p0 * a0 + p1 * a1 + p2 * a2 + p3 * a3
    }

    /// Get a control point for spline calculation, handling boundary conditions
    /// For indices outside the waypoint range, extends the path naturally
    fn get_control_point(&self, index: i32) -> Vec2 {
        let len = self.waypoints.len() as i32;
        
        if index < 0 {
            // Before the start: extend the path backwards
            let first = self.waypoints[0];
            let second = self.waypoints.get(1).copied().unwrap_or(first);
            let direction = first - second;
            first + direction
        } else if index >= len {
            // After the end: extend the path forwards
            let last_idx = (len - 1) as usize;
            let last = self.waypoints[last_idx];
            let second_last = self.waypoints.get(last_idx.saturating_sub(1)).copied().unwrap_or(last);
            let direction = last - second_last;
            last + direction
        } else {
            // Within bounds: return the actual waypoint
            self.waypoints[index as usize]
        }
    }
}

/// Simple wave manager for Phase 1 - manual wave spawning
#[derive(Debug, Resource)]
pub struct WaveManager {
    /// Current wave number
    pub current_wave: u32,
    /// Number of enemies in current wave
    pub enemies_in_wave: u32,
    /// Number of enemies spawned so far in current wave
    pub enemies_spawned: u32,
    /// Timer for spawning enemies
    pub spawn_timer: Timer,
}

impl WaveManager {
    /// Create a new wave manager
    pub fn new() -> Self {
        Self {
            current_wave: 0,
            enemies_in_wave: 0,
            enemies_spawned: 0,
            spawn_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        }
    }

    /// Start a new wave with the given number of enemies
    pub fn start_wave(&mut self, enemy_count: u32) {
        self.current_wave += 1;
        self.enemies_in_wave = enemy_count;
        self.enemies_spawned = 0;
        
        // Scale spawn rate based on wave number for increased intensity
        let spawn_rate = self.calculate_spawn_rate_for_wave();
        self.set_spawn_rate(spawn_rate);
    }
    
    /// Calculate appropriate spawn rate for current wave
    /// Higher waves spawn enemies faster for increased pressure
    fn calculate_spawn_rate_for_wave(&self) -> f32 {
        let wave = self.current_wave.max(1) as f32;
        
        // Progressive spawn rate scaling:
        // Wave 1: 1.0 (1 enemy per second)
        // Wave 2: 1.2 (1 enemy per 0.83 seconds)  
        // Wave 3: 1.4 (1 enemy per 0.71 seconds)
        // Wave 10: 2.8 (1 enemy per 0.36 seconds)
        // Formula: 1.0 + (wave - 1) * 0.2, capped at 3.0
        let base_rate = 1.0;
        let scaling_factor = 0.2;
        let max_rate = 3.0;
        
        (base_rate + (wave - 1.0) * scaling_factor).min(max_rate)
    }

    /// Check if all enemies in the current wave have been spawned
    pub fn wave_complete(&self) -> bool {
        self.enemies_spawned >= self.enemies_in_wave
    }

    /// Check if it's time to spawn the next enemy
    pub fn should_spawn_enemy(&self) -> bool {
        !self.wave_complete() && self.spawn_timer.finished()
    }

    /// Record that an enemy was spawned
    pub fn enemy_spawned(&mut self) {
        self.enemies_spawned += 1;
    }

    /// Update the spawn rate (higher values = faster spawning)
    /// spawn_rate: 0.5 = slow (2 second intervals), 1.0 = normal (1 second), 3.0 = fast (0.33 seconds)
    pub fn set_spawn_rate(&mut self, spawn_rate: f32) {
        let spawn_interval = 1.0 / spawn_rate.max(0.1); // Prevent division by zero/negative
        self.spawn_timer.set_duration(std::time::Duration::from_secs_f32(spawn_interval));
    }
}

impl Default for WaveManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smooth_interpolation_passes_through_waypoints() {
        let waypoints = vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(100.0, 0.0),
            Vec2::new(100.0, 100.0),
            Vec2::new(200.0, 100.0),
        ];
        let path = EnemyPath::new(waypoints.clone());

        // Test that smooth interpolation passes exactly through each waypoint
        let total_segments = waypoints.len() - 1;
        for i in 0..waypoints.len() {
            let progress = i as f32 / total_segments as f32;
            let smooth_pos = path.get_smooth_position_at_progress(progress);
            let expected = waypoints[i];
            
            // Should be very close to the actual waypoint (within floating point precision)
            assert!((smooth_pos - expected).length() < 0.001, 
                "Smooth path should pass through waypoint {} at progress {}, got {:?}, expected {:?}", 
                i, progress, smooth_pos, expected);
        }
    }

    #[test]
    fn test_smooth_interpolation_creates_curves() {
        let waypoints = vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(50.0, 0.0),
            Vec2::new(100.0, 50.0),
            Vec2::new(150.0, 0.0),
        ];
        let path = EnemyPath::new(waypoints);

        // Test that smooth interpolation creates curves (not straight lines)
        // Check a point in the middle of the second segment where curves should be most visible
        let linear_mid = path.get_position_at_progress(0.5);
        let smooth_mid = path.get_smooth_position_at_progress(0.5);

        // The smooth path should deviate from the linear path with this zigzag pattern
        assert!((linear_mid - smooth_mid).length() > 1.0, 
            "Smooth interpolation should create curves, not straight lines. Linear: {:?}, Smooth: {:?}", 
            linear_mid, smooth_mid);
    }

    #[test]
    fn test_edge_case_single_waypoint() {
        let waypoints = vec![Vec2::new(50.0, 50.0)];
        let path = EnemyPath::new(waypoints.clone());

        // Should always return the single waypoint regardless of progress
        assert_eq!(path.get_smooth_position_at_progress(0.0), waypoints[0]);
        assert_eq!(path.get_smooth_position_at_progress(0.5), waypoints[0]);
        assert_eq!(path.get_smooth_position_at_progress(1.0), waypoints[0]);
    }

    #[test]
    fn test_progress_clamping() {
        let waypoints = vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(100.0, 0.0),
            Vec2::new(200.0, 100.0),
        ];
        let path = EnemyPath::new(waypoints.clone());

        // Test negative progress
        assert_eq!(path.get_smooth_position_at_progress(-0.5), waypoints[0]);
        assert_eq!(path.get_smooth_position_at_progress(-100.0), waypoints[0]);

        // Test progress > 1.0
        let last_waypoint = waypoints[waypoints.len() - 1];
        assert_eq!(path.get_smooth_position_at_progress(1.5), last_waypoint);
        assert_eq!(path.get_smooth_position_at_progress(100.0), last_waypoint);
    }

    #[test]
    fn test_backward_compatibility() {
        let waypoints = vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(100.0, 0.0),
            Vec2::new(100.0, 100.0),
        ];
        let path = EnemyPath::new(waypoints.clone());

        // Both methods should work and return reasonable results
        let linear_start = path.get_position_at_progress(0.0);
        let smooth_start = path.get_smooth_position_at_progress(0.0);
        let linear_end = path.get_position_at_progress(1.0);
        let smooth_end = path.get_smooth_position_at_progress(1.0);

        // Start and end points should be identical
        assert_eq!(linear_start, smooth_start);
        assert_eq!(linear_end, smooth_end);
        
        // Both should equal the actual waypoints
        assert_eq!(linear_start, waypoints[0]);
        assert_eq!(smooth_start, waypoints[0]);
        assert_eq!(linear_end, waypoints[waypoints.len() - 1]);
        assert_eq!(smooth_end, waypoints[waypoints.len() - 1]);
    }
}