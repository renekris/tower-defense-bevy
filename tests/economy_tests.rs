use tower_defense_bevy::resources::*;

#[test]
fn test_economy_creation() {
    let economy = Economy::default();
    assert_eq!(economy.money, 100);
    assert_eq!(economy.research_points, 0);
    assert_eq!(economy.materials, 10);
    assert_eq!(economy.energy, 50);
}

#[test]
fn test_economy_spend_money() {
    let mut economy = Economy::default();
    let cost = ResourceCost::money(50);
    
    assert!(economy.can_afford(&cost));
    economy.spend(&cost);
    assert_eq!(economy.money, 50);
}

#[test]
fn test_economy_insufficient_funds() {
    let mut economy = Economy::default();
    let cost = ResourceCost::money(150);
    
    assert!(!economy.can_afford(&cost));
    economy.spend(&cost);
    assert_eq!(economy.money, 100); // Should not change
}

#[test]
fn test_economy_multi_resource_cost() {
    let mut economy = Economy::new(100, 10, 15, 50);
    let cost = ResourceCost::new(30, 5, 2, 10);
    
    assert!(economy.can_afford(&cost));
    economy.spend(&cost);
    assert_eq!(economy.money, 70);
    assert_eq!(economy.research_points, 5);
    assert_eq!(economy.materials, 13);
    assert_eq!(economy.energy, 40);
}

#[test]
fn test_economy_earn_resources() {
    let mut economy = Economy::default();
    let reward = ResourceReward::new(25, 3, 1, 5);
    
    economy.earn(&reward);
    assert_eq!(economy.money, 125);
    assert_eq!(economy.research_points, 3);
    assert_eq!(economy.materials, 11);
    assert_eq!(economy.energy, 55);
}

#[test]
fn test_economy_resource_generation_over_time() {
    let mut economy = Economy::default();
    
    economy.generate_passive_income(1.0); // 1 second passed
    
    // Default passive generation rates
    assert_eq!(economy.money, 102); // +2 per second
    assert_eq!(economy.research_points, 1); // +1 per second
    assert_eq!(economy.materials, 10); // No passive generation
    assert_eq!(economy.energy, 55); // +5 per second
}

#[test]
fn test_tower_costs_balance() {
    let basic_tower = TowerType::Basic.get_cost();
    let advanced_tower = TowerType::Advanced.get_cost();
    let laser_tower = TowerType::Laser.get_cost();
    
    // Basic tower should be cheapest
    assert!(basic_tower.money < advanced_tower.money);
    assert!(basic_tower.money < laser_tower.money);
    
    // Laser tower should require research points
    assert!(laser_tower.research_points > 0);
    
    // Advanced tower should require materials
    assert!(advanced_tower.materials > 0);
}