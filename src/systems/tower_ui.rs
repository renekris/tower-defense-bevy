use bevy::prelude::*;
use crate::resources::*;
use crate::components::*;
use crate::systems::input_system::MouseInputState;

// ============================================================================
// TOWER UI STATE MANAGEMENT
// ============================================================================

/// Resource to manage tower UI state for placement and upgrades
#[derive(Resource, Debug)]
pub struct TowerSelectionState {
    /// Current mode - placement or upgrade
    pub mode: TowerUIMode,
    /// Selected tower type for placement
    pub selected_placement_type: Option<TowerType>,
    /// Selected tower entity for upgrades
    pub selected_tower_entity: Option<Entity>,
    /// Whether the placement panel is visible
    pub placement_panel_visible: bool,
    /// Whether the upgrade panel is visible
    pub upgrade_panel_visible: bool,
}

impl Default for TowerSelectionState {
    fn default() -> Self {
        Self {
            mode: TowerUIMode::Placement,
            selected_placement_type: None,
            selected_tower_entity: None,
            placement_panel_visible: true,
            upgrade_panel_visible: false,
        }
    }
}

impl TowerSelectionState {
    pub fn set_placement_mode(&mut self, tower_type: Option<TowerType>) {
        self.mode = TowerUIMode::Placement;
        self.selected_placement_type = tower_type;
        self.selected_tower_entity = None;
        self.placement_panel_visible = true;
        self.upgrade_panel_visible = false;
    }

    pub fn set_upgrade_mode(&mut self, tower_entity: Entity) {
        self.mode = TowerUIMode::Upgrade;
        self.selected_placement_type = None;
        self.selected_tower_entity = Some(tower_entity);
        self.placement_panel_visible = true; // Keep placement panel visible
        self.upgrade_panel_visible = true;
    }

    pub fn clear_selection(&mut self) {
        self.mode = TowerUIMode::Placement;
        self.selected_placement_type = None;
        self.selected_tower_entity = None;
        self.upgrade_panel_visible = false;
    }

    pub fn is_placement_mode(&self) -> bool {
        matches!(self.mode, TowerUIMode::Placement)
    }

    pub fn is_upgrade_mode(&self) -> bool {
        matches!(self.mode, TowerUIMode::Upgrade)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TowerUIMode {
    Placement,
    Upgrade,
}

// ============================================================================
// UI COMPONENTS
// ============================================================================

/// Marker component for the tower placement panel
#[derive(Component)]
pub struct TowerPlacementPanel;

/// Marker component for the tower upgrade panel
#[derive(Component)]
pub struct TowerUpgradePanel;

/// Component for tower type selection buttons
#[derive(Component)]
pub struct TowerTypeButton {
    pub tower_type: TowerType,
}

/// Component for the upgrade button
#[derive(Component)]
pub struct UpgradeButton;

/// Component for selected tower indicator
#[derive(Component)]
pub struct SelectedTowerIndicator;

// ============================================================================
// UI SYSTEMS
// ============================================================================

/// System to handle tower clicking for upgrade selection
pub fn tower_selection_system(
    mut selection_state: ResMut<TowerSelectionState>,
    mouse_input: Res<MouseInputState>,
    towers_query: Query<(Entity, &Transform), With<TowerStats>>,
) {
    if mouse_input.left_clicked {
        // Check if we clicked on a tower
        let click_pos = mouse_input.world_position;
        let mut closest_tower = None;
        let mut closest_distance = f32::MAX;

        for (entity, transform) in towers_query.iter() {
            let tower_pos = transform.translation.truncate();
            let distance = click_pos.distance(tower_pos);
            
            // Tower click radius (slightly larger than tower size for easier clicking)
            if distance < 40.0 && distance < closest_distance {
                closest_distance = distance;
                closest_tower = Some(entity);
            }
        }

        if let Some(tower_entity) = closest_tower {
            selection_state.set_upgrade_mode(tower_entity);
            println!("Selected tower for upgrade: {:?}", tower_entity);
        } else {
            // If we didn't click on a tower and we're in upgrade mode, clear selection
            if selection_state.is_upgrade_mode() {
                selection_state.clear_selection();
                println!("Cleared tower selection");
            }
        }
    }

    // ESC key to clear any selection
    // TODO: This will be moved to the input system or removed when keyboard controls are removed
}

/// System to handle tower type button clicks
pub fn tower_type_button_system(
    mut selection_state: ResMut<TowerSelectionState>,
    mut interaction_query: Query<
        (&Interaction, &TowerTypeButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, tower_button, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                selection_state.set_placement_mode(Some(tower_button.tower_type));
                *color = Color::srgb(0.6, 0.6, 0.9).into(); // Highlight selected
                println!("Selected tower type: {:?}", tower_button.tower_type);
            }
            Interaction::Hovered => {
                *color = Color::srgb(0.8, 0.8, 1.0).into(); // Hover effect
            }
            Interaction::None => {
                // Check if this is the currently selected tower type
                if Some(tower_button.tower_type) == selection_state.selected_placement_type {
                    *color = Color::srgb(0.6, 0.6, 0.9).into(); // Keep highlighted
                } else {
                    *color = Color::srgb(0.7, 0.7, 0.7).into(); // Default color
                }
            }
        }
    }
}

/// System to handle upgrade button clicks
pub fn upgrade_button_system(
    selection_state: ResMut<TowerSelectionState>,
    mut economy: ResMut<Economy>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<UpgradeButton>),
    >,
    mut towers_query: Query<&mut TowerStats>,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            if let Some(tower_entity) = selection_state.selected_tower_entity {
                if let Ok(mut tower_stats) = towers_query.get_mut(tower_entity) {
                    let upgrade_cost = tower_stats.get_upgrade_cost();
                    
                    if economy.can_afford(&upgrade_cost) && tower_stats.can_upgrade() {
                        economy.spend(&upgrade_cost);
                        tower_stats.upgrade();
                        println!("Tower upgraded to level {}", tower_stats.upgrade_level);
                        *color = Color::srgb(0.4, 0.8, 0.4).into(); // Success feedback
                    } else {
                        println!("Cannot afford upgrade or tower at max level");
                        *color = Color::srgb(0.8, 0.4, 0.4).into(); // Error feedback
                    }
                }
            }
        } else if *interaction == Interaction::Hovered {
            *color = Color::srgb(0.6, 0.9, 0.6).into(); // Hover effect
        } else {
            *color = Color::srgb(0.5, 0.8, 0.5).into(); // Default color
        }
    }
}

/// System to update selected tower visual indicator
pub fn selected_tower_indicator_system(
    mut commands: Commands,
    selection_state: Res<TowerSelectionState>,
    indicator_query: Query<Entity, With<SelectedTowerIndicator>>,
    towers_query: Query<&Transform, With<TowerStats>>,
) {
    // Remove existing indicators
    for entity in indicator_query.iter() {
        commands.entity(entity).despawn();
    }

    // Add indicator for selected tower
    if let Some(tower_entity) = selection_state.selected_tower_entity {
        if let Ok(tower_transform) = towers_query.get(tower_entity) {
            commands.spawn((
                Sprite {
                    color: Color::srgb(1.0, 1.0, 0.0), // Yellow selection ring
                    custom_size: Some(Vec2::new(50.0, 50.0)),
                    ..default()
                },
                Transform::from_translation(
                    tower_transform.translation + Vec3::new(0.0, 0.0, -0.5)
                ),
                SelectedTowerIndicator,
            ));
        }
    }
}

// ============================================================================
// UI SETUP FUNCTIONS
// ============================================================================

/// Setup the tower placement UI panel
pub fn setup_tower_placement_panel(mut commands: Commands) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(20.0),
                top: Val::Px(20.0),
                width: Val::Px(200.0),
                height: Val::Px(300.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(10.0)),
                row_gap: Val::Px(5.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 0.9)),
            TowerPlacementPanel,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("Tower Placement"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            // Tower type buttons
            create_tower_type_button(parent, TowerType::Basic, "Basic Tower\n$25");
            create_tower_type_button(parent, TowerType::Advanced, "Advanced Tower\n$50 + Materials");
            create_tower_type_button(parent, TowerType::Laser, "Laser Tower\n$75 + Research");
            create_tower_type_button(parent, TowerType::Missile, "Missile Tower\n$100 + Materials");
            create_tower_type_button(parent, TowerType::Tesla, "Tesla Tower\n$120 + Energy");

            // Clear selection button
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(30.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::top(Val::Px(10.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.6, 0.4, 0.4)),
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new("Clear Selection"),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
        });
}

/// Setup the tower upgrade UI panel
pub fn setup_tower_upgrade_panel(mut commands: Commands) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(240.0), // Next to placement panel
                top: Val::Px(20.0),
                width: Val::Px(250.0),
                height: Val::Px(350.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(10.0)),
                row_gap: Val::Px(5.0),
                display: Display::None, // Hidden by default
                ..default()
            },
            BackgroundColor(Color::srgba(0.1, 0.1, 0.3, 0.9)),
            TowerUpgradePanel,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("Tower Upgrade"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                UpgradePanelTitle,
            ));

            // Tower info section
            parent.spawn((
                Text::new("Select a tower to upgrade"),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
                TowerInfoText,
            ));

            // Current stats section
            parent.spawn((
                Text::new(""),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                CurrentStatsText,
            ));

            // Upgrade preview section
            parent.spawn((
                Text::new(""),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 1.0, 0.7)),
                UpgradePreviewText,
            ));

            // Cost section
            parent.spawn((
                Text::new(""),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.8, 0.6)),
                UpgradeCostText,
            ));

            // Upgrade button
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(40.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::top(Val::Px(15.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.5, 0.8, 0.5)),
                    UpgradeButton,
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new("UPGRADE"),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        UpgradeButtonText,
                    ));
                });
        });
}

/// Helper function to create tower type buttons
fn create_tower_type_button(parent: &mut ChildSpawnerCommands, tower_type: TowerType, text: &str) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(45.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.7, 0.7, 0.7)),
            TowerTypeButton { tower_type },
        ))
        .with_children(|button| {
            button.spawn((
                Text::new(text),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(Color::BLACK),
            ));
        });
}

// ============================================================================
// UI UPDATE COMPONENTS
// ============================================================================

/// Marker components for UI text elements that need updates
#[derive(Component)]
pub struct UpgradePanelTitle;

#[derive(Component)]
pub struct TowerInfoText;

#[derive(Component)]
pub struct CurrentStatsText;

#[derive(Component)]
pub struct UpgradePreviewText;

#[derive(Component)]
pub struct UpgradeCostText;

#[derive(Component)]
pub struct UpgradeButtonText;

// ============================================================================
// UI UPDATE SYSTEMS  
// ============================================================================

/// System to update the upgrade panel content based on selected tower
pub fn update_upgrade_panel_system(
    selection_state: Res<TowerSelectionState>,
    economy: Res<Economy>,
    towers_query: Query<&TowerStats>,
    mut panel_query: Query<&mut Node, With<TowerUpgradePanel>>,
    mut tower_info_query: Query<&mut Text, (With<TowerInfoText>, Without<CurrentStatsText>, Without<UpgradePreviewText>, Without<UpgradeCostText>, Without<UpgradeButtonText>)>,
    mut current_stats_query: Query<&mut Text, (With<CurrentStatsText>, Without<TowerInfoText>, Without<UpgradePreviewText>, Without<UpgradeCostText>, Without<UpgradeButtonText>)>,
    mut upgrade_preview_query: Query<&mut Text, (With<UpgradePreviewText>, Without<TowerInfoText>, Without<CurrentStatsText>, Without<UpgradeCostText>, Without<UpgradeButtonText>)>,
    mut upgrade_cost_query: Query<&mut Text, (With<UpgradeCostText>, Without<TowerInfoText>, Without<CurrentStatsText>, Without<UpgradePreviewText>, Without<UpgradeButtonText>)>,
    mut upgrade_button_query: Query<&mut Text, (With<UpgradeButtonText>, Without<TowerInfoText>, Without<CurrentStatsText>, Without<UpgradePreviewText>, Without<UpgradeCostText>)>,
    mut upgrade_button_style_query: Query<&mut BackgroundColor, With<UpgradeButton>>,
) {
    // Show/hide upgrade panel
    if let Ok(mut panel_node) = panel_query.single_mut() {
        panel_node.display = if selection_state.upgrade_panel_visible {
            Display::Flex
        } else {
            Display::None
        };
    }

    // Update panel content if a tower is selected
    if let Some(tower_entity) = selection_state.selected_tower_entity {
        if let Ok(tower_stats) = towers_query.get(tower_entity) {
            // Update tower info
            if let Ok(mut text) = tower_info_query.single_mut() {
                **text = format!("{} Tower (Level {})", 
                    match tower_stats.tower_type {
                        TowerType::Basic => "Basic",
                        TowerType::Advanced => "Advanced", 
                        TowerType::Laser => "Laser",
                        TowerType::Missile => "Missile",
                        TowerType::Tesla => "Tesla",
                    },
                    tower_stats.upgrade_level
                );
            }

            // Update current stats
            if let Ok(mut text) = current_stats_query.single_mut() {
                **text = format!(
                    "Current Stats:\nDamage: {:.1}\nRange: {:.1}\nFire Rate: {:.1}",
                    tower_stats.damage,
                    tower_stats.range,
                    tower_stats.fire_rate
                );
            }

            // Update upgrade preview
            if let Ok(mut text) = upgrade_preview_query.single_mut() {
                if tower_stats.can_upgrade() {
                    let mut preview_stats = tower_stats.clone();
                    preview_stats.upgrade();
                    
                    **text = format!(
                        "After Upgrade:\nDamage: {:.1} (+{:.1})\nRange: {:.1} (+{:.1})\nFire Rate: {:.1} (+{:.1})",
                        preview_stats.damage, preview_stats.damage - tower_stats.damage,
                        preview_stats.range, preview_stats.range - tower_stats.range,
                        preview_stats.fire_rate, preview_stats.fire_rate - tower_stats.fire_rate
                    );
                } else {
                    **text = "Max level reached!".to_string();
                }
            }

            // Update upgrade cost
            if let Ok(mut text) = upgrade_cost_query.single_mut() {
                if tower_stats.can_upgrade() {
                    let cost = tower_stats.get_upgrade_cost();
                    **text = format!(
                        "Upgrade Cost:\nMoney: ${}\nResearch: {}\nMaterials: {}\nEnergy: {}",
                        cost.money, cost.research_points, cost.materials, cost.energy
                    );
                } else {
                    **text = "".to_string();
                }
            }

            // Update upgrade button
            if let Ok(mut text) = upgrade_button_query.single_mut() {
                if let Ok(mut color) = upgrade_button_style_query.single_mut() {
                    if tower_stats.can_upgrade() {
                        let cost = tower_stats.get_upgrade_cost();
                        if economy.can_afford(&cost) {
                            **text = "UPGRADE".to_string();
                            *color = Color::srgb(0.5, 0.8, 0.5).into(); // Green
                        } else {
                            **text = "CAN'T AFFORD".to_string();
                            *color = Color::srgb(0.8, 0.4, 0.4).into(); // Red
                        }
                    } else {
                        **text = "MAX LEVEL".to_string();
                        *color = Color::srgb(0.6, 0.6, 0.6).into(); // Gray
                    }
                }
            }
        }
    } else {
        // No tower selected - reset text
        if let Ok(mut text) = tower_info_query.single_mut() {
            **text = "Select a tower to upgrade".to_string();
        }
        if let Ok(mut text) = current_stats_query.single_mut() {
            **text = "".to_string();
        }
        if let Ok(mut text) = upgrade_preview_query.single_mut() {
            **text = "".to_string();
        }
        if let Ok(mut text) = upgrade_cost_query.single_mut() {
            **text = "".to_string();
        }
        if let Ok(mut text) = upgrade_button_query.single_mut() {
            **text = "SELECT TOWER".to_string();
        }
    }
}