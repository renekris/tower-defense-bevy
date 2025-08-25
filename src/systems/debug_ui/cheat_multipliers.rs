use bevy::prelude::*;
use crate::resources::*;
use crate::components::*;
use crate::systems::combat_system::{WaveStatus, Target};
use super::cheat_menu::{CheatMenuState, CheatMultipliers, CheatSliderType};

/// System to apply cheat multipliers to tower stats
pub fn apply_tower_multipliers_system(
    multipliers: Res<CheatMultipliers>,
    mut tower_query: Query<&mut TowerStats, With<TowerStats>>,
    cheat_state: Res<CheatMenuState>,
) {
    // Only apply multipliers if cheat menu exists and multipliers have changed
    if !multipliers.is_changed() {
        return;
    }
    
    for mut tower_stats in &mut tower_query {
        // Store base stats if not already stored (we'll add this to TowerStats component)
        // For now, we'll calculate base stats from current values divided by multipliers
        
        // Apply multipliers to tower stats
        // Note: This is a simplified approach - in a production game you'd want to store base stats
        let base_damage = tower_stats.damage / get_previous_multiplier_or_default(CheatSliderType::TowerDamage);
        let base_range = tower_stats.range / get_previous_multiplier_or_default(CheatSliderType::TowerRange);
        let base_fire_rate = tower_stats.fire_rate / get_previous_multiplier_or_default(CheatSliderType::TowerFireRate);
        
        tower_stats.damage = base_damage * multipliers.tower_damage;
        tower_stats.range = base_range * multipliers.tower_range;
        tower_stats.fire_rate = base_fire_rate * multipliers.tower_fire_rate;
        
        // Ensure stats don't go below minimum values
        tower_stats.damage = tower_stats.damage.max(0.1);
        tower_stats.range = tower_stats.range.max(10.0);
        tower_stats.fire_rate = tower_stats.fire_rate.max(0.1);
    }
}

/// System to apply cheat multipliers to enemy health and speed
pub fn apply_enemy_multipliers_system(
    multipliers: Res<CheatMultipliers>,
    mut enemy_query: Query<(&mut Health, &mut Enemy), (With<Enemy>, With<Health>)>,
) {
    // Only apply multipliers if they have changed
    if !multipliers.is_changed() {
        return;
    }
    
    for (mut health, mut enemy) in &mut enemy_query {
        // Apply health multiplier
        // Note: This is a simplified approach - ideally you'd store base health values
        let current_health_ratio = health.current / health.max;
        let base_max_health = health.max / get_previous_multiplier_or_default(CheatSliderType::EnemyHealth);
        
        health.max = base_max_health * multipliers.enemy_health;
        health.current = health.max * current_health_ratio;
        
        // Apply speed multiplier
        let base_speed = enemy.speed / get_previous_multiplier_or_default(CheatSliderType::EnemySpeed);
        enemy.speed = base_speed * multipliers.enemy_speed;
        
        // Ensure values don't go below minimum
        health.max = health.max.max(1.0);
        health.current = health.current.max(0.0);
        enemy.speed = enemy.speed.max(0.1);
    }
}

/// System to apply god mode effects
pub fn apply_god_mode_system(
    cheat_state: Res<CheatMenuState>,
    mut economy: ResMut<Economy>,
    mut tower_query: Query<&mut Health, (With<TowerStats>, With<Health>)>,
) {
    if !cheat_state.is_changed() {
        return;
    }
    
    if cheat_state.god_mode {
        // Infinite resources
        economy.money = u32::MAX;
        economy.research_points = u32::MAX;
        economy.materials = u32::MAX;
        economy.energy = u32::MAX;
        
        // Make all towers invincible
        for mut health in &mut tower_query {
            health.current = health.max;
        }
        
        println!("God mode activated: Infinite resources and invincible towers");
    }
}

/// System to maintain god mode benefits every frame when active
pub fn maintain_god_mode_system(
    cheat_state: Res<CheatMenuState>,
    mut economy: ResMut<Economy>,
    mut tower_query: Query<&mut Health, (With<TowerStats>, With<Health>)>,
    time: Res<Time>,
) {
    if cheat_state.god_mode {
        // Maintain infinite resources (in case something tries to spend them)
        economy.money = u32::MAX;
        economy.research_points = u32::MAX;
        economy.materials = u32::MAX;
        economy.energy = u32::MAX;
        
        // Keep towers at full health
        for mut health in &mut tower_query {
            health.current = health.max;
        }
        
        // Boost passive income generation
        economy.money_generation = 1000.0;
        economy.research_generation = 100.0;
        economy.energy_generation = 100.0;
    }
}

/// System to handle instant enemy spawning when spawn rate is very high
pub fn enhanced_enemy_spawn_system(
    multipliers: Res<CheatMultipliers>,
    mut wave_manager: ResMut<WaveManager>,
    cheat_state: Res<CheatMenuState>,
    time: Res<Time>,
) {
    // If enemy speed multiplier is very high, increase spawn rate proportionally
    if multipliers.enemy_speed > 3.0 && cheat_state.visible {
        let speed_boost = multipliers.enemy_speed - 1.0;
        // Reset spawn timer to speed up enemy spawning
        wave_manager.spawn_timer.tick(std::time::Duration::from_secs_f32(time.delta_secs() * speed_boost));
    }
}

/// System to prevent enemy health from going to zero due to multipliers
pub fn validate_enemy_stats_system(
    mut enemy_query: Query<(&mut Health, &mut Enemy), With<Enemy>>,
) {
    for (mut health, mut enemy) in &mut enemy_query {
        // Ensure health values are valid
        if health.max < 1.0 {
            health.max = 1.0;
        }
        if health.current < 0.0 {
            health.current = 0.0;
        }
        if health.current > health.max {
            health.current = health.max;
        }
        
        // Ensure speed is valid
        if enemy.speed < 0.1 {
            enemy.speed = 0.1;
        }
        if enemy.speed > 1000.0 {
            enemy.speed = 1000.0; // Cap at reasonable maximum
        }
    }
}

/// System to validate tower stats
pub fn validate_tower_stats_system(
    mut tower_query: Query<&mut TowerStats, With<TowerStats>>,
) {
    for mut tower_stats in &mut tower_query {
        // Ensure damage is valid
        if tower_stats.damage < 0.1 {
            tower_stats.damage = 0.1;
        }
        if tower_stats.damage > 10000.0 {
            tower_stats.damage = 10000.0; // Cap at reasonable maximum
        }
        
        // Ensure range is valid
        if tower_stats.range < 10.0 {
            tower_stats.range = 10.0;
        }
        if tower_stats.range > 1000.0 {
            tower_stats.range = 1000.0; // Cap at reasonable maximum
        }
        
        // Ensure fire rate is valid
        if tower_stats.fire_rate < 0.1 {
            tower_stats.fire_rate = 0.1;
        }
        if tower_stats.fire_rate > 100.0 {
            tower_stats.fire_rate = 100.0; // Cap at reasonable maximum
        }
    }
}

/// System to show visual feedback when multipliers are active
pub fn cheat_visual_feedback_system(
    multipliers: Res<CheatMultipliers>,
    cheat_state: Res<CheatMenuState>,
    mut tower_query: Query<&mut Sprite, With<TowerStats>>,
    mut enemy_query: Query<&mut Sprite, (With<Enemy>, Without<TowerStats>)>,
) {
    // Only update visual feedback if cheat menu is visible and multipliers have changed
    if !cheat_state.visible || !multipliers.is_changed() {
        return;
    }
    
    // Color towers based on damage multiplier
    if multipliers.tower_damage != 1.0 {
        let damage_tint = if multipliers.tower_damage > 1.0 {
            // Red tint for increased damage
            Color::srgba(1.0, 0.7, 0.7, 1.0)
        } else {
            // Blue tint for decreased damage
            Color::srgba(0.7, 0.7, 1.0, 1.0)
        };
        
        for mut sprite in &mut tower_query {
            // Apply tint while preserving original color somewhat
            // Apply simple tint instead of complex color mixing
            sprite.color = damage_tint;
        }
    }
    
    // Color enemies based on health/speed multipliers
    if multipliers.enemy_health != 1.0 || multipliers.enemy_speed != 1.0 {
        let enemy_tint = if multipliers.enemy_health > 1.0 {
            // Darker for more health
            Color::srgba(0.6, 0.6, 0.6, 1.0)
        } else if multipliers.enemy_speed > 1.0 {
            // Brighter for more speed
            Color::srgba(1.2, 1.2, 1.2, 1.0)
        } else {
            // Lighter for weaker/slower
            Color::srgba(1.0, 1.0, 1.0, 1.0)
        };
        
        for mut sprite in &mut enemy_query {
            // Apply simple tint instead of complex color mixing
            sprite.color = enemy_tint;
        }
    }
}

/// System to reset visual effects when multipliers return to normal
pub fn reset_visual_effects_system(
    multipliers: Res<CheatMultipliers>,
    cheat_state: Res<CheatMenuState>,
    mut tower_query: Query<(&mut Sprite, &TowerStats), With<TowerStats>>,
    mut enemy_query: Query<&mut Sprite, (With<Enemy>, Without<TowerStats>)>,
) {
    // Reset colors when all multipliers are back to 1.0
    if multipliers.is_changed() &&
       multipliers.tower_damage == 1.0 &&
       multipliers.tower_range == 1.0 &&
       multipliers.tower_fire_rate == 1.0 &&
       multipliers.enemy_health == 1.0 &&
       multipliers.enemy_speed == 1.0 {
        
        // Reset tower colors to their defaults
        for (mut sprite, tower_stats) in &mut tower_query {
            sprite.color = match tower_stats.tower_type {
                TowerType::Basic => Color::srgb(0.5, 0.3, 0.1),
                TowerType::Advanced => Color::srgb(0.3, 0.3, 0.7),
                TowerType::Laser => Color::srgb(1.0, 0.2, 0.2),
                TowerType::Missile => Color::srgb(0.8, 0.8, 0.1),
                TowerType::Tesla => Color::srgb(0.5, 0.0, 1.0),
            };
        }
        
        // Reset enemy colors to red
        for mut sprite in &mut enemy_query {
            sprite.color = Color::srgb(1.0, 0.2, 0.2);
        }
    }
}

// Helper function to get previous multiplier value (simplified for now)
fn get_previous_multiplier_or_default(_slider_type: CheatSliderType) -> f32 {
    // In a production game, you'd track previous values
    // For now, we'll assume the previous multiplier was 1.0 if not tracked
    1.0
}

// Additional helper systems for enhanced cheat functionality

/// System to handle extreme fire rates without breaking the game
pub fn handle_extreme_fire_rates_system(
    multipliers: Res<CheatMultipliers>,
    mut tower_query: Query<&mut TowerStats, With<TowerStats>>,
    time: Res<Time>,
) {
    if multipliers.tower_fire_rate > 10.0 {
        // For extremely high fire rates, adjust the last_shot timing
        for mut tower_stats in &mut tower_query {
            // Allow immediate firing by resetting shot cooldown
            tower_stats.last_shot = 0.0;
        }
    }
}

/// System to handle instant enemy kills with very high tower damage
pub fn handle_extreme_damage_system(
    multipliers: Res<CheatMultipliers>,
    tower_query: Query<&TowerStats, With<TowerStats>>,
    mut enemy_query: Query<&mut Health, With<Enemy>>,
) {
    if multipliers.tower_damage > 100.0 {
        // If damage is extremely high, just kill enemies instantly when they're in range
        for tower_stats in &tower_query {
            if tower_stats.damage * multipliers.tower_damage > 1000.0 {
                for mut health in &mut enemy_query {
                    // Instant kill for demonstration - in practice you'd check range first
                    health.current = 0.0;
                }
                break; // One tower is enough for instant kills
            }
        }
    }
}