use bevy::prelude::*;
use crate::resources::{TowerType, TowerStats};
use crate::components::{GamePosition, Health};
use crate::systems::combat_system::Target;

/// Component to mark entities that are part of a tower's visual pattern
#[derive(Component)]
pub struct TowerVisualPart {
    pub parent_tower: Entity,
}

/// System to spawn towers with distinctive visual patterns
pub fn spawn_tower_with_pattern(commands: &mut Commands, position: Vec2, tower_type: TowerType) {
    let tower_stats = TowerStats::new(tower_type);
    
    // Spawn the main tower entity (invisible base)
    let tower_entity = commands.spawn((
        Transform::from_translation(position.extend(0.0)),
        Visibility::Hidden, // The base is invisible, only pattern shows
        tower_stats,
        Health::new(100.0),
        GamePosition::new(position.x, position.y),
        Target::default(),
    )).id();

    // Spawn the visual pattern based on tower type
    spawn_visual_pattern(commands, tower_entity, position, tower_type);
}

/// Spawns distinctive visual patterns for each tower type
fn spawn_visual_pattern(commands: &mut Commands, parent_tower: Entity, position: Vec2, tower_type: TowerType) {
    match tower_type {
        TowerType::Basic => spawn_basic_pattern(commands, parent_tower, position),
        TowerType::Advanced => spawn_advanced_pattern(commands, parent_tower, position),
        TowerType::Laser => spawn_laser_pattern(commands, parent_tower, position),
        TowerType::Missile => spawn_missile_pattern(commands, parent_tower, position),
        TowerType::Tesla => spawn_tesla_pattern(commands, parent_tower, position),
    }
}

/// Basic Tower: Diamond with center dot pattern
fn spawn_basic_pattern(commands: &mut Commands, parent_tower: Entity, position: Vec2) {
    let brown_color = Color::srgb(0.6, 0.4, 0.2);
    
    // Outer diamond (rotated square)
    commands.spawn((
        Sprite {
            color: brown_color,
            custom_size: Some(Vec2::new(32.0, 32.0)),
            ..default()
        },
        Transform::from_translation(position.extend(0.1))
            .with_rotation(Quat::from_rotation_z(std::f32::consts::PI / 4.0)),
        TowerVisualPart { parent_tower },
    ));
    
    // Inner diamond frame (smaller, darker)
    commands.spawn((
        Sprite {
            color: Color::srgb(0.4, 0.3, 0.15), // Darker brown
            custom_size: Some(Vec2::new(20.0, 20.0)),
            ..default()
        },
        Transform::from_translation(position.extend(0.2))
            .with_rotation(Quat::from_rotation_z(std::f32::consts::PI / 4.0)),
        TowerVisualPart { parent_tower },
    ));
    
    // Center dot
    commands.spawn((
        Sprite {
            color: Color::srgb(0.8, 0.6, 0.3), // Light brown center
            custom_size: Some(Vec2::new(8.0, 8.0)),
            ..default()
        },
        Transform::from_translation(position.extend(0.3)),
        TowerVisualPart { parent_tower },
    ));
}

/// Advanced Tower: Concentric circles pattern
fn spawn_advanced_pattern(commands: &mut Commands, parent_tower: Entity, position: Vec2) {
    // Outer ring
    commands.spawn((
        Sprite {
            color: Color::srgb(0.4, 0.4, 0.8), // Blue
            custom_size: Some(Vec2::new(36.0, 36.0)),
            ..default()
        },
        Transform::from_translation(position.extend(0.1)),
        TowerVisualPart { parent_tower },
    ));
    
    // Inner circle
    commands.spawn((
        Sprite {
            color: Color::srgb(0.6, 0.6, 1.0), // Light blue
            custom_size: Some(Vec2::new(20.0, 20.0)),
            ..default()
        },
        Transform::from_translation(position.extend(0.2)),
        TowerVisualPart { parent_tower },
    ));
}

/// Laser Tower: Cross/plus pattern for precision
fn spawn_laser_pattern(commands: &mut Commands, parent_tower: Entity, position: Vec2) {
    let red_color = Color::srgb(1.0, 0.2, 0.2);
    
    // Horizontal bar
    commands.spawn((
        Sprite {
            color: red_color,
            custom_size: Some(Vec2::new(36.0, 8.0)),
            ..default()
        },
        Transform::from_translation(position.extend(0.1)),
        TowerVisualPart { parent_tower },
    ));
    
    // Vertical bar
    commands.spawn((
        Sprite {
            color: red_color,
            custom_size: Some(Vec2::new(8.0, 36.0)),
            ..default()
        },
        Transform::from_translation(position.extend(0.2)),
        TowerVisualPart { parent_tower },
    ));
    
    // Center dot
    commands.spawn((
        Sprite {
            color: Color::srgb(1.0, 0.8, 0.8), // Light red center
            custom_size: Some(Vec2::new(12.0, 12.0)),
            ..default()
        },
        Transform::from_translation(position.extend(0.3)),
        TowerVisualPart { parent_tower },
    ));
}

/// Missile Tower: Triangle/arrow pattern pointing up
fn spawn_missile_pattern(commands: &mut Commands, parent_tower: Entity, position: Vec2) {
    let yellow_color = Color::srgb(0.9, 0.9, 0.2);
    
    // Main triangle body (rotated square to form diamond)
    commands.spawn((
        Sprite {
            color: yellow_color,
            custom_size: Some(Vec2::new(28.0, 28.0)),
            ..default()
        },
        Transform::from_translation(position.extend(0.1))
            .with_rotation(Quat::from_rotation_z(std::f32::consts::PI / 4.0)),
        TowerVisualPart { parent_tower },
    ));
    
    // Base platform
    commands.spawn((
        Sprite {
            color: Color::srgb(0.7, 0.7, 0.1), // Darker yellow
            custom_size: Some(Vec2::new(36.0, 8.0)),
            ..default()
        },
        Transform::from_translation((position + Vec2::new(0.0, -16.0)).extend(0.2)),
        TowerVisualPart { parent_tower },
    ));
}

/// Tesla Tower: Concentric squares with energy dots pattern
fn spawn_tesla_pattern(commands: &mut Commands, parent_tower: Entity, position: Vec2) {
    let electric_blue = Color::srgb(0.2, 0.6, 1.0);
    let bright_cyan = Color::srgb(0.4, 0.8, 1.0);
    let white_energy = Color::srgb(0.9, 0.95, 1.0);
    
    // Outer square frame
    commands.spawn((
        Sprite {
            color: electric_blue,
            custom_size: Some(Vec2::new(36.0, 36.0)),
            ..default()
        },
        Transform::from_translation(position.extend(0.1)),
        TowerVisualPart { parent_tower },
    ));
    
    // Middle square frame
    commands.spawn((
        Sprite {
            color: bright_cyan,
            custom_size: Some(Vec2::new(24.0, 24.0)),
            ..default()
        },
        Transform::from_translation(position.extend(0.2)),
        TowerVisualPart { parent_tower },
    ));
    
    // Inner square core
    commands.spawn((
        Sprite {
            color: Color::srgb(0.1, 0.4, 0.8), // Dark electric blue
            custom_size: Some(Vec2::new(12.0, 12.0)),
            ..default()
        },
        Transform::from_translation(position.extend(0.3)),
        TowerVisualPart { parent_tower },
    ));
    
    // Energy dots at cardinal directions
    let energy_dot_positions = [
        Vec2::new(0.0, 20.0),   // Top
        Vec2::new(20.0, 0.0),   // Right
        Vec2::new(0.0, -20.0),  // Bottom
        Vec2::new(-20.0, 0.0),  // Left
    ];
    
    for offset in energy_dot_positions.iter() {
        commands.spawn((
            Sprite {
                color: white_energy,
                custom_size: Some(Vec2::new(6.0, 6.0)),
                ..default()
            },
            Transform::from_translation((position + *offset).extend(0.4)),
            TowerVisualPart { parent_tower },
        ));
    }
    
    // Diagonal energy dots (smaller)
    let diagonal_positions = [
        Vec2::new(14.0, 14.0),   // Top-right
        Vec2::new(-14.0, 14.0),  // Top-left
        Vec2::new(14.0, -14.0),  // Bottom-right
        Vec2::new(-14.0, -14.0), // Bottom-left
    ];
    
    for offset in diagonal_positions.iter() {
        commands.spawn((
            Sprite {
                color: bright_cyan,
                custom_size: Some(Vec2::new(4.0, 4.0)),
                ..default()
            },
            Transform::from_translation((position + *offset).extend(0.35)),
            TowerVisualPart { parent_tower },
        ));
    }
}

/// System to clean up visual parts when tower is despawned
pub fn cleanup_tower_visual_parts(
    mut commands: Commands,
    tower_query: Query<Entity, (With<TowerStats>, With<Health>)>,
    visual_parts: Query<(Entity, &TowerVisualPart)>,
) {
    let existing_towers: std::collections::HashSet<Entity> = tower_query.iter().collect();
    
    for (visual_entity, visual_part) in visual_parts.iter() {
        if !existing_towers.contains(&visual_part.parent_tower) {
            commands.entity(visual_entity).despawn();
        }
    }
}

/// Plugin to add tower rendering systems
pub struct TowerRenderingPlugin;

impl Plugin for TowerRenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, cleanup_tower_visual_parts);
    }
}