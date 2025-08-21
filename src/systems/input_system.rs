use bevy::prelude::*;
use bevy::input::mouse::MouseButtonInput;
use bevy::window::PrimaryWindow;
use crate::resources::*;
use crate::components::*;
use crate::systems::combat_system::Target;

#[derive(Resource, Debug)]
pub struct MouseInputState {
    pub current_position: Vec2,
    pub world_position: Vec2,
    pub left_clicked: bool,
    pub right_clicked: bool,
    pub selected_tower_type: Option<TowerType>,
    pub placement_mode: PlacementMode,
    pub preview_position: Option<Vec2>,
}

impl Default for MouseInputState {
    fn default() -> Self {
        Self {
            current_position: Vec2::ZERO,
            world_position: Vec2::ZERO,
            left_clicked: false,
            right_clicked: false,
            selected_tower_type: None,
            placement_mode: PlacementMode::Hybrid,
            preview_position: None,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum PlacementMode {
    #[default]
    None,
    GridBased,
    FreeForm,
    Hybrid,
}

#[derive(Component)]
pub struct PlacementPreview;

#[derive(Component)]
pub struct PlacementZoneMarker {
    pub zone_type: PlacementZoneType,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlacementZoneType {
    GridZone,
    FreeZone,
    RestrictedZone,
}

// Mouse input tracking system
pub fn mouse_input_system(
    mut mouse_state: ResMut<MouseInputState>,
    mut mouse_button_events: EventReader<MouseButtonInput>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    // Update mouse position
    if let Ok(window) = window_query.get_single() {
        if let Some(screen_pos) = window.cursor_position() {
            mouse_state.current_position = screen_pos;
            
            // Convert to world coordinates
            if let Ok((camera, camera_transform)) = camera_query.get_single() {
                mouse_state.world_position = screen_to_world_position(
                    screen_pos, 
                    camera_transform, 
                    camera,
                    window,
                );
            }
        }
    }

    // Handle mouse clicks
    mouse_state.left_clicked = false;
    mouse_state.right_clicked = false;

    for event in mouse_button_events.read() {
        if event.state.is_pressed() {
            match event.button {
                MouseButton::Left => mouse_state.left_clicked = true,
                MouseButton::Right => mouse_state.right_clicked = true,
                _ => {}
            }
        }
    }
}

// Tower placement system
pub fn tower_placement_system(
    mut commands: Commands,
    mut mouse_state: ResMut<MouseInputState>,
    mut economy: ResMut<Economy>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    existing_towers: Query<&Transform, With<TowerStats>>,
    enemy_path: Res<EnemyPath>,
) {
    // Tower type selection with number keys - UPDATE THE RESOURCE
    if keyboard_input.just_pressed(KeyCode::Digit1) {
        mouse_state.selected_tower_type = Some(TowerType::Basic);
        println!("Selected Basic Tower");
    } else if keyboard_input.just_pressed(KeyCode::Digit2) {
        mouse_state.selected_tower_type = Some(TowerType::Advanced);
        println!("Selected Advanced Tower");
    } else if keyboard_input.just_pressed(KeyCode::Digit3) {
        mouse_state.selected_tower_type = Some(TowerType::Laser);
        println!("Selected Laser Tower");
    } else if keyboard_input.just_pressed(KeyCode::Digit4) {
        mouse_state.selected_tower_type = Some(TowerType::Missile);
        println!("Selected Missile Tower");
    } else if keyboard_input.just_pressed(KeyCode::Digit5) {
        mouse_state.selected_tower_type = Some(TowerType::Tesla);
        println!("Selected Tesla Tower");
    } else if keyboard_input.just_pressed(KeyCode::Escape) {
        mouse_state.selected_tower_type = None;
        println!("Cleared tower selection");
    }

    // Only attempt placement if we have a tower type selected and left click
    if let Some(tower_type) = mouse_state.selected_tower_type {
        if mouse_state.left_clicked {
            println!("Attempting to place {:?} at {:?}", tower_type, mouse_state.world_position);
            let placement_pos = get_placement_position(
                mouse_state.world_position,
                mouse_state.placement_mode,
            );

            // Validate placement
            if is_valid_tower_placement(
                placement_pos,
                &existing_towers,
                &enemy_path.waypoints,
                32.0, // Tower size
            ) {
                let cost = tower_type.get_cost();
                if economy.can_afford(&cost) {
                    // Place the tower
                    spawn_tower(&mut commands, placement_pos, tower_type);
                    economy.spend(&cost);
                }
            }
        }
    }
}

// Preview system for tower placement
pub fn tower_placement_preview_system(
    mut commands: Commands,
    mouse_state: Res<MouseInputState>,
    existing_previews: Query<Entity, With<PlacementPreview>>,
    economy: Res<Economy>,
    existing_towers: Query<&Transform, With<TowerStats>>,
    enemy_path: Res<EnemyPath>,
) {
    // Clear existing previews
    for entity in existing_previews.iter() {
        commands.entity(entity).despawn();
    }

    // Show preview if tower type is selected
    if let Some(tower_type) = mouse_state.selected_tower_type {
        let placement_pos = get_placement_position(
            mouse_state.world_position,
            mouse_state.placement_mode,
        );

        let is_valid = is_valid_tower_placement(
            placement_pos,
            &existing_towers,
            &enemy_path.waypoints,
            32.0,
        );

        let cost = tower_type.get_cost();
        let can_afford = economy.can_afford(&cost);
        let color = if is_valid && can_afford {
            Color::srgba(0.0, 1.0, 0.0, 0.5) // Green
        } else {
            Color::srgba(1.0, 0.0, 0.0, 0.5) // Red
        };

        // Spawn preview sprite
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(32.0, 32.0)),
                    ..default()
                },
                transform: Transform::from_translation(placement_pos.extend(1.0)),
                ..default()
            },
            PlacementPreview,
        ));

        // Show range indicator
        spawn_range_preview(&mut commands, placement_pos, tower_type);
    }
}

// Utility functions
pub fn screen_to_world_position(
    screen_pos: Vec2,
    camera_transform: &GlobalTransform,
    _camera: &Camera,
    window: &Window,
) -> Vec2 {
    let window_size = Vec2::new(window.width(), window.height());
    
    // Convert screen coordinates to normalized device coordinates (NDC)
    let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
    
    // Flip Y coordinate (screen Y increases downward, world Y increases upward)
    let ndc = Vec2::new(ndc.x, -ndc.y);
    
    // Apply camera transform
    let world_pos = camera_transform.translation().truncate() + ndc * window_size * 0.5;
    
    world_pos
}

pub fn snap_to_grid(position: Vec2, grid_size: f32) -> Vec2 {
    Vec2::new(
        (position.x / grid_size).floor() * grid_size,
        (position.y / grid_size).floor() * grid_size,
    )
}

pub fn get_placement_position(world_pos: Vec2, mode: PlacementMode) -> Vec2 {
    match mode {
        PlacementMode::GridBased => snap_to_grid(world_pos, 64.0),
        PlacementMode::FreeForm => world_pos,
        PlacementMode::Hybrid => {
            if is_in_grid_zone(world_pos) {
                snap_to_grid(world_pos, 64.0)
            } else if is_in_free_zone(world_pos) {
                world_pos
            } else {
                world_pos // Fallback
            }
        }
        PlacementMode::None => world_pos,
    }
}

pub fn is_in_grid_zone(position: Vec2) -> bool {
    // Left or right grid zones 
    (position.x >= -400.0 && position.x <= -200.0 && position.y.abs() <= 200.0) ||
    (position.x >= 200.0 && position.x <= 400.0 && position.y.abs() <= 200.0)
}

pub fn is_in_free_zone(position: Vec2) -> bool {
    // Top or bottom free zones
    (position.y >= 125.0 && position.y <= 275.0 && position.x.abs() <= 300.0) ||
    (position.y >= -275.0 && position.y <= -125.0 && position.x.abs() <= 300.0)
}

pub fn is_valid_placement_position(position: Vec2, path_points: &[Vec2], min_distance: f32) -> bool {
    // Check distance to path
    for i in 0..path_points.len().saturating_sub(1) {
        let start = path_points[i];
        let end = path_points[i + 1];
        let distance = distance_to_line_segment(position, start, end);
        if distance < min_distance {
            return false;
        }
    }
    true
}

pub fn is_valid_tower_placement(
    position: Vec2,
    existing_towers: &Query<&Transform, With<TowerStats>>,
    path_points: &[Vec2],
    tower_size: f32,
) -> bool {
    // Check path collision
    if !is_valid_placement_position(position, path_points, tower_size) {
        return false;
    }

    // Check overlap with existing towers
    for transform in existing_towers.iter() {
        let distance = position.distance(transform.translation.truncate());
        if distance < tower_size {
            return false;
        }
    }

    true
}

pub fn distance_to_line_segment(point: Vec2, line_start: Vec2, line_end: Vec2) -> f32 {
    let line_vec = line_end - line_start;
    let point_vec = point - line_start;
    
    if line_vec.length_squared() == 0.0 {
        return point_vec.length();
    }
    
    let t = (point_vec.dot(line_vec) / line_vec.length_squared()).clamp(0.0, 1.0);
    let projection = line_start + line_vec * t;
    point.distance(projection)
}

pub fn spawn_tower(commands: &mut Commands, position: Vec2, tower_type: TowerType) {
    let tower_stats = TowerStats::new(tower_type);
    let color = match tower_type {
        TowerType::Basic => Color::srgb(0.5, 0.3, 0.1),
        TowerType::Advanced => Color::srgb(0.3, 0.3, 0.7),
        TowerType::Laser => Color::srgb(1.0, 0.2, 0.2),
        TowerType::Missile => Color::srgb(0.8, 0.8, 0.1),
        TowerType::Tesla => Color::srgb(0.5, 0.0, 1.0),
    };

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: Some(Vec2::new(32.0, 32.0)),
                ..default()
            },
            transform: Transform::from_translation(position.extend(0.0)),
            ..default()
        },
        tower_stats,
        Health::new(100.0),
        GamePosition::new(position.x, position.y),
        Target::default(), // Enable targeting for this tower
    ));
}

pub fn spawn_range_preview(commands: &mut Commands, position: Vec2, tower_type: TowerType) {
    let tower_stats = TowerStats::new(tower_type);
    let range = tower_stats.range;
    
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(1.0, 1.0, 1.0, 0.1),
                custom_size: Some(Vec2::new(range * 2.0, range * 2.0)),
                ..default()
            },
            transform: Transform::from_translation(position.extend(0.5)),
            ..default()
        },
        PlacementPreview,
    ));
}

// System to setup placement zones - MUCH SIMPLER APPROACH
pub fn setup_placement_zones(mut commands: Commands) {
    // Create large zone indicator rectangles instead of lots of small squares
    // Made much more visible with higher opacity and positive Z values
    
    // Left grid zone (bright green rectangle)
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.0, 1.0, 0.0, 0.3), // Much more visible
                custom_size: Some(Vec2::new(200.0, 400.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(-300.0, 0.0, 0.5)), // Positive Z
            ..default()
        },
        PlacementZoneMarker {
            zone_type: PlacementZoneType::GridZone,
        },
    ));
    
    // Right grid zone (bright green rectangle) 
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.0, 1.0, 0.0, 0.3), // Much more visible
                custom_size: Some(Vec2::new(200.0, 400.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(300.0, 0.0, 0.5)), // Positive Z
            ..default()
        },
        PlacementZoneMarker {
            zone_type: PlacementZoneType::GridZone,
        },
    ));
    
    // Top free zone (bright blue rectangle)
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.0, 0.0, 1.0, 0.25), // Much more visible
                custom_size: Some(Vec2::new(600.0, 150.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 200.0, 0.5)), // Positive Z
            ..default()
        },
        PlacementZoneMarker {
            zone_type: PlacementZoneType::FreeZone,
        },
    ));
    
    // Bottom free zone (bright blue rectangle)
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.0, 0.0, 1.0, 0.25), // Much more visible
                custom_size: Some(Vec2::new(600.0, 150.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, -200.0, 0.5)), // Positive Z
            ..default()
        },
        PlacementZoneMarker {
            zone_type: PlacementZoneType::FreeZone,
        },
    ));
    
    // Add grid lines and boundaries for the grid zones to show snapping points
    add_grid_lines_with_borders(&mut commands, Vec2::new(-300.0, 0.0), Vec2::new(200.0, 400.0), 64.0);
    add_grid_lines_with_borders(&mut commands, Vec2::new(300.0, 0.0), Vec2::new(200.0, 400.0), 64.0);
}

// Helper function to add visual grid lines with borders and separation
fn add_grid_lines_with_borders(commands: &mut Commands, center: Vec2, size: Vec2, grid_size: f32) {
    let half_width = size.x / 2.0;
    let half_height = size.y / 2.0;
    
    // Add thick border lines around the entire grid zone
    // Top border
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::srgba(1.0, 1.0, 1.0, 0.6),
            custom_size: Some(Vec2::new(size.x + 4.0, 3.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(center.x, center.y + half_height, 0.7)),
        ..default()
    });
    
    // Bottom border
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::srgba(1.0, 1.0, 1.0, 0.6),
            custom_size: Some(Vec2::new(size.x + 4.0, 3.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(center.x, center.y - half_height, 0.7)),
        ..default()
    });
    
    // Left border
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::srgba(1.0, 1.0, 1.0, 0.6),
            custom_size: Some(Vec2::new(3.0, size.y + 4.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(center.x - half_width, center.y, 0.7)),
        ..default()
    });
    
    // Right border
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::srgba(1.0, 1.0, 1.0, 0.6),
            custom_size: Some(Vec2::new(3.0, size.y + 4.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(center.x + half_width, center.y, 0.7)),
        ..default()
    });
    
    // Internal vertical grid lines
    let mut x = -half_width + grid_size;
    while x < half_width {
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(1.0, 1.0, 1.0, 0.3),
                custom_size: Some(Vec2::new(1.0, size.y)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(center.x + x, center.y, 0.6)),
            ..default()
        });
        x += grid_size;
    }
    
    // Internal horizontal grid lines  
    let mut y = -half_height + grid_size;
    while y < half_height {
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(1.0, 1.0, 1.0, 0.3),
                custom_size: Some(Vec2::new(size.x, 1.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(center.x, center.y + y, 0.6)),
            ..default()
        });
        y += grid_size;
    }
}