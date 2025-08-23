use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;

/// System that spawns enemies when the wave manager indicates it's time
pub fn enemy_spawning_system(
    mut commands: Commands,
    mut wave_manager: ResMut<WaveManager>,
    enemy_path: Res<EnemyPath>,
    time: Res<Time>,
) {
    // Update the spawn timer
    wave_manager.spawn_timer.tick(time.delta());

    // Check if we should spawn an enemy
    if wave_manager.should_spawn_enemy() {
        // Get the starting position from the path
        let start_pos = enemy_path.get_position_at_progress(0.0);

        // Spawn a new enemy entity with wave-scaled stats for proper difficulty progression
        let current_wave = wave_manager.current_wave;
        commands.spawn((
            Enemy::for_wave(current_wave),                    // Wave-scaled speed and reward
            Health::new(Enemy::health_for_wave(current_wave)), // Wave-scaled health
            PathProgress::new(),
            Sprite {
                color: Color::srgb(1.0, 0.2, 0.2), // Red color for enemies
                custom_size: Some(Vec2::new(20.0, 20.0)), // 20x20 pixel square
                ..default()
            },
            Transform::from_translation(start_pos.extend(1.0)),
        ));

        // Record that we spawned an enemy
        wave_manager.enemy_spawned();
    }
}

/// System that moves enemies along the path based on their speed
pub fn enemy_movement_system(
    mut enemy_query: Query<(&Enemy, &mut PathProgress, &mut Transform)>,
    enemy_path: Res<EnemyPath>,
    time: Res<Time>,
) {
    let path_length = enemy_path.total_length();

    for (enemy, mut path_progress, mut transform) in enemy_query.iter_mut() {
        // Calculate how far the enemy should move this frame
        let distance_this_frame = enemy.speed * time.delta_secs();
        
        // Convert distance to progress (0.0 to 1.0)
        let progress_this_frame = distance_this_frame / path_length;
        
        // Advance the enemy's progress
        path_progress.advance(progress_this_frame);
        
        // Update the enemy's position based on current progress
        let new_position = enemy_path.get_position_at_progress(path_progress.current);
        transform.translation = new_position.extend(0.0);
    }
}

/// System that removes enemies that have reached the end of the path
pub fn enemy_cleanup_system(
    mut commands: Commands,
    mut score: ResMut<Score>,
    enemy_query: Query<(Entity, &PathProgress), With<Enemy>>,
) {
    for (entity, path_progress) in enemy_query.iter() {
        if path_progress.is_complete() {
            // Enemy reached the end - remove it and record as escaped
            commands.entity(entity).despawn();
            score.enemy_escaped();
        }
    }
}

/// System that handles manual wave spawning (for Phase 1)
/// Press SPACE to start the next wave
pub fn manual_wave_system(
    mut wave_manager: ResMut<WaveManager>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        // Start a new wave with 5 enemies
        // Later this can be made configurable or progressive
        if wave_manager.current_wave == 0 || wave_manager.wave_complete() {
            wave_manager.start_wave(5);
        }
    }
}