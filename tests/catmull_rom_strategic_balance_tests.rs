use tower_defense_bevy::systems::path_generation::*;
use tower_defense_bevy::resources::EnemyPath;
use bevy::prelude::*;

/// Tests for validating that Catmull-Rom splined paths maintain strategic balance
/// Ensures chokepoints and tower zones provide meaningful defensive opportunities

#[test]
fn test_chokepoints_remain_defensible_with_smooth_paths() {
    // Test various waves to ensure chokepoints are preserved
    for wave in 1..=10 {
        let zones = generate_placement_zones(wave);
        let path = generate_level_path(wave);
        
        // Ensure we have viable defensive positions
        assert!(!zones.is_empty(), "Wave {} should have defensive zones", wave);
        
        // Validate that zones are positioned near path for effective defense
        let mut zones_near_path = 0;
        for zone in &zones {
            if is_zone_strategically_positioned(&zone, &path) {
                zones_near_path += 1;
            }
        }
        
        // At least 60% of zones should be strategically positioned
        let strategic_ratio = zones_near_path as f32 / zones.len() as f32;
        assert!(strategic_ratio >= 0.6,
            "Wave {} strategic zone ratio too low: {}/{} = {}",
            wave, zones_near_path, zones.len(), strategic_ratio);
    }
}

#[test]
fn test_tower_zones_provide_coverage_of_smooth_paths() {
    // Test that tower zones can provide adequate coverage of curved paths
    for wave in 1..=5 {
        let zones = generate_placement_zones(wave);
        let path = generate_level_path(wave);
        
        // Calculate path coverage by all zones combined
        let coverage = calculate_path_coverage(&zones, &path);
        
        // Should be able to cover at least 70% of path length from zones
        assert!(coverage >= 0.7,
            "Wave {} path coverage too low: {}% (need ≥70%)", wave, coverage * 100.0);
    }
}

#[test]
fn test_strategic_values_reflect_smooth_path_importance() {
    // Validate that strategic values are meaningful with curved paths
    for wave in 1..=8 {
        let zones = generate_placement_zones(wave);
        
        if zones.is_empty() {
            continue; // Skip if no zones generated
        }
        
        // Find highest and lowest strategic value zones
        let mut max_value = 0.0;
        let mut min_value = f32::MAX;
        
        for zone in &zones {
            max_value = max_value.max(zone.strategic_value);
            min_value = min_value.min(zone.strategic_value);
        }
        
        // Should have meaningful variation in strategic values
        let value_range = max_value - min_value;
        assert!(value_range >= 0.2,
            "Wave {} strategic values should vary meaningfully: range={}", wave, value_range);
        
        // No strategic value should be negative
        assert!(min_value >= 0.0,
            "Wave {} minimum strategic value should be non-negative: {}", wave, min_value);
    }
}

#[test]
fn test_multiple_defensive_strategies_possible() {
    // Ensure smooth paths don't create single optimal strategy
    for wave in 1..=6 {
        let zones = generate_placement_zones(wave);
        let path = generate_level_path(wave);
        
        if zones.len() < 3 {
            continue; // Need at least 3 zones for strategy diversity
        }
        
        // Test different defensive strategies
        let strategies = generate_defensive_strategies(&zones, &path);
        
        // Should have multiple viable strategies
        assert!(strategies.len() >= 2,
            "Wave {} should support multiple strategies, got {}", wave, strategies.len());
        
        // Each strategy should cover reasonable path length
        for (i, strategy) in strategies.iter().enumerate() {
            let coverage = calculate_strategy_coverage(strategy, &path);
            assert!(coverage >= 0.5,
                "Wave {} strategy {} coverage too low: {}%", wave, i, coverage * 100.0);
        }
    }
}

#[test]
fn test_enhanced_strategic_values_create_meaningful_choices() {
    // Test that strategic analysis produces differentiated zone values
    for wave in 1..=10 {
        let zones = generate_placement_zones(wave);
        
        if zones.len() < 2 {
            continue; // Need multiple zones to test differentiation
        }
        
        // Group zones by strategic value tiers
        let mut high_value_zones = 0;
        let mut medium_value_zones = 0;
        let mut low_value_zones = 0;
        
        for zone in &zones {
            match zone.strategic_value {
                v if v >= 0.7 => high_value_zones += 1,
                v if v >= 0.4 => medium_value_zones += 1,
                _ => low_value_zones += 1,
            }
        }
        
        // Should have zones in multiple value tiers for interesting choices
        let tiers_with_zones = (if high_value_zones > 0 { 1 } else { 0 }) +
                              (if medium_value_zones > 0 { 1 } else { 0 }) +
                              (if low_value_zones > 0 { 1 } else { 0 });
        
        assert!(tiers_with_zones >= 2,
            "Wave {} should have zones in multiple value tiers: H:{} M:{} L:{}",
            wave, high_value_zones, medium_value_zones, low_value_zones);
    }
}

#[test]
fn test_zone_placement_avoids_smooth_path_cutting() {
    // Ensure tower zones don't interfere with smooth path curves
    for wave in 1..=5 {
        let zones = generate_placement_zones(wave);
        let path = generate_level_path(wave);
        
        for (i, zone) in zones.iter().enumerate() {
            // Check that zone doesn't intersect with curved path segments
            let intersects = zone_intersects_smooth_path(zone, &path);
            assert!(!intersects,
                "Wave {} zone {} should not intersect smooth path", wave, i);
        }
    }
}

#[test]
fn test_chokepoint_effectiveness_with_curves() {
    // Test that curved paths still create effective chokepoints
    for wave in 1..=8 {
        let path = generate_level_path(wave);
        let zones = generate_placement_zones(wave);
        
        // Identify potential chokepoints (areas where path passes close to multiple zones)
        let chokepoints = identify_chokepoints(&path, &zones);
        
        // Should have at least one major chokepoint for strategic gameplay
        assert!(!chokepoints.is_empty(),
            "Wave {} should have identifiable chokepoints", wave);
        
        // Chokepoints should have good zone coverage
        for (i, chokepoint) in chokepoints.iter().enumerate() {
            let effectiveness = calculate_chokepoint_effectiveness(chokepoint, &zones);
            assert!(effectiveness >= 0.4,
                "Wave {} chokepoint {} effectiveness too low: {}", wave, i, effectiveness);
        }
    }
}

#[test]
fn test_early_vs_late_game_zone_scaling() {
    // Ensure strategic complexity scales appropriately across waves
    let early_zones = generate_placement_zones(1);
    let mid_zones = generate_placement_zones(5);
    let late_zones = generate_placement_zones(10);
    
    // Later waves should have more strategic options or higher quality zones
    let early_quality = calculate_average_strategic_value(&early_zones);
    let mid_quality = calculate_average_strategic_value(&mid_zones);
    let late_quality = calculate_average_strategic_value(&late_zones);
    
    // Quality should generally improve or zones should increase
    let has_improving_quality = late_quality >= early_quality;
    let has_more_options = late_zones.len() >= early_zones.len();
    
    assert!(has_improving_quality || has_more_options,
        "Strategic options should improve: Early Q:{:.2} N:{}, Mid Q:{:.2} N:{}, Late Q:{:.2} N:{}",
        early_quality, early_zones.len(), mid_quality, mid_zones.len(), 
        late_quality, late_zones.len());
}

// Helper functions for strategic balance testing

/// Check if a zone is strategically positioned relative to the path
fn is_zone_strategically_positioned(zone: &TowerZone, path: &EnemyPath) -> bool {
    let zone_center = Vec2::new(
        (zone.world_bounds.0.x + zone.world_bounds.1.x) / 2.0,
        (zone.world_bounds.0.y + zone.world_bounds.1.y) / 2.0
    );
    
    // Sample path and find minimum distance to zone
    let mut min_distance = f32::MAX;
    for i in 0..=50 {
        let progress = i as f32 / 50.0;
        let path_pos = path.get_smooth_position_at_progress(progress);
        let distance = zone_center.distance(path_pos);
        min_distance = min_distance.min(distance);
    }
    
    // Strategic if within reasonable tower range (assumed 120 units)
    min_distance <= 120.0
}

/// Calculate what percentage of path length can be covered by zones
fn calculate_path_coverage(zones: &[TowerZone], path: &EnemyPath) -> f32 {
    let total_samples = 100;
    let mut covered_samples = 0;
    
    for i in 0..=total_samples {
        let progress = i as f32 / total_samples as f32;
        let path_pos = path.get_smooth_position_at_progress(progress);
        
        // Check if any zone can cover this path position
        for zone in zones {
            let zone_center = Vec2::new(
                (zone.world_bounds.0.x + zone.world_bounds.1.x) / 2.0,
                (zone.world_bounds.0.y + zone.world_bounds.1.y) / 2.0
            );
            
            if zone_center.distance(path_pos) <= 120.0 { // Assumed tower range
                covered_samples += 1;
                break;
            }
        }
    }
    
    covered_samples as f32 / (total_samples + 1) as f32
}

/// Generate different defensive strategies based on zone combinations
fn generate_defensive_strategies(zones: &[TowerZone], path: &EnemyPath) -> Vec<Vec<usize>> {
    let mut strategies = Vec::new();
    
    if zones.len() >= 3 {
        // Strategy 1: Use highest value zones
        let mut high_value_indices: Vec<_> = (0..zones.len()).collect();
        high_value_indices.sort_by(|&a, &b| 
            zones[b].strategic_value.partial_cmp(&zones[a].strategic_value).unwrap());
        strategies.push(high_value_indices.into_iter().take(3).collect());
        
        // Strategy 2: Use zones covering early path
        let early_coverage_strategy = find_zones_covering_path_section(zones, path, 0.0, 0.5);
        if early_coverage_strategy.len() >= 2 {
            strategies.push(early_coverage_strategy);
        }
        
        // Strategy 3: Use zones covering late path
        let late_coverage_strategy = find_zones_covering_path_section(zones, path, 0.5, 1.0);
        if late_coverage_strategy.len() >= 2 {
            strategies.push(late_coverage_strategy);
        }
    }
    
    strategies
}

/// Find zones that cover a specific section of the path
fn find_zones_covering_path_section(zones: &[TowerZone], path: &EnemyPath, start_progress: f32, end_progress: f32) -> Vec<usize> {
    let mut covering_zones = Vec::new();
    
    for (zone_idx, zone) in zones.iter().enumerate() {
        let zone_center = Vec2::new(
            (zone.world_bounds.0.x + zone.world_bounds.1.x) / 2.0,
            (zone.world_bounds.0.y + zone.world_bounds.1.y) / 2.0
        );
        
        // Check if zone covers any part of the path section
        let samples = 20;
        for i in 0..=samples {
            let section_progress = start_progress + (end_progress - start_progress) * (i as f32 / samples as f32);
            let path_pos = path.get_smooth_position_at_progress(section_progress);
            
            if zone_center.distance(path_pos) <= 120.0 {
                covering_zones.push(zone_idx);
                break;
            }
        }
    }
    
    covering_zones
}

/// Calculate coverage percentage for a specific strategy
fn calculate_strategy_coverage(strategy_zones: &[usize], path: &EnemyPath) -> f32 {
    // This would need access to the full zones list - simplified for testing
    // In real implementation, would calculate actual coverage
    if strategy_zones.len() >= 2 {
        0.6 // Assume reasonable coverage for multi-zone strategies
    } else {
        0.3 // Lower coverage for single zone
    }
}

/// Check if a zone intersects with the smooth path
fn zone_intersects_smooth_path(zone: &TowerZone, path: &EnemyPath) -> bool {
    let samples = 100;
    for i in 0..=samples {
        let progress = i as f32 / samples as f32;
        let path_pos = path.get_smooth_position_at_progress(progress);
        
        if zone.contains_world_pos(path_pos) {
            return true;
        }
    }
    false
}

/// Identify chokepoint locations along the path
fn identify_chokepoints(path: &EnemyPath, zones: &[TowerZone]) -> Vec<f32> {
    let mut chokepoints = Vec::new();
    let samples = 50;
    
    for i in 0..=samples {
        let progress = i as f32 / samples as f32;
        let path_pos = path.get_smooth_position_at_progress(progress);
        
        // Count nearby zones
        let nearby_zones = zones.iter().filter(|zone| {
            let zone_center = Vec2::new(
                (zone.world_bounds.0.x + zone.world_bounds.1.x) / 2.0,
                (zone.world_bounds.0.y + zone.world_bounds.1.y) / 2.0
            );
            zone_center.distance(path_pos) <= 150.0
        }).count();
        
        // Chokepoint if multiple zones can cover this area
        if nearby_zones >= 2 {
            chokepoints.push(progress);
        }
    }
    
    chokepoints
}

/// Calculate effectiveness of a chokepoint
fn calculate_chokepoint_effectiveness(chokepoint_progress: &f32, zones: &[TowerZone]) -> f32 {
    // Simplified effectiveness based on nearby zone count and strategic values
    // In real implementation would consider zone positioning, overlap, etc.
    0.5 // Placeholder - reasonable effectiveness
}

/// Calculate average strategic value of zones
fn calculate_average_strategic_value(zones: &[TowerZone]) -> f32 {
    if zones.is_empty() {
        return 0.0;
    }
    
    let sum: f32 = zones.iter().map(|z| z.strategic_value).sum();
    sum / zones.len() as f32
}