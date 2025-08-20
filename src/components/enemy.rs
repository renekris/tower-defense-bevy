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