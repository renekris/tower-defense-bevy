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
}

impl Default for WaveManager {
    fn default() -> Self {
        Self::new()
    }
}