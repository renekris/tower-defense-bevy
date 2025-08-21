use bevy::prelude::*;

/// Represents the current state of the game
#[derive(Debug, Clone, Copy, PartialEq, Eq, Resource, Default)]
pub enum GameState {
    /// Game is actively being played
    #[default]
    Playing,
    /// Game is paused
    Paused,
    /// Game has ended (player lost)
    GameOver,
    /// Player has won
    Victory,
}