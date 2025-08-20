use tower_defense_bevy::components::Tower;

#[test]
fn test_tower_creation() {
    let tower = Tower::default();
    assert_eq!(tower.damage, 10.0);
    assert_eq!(tower.range, 100.0);
    assert_eq!(tower.fire_rate, 1.0);
    assert_eq!(tower.last_shot, 0.0);
}

#[test]
fn test_tower_custom_values() {
    let tower = Tower {
        damage: 25.0,
        range: 150.0,
        fire_rate: 2.0,
        last_shot: 5.0,
    };
    
    assert_eq!(tower.damage, 25.0);
    assert_eq!(tower.range, 150.0);
    assert_eq!(tower.fire_rate, 2.0);
    assert_eq!(tower.last_shot, 5.0);
}

#[test]
fn test_tower_can_shoot_logic() {
    let mut tower = Tower::default();
    let current_time = 2.0;
    let shot_cooldown = 1.0 / tower.fire_rate;
    
    // Should be able to shoot initially
    assert!(current_time - tower.last_shot >= shot_cooldown);
    
    // Update last shot time
    tower.last_shot = current_time;
    
    // Should not be able to shoot immediately after
    let next_check = current_time + 0.5;
    assert!(next_check - tower.last_shot < shot_cooldown);
    
    // Should be able to shoot after cooldown
    let after_cooldown = current_time + shot_cooldown + 0.1;
    assert!(after_cooldown - tower.last_shot >= shot_cooldown);
}