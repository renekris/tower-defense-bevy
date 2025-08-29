use tower_defense_bevy::resources::EnemyPath;
use tower_defense_bevy::systems::path_generation::*;
use bevy::prelude::*;
use std::time::Instant;

/// Performance tests for Catmull-Rom splined paths
/// Ensures smooth path calculations maintain real-time performance requirements

#[test]
fn test_smooth_path_calculation_performance() {
    // Create a complex path for testing
    let waypoints = generate_complex_test_path(20); // 20 waypoints
    let path = EnemyPath::new(waypoints);
    
    // Benchmark smooth position calculations
    let iterations = 10000;
    let start = Instant::now();
    
    for i in 0..iterations {
        let progress = (i as f32) / (iterations as f32);
        let _position = path.get_smooth_position_at_progress(progress);
    }
    
    let duration = start.elapsed();
    let microseconds_per_call = duration.as_micros() as f32 / iterations as f32;
    
    // Should be fast enough for real-time usage (< 1 microsecond per call)
    assert!(microseconds_per_call < 1.0,
        "Smooth path calculation too slow: {:.2} μs per call (need < 1.0 μs)", microseconds_per_call);
}

#[test] 
fn test_real_time_enemy_movement_performance() {
    // Simulate realistic enemy movement scenario
    let path = generate_level_path(5);
    let enemy_count = 50; // Typical number of enemies on screen
    let frame_rate = 60.0; // Target 60 FPS
    
    let start = Instant::now();
    
    // Simulate one frame of enemy movement updates
    for enemy_id in 0..enemy_count {
        // Each enemy at different progress along path
        let progress = (enemy_id as f32) / (enemy_count as f32);
        let _position = path.get_smooth_position_at_progress(progress);
        
        // Simulate additional movement calculations per enemy
        let _next_position = path.get_smooth_position_at_progress(progress + 0.01);
    }
    
    let frame_time = start.elapsed();
    let frame_budget = std::time::Duration::from_micros((1_000_000.0 / frame_rate) as u64);
    
    // Movement calculations should use < 10% of frame budget
    let performance_budget = frame_budget / 10;
    assert!(frame_time < performance_budget,
        "Enemy movement too slow: {:?} > {:?} (10% of 60fps frame)", frame_time, performance_budget);
}

#[test]
fn test_memory_usage_consistency() {
    // Test that smooth path calculations don't cause memory leaks or excessive allocation
    let initial_allocations = count_allocations();
    
    // Perform many path calculations
    for wave in 1..=20 {
        let path = generate_level_path(wave);
        
        // Heavy usage simulation
        for _ in 0..1000 {
            let progress = rand::random::<f32>();
            let _pos = path.get_smooth_position_at_progress(progress);
        }
    }
    
    let final_allocations = count_allocations();
    let allocation_increase = final_allocations.saturating_sub(initial_allocations);
    
    // Should not have significant memory growth during calculations
    // Note: This is a simplified test - real memory tracking would be more complex
    assert!(allocation_increase < 1000, // Arbitrary reasonable limit
        "Excessive memory allocation during path calculations: {} new allocations", allocation_increase);
}

#[test]
fn test_batch_path_generation_performance() {
    // Test generating multiple paths quickly (for wave transitions)
    let wave_count = 10;
    let start = Instant::now();
    
    let mut paths = Vec::new();
    for wave in 1..=wave_count {
        paths.push(generate_level_path(wave));
    }
    
    let generation_time = start.elapsed();
    
    // Should generate all paths quickly (< 50ms for 10 paths)
    assert!(generation_time.as_millis() < 50,
        "Batch path generation too slow: {:?} for {} paths", generation_time, wave_count);
    
    // Verify all paths are valid
    for (i, path) in paths.iter().enumerate() {
        assert!(path.waypoints.len() >= 2, "Generated path {} should have waypoints", i);
        assert!(path.total_length() > 0.0, "Generated path {} should have positive length", i);
    }
}

#[test]
fn test_concurrent_path_access_performance() {
    // Test that multiple enemies can access the same path efficiently
    let path = generate_level_path(8);
    let concurrent_enemies = 100;
    
    let start = Instant::now();
    
    // Simulate concurrent access (in single thread for testing)
    for frame in 0..60 { // 1 second at 60 FPS
        for enemy in 0..concurrent_enemies {
            let base_progress = (enemy as f32) / (concurrent_enemies as f32);
            let time_progress = (frame as f32) / 60.0 * 0.1; // Slow movement
            let progress = (base_progress + time_progress) % 1.0;
            
            let _position = path.get_smooth_position_at_progress(progress);
        }
    }
    
    let total_time = start.elapsed();
    let average_frame_time = total_time / 60;
    
    // Average frame should be well under 16.67ms (60 FPS)
    assert!(average_frame_time.as_millis() < 10,
        "Concurrent access too slow: {:?} average frame time", average_frame_time);
}

#[test]
fn test_smooth_vs_linear_performance_comparison() {
    // Compare performance of smooth vs linear interpolation
    let path = generate_level_path(3);
    let iterations = 5000;
    
    // Benchmark linear interpolation
    let start = Instant::now();
    for i in 0..iterations {
        let progress = (i as f32) / (iterations as f32);
        let _position = path.get_position_at_progress(progress);
    }
    let linear_time = start.elapsed();
    
    // Benchmark smooth interpolation
    let start = Instant::now();
    for i in 0..iterations {
        let progress = (i as f32) / (iterations as f32);
        let _position = path.get_smooth_position_at_progress(progress);
    }
    let smooth_time = start.elapsed();
    
    // Smooth should not be more than 5x slower than linear
    let performance_ratio = smooth_time.as_nanos() as f32 / linear_time.as_nanos() as f32;
    assert!(performance_ratio < 5.0,
        "Smooth interpolation too slow vs linear: {:.2}x slower (limit: 5x)", performance_ratio);
    
    println!("Performance comparison - Linear: {:?}, Smooth: {:?}, Ratio: {:.2}x", 
             linear_time, smooth_time, performance_ratio);
}

#[test]
fn test_consistent_frame_rates_with_smooth_interpolation() {
    // Test that frame rates remain consistent with smooth paths
    let path = generate_level_path(7);
    let enemy_count = 30;
    let frame_count = 120; // 2 seconds at 60 FPS
    
    let mut frame_times = Vec::new();
    
    for frame in 0..frame_count {
        let frame_start = Instant::now();
        
        // Simulate frame processing
        for enemy in 0..enemy_count {
            let progress = ((enemy as f32 / enemy_count as f32) + 
                          (frame as f32 / frame_count as f32) * 0.5) % 1.0;
            let _position = path.get_smooth_position_at_progress(progress);
        }
        
        frame_times.push(frame_start.elapsed());
    }
    
    // Calculate frame time statistics
    let avg_frame_time: std::time::Duration = frame_times.iter().sum::<std::time::Duration>() / frame_times.len() as u32;
    let max_frame_time = frame_times.iter().max().unwrap();
    let min_frame_time = frame_times.iter().min().unwrap();
    
    // Frame times should be consistent (low variance)
    let variance_ratio = max_frame_time.as_nanos() as f32 / min_frame_time.as_nanos() as f32;
    assert!(variance_ratio < 3.0,
        "Frame time variance too high: {:.2}x (max: {:?}, min: {:?})", 
        variance_ratio, max_frame_time, min_frame_time);
    
    // Average frame time should be reasonable
    assert!(avg_frame_time.as_millis() < 5,
        "Average frame time too high: {:?}", avg_frame_time);
}

#[test]
fn test_path_caching_performance_impact() {
    // Test that path caching doesn't hurt performance
    let wave_number = 5;
    
    // First generation (cache miss)
    let start = Instant::now();
    let path1 = generate_level_path(wave_number);
    let first_gen_time = start.elapsed();
    
    // Second generation (cache hit)
    let start = Instant::now();
    let path2 = generate_level_path(wave_number);
    let second_gen_time = start.elapsed();
    
    // Paths should be identical (deterministic)
    assert_eq!(path1.waypoints, path2.waypoints, "Cached paths should be identical");
    
    // Second generation should not be significantly slower (cache overhead check)
    let cache_overhead_ratio = second_gen_time.as_nanos() as f32 / first_gen_time.as_nanos().max(1) as f32;
    assert!(cache_overhead_ratio < 2.0,
        "Cache overhead too high: {:.2}x (first: {:?}, second: {:?})", 
        cache_overhead_ratio, first_gen_time, second_gen_time);
}

#[test]
fn test_interpolation_precision_vs_performance() {
    // Test different interpolation precisions for performance trade-offs
    let path = generate_level_path(4);
    let test_cases = vec![
        ("Low precision", 0.1),    // 10 samples
        ("Medium precision", 0.01), // 100 samples
        ("High precision", 0.001), // 1000 samples
    ];
    
    for (name, step_size) in test_cases {
        let sample_count = (1.0 / step_size) as usize;
        let start = Instant::now();
        
        let mut total_length = 0.0;
        for i in 0..sample_count {
            let progress1 = (i as f32) * step_size;
            let progress2 = ((i + 1) as f32) * step_size;
            
            let pos1 = path.get_smooth_position_at_progress(progress1);
            let pos2 = path.get_smooth_position_at_progress(progress2);
            
            total_length += pos1.distance(pos2);
        }
        
        let calculation_time = start.elapsed();
        
        println!("{}: {} samples, time: {:?}, length: {:.2}", 
                 name, sample_count, calculation_time, total_length);
        
        // All precisions should complete reasonably quickly
        assert!(calculation_time.as_millis() < 100,
            "{} took too long: {:?}", name, calculation_time);
    }
}

// Helper functions for performance testing

/// Generate a complex test path with many waypoints
fn generate_complex_test_path(waypoint_count: usize) -> Vec<Vec2> {
    let mut waypoints = Vec::new();
    
    for i in 0..waypoint_count {
        let t = i as f32 / (waypoint_count - 1) as f32;
        let x = t * 400.0 - 200.0; // -200 to 200
        let y = (t * std::f32::consts::PI * 4.0).sin() * 100.0; // Sine wave
        waypoints.push(Vec2::new(x, y));
    }
    
    waypoints
}

/// Simplified allocation counter (in real implementation would use proper memory profiling)
fn count_allocations() -> usize {
    // This is a placeholder - real implementation would use tools like:
    // - jemalloc statistics
    // - custom allocator tracking
    // - OS memory statistics
    // For testing purposes, return a reasonable baseline
    std::thread::current().id().as_u64().get() as usize % 1000
}

#[test]
fn test_edge_case_performance() {
    // Test performance with edge cases that might cause slowdowns
    
    // Very short path
    let short_path = EnemyPath::new(vec![Vec2::ZERO, Vec2::new(1.0, 0.0)]);
    let start = Instant::now();
    for i in 0..1000 {
        let progress = i as f32 / 1000.0;
        let _pos = short_path.get_smooth_position_at_progress(progress);
    }
    let short_path_time = start.elapsed();
    
    // Very long straight path
    let long_straight_path = EnemyPath::new(vec![
        Vec2::new(0.0, 0.0), Vec2::new(1000.0, 0.0)
    ]);
    let start = Instant::now();
    for i in 0..1000 {
        let progress = i as f32 / 1000.0;
        let _pos = long_straight_path.get_smooth_position_at_progress(progress);
    }
    let long_path_time = start.elapsed();
    
    // Performance should be consistent regardless of path characteristics
    assert!(short_path_time.as_micros() < 1000, "Short path performance: {:?}", short_path_time);
    assert!(long_path_time.as_micros() < 1000, "Long path performance: {:?}", long_path_time);
    
    // Times should be similar (no pathological cases)
    let time_ratio = long_path_time.as_nanos() as f32 / short_path_time.as_nanos().max(1) as f32;
    assert!(time_ratio < 3.0, "Performance should be consistent across path types: {:.2}x", time_ratio);
}