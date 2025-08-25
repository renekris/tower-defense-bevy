/// Performance tests to verify the dense grid system (576 squares) performs well
use std::time::Instant;
use tower_defense_bevy::systems::path_generation::{generate_level_path, generate_level_path_with_params};
use tower_defense_bevy::systems::path_generation::grid::{PathGrid, GridPos};
use tower_defense_bevy::systems::path_generation::pathfinding::find_path;
use tower_defense_bevy::systems::unified_grid::UnifiedGridSystem;

#[cfg(test)]
mod performance_tests {
    use super::*;

    #[test]
    fn test_path_generation_performance() {
        let start_time = Instant::now();
        
        // Test multiple path generations to ensure consistent performance
        let mut generation_times = Vec::new();
        
        for wave in 1..=10 {
            let gen_start = Instant::now();
            let _path = generate_level_path(wave);
            let gen_duration = gen_start.elapsed();
            generation_times.push(gen_duration.as_millis());
        }
        
        let total_duration = start_time.elapsed();
        let average_time = generation_times.iter().sum::<u128>() / generation_times.len() as u128;
        let max_time = *generation_times.iter().max().unwrap();
        
        println!("Path Generation Performance (10 waves):");
        println!("  Total time: {:?}ms", total_duration.as_millis());
        println!("  Average per wave: {}ms", average_time);
        println!("  Max time per wave: {}ms", max_time);
        
        // Path generation should be fast (<100ms per path for responsive gameplay)
        assert!(max_time < 100, "Path generation too slow: {}ms", max_time);
        assert!(average_time < 50, "Average path generation too slow: {}ms", average_time);
    }

    #[test]
    fn test_unified_grid_system_performance() {
        let start_time = Instant::now();
        
        // Test creating and verifying unified grid system
        let unified_grid = UnifiedGridSystem::default();
        let creation_time = start_time.elapsed();
        
        // Verify performance characteristics
        assert_eq!(unified_grid.total_squares(), 576, "Should have 576 squares");
        assert!(creation_time.as_millis() < 10, "Grid creation should be instant: {:?}ms", creation_time.as_millis());
        
        println!("Unified Grid Performance:");
        println!("  Creation time: {:?}ms", creation_time.as_millis());
        println!("  Total squares: {}", unified_grid.total_squares());
        println!("  Grid area: {:?}", unified_grid.grid_area_size());
    }

    #[test]
    fn test_pathfinding_performance_dense_grid() {
        let grid = PathGrid::new_unified();
        let start_time = Instant::now();
        
        // Test pathfinding from corners and various positions
        let test_cases = vec![
            // Start -> End positions for pathfinding tests
            (GridPos::new(0, 9), GridPos::new(31, 9)),   // Straight across middle
            (GridPos::new(0, 6), GridPos::new(31, 12)),  // Diagonal path
            (GridPos::new(0, 12), GridPos::new(31, 6)),  // Reverse diagonal
            (GridPos::new(0, 8), GridPos::new(31, 10)),  // Slight offset
            (GridPos::new(0, 10), GridPos::new(31, 8)),  // Reverse offset
        ];
        
        let mut pathfinding_times = Vec::new();
        
        for (start, end) in test_cases {
            let path_start = Instant::now();
            let path = find_path(&grid, start, end);
            let path_duration = path_start.elapsed();
            
            assert!(path.is_some(), "Path should be found from {:?} to {:?}", start, end);
            pathfinding_times.push(path_duration.as_micros());
            
            if let Some(path) = path {
                println!("Path {:?} -> {:?}: {} steps, {:?}μs", 
                         start, end, path.len(), path_duration.as_micros());
            }
        }
        
        let total_duration = start_time.elapsed();
        let average_time = pathfinding_times.iter().sum::<u128>() / pathfinding_times.len() as u128;
        let max_time = *pathfinding_times.iter().max().unwrap();
        
        println!("Pathfinding Performance (Dense 32x18 Grid):");
        println!("  Total time: {:?}ms", total_duration.as_millis());
        println!("  Average per path: {}μs", average_time);
        println!("  Max time per path: {}μs", max_time);
        
        // Pathfinding should be very fast (<10ms even for worst case)
        assert!(max_time < 10000, "Pathfinding too slow: {}μs", max_time);
        assert!(average_time < 5000, "Average pathfinding too slow: {}μs", average_time);
    }

    #[test] 
    fn test_coordinate_conversion_performance() {
        let grid = PathGrid::new_unified();
        let start_time = Instant::now();
        
        let mut conversion_times = Vec::new();
        
        // Test coordinate conversions for entire grid
        for y in 0..grid.height {
            for x in 0..grid.width {
                let grid_pos = GridPos::new(x, y);
                
                let conv_start = Instant::now();
                let world_pos = grid.grid_to_world(grid_pos);
                let converted_back = grid.world_to_grid(world_pos);
                let conv_duration = conv_start.elapsed();
                
                conversion_times.push(conv_duration.as_nanos());
                
                // Verify round-trip conversion accuracy
                assert_eq!(converted_back, Some(grid_pos), 
                          "Round-trip conversion failed for {:?}", grid_pos);
            }
        }
        
        let total_duration = start_time.elapsed();
        let total_conversions = grid.width * grid.height * 2; // grid->world + world->grid
        let average_time = conversion_times.iter().sum::<u128>() / conversion_times.len() as u128;
        
        println!("Coordinate Conversion Performance (576 grid positions):");
        println!("  Total time: {:?}ms", total_duration.as_millis());
        println!("  Total conversions: {}", total_conversions);
        println!("  Average per conversion: {}ns", average_time);
        
        // Coordinate conversions should be extremely fast
        assert!(total_duration.as_millis() < 100, "Coordinate conversions too slow: {:?}ms", total_duration.as_millis());
        assert!(average_time < 1000, "Average conversion too slow: {}ns", average_time);
    }

    #[test]
    fn test_wave_generation_scalability() {
        let start_time = Instant::now();
        
        // Test generating paths for many waves to ensure scalability
        let wave_count = 50;
        let mut total_waypoints = 0;
        let mut generation_times = Vec::new();
        
        for wave in 1..=wave_count {
            let gen_start = Instant::now();
            let path = generate_level_path(wave);
            let gen_duration = gen_start.elapsed();
            
            total_waypoints += path.waypoints.len();
            generation_times.push(gen_duration.as_millis());
        }
        
        let total_duration = start_time.elapsed();
        let average_waypoints = total_waypoints / wave_count as usize;
        let average_time = generation_times.iter().sum::<u128>() / generation_times.len() as u128;
        
        println!("Wave Generation Scalability ({} waves):", wave_count);
        println!("  Total time: {:?}ms", total_duration.as_millis());
        println!("  Average waypoints per path: {}", average_waypoints);
        println!("  Average generation time: {}ms", average_time);
        println!("  Total waypoints generated: {}", total_waypoints);
        
        // Should maintain performance across many waves
        assert!(average_time < 50, "Average generation time too slow: {}ms", average_time);
        assert!(average_waypoints >= 4, "Paths too simple: {} waypoints", average_waypoints);
        assert!(total_duration.as_millis() < 5000, "Total generation too slow: {:?}ms", total_duration.as_millis());
    }

    #[test]
    fn test_custom_complexity_performance() {
        let start_time = Instant::now();
        
        // Test path generation with different complexity parameters
        let complexity_levels = vec![0.0, 0.25, 0.5, 0.75, 1.0];
        let mut complexity_times = Vec::new();
        
        for complexity in complexity_levels {
            let comp_start = Instant::now();
            for wave in 1..=5 {
                let _path = generate_level_path_with_params(wave, complexity);
            }
            let comp_duration = comp_start.elapsed();
            
            complexity_times.push(comp_duration.as_millis());
            println!("Complexity {} (5 waves): {:?}ms", complexity, comp_duration.as_millis());
        }
        
        let total_duration = start_time.elapsed();
        let max_complexity_time = *complexity_times.iter().max().unwrap();
        
        println!("Custom Complexity Performance:");
        println!("  Total time: {:?}ms", total_duration.as_millis());
        println!("  Max complexity time: {}ms", max_complexity_time);
        
        // Complex path generation should still be reasonably fast
        assert!(max_complexity_time < 500, "Complex path generation too slow: {}ms", max_complexity_time);
        assert!(total_duration.as_millis() < 2000, "Total complexity testing too slow: {:?}ms", total_duration.as_millis());
    }
}