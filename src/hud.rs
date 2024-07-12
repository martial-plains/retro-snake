use bevy::prelude::*;
use bevy_vector_shapes::{
    painter::ShapeConfig,
    shapes::{RectangleBundle, ShapeBundle},
};

use crate::{utils::DARK_GREEN, GameState, CELL_COUNT, CELL_SIZE};

pub fn plugin(app: &mut App) {
    app.insert_resource(Score(0))
        .add_systems(Startup, draw_border)
        .add_systems(Startup, draw_title)
        .add_systems(Startup, draw_score)
        .add_systems(OnEnter(GameState::FoodEaten), increase_socre)
        .add_systems(OnEnter(GameState::GameOver), reset_socre)
        .add_systems(
            PostUpdate,
            (
                border_position_translation,
                title_position_translation,
                score_position_translation,
            ),
        )
        .add_systems(Update, update_score_text);
}

#[derive(Debug, Resource)]
struct Score(usize);

#[derive(Debug, Component)]
struct Border(Vec2);

#[derive(Debug, Component)]
struct Title(Vec2);

#[derive(Debug, Component)]
struct ScoreText(Vec2);

fn increase_socre(mut score: ResMut<Score>) {
    score.0 += 1;
}

fn reset_socre(mut score: ResMut<Score>) {
    score.0 = 0;
}

fn update_score_text(score: Res<Score>, mut query: Query<&mut Text, With<ScoreText>>) {
    for mut text in &mut query {
        text.sections[0].value = format!("{}", score.0);
    }
}

fn draw_title(mut commands: Commands, asset_server: Res<AssetServer>) {
    let position = Vec2::new(3.0, -1.5);
    commands
        .spawn(Text2dBundle {
            text: Text {
                sections: vec![TextSection::new(
                    "Retro Snake",
                    TextStyle {
                        // This font is loaded and will be used instead of the default font.
                        font: asset_server.load("fonts/Fira_Mono/FiraMono-Bold.ttf"),
                        color: DARK_GREEN,
                        font_size: 40.0,
                    },
                )],
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(position.x, position.y, 0.0)),
            ..Default::default()
        })
        .insert(Title(position));
}

fn draw_border(mut commands: Commands) {
    let position = Vec2::new(12.0, 12.0);
    commands
        .spawn(ShapeBundle::rect(
            &ShapeConfig {
                color: DARK_GREEN,
                hollow: true,
                transform: Transform::from_translation(Vec3::new(position.x, position.y, 0.0)),
                thickness: 5.0,
                ..ShapeConfig::default_2d()
            },
            Vec2::new(CELL_SIZE * CELL_COUNT + 10.0, CELL_SIZE * CELL_COUNT + 10.0),
        ))
        .insert(Border(position));
}

fn draw_score(mut commands: Commands, asset_server: Res<AssetServer>, score: Res<Score>) {
    let position = Vec2::new(0.0, CELL_COUNT + 0.2);
    commands
        .spawn(Text2dBundle {
            text: Text {
                sections: vec![TextSection::new(
                    format!("{}", score.0),
                    TextStyle {
                        // This font is loaded and will be used instead of the default font.
                        font: asset_server.load("fonts/Fira_Mono/FiraMono-Bold.ttf"),
                        color: DARK_GREEN,
                        font_size: 40.0,
                    },
                )],
                justify: JustifyText::Left,
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(position.x, position.y, 0.0)),
            ..Default::default()
        })
        .insert(ScoreText(position));
}

fn border_position_translation(mut query: Query<(&Border, &mut Transform)>) {
    for (border, mut transform) in &mut query {
        transform.translation = Vec3::new(border.0.x * CELL_SIZE, -border.0.y * CELL_SIZE, 0.0);
    }
}

fn title_position_translation(mut query: Query<(&Title, &mut Transform)>) {
    for (title, mut transform) in &mut query {
        transform.translation = Vec3::new(title.0.x * CELL_SIZE, -title.0.y * CELL_SIZE, 0.0);
    }
}

fn score_position_translation(mut query: Query<(&ScoreText, &mut Transform)>) {
    for (text, mut transform) in &mut query {
        transform.translation = Vec3::new(text.0.x * CELL_SIZE, -text.0.y * CELL_SIZE, 0.0);
    }
}
