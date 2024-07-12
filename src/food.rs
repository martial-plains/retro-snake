use bevy::{math::Vec2, prelude::*};
use rand::Rng;

use crate::{snake::Snake, GameState, CELL_COUNT, CELL_SIZE};

#[derive(Component, Clone, Default)]
pub struct Food {
    pub position: Vec2,
}

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup)
        .add_systems(OnEnter(GameState::FoodEaten), generate_random_pos)
        .add_systems(OnEnter(GameState::GameOver), generate_random_pos)
        .add_systems(PostUpdate, position_translation);
}

fn generate_random_pos(snake: ResMut<Snake>, mut food_query: Query<(&mut Food, &mut Transform)>) {
    let (mut f, _) = food_query.single_mut();
    let mut rng = rand::thread_rng();
    f.position = loop {
        let value = Vec2::new(
            rng.gen_range(0.0..CELL_COUNT - 1.0).round(),
            rng.gen_range(0.0..CELL_COUNT - 1.0).round(),
        );

        if !snake.body().contains(&value) {
            break value;
        }
    };
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut rng = rand::thread_rng();
    let food = Food {
        position: Vec2::new(
            rng.gen_range(0.0..CELL_COUNT - 1.0).round(),
            rng.gen_range(0.0..CELL_COUNT - 1.0).round(),
        ),
    };

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(CELL_SIZE, CELL_SIZE)),
                ..default()
            },
            texture: asset_server.load("food.png"),
            ..SpriteBundle::default()
        })
        .insert(Food {
            position: food.position,
        });
}

fn position_translation(mut q: Query<(&Food, &mut Transform)>) {
    for (food, mut transform) in &mut q {
        transform.translation = Vec3::new(
            food.position.x * CELL_SIZE,
            -food.position.y * CELL_SIZE,
            0.0,
        );
    }
}
