use bevy::prelude::*;

/// Enemy component that defines enemy properties
#[derive(Component)]
pub struct Enemy {
    pub speed: f32,
    pub path_index: usize,
    pub reward: u32,
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            speed: 50.0,
            path_index: 0,
            reward: 10,
        }
    }
}

impl Enemy {
    /// Create a new enemy with stats scaled based on wave number
    /// This implements proper difficulty progression
    pub fn for_wave(wave_number: u32) -> Self {
        let wave = wave_number.max(1); // Ensure minimum wave 1
        
        Self {
            speed: 50.0 + (wave as f32 * 5.0),           // Speed increases by 5/wave
            path_index: 0,
            reward: 8 + (wave * 2),                      // Reward scales with difficulty
        }
    }
    
    /// Get the appropriate health for this enemy based on wave number
    pub fn health_for_wave(wave_number: u32) -> f32 {
        let wave = wave_number.max(1); // Ensure minimum wave 1
        50.0 + (wave as f32 * 25.0)  // Health: Wave 1=75, Wave 2=100, Wave 3=125, etc.
    }
}

/// Component that tracks an enemy's progress along the path (0.0 to 1.0)
#[derive(Component)]
pub struct PathProgress {
    /// Current progress along the path (0.0 = start, 1.0 = end)
    pub current: f32,
}

impl PathProgress {
    /// Create a new path progress starting at the beginning
    pub fn new() -> Self {
        Self { current: 0.0 }
    }

    /// Advance the progress by the given amount, clamping to [0.0, 1.0]
    pub fn advance(&mut self, amount: f32) {
        self.current = (self.current + amount).clamp(0.0, 1.0);
    }

    /// Check if the enemy has reached the end of the path
    pub fn is_complete(&self) -> bool {
        self.current >= 1.0
    }
}

impl Default for PathProgress {
    fn default() -> Self {
        Self::new()
    }
}