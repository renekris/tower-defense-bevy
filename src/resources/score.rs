use bevy::prelude::*;

/// Tracks the player's score and game statistics
#[derive(Debug, Clone, Resource)]
pub struct Score {
    /// Current score points
    pub current: u32,
    /// Number of enemies killed
    pub enemies_killed: u32,
    /// Number of enemies that escaped
    pub enemies_escaped: u32,
}

impl Score {
    /// Create a new score starting at zero
    pub fn new() -> Self {
        Self {
            current: 0,
            enemies_killed: 0,
            enemies_escaped: 0,
        }
    }

    /// Record that an enemy was killed and add points to score
    pub fn enemy_killed(&mut self, points: u32) {
        self.current += points;
        self.enemies_killed += 1;
    }

    /// Record that an enemy escaped (reached the end)
    pub fn enemy_escaped(&mut self) {
        self.enemies_escaped += 1;
    }

    /// Get the total number of enemies that have appeared
    pub fn total_enemies(&self) -> u32 {
        self.enemies_killed + self.enemies_escaped
    }
}

impl Default for Score {
    fn default() -> Self {
        Self::new()
    }
}