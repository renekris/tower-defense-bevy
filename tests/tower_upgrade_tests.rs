use tower_defense_bevy::components::*;
use tower_defense_bevy::resources::*;

// ============================================================================
// COMPONENT TESTS - Test upgrade level and stat progression
// ============================================================================

#[test]
fn test_tower_stats_initial_upgrade_level() {
    let tower = TowerStats::new(TowerType::Basic);
    assert_eq!(tower.upgrade_level, 1);
}

#[test]
fn test_tower_upgrade_level_increment() {
    let mut tower = TowerStats::new(TowerType::Basic);
    tower.upgrade();
    assert_eq!(tower.upgrade_level, 2);
    
    tower.upgrade();
    assert_eq!(tower.upgrade_level, 3);
}

#[test]
fn test_tower_max_upgrade_level() {
    let mut tower = TowerStats::new(TowerType::Basic);
    
    // Upgrade to max level (5)
    for _ in 1..5 {
        tower.upgrade();
    }
    assert_eq!(tower.upgrade_level, 5);
    
    // Should not upgrade beyond max level
    tower.upgrade();
    assert_eq!(tower.upgrade_level, 5);
}

#[test]
fn test_tower_can_upgrade() {
    let mut tower = TowerStats::new(TowerType::Basic);
    assert!(tower.can_upgrade());
    
    // Upgrade to max level
    for _ in 1..5 {
        tower.upgrade();
    }
    assert!(!tower.can_upgrade());
}

// ============================================================================
// STAT PROGRESSION TESTS - Test damage/range/fire_rate improvements
// ============================================================================

#[test]
fn test_basic_tower_stat_progression() {
    let mut tower = TowerStats::new(TowerType::Basic);
    let initial_damage = tower.damage;
    let initial_range = tower.range;
    let initial_fire_rate = tower.fire_rate;
    
    tower.upgrade();
    
    // Level 2 stats should be higher than level 1
    assert!(tower.damage > initial_damage);
    assert!(tower.range > initial_range);
    assert!(tower.fire_rate > initial_fire_rate);
}

#[test]
fn test_advanced_tower_stat_progression() {
    let mut tower = TowerStats::new(TowerType::Advanced);
    let initial_damage = tower.damage;
    
    tower.upgrade();
    tower.upgrade(); // Level 3
    
    // Should have significantly higher damage at level 3
    assert!(tower.damage > initial_damage * 1.4); // At least 40% increase
}

#[test]
fn test_laser_tower_specialization() {
    let mut tower = TowerStats::new(TowerType::Laser);
    let initial_fire_rate = tower.fire_rate;
    
    tower.upgrade();
    
    // Laser towers should get more fire rate improvement than others
    let fire_rate_increase = tower.fire_rate - initial_fire_rate;
    assert!(fire_rate_increase > 0.3); // Significant fire rate boost
}

#[test]
fn test_missile_tower_specialization() {
    let mut tower = TowerStats::new(TowerType::Missile);
    let initial_damage = tower.damage;
    
    tower.upgrade();
    
    // Missile towers should get more damage improvement than others
    let damage_increase = tower.damage - initial_damage;
    assert!(damage_increase > 8.0); // Significant damage boost
}

#[test]
fn test_tesla_tower_specialization() {
    let mut tower = TowerStats::new(TowerType::Tesla);
    let initial_range = tower.range;
    
    tower.upgrade();
    
    // Tesla towers should get more range improvement than others (rebalanced)
    let range_increase = tower.range - initial_range;
    assert!(range_increase > 10.0); // Significant range boost (reduced from 15.0 due to rebalancing)
}

// ============================================================================
// UPGRADE COST TESTS - Test cost calculation and scaling
// ============================================================================

#[test]
fn test_upgrade_cost_scaling() {
    let mut tower = TowerStats::new(TowerType::Basic);
    let level_1_cost = tower.get_upgrade_cost();
    
    tower.upgrade();
    let level_2_cost = tower.get_upgrade_cost();
    
    // Level 2 upgrade should cost more than level 1 upgrade
    assert!(level_2_cost.money > level_1_cost.money);
    assert!(level_2_cost.research_points >= level_1_cost.research_points);
}

#[test]
fn test_expensive_tower_upgrade_costs() {
    let tesla_tower = TowerStats::new(TowerType::Tesla);
    let basic_tower = TowerStats::new(TowerType::Basic);
    
    let tesla_cost = tesla_tower.get_upgrade_cost();
    let basic_cost = basic_tower.get_upgrade_cost();
    
    // Tesla tower upgrades should be more expensive than basic tower upgrades
    assert!(tesla_cost.money > basic_cost.money);
    assert!(tesla_cost.energy > basic_cost.energy);
}

#[test]
fn test_upgrade_cost_different_levels() {
    let mut tower = TowerStats::new(TowerType::Advanced);
    
    let level_1_cost = tower.get_upgrade_cost();
    
    tower.upgrade(); // Level 2
    let level_2_cost = tower.get_upgrade_cost();
    
    tower.upgrade(); // Level 3  
    let level_3_cost = tower.get_upgrade_cost();
    
    // Each upgrade should cost more than the previous
    assert!(level_2_cost.money > level_1_cost.money);
    assert!(level_3_cost.money > level_2_cost.money);
}

// ============================================================================
// ECONOMY INTEGRATION TESTS - Test affordability and spending
// ============================================================================

#[test]
fn test_can_afford_basic_tower_upgrade() {
    let mut economy = Economy::new(100, 10, 10, 50);
    let tower = TowerStats::new(TowerType::Basic);
    let upgrade_cost = tower.get_upgrade_cost();
    
    assert!(economy.can_afford(&upgrade_cost));
}

#[test]
fn test_cannot_afford_expensive_upgrade() {
    let mut economy = Economy::new(10, 0, 0, 5);
    let tower = TowerStats::new(TowerType::Tesla);
    let upgrade_cost = tower.get_upgrade_cost();
    
    assert!(!economy.can_afford(&upgrade_cost));
}

#[test]
fn test_successful_upgrade_transaction() {
    let mut economy = Economy::new(200, 20, 20, 100);
    let mut tower = TowerStats::new(TowerType::Advanced);
    let upgrade_cost = tower.get_upgrade_cost();
    
    let initial_money = economy.money;
    let initial_level = tower.upgrade_level;
    let initial_damage = tower.damage;
    
    assert!(economy.can_afford(&upgrade_cost));
    economy.spend(&upgrade_cost);
    tower.upgrade();
    
    // Check that money was spent
    assert!(economy.money < initial_money);
    
    // Check that tower was upgraded
    assert_eq!(tower.upgrade_level, initial_level + 1);
    assert!(tower.damage > initial_damage);
}

// ============================================================================
// UI INTEGRATION TESTS - Test upgrade interaction logic
// ============================================================================

#[test]
fn test_upgrade_button_availability() {
    let economy = Economy::new(50, 5, 5, 25);
    let tower = TowerStats::new(TowerType::Basic);
    
    // Should be able to upgrade if we can afford it and tower can be upgraded
    let can_upgrade = economy.can_afford(&tower.get_upgrade_cost()) && tower.can_upgrade();
    assert!(can_upgrade);
}

#[test]
fn test_upgrade_button_disabled_insufficient_funds() {
    let economy = Economy::new(10, 0, 0, 0);
    let tower = TowerStats::new(TowerType::Tesla);
    
    // Should not be able to upgrade if we can't afford it
    let can_upgrade = economy.can_afford(&tower.get_upgrade_cost()) && tower.can_upgrade();
    assert!(!can_upgrade);
}

#[test]
fn test_upgrade_button_disabled_max_level() {
    let economy = Economy::new(1000, 100, 100, 100);
    let mut tower = TowerStats::new(TowerType::Basic);
    
    // Upgrade to max level
    for _ in 1..5 {
        tower.upgrade();
    }
    
    // Should not be able to upgrade even with money
    let can_upgrade = economy.can_afford(&tower.get_upgrade_cost()) && tower.can_upgrade();
    assert!(!can_upgrade);
}

// ============================================================================
// PERFORMANCE TESTS - Test upgrade impact on game balance
// ============================================================================

#[test]
fn test_upgrade_value_efficiency() {
    let mut tower = TowerStats::new(TowerType::Basic);
    let initial_dps = tower.damage * tower.fire_rate;
    let upgrade_cost = tower.get_upgrade_cost();
    
    tower.upgrade();
    let upgraded_dps = tower.damage * tower.fire_rate;
    
    let dps_increase = upgraded_dps - initial_dps;
    let cost_per_dps = upgrade_cost.money as f32 / dps_increase;
    
    // Upgrades should provide reasonable value (not too expensive per DPS)
    assert!(cost_per_dps < 10.0);
}

#[test]
fn test_tower_type_upgrade_balance() {
    // Test that all tower types have reasonable upgrade scaling
    let tower_types = [
        TowerType::Basic,
        TowerType::Advanced, 
        TowerType::Laser,
        TowerType::Missile,
        TowerType::Tesla,
    ];
    
    for tower_type in tower_types {
        let mut tower = TowerStats::new(tower_type);
        let initial_dps = tower.damage * tower.fire_rate;
        
        tower.upgrade();
        let upgraded_dps = tower.damage * tower.fire_rate;
        
        // Each tower type should get meaningful but balanced improvement (current system)
        let improvement_ratio = upgraded_dps / initial_dps;
        assert!(improvement_ratio > 1.15, "Tower type {:?} upgrade too weak", tower_type);
        assert!(improvement_ratio < 2.50, "Tower type {:?} upgrade too strong", tower_type); // Adjusted for current system balance
    }
}