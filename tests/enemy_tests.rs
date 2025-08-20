use tower_defense_bevy::components::Enemy;

#[test]
fn test_enemy_creation() {
    let enemy = Enemy::default();
    assert_eq!(enemy.speed, 50.0);
    assert_eq!(enemy.path_index, 0);
    assert_eq!(enemy.reward, 10);
}

#[test]
fn test_enemy_custom_values() {
    let enemy = Enemy {
        speed: 75.0,
        path_index: 3,
        reward: 25,
    };
    
    assert_eq!(enemy.speed, 75.0);
    assert_eq!(enemy.path_index, 3);
    assert_eq!(enemy.reward, 25);
}

#[test]
fn test_enemy_path_progression() {
    let mut enemy = Enemy::default();
    assert_eq!(enemy.path_index, 0);
    
    // Simulate progressing along path
    enemy.path_index += 1;
    assert_eq!(enemy.path_index, 1);
    
    enemy.path_index += 1;
    assert_eq!(enemy.path_index, 2);
}

#[test]
fn test_enemy_speed_variants() {
    let slow_enemy = Enemy {
        speed: 25.0,
        ..Default::default()
    };
    
    let fast_enemy = Enemy {
        speed: 100.0,
        ..Default::default()
    };
    
    assert!(slow_enemy.speed < fast_enemy.speed);
    assert_eq!(slow_enemy.path_index, fast_enemy.path_index);
}