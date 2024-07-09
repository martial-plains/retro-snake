use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioChannel, AudioControl, AudioPlugin, AudioSource, MainTrack};

use crate::GameState;

pub fn plugin(app: &mut App) {
    app.add_plugins(AudioPlugin)
        .add_systems(PreStartup, load_audio)
        .add_systems(OnEnter(GameState::FoodEaten), play_eat_sfx)
        .add_systems(OnEnter(GameState::GameOver), play_wall_sfx);
}

#[derive(Resource)]
#[allow(dead_code)]
struct AudioState {
    wall_handle: Handle<AudioSource>,
    eat_handle: Handle<AudioSource>,
    sfx_channel: AudioChannel<MainTrack>,
    volume: f64,
}

fn play_wall_sfx(
    audio: Res<Audio>,
    audio_state: Res<AudioState>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    audio.play(audio_state.wall_handle.clone());
    next_game_state.set(GameState::Playing);
}

fn play_eat_sfx(
    audio: Res<Audio>,
    audio_state: Res<AudioState>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    audio.play(audio_state.eat_handle.clone());
    next_game_state.set(GameState::Playing);
}

fn load_audio(mut commands: Commands, audio: Res<Audio>, assets: Res<AssetServer>) {
    let eat_handle = assets.load("sounds/eat.ogg");
    let wall_handle = assets.load("sounds/wall.ogg");

    let sfx_channel = AudioChannel::default();
    let volume = 0.5;

    audio.set_volume(volume);

    commands.insert_resource(AudioState {
        wall_handle,
        eat_handle,
        sfx_channel,
        volume,
    });
}
