use bevy::{math::Vec2, prelude::*};
use rand::Rng;

use crate::{CELL_COUNT, CELL_SIZE};

#[derive(Component, Clone)]
pub struct Food {
    pub position: Vec2,
}

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup)
        .add_systems(PostUpdate, position_translation);
}

fn generate_random_pos() -> Vec2 {
    let mut rng = rand::thread_rng();

    Vec2::new(
        rng.gen_range(0.0..CELL_COUNT - 1.0).round(),
        rng.gen_range(0.0..CELL_COUNT - 1.0).round(),
    )
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let food = Food {
        position: generate_random_pos(),
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
