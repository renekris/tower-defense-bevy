use bevy::prelude::*;
use crate::resources::*;
use crate::components::*;
use crate::systems::combat_system::{WaveStatus, Target};
use super::cheat_menu::*;

/// System to handle cheat button interactions
pub fn handle_cheat_button_interactions(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &CheatButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut economy: ResMut<Economy>,
    mut cheat_state: ResMut<CheatMenuState>,
    mut wave_manager: ResMut<WaveManager>,
    mut wave_status: ResMut<WaveStatus>,
    mut game_state: ResMut<GameState>,
    enemy_query: Query<Entity, With<Enemy>>,
    projectile_query: Query<Entity, With<Projectile>>,
    tower_query: Query<Entity, With<TowerStats>>,
    mut god_mode_text_query: Query<&mut Text, (With<CheatButton>, Without<CheatSliderValueText>)>,
    // Consume mouse clicks to prevent pass-through
    mut mouse_input_state: ResMut<crate::systems::input_system::MouseInputState>,
) {
    for (interaction, cheat_button, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                // Consume the mouse click to prevent pass-through to game world
                mouse_input_state.left_clicked = false;
                
                match cheat_button.button_type {
                    // Currency cheats
                    CheatButtonType::AddMoney100 => {
                        economy.money += 100;
                        println!("Cheat: Added 100 money. Total: {}", economy.money);
                    }
                    CheatButtonType::AddMoney1K => {
                        economy.money += 1000;
                        println!("Cheat: Added 1K money. Total: {}", economy.money);
                    }
                    CheatButtonType::AddMoney10K => {
                        economy.money += 10000;
                        println!("Cheat: Added 10K money. Total: {}", economy.money);
                    }
                    CheatButtonType::SetMoneyMax => {
                        economy.money = 999999;
                        println!("Cheat: Set money to maximum: {}", economy.money);
                    }
                    CheatButtonType::AddResearch10 => {
                        economy.research_points += 10;
                        println!("Cheat: Added 10 research points. Total: {}", economy.research_points);
                    }
                    CheatButtonType::AddResearch100 => {
                        economy.research_points += 100;
                        println!("Cheat: Added 100 research points. Total: {}", economy.research_points);
                    }
                    CheatButtonType::SetResearchMax => {
                        economy.research_points = 999999;
                        println!("Cheat: Set research points to maximum: {}", economy.research_points);
                    }
                    CheatButtonType::AddMaterials10 => {
                        economy.materials += 10;
                        println!("Cheat: Added 10 materials. Total: {}", economy.materials);
                    }
                    CheatButtonType::AddMaterials100 => {
                        economy.materials += 100;
                        println!("Cheat: Added 100 materials. Total: {}", economy.materials);
                    }
                    CheatButtonType::SetMaterialsMax => {
                        economy.materials = 999999;
                        println!("Cheat: Set materials to maximum: {}", economy.materials);
                    }
                    CheatButtonType::AddEnergy10 => {
                        economy.energy += 10;
                        println!("Cheat: Added 10 energy. Total: {}", economy.energy);
                    }
                    CheatButtonType::AddEnergy100 => {
                        economy.energy += 100;
                        println!("Cheat: Added 100 energy. Total: {}", economy.energy);
                    }
                    CheatButtonType::SetEnergyMax => {
                        economy.energy = 999999;
                        println!("Cheat: Set energy to maximum: {}", economy.energy);
                    }
                    CheatButtonType::ResetAllResources => {
                        let default_economy = Economy::default();
                        economy.money = default_economy.money;
                        economy.research_points = default_economy.research_points;
                        economy.materials = default_economy.materials;
                        economy.energy = default_economy.energy;
                        println!("Cheat: Reset all resources to default values");
                    }
                    
                    // Game state cheats
                    CheatButtonType::NextWave => {
                        // Clear all current enemies to force wave completion
                        for entity in enemy_query.iter() {
                            commands.entity(entity).despawn();
                        }
                        wave_status.enemies_remaining = 0;
                        wave_status.wave_complete = true;
                        wave_manager.current_wave += 1;
                        println!("Cheat: Skipped to next wave: {}", wave_manager.current_wave);
                    }
                    CheatButtonType::InstantWin => {
                        // Clear all enemies and set game to victory
                        for entity in enemy_query.iter() {
                            commands.entity(entity).despawn();
                        }
                        *game_state = GameState::Victory;
                        println!("Cheat: Instant victory activated!");
                    }
                    CheatButtonType::ResetGame => {
                        println!("Cheat: Resetting game state...");
                        
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
                        
                        // Reset resources to default
                        let default_economy = Economy::default();
                        *economy = default_economy;
                        
                        // Reset wave manager
                        wave_manager.current_wave = 0;
                        wave_manager.enemies_in_wave = 0;
                        wave_manager.enemies_spawned = 0;
                        
                        // Reset wave status
                        wave_status.enemies_remaining = 0;
                        wave_status.enemies_killed = 0;
                        wave_status.enemies_escaped = 0;
                        wave_status.wave_complete = false;
                        
                        // Reset game state
                        *game_state = GameState::Playing;
                        
                        // Disable god mode
                        cheat_state.god_mode = false;
                        
                        println!("Cheat: Game reset complete!");
                    }
                    CheatButtonType::ToggleGodMode => {
                        cheat_state.god_mode = !cheat_state.god_mode;
                        println!("Cheat: God mode {}", if cheat_state.god_mode { "ON" } else { "OFF" });
                        
                        // Update button text - we'll handle this in a separate system for clarity
                    }
                }
                
                // Visual feedback for button press
                *color = Color::srgba(1.0, 1.0, 1.0, 1.0).into();
            }
            Interaction::Hovered => {
                // Button hover effect
                let hover_color = get_button_hover_color(cheat_button.button_type);
                *color = hover_color.into();
            }
            Interaction::None => {
                // Return to normal color
                let normal_color = get_button_normal_color(cheat_button.button_type);
                *color = normal_color.into();
            }
        }
    }
}

/// System to handle cheat slider interactions
pub fn handle_cheat_slider_interactions(
    mut interaction_query: Query<
        (&Interaction, &CheatSlider, &mut BackgroundColor),
        (Changed<Interaction>, With<CheatSliderHandle>),
    >,
    mut drag_state: ResMut<CheatSliderDragState>,
    // Consume mouse clicks to prevent pass-through
    mut mouse_input_state: ResMut<crate::systems::input_system::MouseInputState>,
) {
    for (interaction, slider, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                // Consume the mouse click to prevent pass-through to game world
                mouse_input_state.left_clicked = false;
                
                drag_state.dragging = Some(slider.slider_type);
                *color = Color::srgba(0.6, 0.6, 1.0, 1.0).into(); // Blue when dragging
                println!("Started dragging cheat slider: {:?}", slider.slider_type);
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

/// System to update cheat slider values
pub fn update_cheat_slider_values(
    mut slider_query: Query<(&mut CheatSlider, &mut Node), With<CheatSliderHandle>>,
    mut multipliers: ResMut<CheatMultipliers>,
    mut text_query: Query<(&CheatSliderValueText, &mut Text)>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut drag_state: ResMut<CheatSliderDragState>,
    windows: Query<&Window>,
    cheat_state: Res<CheatMenuState>,
) {
    // Only process if cheat menu is visible
    if !cheat_state.visible {
        return;
    }
    
    // Stop dragging when mouse is released
    if !mouse_input.pressed(MouseButton::Left) && drag_state.dragging.is_some() {
        println!("Stopped dragging cheat slider");
        drag_state.dragging = None;
    }

    // Update slider values based on mouse position when dragging
    if let Some(dragging_type) = drag_state.dragging {
        if let Ok(window) = windows.single() {
            if let Some(mouse_pos) = window.cursor_position() {
                // Find the dragging slider and update its position
                for (mut slider, mut node) in &mut slider_query {
                    if slider.slider_type == dragging_type {
                        // Calculate relative position within the cheat menu panel
                        // Since the cheat menu is centered, calculate from panel bounds
                        let panel_width = 370.0; // Panel width minus padding
                        let panel_center_x = window.width() / 2.0;
                        let slider_left = panel_center_x - panel_width / 2.0;
                        let slider_right = panel_center_x + panel_width / 2.0;
                        
                        // Calculate relative position within slider bounds
                        let mouse_x = mouse_pos.x;
                        let relative_pos = if mouse_x >= slider_left && mouse_x <= slider_right {
                            (mouse_x - slider_left) / panel_width
                        } else {
                            // Clamp to slider bounds
                            if mouse_x < slider_left { 0.0 } else { 1.0 }
                        };
                        
                        // Convert to slider value
                        let new_value = slider.min_value + (slider.max_value - slider.min_value) * relative_pos;
                        let clamped_value = new_value.clamp(slider.min_value, slider.max_value);
                        
                        // Only update if value actually changed to prevent spam
                        if (slider.current_value - clamped_value).abs() > 0.01 {
                            slider.current_value = clamped_value;
                            
                            // Update handle position
                            let handle_pos = ((slider.current_value - slider.min_value) / 
                                (slider.max_value - slider.min_value)) * 100.0;
                            node.left = Val::Percent(handle_pos - 2.0); // Center the handle
                            
                            println!("Cheat slider {:?} updated to: {:.2}", slider.slider_type, clamped_value);
                        }
                        break;
                    }
                }
            }
        }
    }

    // Update multipliers based on slider values
    for (slider, _) in &slider_query {
        match slider.slider_type {
            CheatSliderType::TowerDamage => {
                multipliers.tower_damage = slider.current_value;
            }
            CheatSliderType::TowerRange => {
                multipliers.tower_range = slider.current_value;
            }
            CheatSliderType::TowerFireRate => {
                multipliers.tower_fire_rate = slider.current_value;
            }
            CheatSliderType::EnemyHealth => {
                multipliers.enemy_health = slider.current_value;
            }
            CheatSliderType::EnemySpeed => {
                multipliers.enemy_speed = slider.current_value;
            }
        }
    }

    // Update text displays
    for (text_info, mut text) in &mut text_query {
        if let Some((slider, _)) = slider_query.iter().find(|(s, _)| s.slider_type == text_info.slider_type) {
            let label = match slider.slider_type {
                CheatSliderType::TowerDamage => "Tower Damage",
                CheatSliderType::TowerRange => "Tower Range",
                CheatSliderType::TowerFireRate => "Fire Rate",
                CheatSliderType::EnemyHealth => "Enemy Health",
                CheatSliderType::EnemySpeed => "Enemy Speed",
            };
            **text = format!("{}: {:.2}x", label, slider.current_value);
        }
    }
}

/// System to update god mode button text
pub fn update_god_mode_button_text(
    cheat_state: Res<CheatMenuState>,
    // Remove broken parent query for now - god mode button text will be handled elsewhere
    // mut button_query: Query<(&mut Text, &Parent), (With<CheatButton>, Without<CheatSliderValueText>)>,
    parent_query: Query<&CheatButton, Without<Text>>,
) {
    // TODO: Implement god mode button text update when Bevy hierarchy access is fixed
    // For now, the button text change is handled via console output
}

/// Helper function to get button hover colors
fn get_button_hover_color(button_type: CheatButtonType) -> Color {
    match button_type {
        // Currency buttons - brighter green
        CheatButtonType::AddMoney100 | CheatButtonType::AddMoney1K | CheatButtonType::AddMoney10K |
        CheatButtonType::SetMoneyMax | CheatButtonType::AddResearch10 | CheatButtonType::AddResearch100 |
        CheatButtonType::SetResearchMax | CheatButtonType::AddMaterials10 | CheatButtonType::AddMaterials100 |
        CheatButtonType::SetMaterialsMax | CheatButtonType::AddEnergy10 | CheatButtonType::AddEnergy100 |
        CheatButtonType::SetEnergyMax => Color::srgb(0.4, 0.7, 0.4),
        
        // Reset button - brighter red
        CheatButtonType::ResetAllResources | CheatButtonType::ResetGame => Color::srgb(0.8, 0.3, 0.3),
        
        // Game state buttons
        CheatButtonType::NextWave => Color::srgb(0.4, 0.4, 0.8),
        CheatButtonType::InstantWin => Color::srgb(0.4, 0.8, 0.4),
        CheatButtonType::ToggleGodMode => Color::srgb(0.8, 0.8, 0.4),
    }
}

/// Helper function to get button normal colors
fn get_button_normal_color(button_type: CheatButtonType) -> Color {
    match button_type {
        // Currency buttons - green
        CheatButtonType::AddMoney100 | CheatButtonType::AddMoney1K | CheatButtonType::AddMoney10K |
        CheatButtonType::SetMoneyMax | CheatButtonType::AddResearch10 | CheatButtonType::AddResearch100 |
        CheatButtonType::SetResearchMax | CheatButtonType::AddMaterials10 | CheatButtonType::AddMaterials100 |
        CheatButtonType::SetMaterialsMax | CheatButtonType::AddEnergy10 | CheatButtonType::AddEnergy100 |
        CheatButtonType::SetEnergyMax => Color::srgb(0.3, 0.5, 0.3),
        
        // Reset button - red
        CheatButtonType::ResetAllResources | CheatButtonType::ResetGame => Color::srgb(0.6, 0.2, 0.2),
        
        // Game state buttons
        CheatButtonType::NextWave => Color::srgb(0.3, 0.3, 0.7),
        CheatButtonType::InstantWin => Color::srgb(0.3, 0.7, 0.3),
        CheatButtonType::ToggleGodMode => Color::srgb(0.7, 0.7, 0.3),
    }
}