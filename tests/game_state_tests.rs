use tower_defense_bevy::resources::*;

#[test]
fn test_game_state_creation() {
    let state = GameState::Playing;
    assert!(matches!(state, GameState::Playing));
}

#[test]
fn test_game_state_transitions() {
    let mut state = GameState::Playing;
    state = GameState::GameOver;
    assert!(matches!(state, GameState::GameOver));
}

#[test]
fn test_score_creation() {
    let score = Score::new();
    assert_eq!(score.current, 0);
    assert_eq!(score.enemies_killed, 0);
    assert_eq!(score.enemies_escaped, 0);
}

#[test]
fn test_score_enemy_killed() {
    let mut score = Score::new();
    score.enemy_killed(10);
    assert_eq!(score.current, 10);
    assert_eq!(score.enemies_killed, 1);
    assert_eq!(score.enemies_escaped, 0);
}

#[test]
fn test_score_enemy_escaped() {
    let mut score = Score::new();
    score.enemy_escaped();
    assert_eq!(score.current, 0);
    assert_eq!(score.enemies_killed, 0);
    assert_eq!(score.enemies_escaped, 1);
}

#[test]
fn test_enemy_path_creation() {
    let path = EnemyPath::new(vec![
        (0.0, 0.0).into(),
        (100.0, 0.0).into(),
        (100.0, 100.0).into(),
    ]);
    assert_eq!(path.waypoints.len(), 3);
    assert_eq!(path.waypoints[0].x, 0.0);
    assert_eq!(path.waypoints[2].y, 100.0);
}

#[test]
fn test_enemy_path_get_position_at_start() {
    let path = EnemyPath::new(vec![
        (0.0, 0.0).into(),
        (100.0, 0.0).into(),
    ]);
    let pos = path.get_position_at_progress(0.0);
    assert_eq!(pos.x, 0.0);
    assert_eq!(pos.y, 0.0);
}

#[test]
fn test_enemy_path_get_position_at_end() {
    let path = EnemyPath::new(vec![
        (0.0, 0.0).into(),
        (100.0, 0.0).into(),
    ]);
    let pos = path.get_position_at_progress(1.0);
    assert_eq!(pos.x, 100.0);
    assert_eq!(pos.y, 0.0);
}

#[test]
fn test_enemy_path_get_position_midway() {
    let path = EnemyPath::new(vec![
        (0.0, 0.0).into(),
        (100.0, 0.0).into(),
    ]);
    let pos = path.get_position_at_progress(0.5);
    assert_eq!(pos.x, 50.0);
    assert_eq!(pos.y, 0.0);
}