use bevy::prelude::*;

#[derive(Component)]
pub struct GamePosition {
    pub x: f32,
    pub y: f32,
}

impl GamePosition {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn distance_to(&self, other: &GamePosition) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }

    pub fn to_vec3(&self) -> Vec3 {
        Vec3::new(self.x, self.y, 0.0)
    }
}

impl Default for GamePosition {
    fn default() -> Self {
        Self::new(0.0, 0.0)
    }
}

impl From<Vec2> for GamePosition {
    fn from(vec: Vec2) -> Self {
        Self::new(vec.x, vec.y)
    }
}

impl From<GamePosition> for Vec2 {
    fn from(pos: GamePosition) -> Self {
        Vec2::new(pos.x, pos.y)
    }
}