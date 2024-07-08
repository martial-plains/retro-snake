use std::{collections::VecDeque, time::Duration};

use bevy::{prelude::*, time::common_conditions::on_timer};
use bevy_vector_shapes::{
    painter::ShapeConfig,
    shapes::{RectangleBundle, ShapeBundle},
};

use crate::{utils::DARK_GREEN, CELL_SIZE, GAME_OVER};

#[derive(Debug, Resource, Clone)]
pub struct Snake {
    body: VecDeque<Vec2>,
    pub is_growing: bool,
    pub should_reset: bool,
}

impl Snake {
    pub fn head(&self) -> Option<&Vec2> {
        self.body.front()
    }

    pub fn body(&self) -> &VecDeque<Vec2> {
        &self.body
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup)
        .add_systems(Update, update.run_if(on_timer(Duration::from_millis(200))))
        .add_systems(Update, keyboard_input.after(update))
        .add_systems(PostUpdate, position_translation);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Resource)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    fn to_vec2(self) -> Vec2 {
        match self {
            Self::Up => Vec2 { x: 0.0, y: -1.0 },
            Self::Down => Vec2 { x: 0.0, y: 1.0 },
            Self::Right => Vec2 { x: 1.0, y: 0.0 },
            Self::Left => Vec2 { x: -1.0, y: 0.0 },
        }
    }

    fn opposite(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Right => Self::Left,
            Self::Left => Self::Right,
        }
    }
}

#[derive(Debug, Component)]
struct Segment(Vec2);

impl Default for Snake {
    fn default() -> Self {
        Self {
            body: [
                Vec2::new(6.0, 9.0),
                Vec2::new(5.0, 9.0),
                Vec2::new(4.0, 9.0),
            ]
            .iter()
            .copied()
            .collect::<VecDeque<_>>(),
            is_growing: false,
            should_reset: false,
        }
    }
}

fn setup(mut commands: Commands) {
    let snake = Snake::default();

    for position in &snake.body {
        spawn_snake_segment(&mut commands, *position);
    }

    commands.insert_resource(snake);
    commands.insert_resource(Direction::Right);
}

fn update(
    mut commands: Commands,
    mut snake: ResMut<Snake>,
    mut direction: ResMut<Direction>,
    q: Query<(Entity, &Segment)>,
) {
    if snake.should_reset {
        snake.body = [
            Vec2::new(6.0, 9.0),
            Vec2::new(5.0, 9.0),
            Vec2::new(4.0, 9.0),
        ]
        .iter()
        .copied()
        .collect::<VecDeque<_>>();
        *direction = Direction::Right;
        snake.should_reset = false;

        for (entity_id, _) in q.iter() {
            commands.entity(entity_id).despawn();
        }

        for position in &snake.body {
            spawn_snake_segment(&mut commands, *position);
        }
    }

    if unsafe { GAME_OVER } {
        return;
    }

    if let Some(head) = snake.head().copied() {
        snake.body.push_front(head + direction.to_vec2());
    }

    if snake.is_growing {
        snake.is_growing = false;
    } else {
        snake.body.pop_back();

        for (entity_id, _) in q.iter() {
            commands.entity(entity_id).despawn();
        }
    }

    for position in &snake.body {
        spawn_snake_segment(&mut commands, *position);
    }
}

fn keyboard_input(keys: Res<ButtonInput<KeyCode>>, mut direction: ResMut<Direction>) {
    if keys.just_pressed(KeyCode::ArrowUp) && *direction != Direction::Up.opposite() {
        *direction = Direction::Up;
        unsafe {
            GAME_OVER = false;
        }
    }
    if keys.just_released(KeyCode::ArrowDown) && *direction != Direction::Down.opposite() {
        *direction = Direction::Down;
        unsafe {
            GAME_OVER = false;
        }
    }
    if keys.pressed(KeyCode::ArrowLeft) && *direction != Direction::Left.opposite() {
        *direction = Direction::Left;
        unsafe {
            GAME_OVER = false;
        }
    }

    if keys.pressed(KeyCode::ArrowRight) && *direction != Direction::Right.opposite() {
        *direction = Direction::Right;
        unsafe {
            GAME_OVER = false;
        }
    }
}

fn spawn_snake_segment(commands: &mut Commands, position: Vec2) {
    commands
        .spawn(ShapeBundle::rect(
            &ShapeConfig {
                color: DARK_GREEN,
                // Adjust size as needed
                transform: Transform::from_translation(Vec3::new(position.x, position.y, 0.0)),
                corner_radii: Vec4::splat(5.0),
                ..ShapeConfig::default_2d()
            },
            Vec2::splat(CELL_SIZE),
        ))
        .insert(Segment(position));
}

fn position_translation(mut q: Query<(&Segment, &mut Transform)>) {
    for (segment, mut transform) in &mut q {
        transform.translation = Vec3::new(segment.0.x * CELL_SIZE, -segment.0.y * CELL_SIZE, 0.0);
    }
}
