use bevy::prelude::*;

/// Represents the current state of the game
#[derive(Debug, Clone, Copy, PartialEq, Eq, Resource)]
pub enum GameState {
    /// Game is actively being played
    Playing,
    /// Game is paused
    Paused,
    /// Game has ended (player lost)
    GameOver,
    /// Player has won
    Victory,
}

impl Default for GameState {
    fn default() -> Self {
        GameState::Playing
    }
}