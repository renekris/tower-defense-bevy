use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;
use crate::systems::debug_visualization::DebugVisualizationState;
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
) {
    for (interaction, toggle_button, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                // Toggle the corresponding debug state
                match toggle_button.toggle_type {
                    ToggleType::Grid => {
                        debug_state.show_grid = !debug_state.show_grid;
                        println!("Grid visualization: {}", debug_state.show_grid);
                    }
                    ToggleType::Path => {
                        debug_state.show_path = !debug_state.show_path;
                        println!("Path visualization: {}", debug_state.show_path);
                    }
                    ToggleType::Zones => {
                        debug_state.show_tower_zones = !debug_state.show_tower_zones;
                        println!("Zone visualization: {}", debug_state.show_tower_zones);
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
                    ToggleType::Grid => debug_state.show_grid,
                    ToggleType::Path => debug_state.show_path,
                    ToggleType::Zones => debug_state.show_tower_zones,
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

/// System to sync UI state with debug visualization state
pub fn sync_ui_with_debug_state(
    debug_state: Res<DebugVisualizationState>,
    mut toggle_query: Query<(&ToggleButton, &mut BackgroundColor), With<Button>>,
) {
    if debug_state.is_changed() {
        for (toggle_button, mut color) in &mut toggle_query {
            let is_active = match toggle_button.toggle_type {
                ToggleType::Grid => debug_state.show_grid,
                ToggleType::Path => debug_state.show_path,
                ToggleType::Zones => debug_state.show_tower_zones,
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

/// System to handle slider interactions (clicking and basic feedback)
pub fn handle_slider_interactions(
    mut interaction_query: Query<
        (&Interaction, &ParameterSlider, &mut BackgroundColor),
        (Changed<Interaction>, With<SliderHandle>),
    >,
    mut drag_state: ResMut<SliderDragState>,
) {
    for (interaction, slider, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                drag_state.dragging = Some(slider.slider_type);
                *color = Color::srgba(0.6, 0.6, 1.0, 1.0).into(); // Blue when dragging
                println!("Started dragging slider: {:?}", slider.slider_type);
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

/// System to handle slider value changes and update UI state
pub fn update_slider_values(
    mut slider_query: Query<(&mut ParameterSlider, &mut Node), With<SliderHandle>>,
    mut ui_state: ResMut<DebugUIState>,
    mut text_query: Query<(&SliderValueText, &mut Text)>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut drag_state: ResMut<SliderDragState>,
    windows: Query<&Window>,
    _camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    // Stop dragging when mouse is released
    if !mouse_input.pressed(MouseButton::Left)
        && drag_state.dragging.is_some() {
            println!("Stopped dragging slider");
            drag_state.dragging = None;
        }

    // Update slider values based on mouse position when dragging
    if let Some(dragging_type) = drag_state.dragging {
        if let Ok(window) = windows.single() {
            if let Some(mouse_pos) = window.cursor_position() {
                // Find the dragging slider and update its position
                for (mut slider, mut node) in &mut slider_query {
                    if slider.slider_type == dragging_type {
                        // Simple horizontal drag logic - convert mouse position to slider value
                        // This is a simplified version - in practice you'd need proper track bounds
                        let window_width = window.width();
                        let slider_width = 250.0; // Approximate slider track width
                        let slider_right = window_width - 50.0; // Account for panel positioning
                        let slider_left = slider_right - slider_width;
                        
                        // Calculate relative position within slider bounds
                        let mouse_x = mouse_pos.x;
                        let relative_pos = if mouse_x >= slider_left && mouse_x <= slider_right {
                            (mouse_x - slider_left) / slider_width
                        } else {
                            // Clamp to slider bounds
                            if mouse_x < slider_left { 0.0 } else { 1.0 }
                        };
                        
                        // Convert to slider value
                        let new_value = slider.min_value + (slider.max_value - slider.min_value) * relative_pos;
                        slider.current_value = new_value.clamp(slider.min_value, slider.max_value);
                        
                        // Update handle position
                        let handle_pos = ((slider.current_value - slider.min_value) / 
                            (slider.max_value - slider.min_value)) * 100.0;
                        node.left = Val::Percent(handle_pos - 2.0); // Center the handle
                        
                        println!("Slider {:?}: {:.2}", slider.slider_type, slider.current_value);
                        break;
                    }
                }
            }
        }
    }

    // Update UI state based on slider values
    for (slider, _) in &slider_query {
        match slider.slider_type {
            SliderType::ObstacleDensity => {
                ui_state.current_obstacle_density = slider.current_value;
            }
            SliderType::EnemySpawnRate => {
                ui_state.enemy_spawn_rate = slider.current_value;
            }
            SliderType::TowerDamageMultiplier => {
                ui_state.tower_damage_multiplier = slider.current_value;
            }
        }
    }

    // Update text displays
    for (text_info, mut text) in &mut text_query {
        if let Some((slider, _)) = slider_query.iter().find(|(s, _)| s.slider_type == text_info.slider_type) {
            let label = match slider.slider_type {
                SliderType::ObstacleDensity => "Obstacle Density",
                SliderType::EnemySpawnRate => "Enemy Spawn Rate",
                SliderType::TowerDamageMultiplier => "Tower Damage",
            };
            **text = format!("{}: {:.2}", label, slider.current_value);
        }
    }
}

/// System to update the enemy path resource when UI parameters change
pub fn update_enemy_path_from_ui(
    mut commands: Commands,
    ui_state: Res<DebugUIState>,
    debug_state: Res<crate::systems::debug_visualization::DebugVisualizationState>,
    mut enemy_path: ResMut<EnemyPath>,
    path_line_query: Query<Entity, With<GamePathLine>>,
) {
    // Only update if UI state has changed and debug is enabled
    if ui_state.is_changed() && debug_state.enabled {
        // Generate new path with current UI parameters
        let new_path = crate::systems::path_generation::generate_level_path_with_params(
            debug_state.current_wave,
            ui_state.current_obstacle_density
        );
        
        // Remove old path line visualization
        for entity in path_line_query.iter() {
            commands.entity(entity).despawn();
        }
        
        // Create new path line visualization
        for i in 0..new_path.waypoints.len() - 1 {
            let start = new_path.waypoints[i];
            let end = new_path.waypoints[i + 1];
            let midpoint = (start + end) / 2.0;
            let length = start.distance(end);
            let angle = (end - start).angle_to(Vec2::X);

            commands.spawn((
                Sprite {
                    color: Color::srgb(0.3, 0.3, 0.3),
                    custom_size: Some(Vec2::new(length, 4.0)),
                    ..default()
                },
                Transform::from_translation(midpoint.extend(-1.0))
                    .with_rotation(Quat::from_rotation_z(angle)),
                GamePathLine,
            ));
        }
        
        // Update the enemy path resource that the game systems use
        *enemy_path = new_path;
        println!("Updated enemy path with obstacle density: {:.2}", ui_state.current_obstacle_density);
    }
}

/// System to update enemy spawn rate when UI parameters change
pub fn update_spawn_rate_from_ui(
    ui_state: Res<DebugUIState>,
    debug_state: Res<crate::systems::debug_visualization::DebugVisualizationState>,
    mut wave_manager: ResMut<crate::resources::WaveManager>,
) {
    // Only update if UI state has changed and debug is enabled
    if ui_state.is_changed() && debug_state.enabled {
        // Update the wave manager spawn rate based on the slider value
        wave_manager.set_spawn_rate(ui_state.enemy_spawn_rate);
        println!("Updated enemy spawn rate: {:.2} (interval: {:.2}s)", 
            ui_state.enemy_spawn_rate, 
            1.0 / ui_state.enemy_spawn_rate.max(0.1));
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
) {
    for (interaction, action_button, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
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