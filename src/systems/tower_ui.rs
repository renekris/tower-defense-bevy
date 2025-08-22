use bevy::prelude::*;
use crate::resources::*;
use crate::components::*;
use crate::systems::input_system::MouseInputState;
use crate::systems::enemy_system::StartWaveEvent;

// ============================================================================
// UI COLOR CONSTANTS
// ============================================================================

/// Professional color palette inspired by Bloons TD6 with enhanced visual hierarchy
struct UIColors;

impl UIColors {
    // Panel colors - Enhanced with subtle gradients
    const PANEL_BG: Color = Color::srgb(0.08, 0.12, 0.18);           // Darker, more premium feel
    const PANEL_BORDER: Color = Color::srgb(0.22, 0.28, 0.38);       // Softer border
    const PANEL_SHADOW: Color = Color::srgb(0.02, 0.03, 0.08);       // Shadow/depth
    const HEADER_BG: Color = Color::srgb(0.06, 0.10, 0.16);          // Even darker header
    
    // Button states - Improved contrast and visual feedback
    const BUTTON_DEFAULT: Color = Color::srgb(0.15, 0.20, 0.28);     // Slightly darker default
    const BUTTON_HOVER: Color = Color::srgb(0.20, 0.28, 0.38);       // More pronounced hover
    const BUTTON_SELECTED: Color = Color::srgb(0.12, 0.40, 0.70);    // Refined blue selection
    const BUTTON_SELECTED_HOVER: Color = Color::srgb(0.18, 0.48, 0.78); // Enhanced selected hover
    const BUTTON_DISABLED: Color = Color::srgb(0.10, 0.12, 0.16);    // Clearly disabled state
    
    // Border colors - Better visual hierarchy
    const BORDER_DEFAULT: Color = Color::srgb(0.32, 0.38, 0.48);     // Subtle default
    const BORDER_HOVER: Color = Color::srgb(0.48, 0.58, 0.70);       // Clear hover indication
    const BORDER_SELECTED: Color = Color::srgb(0.35, 0.60, 0.85);    // Strong selection
    const BORDER_SELECTED_HOVER: Color = Color::srgb(0.45, 0.70, 0.95); // Bright selected hover
    const BORDER_DISABLED: Color = Color::srgb(0.18, 0.22, 0.28);    // Muted disabled
    
    // Text colors - Enhanced readability and hierarchy
    const TEXT_PRIMARY: Color = Color::srgb(0.96, 0.96, 0.98);       // Crisp white text
    const TEXT_SECONDARY: Color = Color::srgb(0.78, 0.82, 0.88);     // Clear secondary
    const TEXT_MUTED: Color = Color::srgb(0.58, 0.62, 0.68);         // Subtle muted text
    const TEXT_ACCENT: Color = Color::srgb(0.88, 0.92, 0.62);        // Warmer yellow
    const TEXT_SUCCESS: Color = Color::srgb(0.58, 0.88, 0.68);       // Clear green
    const TEXT_WARNING: Color = Color::srgb(1.0, 0.78, 0.58);        // Warm orange
    const TEXT_ERROR: Color = Color::srgb(1.0, 0.58, 0.58);          // Clear red
    const TEXT_INFO: Color = Color::srgb(0.58, 0.78, 1.0);           // Cool blue info
    
    // Tooltip colors - Enhanced readability
    const TOOLTIP_BG: Color = Color::srgba(0.02, 0.05, 0.12, 0.96);  // Darker, more opaque
    const TOOLTIP_BORDER: Color = Color::srgb(0.38, 0.48, 0.62);     // Clearer border
    const TOOLTIP_SHADOW: Color = Color::srgba(0.0, 0.0, 0.0, 0.4);  // Subtle shadow
    
    // Resource colors - Better visual coding
    const RESOURCE_BG: Color = Color::srgb(0.06, 0.08, 0.12);        // Consistent with panel
    const RESOURCE_BORDER: Color = Color::srgb(0.18, 0.22, 0.28);    // Subtle border
    const RESOURCE_MONEY: Color = Color::srgb(0.88, 0.92, 0.62);     // Gold for money
    const RESOURCE_RESEARCH: Color = Color::srgb(0.58, 0.78, 1.0);   // Blue for research
    const RESOURCE_MATERIALS: Color = Color::srgb(0.78, 0.68, 0.58); // Brown for materials
    const RESOURCE_ENERGY: Color = Color::srgb(0.88, 0.58, 0.88);    // Purple for energy
    
    // Cost affordability colors
    const COST_AFFORDABLE: Color = Color::srgb(0.58, 0.88, 0.68);    // Green when affordable
    const COST_EXPENSIVE: Color = Color::srgb(1.0, 0.78, 0.58);      // Orange when expensive
    const COST_UNAFFORDABLE: Color = Color::srgb(1.0, 0.58, 0.58);   // Red when can't afford
}

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

/// Resource to manage tower stat popup state
#[derive(Resource, Debug)]
pub struct TowerStatPopupState {
    /// Currently displayed tower type in popup (None = hidden)
    pub active_tower_type: Option<TowerType>,
    /// Position where the popup should appear
    pub position: Vec2,
    /// Whether popup is visible
    pub visible: bool,
}

impl Default for TowerStatPopupState {
    fn default() -> Self {
        Self {
            active_tower_type: None,
            position: Vec2::new(300.0, 200.0),
            visible: false,
        }
    }
}

impl TowerStatPopupState {
    pub fn show_for_tower(&mut self, tower_type: TowerType, position: Vec2) {
        self.active_tower_type = Some(tower_type);
        self.position = position;
        self.visible = true;
    }

    pub fn hide(&mut self) {
        self.active_tower_type = None;
        self.visible = false;
    }

    pub fn is_showing(&self) -> bool {
        self.visible && self.active_tower_type.is_some()
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

/// Component for tower tooltips
#[derive(Component)]
pub struct TowerTooltip;

/// Component for tower stat popup
#[derive(Component)]
pub struct TowerStatPopup;

/// Component for popup trigger linking tower buttons to popups
#[derive(Component)]
pub struct PopupTrigger {
    pub tower_type: TowerType,
}

/// Component for popup content sections
#[derive(Component)]
pub struct PopupHeader;

#[derive(Component)]
pub struct PopupStatsSection;

#[derive(Component)]
pub struct PopupCostSection;

#[derive(Component)]
pub struct PopupDescriptionSection;

#[derive(Component)]
pub struct PopupUpgradeSection;

/// Component for popup close button
#[derive(Component)]
pub struct PopupCloseButton;

/// Component for resource status display
#[derive(Component)]
pub struct ResourceStatus;

/// Component for hover state management
#[derive(Component)]
pub struct HoverState {
    pub is_hovered: bool,
    pub tower_type: TowerType,
}

/// Component for the Start Wave button
#[derive(Component)]
pub struct StartWaveButton;

/// Component for the Start Wave button text (for updates)
#[derive(Component)]
pub struct StartWaveButtonText;

// ============================================================================
// UI SYSTEMS
// ============================================================================

/// System to handle tower clicking for upgrade selection with right-click unselection
pub fn tower_selection_system(
    mut selection_state: ResMut<TowerSelectionState>,
    mouse_input: Res<MouseInputState>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    towers_query: Query<(Entity, &Transform), With<TowerStats>>,
) {
    // Handle right-click unselection
    if mouse_button_input.just_pressed(MouseButton::Right) {
        if selection_state.selected_placement_type.is_some() || selection_state.selected_tower_entity.is_some() {
            selection_state.clear_selection();
            println!("Right-click: Cleared all tower selections");
        }
        return; // Exit early to prevent left-click processing
    }

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

/// System to handle tower type button clicks with enhanced styling and popup triggers
pub fn tower_type_button_system(
    mut selection_state: ResMut<TowerSelectionState>,
    mut popup_state: ResMut<TowerStatPopupState>,
    mut mouse_input_state: ResMut<MouseInputState>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut button_queries: ParamSet<(
        // Query for handling interactions (Changed<Interaction>)
        Query<
            (&Interaction, &TowerTypeButton, &mut BackgroundColor, &mut BorderColor, &mut HoverState, &GlobalTransform),
            (Changed<Interaction>, With<Button>),
        >,
        // Query for updating all buttons when selection state changes
        Query<
            (&TowerTypeButton, &mut BackgroundColor, &mut BorderColor, &HoverState),
            With<Button>,
        >,
    )>,
) {
    // First, handle button interactions using the first query
    {
        let mut interaction_query = button_queries.p0();
        for (interaction, tower_button, mut bg_color, mut border_color, mut hover_state, global_transform) in interaction_query.iter_mut() {
            let is_selected = Some(tower_button.tower_type) == selection_state.selected_placement_type;
            
            match *interaction {
                Interaction::Pressed => {
                    // Check which mouse button was pressed
                    if mouse_button_input.pressed(MouseButton::Left) {
                        // Left click: Select tower for placement (existing functionality)
                        mouse_input_state.left_clicked = false;
                        selection_state.set_placement_mode(Some(tower_button.tower_type));
                        *bg_color = UIColors::BUTTON_SELECTED.into();
                        *border_color = UIColors::BORDER_SELECTED.into();
                        println!("Selected tower type: {:?}", tower_button.tower_type);
                    } else if mouse_button_input.pressed(MouseButton::Right) {
                        // Right click: Show stat popup
                        let button_pos = global_transform.translation().truncate();
                        // Position popup to the left of the button to avoid UI overlap
                        let popup_pos = Vec2::new(button_pos.x - 320.0, button_pos.y);
                        popup_state.show_for_tower(tower_button.tower_type, popup_pos);
                        println!("Showing stat popup for tower: {:?}", tower_button.tower_type);
                    }
                }
                Interaction::Hovered => {
                    hover_state.is_hovered = true;
                    if is_selected {
                        *bg_color = UIColors::BUTTON_SELECTED_HOVER.into();
                        *border_color = UIColors::BORDER_SELECTED_HOVER.into();
                    } else {
                        *bg_color = UIColors::BUTTON_HOVER.into();
                        *border_color = UIColors::BORDER_HOVER.into();
                    }
                }
                Interaction::None => {
                    hover_state.is_hovered = false;
                    if is_selected {
                        *bg_color = UIColors::BUTTON_SELECTED.into();
                        *border_color = UIColors::BORDER_SELECTED.into();
                    } else {
                        *bg_color = UIColors::BUTTON_DEFAULT.into();
                        *border_color = UIColors::BORDER_DEFAULT.into();
                    }
                }
            }
        }
    }

    // Then, update ALL buttons when selection state changes using the second query
    if selection_state.is_changed() {
        let mut all_buttons_query = button_queries.p1();
        for (tower_button, mut bg_color, mut border_color, hover_state) in all_buttons_query.iter_mut() {
            let is_selected = Some(tower_button.tower_type) == selection_state.selected_placement_type;
            
            if is_selected {
                if hover_state.is_hovered {
                    *bg_color = UIColors::BUTTON_SELECTED_HOVER.into();
                    *border_color = UIColors::BORDER_SELECTED_HOVER.into();
                } else {
                    *bg_color = UIColors::BUTTON_SELECTED.into();
                    *border_color = UIColors::BORDER_SELECTED.into();
                }
            } else {
                if hover_state.is_hovered {
                    *bg_color = UIColors::BUTTON_HOVER.into();
                    *border_color = UIColors::BORDER_HOVER.into();
                } else {
                    *bg_color = UIColors::BUTTON_DEFAULT.into();
                    *border_color = UIColors::BORDER_DEFAULT.into();
                }
            }
        }
    }
}

/// System to handle upgrade button clicks
pub fn upgrade_button_system(
    selection_state: ResMut<TowerSelectionState>,
    mut economy: ResMut<Economy>,
    mut mouse_input_state: ResMut<MouseInputState>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<UpgradeButton>),
    >,
    mut towers_query: Query<&mut TowerStats>,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            // CRITICAL FIX: Consume the mouse click to prevent tower placement
            mouse_input_state.left_clicked = false;
            
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

/// Setup the Bloons TD6-style tower placement UI panel
pub fn setup_tower_placement_panel(mut commands: Commands) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(20.0),
                top: Val::Px(20.0),
                width: Val::Px(250.0),   // Slightly wider for better proportions
                height: Val::Px(420.0),  // Taller to accommodate better spacing
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(16.0)),  // More generous padding
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(UIColors::PANEL_BG),
            BorderColor(UIColors::PANEL_BORDER),
            TowerPlacementPanel,
        ))
        .with_children(|parent| {
            // Panel title with enhanced typography and visual hierarchy
            parent.spawn((
                Text::new("TOWER SELECTION"),
                TextFont {
                    font_size: 20.0,  // More prominent title
                    ..default()
                },
                TextColor(UIColors::TEXT_PRIMARY),
                Node {
                    margin: UiRect::bottom(Val::Px(18.0)),  // Better spacing
                    align_self: AlignSelf::Center,
                    ..default()
                },
            ));

            // 2-column grid container for tower buttons
            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(12.0),  // Better vertical rhythm
                    ..default()
                },
            )).with_children(|grid_container| {
                // Row 1: Basic (B) + Advanced (A)
                create_tower_button_row(grid_container, 
                    &[(TowerType::Basic, 'B'), (TowerType::Advanced, 'A')]);
                
                // Row 2: Laser (L) + Missile (M)
                create_tower_button_row(grid_container, 
                    &[(TowerType::Laser, 'L'), (TowerType::Missile, 'M')]);
                
                // Row 3: Tesla (T) - full width
                create_tower_button_full_width(grid_container, TowerType::Tesla, 'T');
            });

            // Resource status footer
            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(40.0),
                    padding: UiRect::all(Val::Px(8.0)),
                    margin: UiRect::top(Val::Px(15.0)),
                    border: UiRect::all(Val::Px(1.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(UIColors::RESOURCE_BG),
                BorderColor(UIColors::RESOURCE_BORDER),
                ResourceStatus,
            )).with_children(|footer| {
                footer.spawn((
                    Text::new("$-- | R:-- | M:-- | E:--"),
                    TextFont {
                        font_size: 12.0,
                        ..default()
                    },
                    TextColor(UIColors::TEXT_ACCENT),
                    ResourceStatusText,
                ));
            });

            // Start Wave button at the bottom
            parent.spawn((
                Button,
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(40.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(2.0)),
                    margin: UiRect::top(Val::Px(10.0)),
                    ..default()
                },
                BackgroundColor(UIColors::BUTTON_DEFAULT),
                BorderColor(UIColors::BORDER_DEFAULT),
                BorderRadius::all(Val::Px(6.0)),
                StartWaveButton,
            )).with_children(|button| {
                button.spawn((
                    Text::new("START WAVE"),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(UIColors::TEXT_PRIMARY),
                    StartWaveButtonText,
                ));
            });
        });

    // Create enhanced tooltip container with better styling and proper Z-order
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            display: Display::None,
            width: Val::Px(250.0),  // Wider for better readability
            padding: UiRect::all(Val::Px(12.0)),  // More generous padding
            border: UiRect::all(Val::Px(2.0)),  // Thicker border for definition
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(6.0),  // Better vertical spacing
            // Note: Z-index handled through spawn order in Bevy 0.14
            ..default()
        },
        BackgroundColor(UIColors::TOOLTIP_BG),
        BorderColor(UIColors::TOOLTIP_BORDER),
        TowerTooltip,
    ))
    .with_children(|tooltip| {
        tooltip.spawn((
            Text::new(""),
            TextFont {
                font_size: 11.0,  // Improved readability
                ..default()
            },
            TextColor(UIColors::TEXT_PRIMARY),
            Node {
                align_self: AlignSelf::Start,
                ..default()
            },
            TooltipText,
        ));
    });
}

/// Setup the enhanced tower stat popup system
pub fn setup_tower_stat_popup(mut commands: Commands) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(50.0), // Will be updated dynamically
                top: Val::Px(100.0), // Will be updated dynamically
                width: Val::Px(340.0), // Wider for comprehensive info
                height: Val::Auto, // Auto height to fit content
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(20.0)),
                border: UiRect::all(Val::Px(2.0)),
                row_gap: Val::Px(16.0),
                display: Display::None, // Hidden by default
                ..default()
            },
            BackgroundColor(UIColors::TOOLTIP_BG), // Use tooltip background for better contrast
            BorderColor(UIColors::TOOLTIP_BORDER),
            TowerStatPopup,
        ))
        .with_children(|parent| {
            // Header section with tower name and close button
            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    margin: UiRect::bottom(Val::Px(12.0)),
                    ..default()
                },
            )).with_children(|header| {
                // Tower name
                header.spawn((
                    Text::new("Tower Information"),
                    TextFont {
                        font_size: 22.0,
                        ..default()
                    },
                    TextColor(UIColors::TEXT_PRIMARY),
                    PopupHeader,
                ));

                // Close button (X)
                header.spawn((
                    Button,
                    Node {
                        width: Val::Px(28.0),
                        height: Val::Px(28.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(1.0)),
                        ..default()
                    },
                    BackgroundColor(UIColors::BUTTON_DEFAULT),
                    BorderColor(UIColors::BORDER_DEFAULT),
                    PopupCloseButton,
                )).with_children(|button| {
                    button.spawn((
                        Text::new("X"),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(UIColors::TEXT_PRIMARY),
                    ));
                });
            });

            // Description section
            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    padding: UiRect::all(Val::Px(12.0)),
                    border: UiRect::all(Val::Px(1.0)),
                    margin: UiRect::bottom(Val::Px(8.0)),
                    ..default()
                },
                BackgroundColor(UIColors::HEADER_BG),
                BorderColor(UIColors::BORDER_DEFAULT),
            )).with_children(|section| {
                section.spawn((
                    Text::new("Tower description will appear here"),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(UIColors::TEXT_SECONDARY),
                    PopupDescriptionSection,
                ));
            });

            // Stats section
            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    padding: UiRect::all(Val::Px(12.0)),
                    border: UiRect::all(Val::Px(1.0)),
                    margin: UiRect::bottom(Val::Px(8.0)),
                    ..default()
                },
                BackgroundColor(UIColors::RESOURCE_BG),
                BorderColor(UIColors::RESOURCE_BORDER),
            )).with_children(|section| {
                section.spawn((
                    Text::new("Stats"),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(UIColors::TEXT_ACCENT),
                    Node {
                        margin: UiRect::bottom(Val::Px(8.0)),
                        ..default()
                    },
                ));
                section.spawn((
                    Text::new("Tower stats will appear here"),
                    TextFont {
                        font_size: 13.0,
                        ..default()
                    },
                    TextColor(UIColors::TEXT_PRIMARY),
                    PopupStatsSection,
                ));
            });

            // Cost section
            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    padding: UiRect::all(Val::Px(12.0)),
                    border: UiRect::all(Val::Px(1.0)),
                    margin: UiRect::bottom(Val::Px(8.0)),
                    ..default()
                },
                BackgroundColor(UIColors::RESOURCE_BG),
                BorderColor(UIColors::RESOURCE_BORDER),
            )).with_children(|section| {
                section.spawn((
                    Text::new("Cost"),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(UIColors::TEXT_ACCENT),
                    Node {
                        margin: UiRect::bottom(Val::Px(8.0)),
                        ..default()
                    },
                ));
                section.spawn((
                    Text::new("Tower costs will appear here"),
                    TextFont {
                        font_size: 13.0,
                        ..default()
                    },
                    TextColor(UIColors::TEXT_PRIMARY),
                    PopupCostSection,
                ));
            });

            // Upgrade preview section
            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    padding: UiRect::all(Val::Px(12.0)),
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                BackgroundColor(UIColors::HEADER_BG),
                BorderColor(UIColors::BORDER_DEFAULT),
            )).with_children(|section| {
                section.spawn((
                    Text::new("Upgrade Preview"),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(UIColors::TEXT_SUCCESS),
                    Node {
                        margin: UiRect::bottom(Val::Px(8.0)),
                        ..default()
                    },
                ));
                section.spawn((
                    Text::new("Upgrade information will appear here"),
                    TextFont {
                        font_size: 13.0,
                        ..default()
                    },
                    TextColor(UIColors::TEXT_SECONDARY),
                    PopupUpgradeSection,
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

/// Helper function to create a row of two tower buttons
fn create_tower_button_row(parent: &mut ChildSpawnerCommands, tower_pairs: &[(TowerType, char)]) {
    parent.spawn((
        Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(12.0),  // Better horizontal rhythm
            ..default()
        },
    )).with_children(|row| {
        for (tower_type, letter) in tower_pairs {
            create_single_tower_button(row, *tower_type, *letter, false);
        }
    });
}

/// Helper function to create a full-width tower button (for Tesla)
fn create_tower_button_full_width(parent: &mut ChildSpawnerCommands, tower_type: TowerType, letter: char) {
    create_single_tower_button(parent, tower_type, letter, true);
}

/// Core function to create individual tower buttons with enhanced styling and affordability feedback
fn create_single_tower_button(parent: &mut ChildSpawnerCommands, tower_type: TowerType, letter: char, full_width: bool) {
    let width = if full_width { 
        Val::Percent(100.0) 
    } else { 
        Val::Px(106.0) 
    };
    
    parent
        .spawn((
            Button,
            Node {
                width,
                height: Val::Px(84.0),  // Slightly taller for better proportions
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(2.0)),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(4.0)),  // Inner padding for better spacing
                ..default()
            },
            BackgroundColor(UIColors::BUTTON_DEFAULT),
            BorderColor(UIColors::BORDER_DEFAULT),
            TowerTypeButton { tower_type },
            HoverState { is_hovered: false, tower_type },
        ))
        .with_children(|button| {
            // Single letter label with enhanced hierarchy
            button.spawn((
                Text::new(letter.to_string()),
                TextFont {
                    font_size: 28.0,  // Balanced size for clarity
                    ..default()
                },
                TextColor(UIColors::TEXT_PRIMARY),
                Node {
                    margin: UiRect::bottom(Val::Px(4.0)),  // Balanced spacing
                    align_self: AlignSelf::Center,
                    ..default()
                },
            ));
            
            // Enhanced cost indicator with better formatting
            let cost = tower_type.get_cost();
            let cost_text = if cost.money > 0 && (cost.research_points > 0 || cost.materials > 0 || cost.energy > 0) {
                format!("${}+", cost.money) // Show + for complex costs
            } else {
                format!("${}", cost.money)
            };
            
            button.spawn((
                Text::new(cost_text),
                TextFont {
                    font_size: 12.0,  // Improved readability
                    ..default()
                },
                TextColor(UIColors::TEXT_ACCENT),
                Node {
                    align_self: AlignSelf::Center,
                    ..default()
                },
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

#[derive(Component)]
pub struct ResourceStatusText;

#[derive(Component)]
pub struct TooltipText;

// ============================================================================
// UI UPDATE SYSTEMS  
// ============================================================================

/// Enhanced system to update resource status display with better formatting
pub fn update_resource_status_system(
    economy: Res<Economy>,
    mut resource_query: Query<&mut Text, With<ResourceStatusText>>,
) {
    if economy.is_changed() {
        if let Ok(mut text) = resource_query.single_mut() {
            // Enhanced formatting with ASCII symbols for better compatibility
            **text = format!(
                "${}  |  R:{}  |  M:{}  |  E:{}",
                economy.money,
                economy.research_points,
                economy.materials,
                economy.energy
            );
        }
    }
}

/// System to handle hover tooltips for tower buttons with improved positioning
pub fn tower_tooltip_system(
    button_query: Query<(&HoverState, &GlobalTransform, &TowerTypeButton), With<Button>>,
    mut tooltip_query: Query<(&mut Node, &mut Text), (With<TowerTooltip>, Without<TowerTypeButton>)>,
    economy: Res<Economy>,
) {
    let mut show_tooltip = false;
    let mut tooltip_content = String::new();
    let mut tooltip_position = Vec2::ZERO;

    // Find any hovered button and get its position
    for (hover_state, _global_transform, tower_button) in button_query.iter() {
        if hover_state.is_hovered {
            show_tooltip = true;
            let tower_type = tower_button.tower_type;
            let cost = tower_type.get_cost();
            let stats = TowerStats::new(tower_type);
            let can_afford = economy.can_afford(&cost);
            
            // Calculate DPS (Damage Per Second) for enhanced tooltip
            let dps = stats.damage * stats.fire_rate;
            
            // Enhanced formatting with better visual hierarchy
            let mut cost_parts = Vec::new();
            cost_parts.push(format!("${}", cost.money));
            if cost.research_points > 0 {
                cost_parts.push(format!("R:{}", cost.research_points));
            }
            if cost.materials > 0 {
                cost_parts.push(format!("M:{}", cost.materials));
            }
            if cost.energy > 0 {
                cost_parts.push(format!("E:{}", cost.energy));
            }
            let cost_display = cost_parts.join(" | ");
            
            // Affordability status with clear indicators - using ASCII
            let affordability = if can_afford {
                "[OK] AFFORDABLE"
            } else {
                "[X] INSUFFICIENT RESOURCES"
            };
            
            tooltip_content = format!(
                "{}\n{}\n\nCost: {}\nStatus: {}\n\nPerformance:\n* DPS: {:.1}\n* Damage: {:.1}\n* Range: {:.1}\n* Fire Rate: {:.1}/sec",
                tower_type.get_name(),
                tower_type.get_description(),
                cost_display,
                affordability,
                dps,
                stats.damage,
                stats.range,
                stats.fire_rate
            );
            
            // Position tooltip to the left of the tower selection panel
            // Since tower buttons are in a fixed UI panel on the right side,
            // we can use fixed positioning relative to the panel
            tooltip_position = Vec2::new(
                50.0,  // Fixed position on left side of screen
                200.0 + (tower_type as u8 as f32 * 100.0)  // Staggered vertically by tower type
            );
            break; // Only show tooltip for first hovered button
        }
    }

    // Update tooltip visibility and content
    if let Ok((mut tooltip_node, mut tooltip_text)) = tooltip_query.single_mut() {
        if show_tooltip {
            tooltip_node.display = Display::Flex;
            tooltip_node.left = Val::Px(tooltip_position.x);
            tooltip_node.top = Val::Px(tooltip_position.y);
            **tooltip_text = tooltip_content;
        } else {
            tooltip_node.display = Display::None;
        }
    }
}

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

/// System to automatically show/hide stat popup on hover
pub fn hover_stat_popup_system(
    mut popup_state: ResMut<TowerStatPopupState>,
    button_query: Query<(&HoverState, &GlobalTransform, &TowerTypeButton), With<Button>>,
) {
    let mut any_hovered = false;
    let mut hovered_tower = None;
    let mut hover_position = Vec2::ZERO;

    // Check for any currently hovered button
    for (hover_state, global_transform, tower_button) in button_query.iter() {
        if hover_state.is_hovered {
            any_hovered = true;
            hovered_tower = Some(tower_button.tower_type);
            let button_pos = global_transform.translation().truncate();
            // Position popup to the left of the tower selection panel to avoid overlap
            // Fixed position ensures consistent placement
            hover_position = Vec2::new(50.0, button_pos.y);
            break; // Only show popup for first hovered button
        }
    }

    // Show popup if hovering, hide if not
    if any_hovered {
        if let Some(tower_type) = hovered_tower {
            // Only update if it's a different tower type or not currently showing
            if popup_state.active_tower_type != Some(tower_type) || !popup_state.visible {
                popup_state.show_for_tower(tower_type, hover_position);
            }
        }
    } else {
        // Hide popup if no buttons are hovered
        if popup_state.visible {
            popup_state.hide();
        }
    }
}

/// System to handle tower stat popup close button
pub fn popup_close_button_system(
    mut popup_state: ResMut<TowerStatPopupState>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PopupCloseButton>),
    >,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                popup_state.hide();
                println!("Popup closed via close button");
                *color = UIColors::BUTTON_SELECTED.into(); // Brief feedback
            }
            Interaction::Hovered => {
                *color = UIColors::BUTTON_HOVER.into();
            }
            Interaction::None => {
                *color = UIColors::BUTTON_DEFAULT.into();
            }
        }
    }
}

/// System to manage tower stat popup visibility and content updates
pub fn tower_stat_popup_system(
    popup_state: Res<TowerStatPopupState>,
    economy: Res<Economy>,
    mut popup_query: Query<&mut Node, With<TowerStatPopup>>,
    mut header_query: Query<&mut Text, (With<PopupHeader>, Without<PopupDescriptionSection>, Without<PopupStatsSection>, Without<PopupCostSection>, Without<PopupUpgradeSection>)>,
    mut description_query: Query<&mut Text, (With<PopupDescriptionSection>, Without<PopupHeader>, Without<PopupStatsSection>, Without<PopupCostSection>, Without<PopupUpgradeSection>)>,
    mut stats_query: Query<&mut Text, (With<PopupStatsSection>, Without<PopupHeader>, Without<PopupDescriptionSection>, Without<PopupCostSection>, Without<PopupUpgradeSection>)>,
    mut cost_query: Query<&mut Text, (With<PopupCostSection>, Without<PopupHeader>, Without<PopupDescriptionSection>, Without<PopupStatsSection>, Without<PopupUpgradeSection>)>,
    mut upgrade_query: Query<&mut Text, (With<PopupUpgradeSection>, Without<PopupHeader>, Without<PopupDescriptionSection>, Without<PopupStatsSection>, Without<PopupCostSection>)>,
) {
    // Update popup visibility and position
    if let Ok(mut popup_node) = popup_query.single_mut() {
        if popup_state.is_showing() {
            popup_node.display = Display::Flex;
            popup_node.left = Val::Px(popup_state.position.x);
            popup_node.top = Val::Px(popup_state.position.y);
        } else {
            popup_node.display = Display::None;
        }
    }

    // Update popup content when visible and tower type is available
    if let Some(tower_type) = popup_state.active_tower_type {
        let stats = TowerStats::new(tower_type);
        let cost = tower_type.get_cost();
        let can_afford = economy.can_afford(&cost);

        // Update header
        if let Ok(mut text) = header_query.single_mut() {
            **text = format!("{}", tower_type.get_name());
        }

        // Update description
        if let Ok(mut text) = description_query.single_mut() {
            **text = format!("{}", tower_type.get_description());
        }

        // Update stats - calculate DPS and efficiency metrics
        if let Ok(mut text) = stats_query.single_mut() {
            let dps = stats.damage * stats.fire_rate;
            let efficiency = dps / cost.money as f32; // Damage per dollar
            
            **text = format!(
                "Damage: {:.1}\nRange: {:.1}\nFire Rate: {:.1}/sec\nDPS: {:.1}\nEfficiency: {:.2} DPS/$",
                stats.damage,
                stats.range,
                stats.fire_rate,
                dps,
                efficiency
            );
        }

        // Update cost with affordability indicators
        if let Ok(mut text) = cost_query.single_mut() {
            let affordability_status = if can_afford {
                "[AFFORDABLE]"
            } else {
                "[INSUFFICIENT RESOURCES]"
            };

            let mut cost_parts = Vec::new();
            if cost.money > 0 {
                cost_parts.push(format!("Money: ${}", cost.money));
            }
            if cost.research_points > 0 {
                cost_parts.push(format!("Research: {}", cost.research_points));
            }
            if cost.materials > 0 {
                cost_parts.push(format!("Materials: {}", cost.materials));
            }
            if cost.energy > 0 {
                cost_parts.push(format!("Energy: {}", cost.energy));
            }

            **text = format!(
                "{}\n\n{}",
                cost_parts.join("\n"),
                affordability_status
            );
        }

        // Update upgrade preview
        if let Ok(mut text) = upgrade_query.single_mut() {
            let mut preview_stats = stats.clone();
            if preview_stats.can_upgrade() {
                let upgrade_cost = preview_stats.get_upgrade_cost();
                preview_stats.upgrade();
                
                let damage_increase = preview_stats.damage - stats.damage;
                let range_increase = preview_stats.range - stats.range;
                let fire_rate_increase = preview_stats.fire_rate - stats.fire_rate;
                
                **text = format!(
                    "Level 2 Stats:\nDamage: {:.1} (+{:.1})\nRange: {:.1} (+{:.1})\nFire Rate: {:.1} (+{:.1})\n\nUpgrade Cost: ${} R:{} M:{} E:{}",
                    preview_stats.damage, damage_increase,
                    preview_stats.range, range_increase,
                    preview_stats.fire_rate, fire_rate_increase,
                    upgrade_cost.money, upgrade_cost.research_points, 
                    upgrade_cost.materials, upgrade_cost.energy
                );
            } else {
                **text = "This tower cannot be upgraded further.".to_string();
            }
        }
    }
}

/// System to handle clicking outside the popup to close it
pub fn popup_outside_click_system(
    mut popup_state: ResMut<TowerStatPopupState>,
    mouse_input: Res<MouseInputState>,
    popup_query: Query<&GlobalTransform, With<TowerStatPopup>>,
) {
    if popup_state.is_showing() && mouse_input.left_clicked {
        // Check if click is outside popup bounds
        if let Ok(popup_transform) = popup_query.single() {
            let popup_pos = popup_transform.translation().truncate();
            let click_pos = mouse_input.world_position;
            
            // Define popup bounds (approximate)
            let popup_bounds = Rect::from_center_size(popup_pos, Vec2::new(340.0, 400.0));
            
            if !popup_bounds.contains(click_pos) {
                popup_state.hide();
                println!("Popup closed via outside click");
            }
        }
    }
}

/// System to handle Start Wave button clicks
pub fn start_wave_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<StartWaveButton>),
    >,
    mut wave_start_events: EventWriter<StartWaveEvent>,
    mut mouse_input_state: ResMut<MouseInputState>,
    wave_manager: Res<WaveManager>,
) {
    for (interaction, mut bg_color, mut border_color) in &mut interaction_query {
        // Check if wave can be started
        let can_start_wave = wave_manager.current_wave == 0 || wave_manager.wave_complete();
        
        match *interaction {
            Interaction::Pressed => {
                // CRITICAL FIX: Consume the mouse click to prevent tower placement
                mouse_input_state.left_clicked = false;
                
                if can_start_wave {
                    // Send event to start new wave
                    wave_start_events.write(StartWaveEvent);
                    info!("Start Wave button pressed - new wave starting");
                    *bg_color = BackgroundColor(UIColors::BUTTON_SELECTED);
                } else {
                    info!("Cannot start wave - current wave still in progress");
                    *bg_color = BackgroundColor(UIColors::COST_UNAFFORDABLE);
                }
            }
            Interaction::Hovered => {
                if can_start_wave {
                    *bg_color = BackgroundColor(UIColors::BUTTON_HOVER);
                    *border_color = BorderColor(UIColors::BORDER_HOVER);
                } else {
                    *bg_color = BackgroundColor(UIColors::BUTTON_DISABLED);
                    *border_color = BorderColor(UIColors::BORDER_DISABLED);
                }
            }
            Interaction::None => {
                if can_start_wave {
                    *bg_color = BackgroundColor(UIColors::BUTTON_DEFAULT);
                    *border_color = BorderColor(UIColors::BORDER_DEFAULT);
                } else {
                    *bg_color = BackgroundColor(UIColors::BUTTON_DISABLED);
                    *border_color = BorderColor(UIColors::BORDER_DISABLED);
                }
            }
        }
    }
}

/// System to update Start Wave button text and state based on wave manager
pub fn update_start_wave_button_system(
    wave_manager: Res<WaveManager>,
    mut text_query: Query<&mut Text, With<StartWaveButtonText>>,
    mut button_query: Query<(&mut BackgroundColor, &mut BorderColor), (With<StartWaveButton>, Without<StartWaveButtonText>)>,
) {
    if wave_manager.is_changed() {
        let can_start_wave = wave_manager.current_wave == 0 || wave_manager.wave_complete();
        
        // Update button text
        if let Ok(mut text) = text_query.single_mut() {
            **text = if can_start_wave {
                if wave_manager.current_wave == 0 {
                    "START FIRST WAVE".to_string()
                } else {
                    format!("START WAVE {}", wave_manager.current_wave + 1)
                }
            } else {
                format!("WAVE {} IN PROGRESS", wave_manager.current_wave)
            };
        }
        
        // Update button appearance
        if let Ok((mut bg_color, mut border_color)) = button_query.single_mut() {
            if can_start_wave {
                *bg_color = BackgroundColor(UIColors::BUTTON_DEFAULT);
                *border_color = BorderColor(UIColors::BORDER_DEFAULT);
            } else {
                *bg_color = BackgroundColor(UIColors::BUTTON_DISABLED);
                *border_color = BorderColor(UIColors::BORDER_DISABLED);
            }
        }
    }
}

/// System to provide real-time affordability feedback on tower buttons
pub fn tower_affordability_system(
    economy: Res<Economy>,
    selection_state: Res<TowerSelectionState>,
    mut button_query: Query<(&TowerTypeButton, &mut BackgroundColor, &mut BorderColor), (With<Button>, Without<HoverState>)>,
    hover_query: Query<&HoverState, With<TowerTypeButton>>,
) {
    if economy.is_changed() {
        for (tower_button, mut bg_color, mut border_color) in button_query.iter_mut() {
            let cost = tower_button.tower_type.get_cost();
            let can_afford = economy.can_afford(&cost);
            let is_selected = Some(tower_button.tower_type) == selection_state.selected_placement_type;
            
            // Check if button is currently hovered
            let is_hovered = hover_query.iter().any(|hover| hover.tower_type == tower_button.tower_type && hover.is_hovered);
            
            // Apply colors based on state with enhanced visual feedback
            if is_selected {
                if is_hovered {
                    *bg_color = UIColors::BUTTON_SELECTED_HOVER.into();
                    *border_color = UIColors::BORDER_SELECTED_HOVER.into();
                } else {
                    *bg_color = UIColors::BUTTON_SELECTED.into();
                    *border_color = UIColors::BORDER_SELECTED.into();
                }
            } else if !can_afford {
                if is_hovered {
                    *bg_color = UIColors::BUTTON_HOVER.into();
                    *border_color = UIColors::BORDER_DISABLED.into();
                } else {
                    *bg_color = UIColors::BUTTON_DISABLED.into();
                    *border_color = UIColors::BORDER_DISABLED.into();
                }
            } else {
                if is_hovered {
                    *bg_color = UIColors::BUTTON_HOVER.into();
                    *border_color = UIColors::BORDER_HOVER.into();
                } else {
                    *bg_color = UIColors::BUTTON_DEFAULT.into();
                    *border_color = UIColors::BORDER_DEFAULT.into();
                }
            }
        }
    }
}