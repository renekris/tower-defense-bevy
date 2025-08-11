use bevy::prelude::*;
use super::components::*;

/// System to update performance metrics
pub fn update_performance_metrics(
    mut metrics: ResMut<PerformanceMetrics>,
    time: Res<Time>,
    entities: Query<Entity>,
) {
    // Calculate FPS and frame time
    let delta_time = time.delta_secs();
    if delta_time > 0.0 {
        // Simple moving average for smoother display
        let new_fps = 1.0 / delta_time;
        metrics.fps = metrics.fps * 0.9 + new_fps * 0.1;
        metrics.frame_time_ms = delta_time * 1000.0;
    }
    
    // Count entities
    metrics.entity_count = entities.iter().count();
    
    // Update timestamp
    metrics.last_update_time = time.elapsed_secs();
}

/// System to update performance metrics display
pub fn update_performance_display(
    metrics: Res<PerformanceMetrics>,
    mut text_query: Query<(&PerformanceMetricText, &mut Text)>,
) {
    // Only update display every few frames to avoid flickering
    if metrics.last_update_time % 0.1 < 0.016 { // Update ~10 times per second
        for (metric_info, mut text) in &mut text_query {
            let display_text = match metric_info.metric_type {
                MetricType::FPS => format!("FPS: {:.1}", metrics.fps),
                MetricType::FrameTime => format!("Frame Time: {:.1}ms", metrics.frame_time_ms),
                MetricType::EntityCount => format!("Entities: {}", metrics.entity_count),
                MetricType::PathGenTime => format!("Path Gen: {:.1}ms", metrics.path_generation_time_ms),
            };
            **text = display_text;
        }
    }
}