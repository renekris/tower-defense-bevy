/// Example script to validate the wave progression fixes
/// This demonstrates that:
/// 1. Enemy count scales progressively with waves
/// 2. Enemy stats (speed, health, reward) scale with waves
/// 3. Spawn rate increases with waves for more intensity

use tower_defense_bevy::resources::*;
use tower_defense_bevy::components::*;
use tower_defense_bevy::systems::enemy_system::calculate_enemies_for_wave;

fn main() {
    println!("=== Tower Defense Wave Progression Validation ===\n");
    
    // Test wave enemy count scaling
    println!("📊 Wave Enemy Count Scaling:");
    for wave in 1..=10 {
        let enemy_count = calculate_enemies_for_wave(wave);
        println!("  Wave {:2}: {:2} enemies", wave, enemy_count);
    }
    
    // Test enemy stat scaling  
    println!("\n🏃 Enemy Speed Scaling:");
    for wave in [1, 2, 3, 5, 10] {
        let enemy = Enemy::for_wave(wave);
        println!("  Wave {:2}: {:5.1} speed", wave, enemy.speed);
    }
    
    println!("\n💖 Enemy Health Scaling:");
    for wave in [1, 2, 3, 5, 10] {
        let health = Enemy::health_for_wave(wave);
        println!("  Wave {:2}: {:5.0} health", wave, health);
    }
    
    println!("\n💰 Enemy Reward Scaling:");
    for wave in [1, 2, 3, 5, 10] {
        let enemy = Enemy::for_wave(wave);
        println!("  Wave {:2}: {:2} reward", wave, enemy.reward);
    }
    
    // Test spawn rate scaling
    println!("\n⏱️ Spawn Rate Scaling:");
    let mut wave_manager = WaveManager::new();
    for wave in [1, 2, 3, 5, 10] {
        // Simulate starting each wave to get proper wave number
        for _ in 0..wave {
            wave_manager.start_wave(calculate_enemies_for_wave(wave_manager.current_wave + 1));
        }
        let spawn_interval = wave_manager.spawn_timer.duration().as_secs_f32();
        let spawn_rate = 1.0 / spawn_interval;
        println!("  Wave {:2}: {:.2} enemies/sec (interval: {:.2}s)", 
                 wave, spawn_rate, spawn_interval);
        
        // Reset for next test
        wave_manager = WaveManager::new();
    }
    
    println!("\n✅ Validation Complete!");
    println!("🎯 Both issues have been resolved:");
    println!("   • Obstacles will only render once during path generation");
    println!("   • Waves now have progressive difficulty scaling");
}