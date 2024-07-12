// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![feature(const_fn_floating_point_arithmetic)]
#![warn(clippy::pedantic)]
#![allow(
    clippy::too_many_arguments,
    clippy::type_complexity,
    clippy::needless_pass_by_value
)]

use bevy::{asset::AssetMetaCheck, prelude::*, window::WindowResolution};
#[cfg(debug_assertions)]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_vector_shapes::ShapePlugin;
use food::Food;
use snake::Snake;
use utils::GREEN;

mod audio;
mod food;
mod hud;
mod input;
pub(crate) mod snake;
mod utils;

const CELL_SIZE: f32 = 30.0;
const CELL_COUNT: f32 = 25.0;

const OFFSET: f32 = 75.0;

const SCREEN_SIZE: f32 = 2.0 * OFFSET + CELL_SIZE * CELL_COUNT;

#[derive(Debug, Default, Clone, Copy, States, Hash, PartialEq, Eq)]
pub enum GameState {
    #[default]
    Playing,
    FoodEaten,
    GameOver,
    Paused,
}

#[derive(Event)]
enum CollisionEvent {
    Food,
    Edges,
    Tail,
}

fn main() {
    let mut binding = App::new();

    binding
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    // Wasm builds will check for meta files (that don't exist) if this isn't set.
                    // This causes errors and even panics in web builds on itch.
                    // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(SCREEN_SIZE, SCREEN_SIZE),
                        title: String::from("Retro Snake"),
                        ..Window::default()
                    }),
                    ..WindowPlugin::default()
                }),
        )
        .add_plugins(audio::plugin)
        .add_plugins(input::plugin)
        .add_plugins(ShapePlugin::default())
        .add_plugins((food::plugin, snake::plugin, hud::plugin))
        .add_event::<CollisionEvent>()
        .insert_state(GameState::Playing)
        .insert_resource(ClearColor(GREEN))
        .add_systems(Startup, setup_camera)
        .add_systems(
            PreUpdate,
            (
                check_collision_with_edges,
                check_collision_with_food,
                check_collision_with_tail,
                handle_collision_event,
            ),
        );

    #[cfg(debug_assertions)]
    binding.add_plugins(WorldInspectorPlugin::new());

    binding.run();
}

fn check_collision_with_food(
    snake: Res<Snake>,
    food_query: Query<(&Food, &mut Transform)>,
    mut collision_event: EventWriter<CollisionEvent>,
) {
    let food_collided = snake.head().is_some_and(|snake_pos| {
        let (food, _) = food_query.single();

        food.position == *snake_pos
    });

    if food_collided {
        collision_event.send(CollisionEvent::Food);
    }
}

fn check_collision_with_edges(snake: Res<Snake>, mut collision_event: EventWriter<CollisionEvent>) {
    let Some(head) = snake.head() else { return };

    if head.x == CELL_COUNT || (head.x - -1.0).abs() < f32::EPSILON {
        collision_event.send(CollisionEvent::Edges);
    }

    if head.y == CELL_COUNT || (head.y - -1.0).abs() < f32::EPSILON {
        collision_event.send(CollisionEvent::Edges);
    }
}

fn check_collision_with_tail(snake: Res<Snake>, mut collision_event: EventWriter<CollisionEvent>) {
    let mut headless_body = snake.body().clone();
    let Some(head) = headless_body.pop_front() else {
        return;
    };

    if headless_body.contains(&head) {
        collision_event.send(CollisionEvent::Tail);
    }
}

fn handle_collision_event(
    mut snake: ResMut<Snake>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut collision_event: EventReader<CollisionEvent>,
) {
    for event in collision_event.read() {
        match event {
            CollisionEvent::Food => {
                next_game_state.set(GameState::FoodEaten);

                snake.is_growing = true;
            }
            CollisionEvent::Edges | CollisionEvent::Tail => {
                next_game_state.set(GameState::GameOver);
            }
        }
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle {
        projection: OrthographicProjection {
            viewport_origin: Vec2::new(0.0, 1.0),
            ..Camera2dBundle::default().projection
        },
        transform: Transform::from_translation(Vec3::new(-OFFSET, OFFSET, 0.0)),
        ..default()
    },));
}
