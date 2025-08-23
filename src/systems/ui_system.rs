use bevy::prelude::*;
use crate::resources::*;
use crate::systems::input_system::MouseInputState;

#[derive(Component)]
pub struct EconomyUI;

#[derive(Component)]
pub struct TowerSelectionUI;

pub fn update_ui_system(
    mut commands: Commands,
    economy: Res<Economy>,
    _mouse_state: Res<MouseInputState>,
    mut economy_ui_query: Query<&mut Text, (With<EconomyUI>, Without<TowerSelectionUI>)>,
    mut selection_ui_query: Query<&mut Text, (With<TowerSelectionUI>, Without<EconomyUI>)>,
    economy_ui_exists: Query<(), With<EconomyUI>>,
    selection_ui_exists: Query<(), With<TowerSelectionUI>>,
) {
    // Update or create economy UI
    let economy_text = format!(
        "Money: {} | Research: {} | Materials: {} | Energy: {}",
        economy.money, economy.research_points, economy.materials, economy.energy
    );

    if let Ok(mut text) = economy_ui_query.single_mut() {
        **text = economy_text;
    } else if economy_ui_exists.is_empty() {
        // Create economy UI only if none exists
        commands.spawn((
            Text2d::new(economy_text),
            TextFont {
                font_size: 18.0,
                ..default()
            },
            TextColor(Color::srgb(1.0, 1.0, 0.0)),
            Transform::from_translation(Vec3::new(-500.0, 340.0, 1.0)),
            EconomyUI,
        ));
    }

    // Update or create tower selection UI - now handled by tower UI panels
    let selection_text = "Use UI panels to select towers and upgrades".to_string();

    if let Ok(mut text) = selection_ui_query.single_mut() {
        **text = selection_text;
    } else if selection_ui_exists.is_empty() {
        // Create tower selection UI only if none exists
        commands.spawn((
            Text2d::new(selection_text),
            TextFont {
                font_size: 16.0,
                ..default()
            },
            TextColor(Color::srgb(0.8, 0.8, 1.0)),
            Transform::from_translation(Vec3::new(-500.0, 300.0, 1.0)),
            TowerSelectionUI,
        ));
    }
}