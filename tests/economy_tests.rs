use tower_defense_bevy::resources::*;

#[test]
fn test_economy_creation() {
    let economy = Economy::default();
    assert_eq!(economy.money, 50);      // Rebalanced from 100
    assert_eq!(economy.research_points, 0);
    assert_eq!(economy.materials, 5);       // Rebalanced from 10
    assert_eq!(economy.energy, 30);         // Rebalanced from 50
}

#[test]
fn test_economy_spend_money() {
    let mut economy = Economy::default();
    let cost = ResourceCost::money(50);
    
    assert!(economy.can_afford(&cost));
    economy.spend(&cost);
    assert_eq!(economy.money, 0);           // 50 - 50 = 0
}

#[test]
fn test_economy_insufficient_funds() {
    let mut economy = Economy::default();
    let cost = ResourceCost::money(150);
    
    assert!(!economy.can_afford(&cost));
    economy.spend(&cost);
    assert_eq!(economy.money, 50);  // Should not change from new starting value
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
    assert_eq!(economy.money, 75);          // 50 + 25 = 75
    assert_eq!(economy.research_points, 3);
    assert_eq!(economy.materials, 6);       // 5 + 1 = 6
    assert_eq!(economy.energy, 35);         // 30 + 5 = 35
}

#[test]
fn test_economy_resource_generation_over_time() {
    let mut economy = Economy::default();
    
    economy.generate_passive_income(1.0); // 1 second passed
    
    // Rebalanced passive generation rates
    assert_eq!(economy.money, 50);          // 50 + 0.5 = 50 (rounded down)
    assert_eq!(economy.research_points, 0); // 0 + 0.3 = 0 (rounded down)
    assert_eq!(economy.materials, 5);       // No passive generation
    assert_eq!(economy.energy, 32);         // 30 + 2.0 = 32
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