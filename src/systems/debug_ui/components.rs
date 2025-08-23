use bevy::prelude::*;

/// Resource to manage debug UI state
#[derive(Resource, Debug)]
pub struct DebugUIState {
    pub panel_visible: bool,
    pub grid_visible: bool,
    pub path_visible: bool,
    pub zones_visible: bool,
    pub performance_visible: bool,
    pub current_difficulty: f32,
    pub current_obstacle_density: f32,
    pub enemy_spawn_rate: f32,
    pub tower_damage_multiplier: f32,
    pub current_wave: u32,
}

impl Default for DebugUIState {
    fn default() -> Self {
        Self {
            panel_visible: false,
            grid_visible: true,
            path_visible: true,
            zones_visible: true,
            performance_visible: true,
            current_difficulty: 0.15,
            current_obstacle_density: 0.15,
            enemy_spawn_rate: 1.0,
            tower_damage_multiplier: 1.0,
            current_wave: 1,
        }
    }
}

/// Component marker for the debug UI panel
#[derive(Component)]
pub struct DebugUIPanel;

/// Component marker for UI sections
#[derive(Component)]
pub struct DebugUISection {
    pub section_type: UISectionType,
}

#[derive(Debug, Clone, Copy)]
pub enum UISectionType {
    Controls,
    Metrics,
    Parameters,
    Actions,
    Help,
}

/// Component for toggle buttons
#[derive(Component)]
pub struct ToggleButton {
    pub toggle_type: ToggleType,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ToggleType {
    Grid,
    Path,
    Zones,
    Performance,
}

/// Component for parameter sliders
#[derive(Component)]
pub struct ParameterSlider {
    pub slider_type: SliderType,
    pub min_value: f32,
    pub max_value: f32,
    pub current_value: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SliderType {
    ObstacleDensity,
    EnemySpawnRate,
    TowerDamageMultiplier,
}

/// Component for slider handle (draggable part)
#[derive(Component)]
pub struct SliderHandle {
    pub slider_type: SliderType,
}

/// Component for slider track (background bar)
#[derive(Component)]
pub struct SliderTrack {
    pub slider_type: SliderType,
}

/// Component for slider value text display
#[derive(Component)]
pub struct SliderValueText {
    pub slider_type: SliderType,
}

/// Resource to track which slider is being dragged
#[derive(Resource, Default)]
pub struct SliderDragState {
    pub dragging: Option<SliderType>,
}

/// Resource to track performance metrics
#[derive(Resource)]
pub struct PerformanceMetrics {
    pub fps: f32,
    pub frame_time_ms: f32,
    pub entity_count: usize,
    pub path_generation_time_ms: f32,
    pub last_update_time: f32,
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            fps: 60.0,
            frame_time_ms: 16.67,
            entity_count: 0,
            path_generation_time_ms: 0.0,
            last_update_time: 0.0,
        }
    }
}

/// Component marker for game path line entities
#[derive(Component)]
pub struct GamePathLine;

/// Component for performance metric text displays
#[derive(Component)]
pub struct PerformanceMetricText {
    pub metric_type: MetricType,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MetricType {
    FPS,
    FrameTime,
    EntityCount,
    PathGenTime,
}

/// Component marker for action buttons
#[derive(Component)]
pub struct ActionButton {
    pub action_type: ActionType,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ActionType {
    ResetGame,
    RandomizeMap,
    SaveState,
    LoadState,
}