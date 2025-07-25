use bevy::prelude::*;

// Re-export the main path generation resources
pub use crate::systems::path_generation::{PathGrid, PathCache, TowerZone};

/// Configuration for procedural path generation
#[derive(Resource)]
pub struct PathGenerationConfig {
    /// Grid width in cells
    pub grid_width: usize,
    /// Grid height in cells
    pub grid_height: usize,
    /// Size of each grid cell in world units
    pub cell_size: f32,
    /// Maximum cache size for generated paths
    pub cache_size: usize,
    /// Difficulty scaling factor per wave
    pub difficulty_scaling: f32,
    /// Base seed for deterministic generation
    pub base_seed: u64,
}

impl Default for PathGenerationConfig {
    fn default() -> Self {
        Self {
            grid_width: 20,
            grid_height: 12,
            cell_size: 64.0,
            cache_size: 20,
            difficulty_scaling: 0.15,
            base_seed: 12345,
        }
    }
}

/// Current state of path generation for UI display or debugging
#[derive(Resource, Default)]
pub struct PathGenerationState {
    /// Currently active path grid
    pub current_grid: Option<PathGrid>,
    /// Current wave number for which path was generated
    pub current_wave: u32,
    /// Time taken for last path generation (for performance monitoring)
    pub last_generation_time_ms: f32,
    /// Whether path generation is enabled
    pub enabled: bool,
}

impl PathGenerationState {
    pub fn new() -> Self {
        Self {
            current_grid: None,
            current_wave: 0,
            last_generation_time_ms: 0.0,
            enabled: true,
        }
    }
    
    pub fn update_for_wave(&mut self, wave: u32, grid: PathGrid, generation_time_ms: f32) {
        self.current_wave = wave;
        self.current_grid = Some(grid);
        self.last_generation_time_ms = generation_time_ms;
    }
    
    pub fn disable(&mut self) {
        self.enabled = false;
    }
    
    pub fn enable(&mut self) {
        self.enabled = true;
    }
}