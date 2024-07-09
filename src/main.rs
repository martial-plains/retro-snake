// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![feature(const_fn_floating_point_arithmetic)]
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy::{asset::AssetMetaCheck, prelude::*, window::WindowResolution};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_vector_shapes::ShapePlugin;
use food::Food;
use rand::Rng;
use snake::Snake;
use utils::GREEN;

mod audio;
mod food;
mod hud;
mod snake;
mod utils;

const CELL_SIZE: f32 = 30.0;
const CELL_COUNT: f32 = 25.0;

const OFFSET: f32 = 75.0;

static mut GAME_OVER: bool = false;

const SCREEN_SIZE: f32 = 2.0 * OFFSET + CELL_SIZE * CELL_COUNT;

#[derive(Event)]
enum CollisionEvent {
    Food,
    Edges,
    Tail,
}

#[derive(Debug, Resource)]
struct Score(usize);

#[derive(Debug, Default, Clone, Copy, States, Hash, PartialEq, Eq)]
pub enum GameState {
    #[default]
    Playing,
    FoodEaten,
    GameOver,
    Paused,
}

fn main() {
    let mut binding = App::new();

    binding
        .insert_resource(Score(0))
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
        .add_plugins(ShapePlugin::default())
        .add_plugins((food::plugin, snake::plugin, hud::plugin))
        .add_event::<CollisionEvent>()
        .insert_state(GameState::Playing)
        .insert_resource(ClearColor(GREEN))
        .add_systems(Startup, setup_camera)
        .add_systems(
            Update,
            (
                check_collision_with_edges,
                check_collision_with_food,
                check_collision_with_tail,
            ),
        )
        .add_systems(
            Update,
            handle_collision_event.after(check_collision_with_food),
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

    if head.x == CELL_COUNT - 1.0 || head.x == -1.0 {
        collision_event.send(CollisionEvent::Edges);
    }

    if head.y == CELL_COUNT - 1.0 || head.y == -1.0 {
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
    mut score: ResMut<Score>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut food_query: Query<(&mut Food, &mut Transform)>,
    mut collision_event: EventReader<CollisionEvent>,
) {
    for event in collision_event.read() {
        match event {
            CollisionEvent::Food => {
                next_game_state.set(GameState::FoodEaten);

                snake.is_growing = true;
                score.0 += 1;
                let (mut f, _) = food_query.single_mut();
                f.position = {
                    let mut rng = rand::thread_rng();
                    loop {
                        let value = Vec2::new(
                            (rng.gen_range(0..CELL_COUNT as usize - 1)) as f32,
                            (rng.gen_range(0..CELL_COUNT as usize - 1)) as f32,
                        );

                        if !snake.body().contains(&value) {
                            break value;
                        }
                    }
                };
            }
            CollisionEvent::Edges | CollisionEvent::Tail => game_over(
                &mut snake,
                &mut score,
                &mut next_game_state,
                &mut food_query,
            ),
        }
    }
}

fn game_over(
    snake: &mut ResMut<Snake>,
    score: &mut ResMut<Score>,
    next_game_state: &mut ResMut<NextState<GameState>>,
    food_query: &mut Query<(&mut Food, &mut Transform)>,
) {
    let mut rng = rand::thread_rng();
    let (mut food, _) = food_query.single_mut();

    score.0 = 0;
    snake.should_reset = true;
    food.position = Vec2::new(
        (rng.gen_range(0..CELL_COUNT as usize - 1)) as f32,
        (rng.gen_range(0..CELL_COUNT as usize - 1)) as f32,
    );

    next_game_state.set(GameState::GameOver);

    unsafe {
        GAME_OVER = true;
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
