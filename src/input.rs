use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};

use crate::{snake::Direction, GameState};

pub fn plugin(app: &mut App) {
    app.add_systems(
        PreUpdate,
        keyboard.after(on_timer(Duration::from_millis(200))),
    );
}

fn keyboard(
    keys: Res<ButtonInput<KeyCode>>,
    mut direction: ResMut<Direction>,
    game_state: Res<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if keys.pressed(KeyCode::ArrowUp) && *direction != Direction::Up.opposite() {
        *direction = Direction::Up;

        if matches!(game_state.get(), GameState::Paused) {
            next_game_state.set(GameState::Playing);
        }
    }

    if keys.pressed(KeyCode::ArrowDown) && *direction != Direction::Down.opposite() {
        *direction = Direction::Down;

        if matches!(game_state.get(), GameState::Paused) {
            next_game_state.set(GameState::Playing);
        }
    }

    if keys.pressed(KeyCode::ArrowLeft) && *direction != Direction::Left.opposite() {
        *direction = Direction::Left;

        if matches!(game_state.get(), GameState::Paused) {
            next_game_state.set(GameState::Playing);
        }
    }

    if keys.pressed(KeyCode::ArrowRight) && *direction != Direction::Right.opposite() {
        *direction = Direction::Right;

        if matches!(game_state.get(), GameState::Paused) {
            next_game_state.set(GameState::Playing);
        }
    }
}
