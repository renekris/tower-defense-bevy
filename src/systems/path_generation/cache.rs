use std::collections::HashMap;
use bevy::prelude::*;
use crate::resources::EnemyPath;
use super::grid::TowerZone;

/// Cache for generated paths and zones to improve performance
#[derive(Resource, Default)]
pub struct PathCache {
    /// Cached paths and zones by seed
    cache: HashMap<u64, (EnemyPath, Vec<TowerZone>)>,
    /// Maximum number of entries to keep in cache
    max_entries: usize,
    /// Access order for LRU eviction
    access_order: Vec<u64>,
}

impl PathCache {
    /// Create a new path cache with specified capacity
    pub fn new(max_entries: usize) -> Self {
        Self {
            cache: HashMap::new(),
            max_entries,
            access_order: Vec::new(),
        }
    }
    
    /// Get cached path and zones, or generate if not cached
    pub fn get_or_generate(
        &mut self,
        seed: u64,
        difficulty: f32,
    ) -> (EnemyPath, Vec<TowerZone>) {
        // Check cache first
        if let Some(cached) = self.cache.get(&seed).cloned() {
            self.update_access_order(seed);
            return cached;
        }
        
        // Generate new path and zones
        let (path, zones) = self.generate_path_and_zones(seed, difficulty);
        
        // Add to cache with LRU eviction
        self.insert_with_eviction(seed, path.clone(), zones.clone());
        
        (path, zones)
    }
    
    /// Generate path and zones (internal implementation)
    fn generate_path_and_zones(&self, seed: u64, difficulty: f32) -> (EnemyPath, Vec<TowerZone>) {
        // Generate the grid-based map
        let grid = super::obstacles::generate_procedural_map(seed, difficulty);
        
        // Find optimal path through the generated obstacles  
        let grid_path = super::pathfinding::find_path(&grid, grid.entry_point, grid.exit_point)
            .expect("Generated map must have valid path");
        
        // Convert to world coordinates for enemy movement
        let enemy_path = grid.to_enemy_path(grid_path.clone());
        
        // Generate optimized placement zones
        let zones = super::zone_optimization::calculate_optimal_tower_zones(&grid, &grid_path);
        
        (enemy_path, zones)
    }
    
    /// Insert item into cache with LRU eviction
    fn insert_with_eviction(&mut self, seed: u64, path: EnemyPath, zones: Vec<TowerZone>) {
        // Remove oldest entry if at capacity
        if self.cache.len() >= self.max_entries && !self.cache.contains_key(&seed) {
            if let Some(&oldest_seed) = self.access_order.first() {
                self.cache.remove(&oldest_seed);
                self.access_order.retain(|&s| s != oldest_seed);
            }
        }
        
        // Insert new entry
        self.cache.insert(seed, (path, zones));
        self.update_access_order(seed);
    }
    
    /// Update access order for LRU tracking
    fn update_access_order(&mut self, seed: u64) {
        // Remove from current position
        self.access_order.retain(|&s| s != seed);
        
        // Add to end (most recently used)
        self.access_order.push(seed);
    }
    
    /// Clear the cache
    pub fn clear(&mut self) {
        self.cache.clear();
        self.access_order.clear();
    }
    
    /// Get cache statistics
    pub fn stats(&self) -> CacheStats {
        CacheStats {
            entries: self.cache.len(),
            capacity: self.max_entries,
            hit_ratio: 0.0, // Would need hit/miss tracking for real implementation
        }
    }
    
    /// Pre-generate paths for future waves (background processing)
    pub fn pregenerate_paths(&mut self, start_wave: u32, count: u32) {
        for wave in start_wave..(start_wave + count) {
            let difficulty = (wave as f32 * 0.15).min(1.0);
            let seed = wave as u64 * 12345 + 67890;
            
            // Only generate if not already cached
            if !self.cache.contains_key(&seed) {
                let (path, zones) = self.generate_path_and_zones(seed, difficulty);
                self.insert_with_eviction(seed, path, zones);
            }
        }
    }
}

/// Cache performance statistics
#[derive(Debug)]
pub struct CacheStats {
    pub entries: usize,
    pub capacity: usize,
    pub hit_ratio: f32,
}

/// System to initialize the path cache resource
pub fn setup_path_cache(mut commands: Commands) {
    commands.insert_resource(PathCache::new(20)); // Cache up to 20 paths
}

/// System to pregenerate paths for upcoming waves
pub fn pregenerate_upcoming_paths(
    mut cache: ResMut<PathCache>,
    // Could add wave manager here to know current wave
) {
    // For now, just pregenerate a few paths ahead
    // In a real implementation, this would be triggered based on current wave
    static mut LAST_PREGENERATED: u32 = 0;
    
    unsafe {
        if LAST_PREGENERATED == 0 {
            cache.pregenerate_paths(1, 5); // Pregenerate waves 1-5
            LAST_PREGENERATED = 5;
        }
    }
}

/// Resource for tracking cache performance metrics
#[derive(Resource, Default)]
pub struct CacheMetrics {
    pub hits: u64,
    pub misses: u64,
    pub generations: u64,
}

impl CacheMetrics {
    pub fn hit_ratio(&self) -> f32 {
        if self.hits + self.misses == 0 {
            0.0
        } else {
            self.hits as f32 / (self.hits + self.misses) as f32
        }
    }
    
    pub fn record_hit(&mut self) {
        self.hits += 1;
    }
    
    pub fn record_miss(&mut self) {
        self.misses += 1;
        self.generations += 1;
    }
}