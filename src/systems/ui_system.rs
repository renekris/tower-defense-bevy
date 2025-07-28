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
    mouse_state: Res<MouseInputState>,
    mut economy_ui_query: Query<(Entity, &mut Text), (With<EconomyUI>, Without<TowerSelectionUI>)>,
    mut selection_ui_query: Query<(Entity, &mut Text), (With<TowerSelectionUI>, Without<EconomyUI>)>,
) {
    // Update or create economy UI
    let economy_text = format!(
        "Money: {} | Research: {} | Materials: {} | Energy: {}",
        economy.money, economy.research_points, economy.materials, economy.energy
    );

    if let Ok((_, mut text)) = economy_ui_query.get_single_mut() {
        text.sections[0].value = economy_text;
    } else {
        // Create economy UI
        commands.spawn((
            Text2dBundle {
                text: Text::from_section(
                    economy_text,
                    TextStyle {
                        font_size: 18.0,
                        color: Color::srgb(1.0, 1.0, 0.0),
                        ..default()
                    },
                ),
                transform: Transform::from_translation(Vec3::new(-500.0, 340.0, 1.0)),
                ..default()
            },
            EconomyUI,
        ));
    }

    // Update or create tower selection UI
    let selection_text = if let Some(tower_type) = mouse_state.selected_tower_type {
        let cost = tower_type.get_cost();
        format!(
            "Selected: {:?} | Cost: ${}  R:{} M:{} E:{}",
            tower_type, cost.money, cost.research_points, cost.materials, cost.energy
        )
    } else {
        "No tower selected. Press 1-5 to select tower type.".to_string()
    };

    if let Ok((_, mut text)) = selection_ui_query.get_single_mut() {
        text.sections[0].value = selection_text;
    } else {
        // Create tower selection UI
        commands.spawn((
            Text2dBundle {
                text: Text::from_section(
                    selection_text,
                    TextStyle {
                        font_size: 16.0,
                        color: Color::srgb(0.8, 0.8, 1.0),
                        ..default()
                    },
                ),
                transform: Transform::from_translation(Vec3::new(-500.0, 300.0, 1.0)),
                ..default()
            },
            TowerSelectionUI,
        ));
    }
}