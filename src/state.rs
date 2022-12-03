use crate::GameSeed;
use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    SeedMenu,
    InGame,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    OutOfGame,
    SampleRandom,
    Game,
    Victory,
    GameOver,
}

#[derive(Component)]
pub struct IsGlod;

#[derive(Component)]
pub struct Drawn;

#[derive(Component)]
pub struct IsBase;

#[derive(Component)]
pub struct EnemyBase;

#[derive(Resource)]
pub struct FriendStartingPoint(pub Vec3);

#[derive(Resource)]
pub struct FoeStartingPoint(pub Vec3);

#[derive(Resource)]
pub struct Score(pub u8);

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score(0))
            .add_state(AppState::MainMenu)
            .add_state(GameState::OutOfGame)
            .add_system_set(SystemSet::on_update(GameState::OutOfGame).with_system(next_game))
            .add_system_set(SystemSet::on_update(GameState::SampleRandom).with_system(start_game))
            .add_system_set(SystemSet::on_enter(GameState::Game).with_system(reset_score));
    }
}

fn reset_score(mut score: ResMut<Score>) {
    *score = match *score {
        Score(_) => Score(0),
    };
}

fn start_game(mut game_state: ResMut<State<GameState>>) {
    game_state.set(GameState::Game).unwrap();
}

fn next_game(
    app_state: ResMut<State<AppState>>,
    mut game_state: ResMut<State<GameState>>,
    mut game_seed: ResMut<GameSeed>,
) {
    if *app_state.current() == AppState::InGame {
        *game_seed = GameSeed::default();
        game_state.set(GameState::SampleRandom).unwrap();
    };
}
