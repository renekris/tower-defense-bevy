use bevy::prelude::*;

/// Main application state using Bevy's state system
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    /// Game is actively being played
    #[default]
    Playing,
    /// Game is paused - pause menu visible
    Paused,
    /// Settings menu is open
    Settings,
}

/// Game state for tracking win/loss conditions (separate from UI state)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Resource, Default)]
pub enum GameState {
    /// Game is actively being played
    #[default]
    Playing,
    /// Game has ended (player lost)
    GameOver,
    /// Player has won
    Victory,
}

/// System sets for organizing systems by state and purpose
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum GameSystemSet {
    /// Gameplay systems - only run in Playing state
    Gameplay,
    /// UI systems - run in all states
    UI,
    /// Pause menu systems - only run in Paused state
    Pause,
    /// Settings menu systems - only run in Settings state
    Settings,
    /// Input systems - run in all states but handle differently
    Input,
}