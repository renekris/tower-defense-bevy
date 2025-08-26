use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;
use crate::systems::path_generation::generate_level_path;

/// Event sent when the player clicks the Start Wave button
#[derive(Event)]
pub struct StartWaveEvent;

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
/// Now controlled via UI button instead of keyboard
pub fn manual_wave_system(
    mut wave_manager: ResMut<WaveManager>,
    mut wave_start_events: EventReader<StartWaveEvent>,
) {
    for _event in wave_start_events.read() {
        if wave_manager.current_wave == 0 || wave_manager.wave_complete() {
            // Calculate progressive enemy count based on wave number
            let next_wave = wave_manager.current_wave + 1;
            let enemy_count = calculate_enemies_for_wave(next_wave);
            
            // Start wave with progressive scaling
            wave_manager.start_wave(enemy_count);
            info!("Started wave {} with {} enemies", next_wave, enemy_count);
        }
    }
}

/// Calculate the number of enemies for a given wave with progressive difficulty scaling
pub fn calculate_enemies_for_wave(wave_number: u32) -> u32 {
    let wave = wave_number.max(1); // Ensure minimum wave 1
    
    // Progressive scaling formula:
    // Wave 1: 5 enemies (base)
    // Wave 2: 7 enemies (+2)
    // Wave 3: 10 enemies (+3) 
    // Wave 4: 14 enemies (+4)
    // Wave 5: 19 enemies (+5)
    // Formula: base + sum of increases = 5 + (wave-1) + (wave-1)*(wave-2)/2
    
    let base_enemies = 5u32;
    let linear_scaling = wave - 1;  // +1 per wave after wave 1
    let exponential_scaling = (wave - 1) * (wave.saturating_sub(2)) / 2; // Accelerating increase
    
    base_enemies + linear_scaling + exponential_scaling
}

/// System that generates the initial path when the game starts
/// Path persists across all waves for consistency
pub fn path_generation_system(
    mut enemy_path: ResMut<EnemyPath>,
    wave_manager: Res<WaveManager>,
) {
    // Only generate path once when the game first starts
    // This ensures the path stays the same across all waves
    if wave_manager.is_added() || (wave_manager.current_wave == 1 && wave_manager.enemies_spawned == 0 && enemy_path.waypoints.is_empty()) {
        let new_path = generate_level_path(1); // Use wave 1 seed for consistent path
        *enemy_path = new_path;
        info!(
            "Generated persistent path with {} waypoints (will be used for all waves)", 
            enemy_path.waypoints.len()
        );
    }
}

/// System that updates path visualization when the path changes
/// This creates/updates visual path segments that show players where enemies will move
pub fn path_visualization_system(
    mut commands: Commands,
    enemy_path: Res<EnemyPath>,
    existing_path_viz: Query<Entity, With<crate::components::PathVisualization>>,
) {
    // Only update visualization when the path resource changes
    if enemy_path.is_changed() && !enemy_path.is_added() {
        // Remove existing path visualization entities
        for entity in existing_path_viz.iter() {
            commands.entity(entity).despawn();
        }
        
        // Create new path visualization based on current path
        for i in 0..enemy_path.waypoints.len() - 1 {
            let start = enemy_path.waypoints[i];
            let end = enemy_path.waypoints[i + 1];
            let midpoint = (start + end) / 2.0;
            let length = start.distance(end);
            
            // Calculate rotation angle to align the rectangle with the path segment
            let direction = end - start;
            let angle = direction.y.atan2(direction.x);
            
            commands.spawn((
                Sprite {
                    color: Color::srgb(0.5, 0.5, 0.5),
                    custom_size: Some(Vec2::new(length, 5.0)),
                    ..default()
                },
                Transform::from_translation(midpoint.extend(-1.0))
                    .with_rotation(Quat::from_rotation_z(angle)),
                crate::components::PathVisualization,
            ));
        }
        
        info!("Updated path visualization with {} segments", enemy_path.waypoints.len() - 1);
    } 
    // On first run (when resource is added), create initial visualization
    else if enemy_path.is_added() {
        for i in 0..enemy_path.waypoints.len() - 1 {
            let start = enemy_path.waypoints[i];
            let end = enemy_path.waypoints[i + 1];
            let midpoint = (start + end) / 2.0;
            let length = start.distance(end);
            
            let direction = end - start;
            let angle = direction.y.atan2(direction.x);
            
            commands.spawn((
                Sprite {
                    color: Color::srgb(0.5, 0.5, 0.5),
                    custom_size: Some(Vec2::new(length, 5.0)),
                    ..default()
                },
                Transform::from_translation(midpoint.extend(-1.0))
                    .with_rotation(Quat::from_rotation_z(angle)),
                crate::components::PathVisualization,
            ));
        }
        
        info!("Created initial path visualization with {} segments", enemy_path.waypoints.len() - 1);
    }
}