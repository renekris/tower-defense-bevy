use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;

// ============================================================================
// COMPONENTS
// ============================================================================

/// Component for towers to track their current target and shooting state
#[derive(Component, Default)]
pub struct Target {
    pub entity: Option<Entity>,  // Which enemy this tower is targeting
    pub last_shot_time: f32,     // For fire rate control
}

// Projectile component is now defined in components/projectile.rs

// ============================================================================
// RESOURCES  
// ============================================================================

/// Resource to track wave progress and completion
#[derive(Resource, Default)]
pub struct WaveStatus {
    pub enemies_remaining: u32,
    pub enemies_killed: u32,
    pub enemies_escaped: u32,
    pub wave_complete: bool,
}

impl WaveStatus {
    pub fn initialize_wave(&mut self, enemy_count: u32) {
        self.enemies_remaining = enemy_count;
        self.enemies_killed = 0;
        self.enemies_escaped = 0;
        self.wave_complete = false;
    }
}

// ============================================================================
// SYSTEMS
// ============================================================================

/// System 1: Tower Targeting - Find closest enemies in range
pub fn tower_targeting_system(
    mut towers: Query<(&mut Target, &TowerStats, &Transform), With<TowerStats>>,
    enemies: Query<(Entity, &Transform), (With<Enemy>, Without<TowerStats>)>,
) {
    for (mut target, stats, tower_transform) in towers.iter_mut() {
        let tower_pos = tower_transform.translation.truncate();
        
        // Find closest enemy within range
        let mut closest_enemy = None;
        let mut closest_distance = f32::MAX;
        
        for (enemy_entity, enemy_transform) in enemies.iter() {
            let enemy_pos = enemy_transform.translation.truncate();
            let distance = tower_pos.distance(enemy_pos);
            
            if distance <= stats.range && distance < closest_distance {
                closest_distance = distance;
                closest_enemy = Some(enemy_entity);
            }
        }
        
        target.entity = closest_enemy;
    }
}

/// System 2: Projectile Spawning - Fire at targeted enemies
pub fn projectile_spawning_system(
    mut commands: Commands,
    time: Res<Time>,
    mut towers: Query<(&mut Target, &TowerStats, &Transform)>,
    enemies: Query<&Transform, (With<Enemy>, Without<TowerStats>)>,
) {
    let current_time = time.elapsed_seconds();
    
    for (mut target, stats, tower_transform) in towers.iter_mut() {
        // Check if we can shoot (fire rate control)
        if current_time - target.last_shot_time < (1.0 / stats.fire_rate) {
            continue;
        }
        
        // Check if we have a valid target
        if let Some(target_entity) = target.entity {
            if let Ok(target_transform) = enemies.get(target_entity) {
                // Get projectile properties based on tower type
                let (projectile_speed, projectile_color) = match stats.tower_type {
                    TowerType::Basic => (300.0, Color::srgb(1.0, 1.0, 0.0)), // Yellow
                    TowerType::Advanced => (400.0, Color::srgb(0.0, 0.8, 1.0)), // Cyan
                    TowerType::Laser => (800.0, Color::srgb(1.0, 0.2, 0.2)), // Red
                    TowerType::Missile => (200.0, Color::srgb(1.0, 0.5, 0.0)), // Orange
                    TowerType::Tesla => (600.0, Color::srgb(0.8, 0.0, 1.0)), // Purple
                };
                
                // Spawn projectile
                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: projectile_color,
                            custom_size: Some(Vec2::new(6.0, 6.0)),
                            ..default()
                        },
                        transform: Transform::from_translation(tower_transform.translation),
                        ..default()
                    },
                    Projectile::new(
                        stats.damage,
                        projectile_speed,
                        target_entity,
                        target_transform.translation.truncate(),
                        stats.tower_type,
                    ),
                ));
                
                target.last_shot_time = current_time;
            }
        }
    }
}

/// System 3: Projectile Movement - Move projectiles toward targets
pub fn projectile_movement_system(
    mut commands: Commands,
    time: Res<Time>,
    mut projectiles: Query<(Entity, &mut Transform, &Projectile)>,
    enemies: Query<&Transform, (With<Enemy>, Without<Projectile>)>,
) {
    let delta_time = time.delta_seconds();
    
    for (projectile_entity, mut projectile_transform, projectile) in projectiles.iter_mut() {
        // Determine target position (lead the target if it still exists)
        let target_position = if let Ok(enemy_transform) = enemies.get(projectile.target_entity) {
            // Target still exists - lead it (aim for current position)
            enemy_transform.translation.truncate()
        } else {
            // Target destroyed - continue to last known position
            projectile.target_position
        };
        
        // Move projectile toward target
        let current_pos = projectile_transform.translation.truncate();
        let direction = (target_position - current_pos).normalize_or_zero();
        let movement = direction * projectile.speed * delta_time;
        
        projectile_transform.translation += movement.extend(0.0);
        
        // Remove projectile if it has traveled too far (missed target)
        let travel_distance = current_pos.distance(projectile.target_position);
        if travel_distance > 1000.0 {
            commands.entity(projectile_entity).despawn();
        }
    }
}

/// System 4: Collision Detection - Handle projectile hits and enemy damage
pub fn collision_system(
    mut commands: Commands,
    mut economy: ResMut<Economy>,
    mut wave_status: ResMut<WaveStatus>,
    debug_ui_state: Option<Res<crate::systems::debug_ui::DebugUIState>>,
    debug_state: Option<Res<crate::systems::debug_visualization::DebugVisualizationState>>,
    projectiles: Query<(Entity, &Transform, &Projectile)>,
    mut enemies: Query<(Entity, &Transform, &mut Health), With<Enemy>>,
) {
    for (projectile_entity, projectile_transform, projectile_data) in projectiles.iter() {
        for (enemy_entity, enemy_transform, mut enemy_health) in enemies.iter_mut() {
            // Simple circle collision detection
            let distance = projectile_transform.translation.truncate()
                .distance(enemy_transform.translation.truncate());
            
            if distance < 16.0 { // Collision threshold
                // Calculate effective damage with UI multiplier
                let damage_multiplier = if let (Some(ui_state), Some(debug_state)) = (&debug_ui_state, &debug_state) {
                    if debug_state.enabled {
                        ui_state.tower_damage_multiplier
                    } else {
                        1.0
                    }
                } else {
                    1.0
                };
                
                let effective_damage = projectile_data.damage * damage_multiplier;
                
                // Debug output for damage multiplier (only when different from 1.0)
                if damage_multiplier != 1.0 {
                    println!("Applied damage multiplier {:.2}: {:.1} -> {:.1} damage", 
                        damage_multiplier, projectile_data.damage, effective_damage);
                }
                
                // Apply damage to enemy
                enemy_health.take_damage(effective_damage);
                
                // Remove projectile (it hit something)
                commands.entity(projectile_entity).despawn();
                
                // Check if enemy died from damage
                if enemy_health.is_dead() {
                    // Award resources based on tower type (different towers give different rewards)
                    let money_reward = match projectile_data.tower_type {
                        TowerType::Basic => 5,
                        TowerType::Advanced => 8,
                        TowerType::Laser => 10,
                        TowerType::Missile => 12,
                        TowerType::Tesla => 15,
                    };
                    
                    economy.money += money_reward;
                    economy.research_points += 1;
                    
                    // Remove dead enemy
                    commands.entity(enemy_entity).despawn();
                    
                    // Update wave progress
                    wave_status.enemies_killed += 1;
                    wave_status.enemies_remaining = wave_status.enemies_remaining.saturating_sub(1);
                    
                    // Check if wave is complete
                    if wave_status.enemies_remaining == 0 {
                        wave_status.wave_complete = true;
                        println!("Wave complete! {} enemies eliminated", wave_status.enemies_killed);
                    }
                }
                
                break; // Projectile can only hit one enemy
            }
        }
    }
}

/// System 5: Game State Management - Handle win/lose conditions and wave progression
pub fn game_state_system(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut wave_status: ResMut<WaveStatus>,
    mut wave_manager: ResMut<WaveManager>,
    enemies: Query<(Entity, &Transform), With<Enemy>>,
    enemy_path: Res<EnemyPath>,
) {
    // Check for enemies that have reached the end of the path
    let mut enemies_to_remove = Vec::new();
    let mut new_escapes = 0;
    
    for (enemy_entity, enemy_transform) in enemies.iter() {
        let enemy_pos = enemy_transform.translation.truncate();
        if let Some(path_end) = enemy_path.waypoints.last() {
            if enemy_pos.distance(*path_end) < 32.0 {
                enemies_to_remove.push(enemy_entity);
                new_escapes += 1;
            }
        }
    }
    
    // Remove enemies that reached the end
    for enemy_entity in enemies_to_remove {
        commands.entity(enemy_entity).despawn();
    }
    
    // Update escape count
    wave_status.enemies_escaped += new_escapes;
    wave_status.enemies_remaining = wave_status.enemies_remaining.saturating_sub(new_escapes);
    
    if new_escapes > 0 {
        println!("{} enemies escaped! Total escapes: {}", new_escapes, wave_status.enemies_escaped);
    }
    
    // Check win condition: Wave complete and no more waves
    if wave_status.wave_complete && wave_manager.current_wave >= 3 { // 3 waves total
        *game_state = GameState::Victory;
        println!("🎉 VICTORY! All waves defended successfully!");
        return;
    }
    
    // Check lose condition: Too many enemies escaped  
    if wave_status.enemies_escaped >= 10 {
        *game_state = GameState::GameOver;
        println!("💀 GAME OVER! {} enemies reached the end!", wave_status.enemies_escaped);
        return;
    }
    
    // Auto-progress to next wave if current wave is complete
    if wave_status.wave_complete && wave_manager.current_wave < 3 {
        wave_manager.current_wave += 1;
        wave_status.initialize_wave(wave_manager.enemies_in_wave);
        println!("🚨 Wave {} incoming! Prepare your defenses!", wave_manager.current_wave);
    }
}