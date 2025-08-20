use tower_defense_bevy::components::Health;

#[test]
fn test_health_creation() {
    let health = Health::new(100.0);
    assert_eq!(health.current, 100.0);
    assert_eq!(health.max, 100.0);
}

#[test]
fn test_health_default() {
    let health = Health::default();
    assert_eq!(health.current, 100.0);
    assert_eq!(health.max, 100.0);
}

#[test]
fn test_take_damage() {
    let mut health = Health::new(100.0);
    health.take_damage(20.0);
    assert_eq!(health.current, 80.0);
    assert_eq!(health.max, 100.0);
}

#[test]
fn test_take_damage_beyond_zero() {
    let mut health = Health::new(50.0);
    health.take_damage(75.0);
    assert_eq!(health.current, 0.0);
    assert!(health.current >= 0.0);
}

#[test]
fn test_is_dead() {
    let mut health = Health::new(10.0);
    assert!(!health.is_dead());
    
    health.take_damage(10.0);
    assert!(health.is_dead());
}

#[test]
fn test_heal() {
    let mut health = Health::new(100.0);
    health.take_damage(30.0);
    assert_eq!(health.current, 70.0);
    
    health.heal(15.0);
    assert_eq!(health.current, 85.0);
}

#[test]
fn test_heal_beyond_max() {
    let mut health = Health::new(100.0);
    health.take_damage(20.0);
    
    health.heal(50.0);
    assert_eq!(health.current, 100.0);
    assert!(health.current <= health.max);
}

#[test]
fn test_heal_when_full() {
    let mut health = Health::new(100.0);
    health.heal(20.0);
    assert_eq!(health.current, 100.0);
}