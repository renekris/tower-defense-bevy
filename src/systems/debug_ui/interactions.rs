use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;
use crate::systems::debug_visualization::DebugVisualizationState;
use crate::systems::unified_grid::UnifiedGridSystem;
use super::components::*;

/// System to handle F2 key toggle for debug UI
pub fn debug_ui_toggle_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut ui_state: ResMut<DebugUIState>,
) {
    if keyboard_input.just_pressed(KeyCode::F2) {
        ui_state.panel_visible = !ui_state.panel_visible;
        println!("Debug UI panel: {}", if ui_state.panel_visible { "enabled" } else { "disabled" });
    }
}

/// System to update UI panel visibility based on state
pub fn update_debug_ui_visibility(
    ui_state: Res<DebugUIState>,
    mut panel_query: Query<&mut Node, With<DebugUIPanel>>,
) {
    if ui_state.is_changed() {
        for mut node in &mut panel_query {
            node.display = if ui_state.panel_visible {
                Display::Flex
            } else {
                Display::None
            };
        }
    }
}

/// System to handle toggle button interactions
pub fn handle_toggle_button_interactions(
    mut interaction_query: Query<
        (&Interaction, &ToggleButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut debug_state: ResMut<DebugVisualizationState>,
    mut unified_grid: ResMut<UnifiedGridSystem>,
    // CRITICAL FIX: Add mouse input state to consume clicks and prevent pass-through
    mut mouse_input_state: ResMut<crate::systems::input_system::MouseInputState>,
) {
    for (interaction, toggle_button, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                // CRITICAL FIX: Consume the mouse click to prevent pass-through to game world
                mouse_input_state.left_clicked = false;
                
                // Toggle the corresponding state in unified grid system
                match toggle_button.toggle_type {
                    ToggleType::Grid => {
                        unified_grid.show_grid = !unified_grid.show_grid;
                        println!("Grid visualization: {}", unified_grid.show_grid);
                    }
                    ToggleType::Path => {
                        unified_grid.show_path = !unified_grid.show_path;
                        println!("Path visualization: {}", unified_grid.show_path);
                    }
                    ToggleType::Zones => {
                        unified_grid.show_zones = !unified_grid.show_zones;
                        println!("Zone visualization: {}", unified_grid.show_zones);
                    }
                    ToggleType::Performance => {
                        // Toggle performance metrics (placeholder for now)
                        println!("Performance metrics toggled");
                    }
                }
            }
            Interaction::Hovered => {
                *color = Color::srgba(0.3, 0.3, 0.7, 0.8).into();
            }
            Interaction::None => {
                // Return to normal color based on toggle state
                let is_active = match toggle_button.toggle_type {
                    ToggleType::Grid => unified_grid.show_grid,
                    ToggleType::Path => unified_grid.show_path,
                    ToggleType::Zones => unified_grid.show_zones,
                    ToggleType::Performance => true, // Always true for now
                };
                
                *color = if is_active {
                    Color::srgba(0.2, 0.7, 0.2, 0.8).into() // Green when active
                } else {
                    Color::srgba(0.7, 0.2, 0.2, 0.8).into() // Red when inactive
                };
            }
        }
    }
}

/// System to sync UI state with unified grid system state
pub fn sync_ui_with_debug_state(
    unified_grid: Res<UnifiedGridSystem>,
    mut toggle_query: Query<(&ToggleButton, &mut BackgroundColor), With<Button>>,
) {
    if unified_grid.is_changed() {
        for (toggle_button, mut color) in &mut toggle_query {
            let is_active = match toggle_button.toggle_type {
                ToggleType::Grid => unified_grid.show_grid,
                ToggleType::Path => unified_grid.show_path,
                ToggleType::Zones => unified_grid.show_zones,
                ToggleType::Performance => true, // Always true for now
            };
            
            *color = if is_active {
                Color::srgba(0.2, 0.7, 0.2, 0.8).into() // Green when active
            } else {
                Color::srgba(0.7, 0.2, 0.2, 0.8).into() // Red when inactive
            };
        }
    }
}

/// System to handle slider interactions (simplified approach)
pub fn handle_slider_interactions(
    mut interaction_query: Query<
        (&Interaction, &mut ParameterSlider, &mut BackgroundColor),
        (Changed<Interaction>, With<SliderHandle>),
    >,
    mut ui_state: ResMut<DebugUIState>,
    // CRITICAL FIX: Add mouse input state to consume clicks and prevent pass-through
    mut mouse_input_state: ResMut<crate::systems::input_system::MouseInputState>,
) {
    for (interaction, mut slider, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                // CRITICAL FIX: Consume the mouse click to prevent pass-through to game world
                mouse_input_state.left_clicked = false;
                
                // Simple increment approach - each click increases by 10% of range
                let increment = (slider.max_value - slider.min_value) * 0.1;
                slider.current_value += increment;
                if slider.current_value > slider.max_value {
                    slider.current_value = slider.min_value; // Wrap around
                }
                
                // Update UI state
                match slider.slider_type {
                    SliderType::ObstacleDensity => ui_state.current_obstacle_density = slider.current_value,
                    SliderType::EnemySpawnRate => ui_state.enemy_spawn_rate = slider.current_value,
                    SliderType::TowerDamageMultiplier => ui_state.tower_damage_multiplier = slider.current_value,
                }
                
                *color = Color::srgba(0.6, 0.6, 1.0, 1.0).into(); // Blue when clicked
                println!("Slider {:?} changed to: {:.2}", slider.slider_type, slider.current_value);
            }
            Interaction::Hovered => {
                *color = Color::srgba(1.0, 1.0, 1.0, 1.0).into(); // White when hovered
            }
            Interaction::None => {
                *color = Color::srgba(0.8, 0.8, 0.8, 1.0).into(); // Gray when normal
            }
        }
    }
}

/// System to update slider text displays
pub fn update_slider_values(
    slider_query: Query<&ParameterSlider, (With<SliderHandle>, Changed<ParameterSlider>)>,
    mut text_query: Query<(&SliderValueText, &mut Text)>,
) {
    // Update text displays when slider values change
    for slider in slider_query.iter() {
        for (text_info, mut text) in text_query.iter_mut() {
            if slider.slider_type == text_info.slider_type {
                let label = match slider.slider_type {
                    SliderType::ObstacleDensity => "Obstacle Density",
                    SliderType::EnemySpawnRate => "Enemy Spawn Rate",
                    SliderType::TowerDamageMultiplier => "Tower Damage",
                };
                **text = format!("{}: {:.2}", label, slider.current_value);
            }
        }
    }
}

/// System to update the enemy path resource when UI parameters change
pub fn update_enemy_path_from_ui(
    _commands: Commands,
    mut ui_state: ResMut<DebugUIState>,
    debug_state: Res<crate::systems::debug_visualization::DebugVisualizationState>,
    _enemy_path: ResMut<EnemyPath>,
    _path_line_query: Query<Entity, With<GamePathLine>>,
) {
    // Only update if UI state has changed and debug is enabled
    if ui_state.is_changed() && debug_state.enabled {
        // Only log if value actually changed to prevent spam
        if (ui_state.current_obstacle_density - ui_state.last_logged_obstacle_density).abs() > 0.01 {
            // Note: Path generation is using static path for stability
            // Dynamic path regeneration will be implemented in future updates
            ui_state.last_logged_obstacle_density = ui_state.current_obstacle_density;
        }
    }
}

/// System to update enemy spawn rate when UI parameters change
pub fn update_spawn_rate_from_ui(
    mut ui_state: ResMut<DebugUIState>,
    debug_state: Res<crate::systems::debug_visualization::DebugVisualizationState>,
    _wave_manager: ResMut<crate::resources::WaveManager>,
) {
    // Only update if UI state has changed and debug is enabled
    if ui_state.is_changed() && debug_state.enabled {
        // Only log if value actually changed to prevent spam
        if (ui_state.enemy_spawn_rate - ui_state.last_logged_spawn_rate).abs() > 0.01 {
            println!("Debug UI: Enemy spawn rate changed to {:.2} (interval: {:.2}s)", 
                ui_state.enemy_spawn_rate, 
                1.0 / ui_state.enemy_spawn_rate.max(0.1));
            ui_state.last_logged_spawn_rate = ui_state.enemy_spawn_rate;
        }
        
        // Note: Wave manager spawn rate update temporarily disabled
        // wave_manager.set_spawn_rate(ui_state.enemy_spawn_rate);
    }
}

/// System to handle keyboard shortcuts for debug UI
pub fn handle_debug_keyboard_shortcuts(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut ui_state: ResMut<DebugUIState>,
    mut wave_manager: ResMut<WaveManager>,
    mut economy: ResMut<Economy>,
    mut wave_status: ResMut<crate::systems::combat_system::WaveStatus>,
    mut game_state: ResMut<GameState>,
    mut commands: Commands,
    enemy_query: Query<Entity, With<Enemy>>,
    projectile_query: Query<Entity, With<Projectile>>,
    tower_query: Query<Entity, With<TowerStats>>,
) {
    // R key - Reset game
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        println!("Keyboard shortcut: Resetting game (R key)");
        
        // Reset all game entities
        for entity in enemy_query.iter() {
            commands.entity(entity).despawn();
        }
        for entity in projectile_query.iter() {
            commands.entity(entity).despawn();
        }
        for entity in tower_query.iter() {
            commands.entity(entity).despawn();
        }
        
        // Reset resources
        wave_manager.current_wave = 0;
        wave_manager.enemies_in_wave = 0;
        wave_manager.enemies_spawned = 0;
        
        economy.money = 100;
        economy.research_points = 0;
        
        wave_status.enemies_remaining = 0;
        wave_status.enemies_killed = 0;
        wave_status.enemies_escaped = 0;
        wave_status.wave_complete = false;
        
        *game_state = GameState::Playing;
    }
    
    // M key - Randomize map
    if keyboard_input.just_pressed(KeyCode::KeyM) {
        println!("Keyboard shortcut: Randomizing map (M key)");
        
        use rand::Rng;
        let mut rng = rand::rng();
        ui_state.current_obstacle_density = rng.random_range(0.1..=0.8);
        ui_state.set_changed(); // Trigger path regeneration
    }
    
    // Number keys 1-5 - Quick adjust spawn rate (1=slow, 5=fast)
    if keyboard_input.just_pressed(KeyCode::Digit1) {
        ui_state.enemy_spawn_rate = 0.5; // Slow
        ui_state.set_changed();
        println!("Keyboard shortcut: Spawn rate set to SLOW (1 key)");
    }
    if keyboard_input.just_pressed(KeyCode::Digit2) {
        ui_state.enemy_spawn_rate = 1.0; // Normal
        ui_state.set_changed();
        println!("Keyboard shortcut: Spawn rate set to NORMAL (2 key)");
    }
    if keyboard_input.just_pressed(KeyCode::Digit3) {
        ui_state.enemy_spawn_rate = 2.0; // Fast
        ui_state.set_changed();
        println!("Keyboard shortcut: Spawn rate set to FAST (3 key)");
    }
    if keyboard_input.just_pressed(KeyCode::Digit4) {
        ui_state.enemy_spawn_rate = 3.0; // Very Fast
        ui_state.set_changed();
        println!("Keyboard shortcut: Spawn rate set to VERY FAST (4 key)");
    }
    if keyboard_input.just_pressed(KeyCode::Digit5) {
        ui_state.enemy_spawn_rate = 5.0; // Ultra Fast
        ui_state.set_changed();
        println!("Keyboard shortcut: Spawn rate set to ULTRA FAST (5 key)");
    }
    
    // Plus/Minus keys - Adjust tower damage multiplier
    if keyboard_input.just_pressed(KeyCode::Equal) { // Plus key (without shift)
        ui_state.tower_damage_multiplier = (ui_state.tower_damage_multiplier + 0.5).clamp(0.1, 10.0);
        println!("Keyboard shortcut: Tower damage increased to {:.1}x (+ key)", ui_state.tower_damage_multiplier);
    }
    if keyboard_input.just_pressed(KeyCode::Minus) {
        ui_state.tower_damage_multiplier = (ui_state.tower_damage_multiplier - 0.5).clamp(0.1, 10.0);
        println!("Keyboard shortcut: Tower damage decreased to {:.1}x (- key)", ui_state.tower_damage_multiplier);
    }
}

/// System to handle action button clicks
pub fn handle_action_buttons(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &ActionButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut ui_state: ResMut<DebugUIState>,
    mut wave_manager: ResMut<WaveManager>,
    mut economy: ResMut<Economy>,
    mut wave_status: ResMut<crate::systems::combat_system::WaveStatus>,
    mut game_state: ResMut<GameState>,
    enemy_query: Query<Entity, With<Enemy>>,
    projectile_query: Query<Entity, With<Projectile>>,
    tower_query: Query<Entity, With<TowerStats>>,
    _path_line_query: Query<Entity, With<GamePathLine>>,
    _enemy_path: ResMut<EnemyPath>,
    // CRITICAL FIX: Add mouse input state to consume clicks and prevent pass-through
    mut mouse_input_state: ResMut<crate::systems::input_system::MouseInputState>,
) {
    for (interaction, action_button, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                // CRITICAL FIX: Consume the mouse click to prevent pass-through to game world
                mouse_input_state.left_clicked = false;
                
                // Button press visual feedback
                *color = Color::srgb(1.0, 1.0, 1.0).into();
                
                match action_button.action_type {
                    ActionType::ResetGame => {
                        // Reset all game state
                        println!("Resetting game state...");
                        
                        // Despawn all game entities
                        for entity in enemy_query.iter() {
                            commands.entity(entity).despawn();
                        }
                        for entity in projectile_query.iter() {
                            commands.entity(entity).despawn();
                        }
                        for entity in tower_query.iter() {
                            commands.entity(entity).despawn();
                        }
                        
                        // Reset resources
                        wave_manager.current_wave = 0;
                        wave_manager.enemies_in_wave = 0;
                        wave_manager.enemies_spawned = 0;
                        
                        economy.money = 100;
                        economy.research_points = 0;
                        
                        wave_status.enemies_remaining = 0;
                        wave_status.enemies_killed = 0;
                        wave_status.enemies_escaped = 0;
                        wave_status.wave_complete = false;
                        
                        *game_state = GameState::Playing;
                        
                        println!("Game reset complete!");
                    },
                    ActionType::RandomizeMap => {
                        println!("Randomizing map...");
                        
                        // Generate new random path parameters
                        use rand::Rng;
                        let mut rng = rand::rng();
                        ui_state.current_obstacle_density = rng.random_range(0.1..=0.8);
                        
                        // Trigger path regeneration by changing the parameter
                        ui_state.set_changed();
                        
                        println!("Map randomized with obstacle density: {:.2}", ui_state.current_obstacle_density);
                    },
                    ActionType::SaveState => {
                        println!("Saving game state... (Feature not yet implemented)");
                        // TODO: Implement save functionality
                        // Could save to file: wave progress, economy, tower positions, etc.
                    },
                    ActionType::LoadState => {
                        println!("Loading game state... (Feature not yet implemented)");
                        // TODO: Implement load functionality
                        // Could load from file: restore game state
                    },
                }
            },
            Interaction::Hovered => {
                // Button hover visual feedback
                let hover_color = match action_button.action_type {
                    ActionType::ResetGame => Color::srgb(1.0, 0.4, 0.4),
                    ActionType::RandomizeMap => Color::srgb(0.4, 0.7, 1.0),
                    ActionType::SaveState => Color::srgb(0.4, 1.0, 0.4),
                    ActionType::LoadState => Color::srgb(1.0, 1.0, 0.4),
                };
                *color = hover_color.into();
            },
            Interaction::None => {
                // Button normal visual state
                let normal_color = match action_button.action_type {
                    ActionType::ResetGame => Color::srgb(0.8, 0.3, 0.3),
                    ActionType::RandomizeMap => Color::srgb(0.3, 0.6, 0.8),
                    ActionType::SaveState => Color::srgb(0.3, 0.8, 0.3),
                    ActionType::LoadState => Color::srgb(0.8, 0.8, 0.3),
                };
                *color = normal_color.into();
            },
        }
    }
}