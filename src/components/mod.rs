pub mod tower;
pub mod enemy;
pub mod projectile;
pub mod health;
pub mod position;

pub use tower::*;
pub use enemy::*;
pub use projectile::*;
pub use health::*;
pub use position::*;

use bevy::prelude::Component;

/// Marker component for path visualization entities that need to be updated when path changes
#[derive(Component)]
pub struct PathVisualization;