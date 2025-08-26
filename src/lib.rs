pub mod components;
pub mod game;
pub mod resources;
pub mod systems;
pub mod utils;

// Explicit exports to prevent namespace conflicts
pub use components::{Enemy, Health, GamePosition, Projectile, Tower};
pub use resources::{Economy, GameState, Score, WaveManager, EnemyPath, TowerType};
pub use systems::enemy_system::{enemy_spawning_system, enemy_movement_system, enemy_cleanup_system};
pub use systems::combat_system::{tower_targeting_system, projectile_spawning_system, projectile_movement_system, collision_system};
pub use systems::input_system::{mouse_input_system};
pub use systems::ui_system::{update_ui_system};
// Note: Path generation systems are internal and don't need to be exported