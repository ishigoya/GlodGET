use crate::log;
use crate::GameSeed;
use crate::{AppState, GameState, Score};
use bevy::prelude::*;

#[derive(Component)]
pub struct Ui;

pub struct UIMenuPlugin;

impl Plugin for UIMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(main_menu))
            .add_system_set(
                SystemSet::on_update(AppState::MainMenu).with_system(main_menu_key_input),
            )
            .add_system_set(SystemSet::on_exit(AppState::MainMenu).with_system(exit_ui_despawn))
            .add_system_set(SystemSet::on_enter(AppState::SeedMenu).with_system(seed_menu))
            .add_system_set(
                SystemSet::on_update(AppState::SeedMenu)
                    .with_system(exit_ui_despawn.before("draw"))
                    .with_system(seed_menu_key_input.label("input"))
                    .with_system(seed_menu.after("input").label("draw")),
            )
            .add_system_set(SystemSet::on_exit(AppState::SeedMenu).with_system(exit_ui_despawn))
            .add_system_set(SystemSet::on_enter(GameState::Victory).with_system(victory))
            .add_system_set(
                SystemSet::on_update(GameState::Victory).with_system(end_game_key_input),
            )
            .add_system_set(SystemSet::on_exit(GameState::Victory).with_system(exit_ui_despawn))
            .add_system_set(SystemSet::on_enter(GameState::GameOver).with_system(gameover))
            .add_system_set(
                SystemSet::on_update(GameState::GameOver).with_system(end_game_key_input),
            )
            .add_system_set(SystemSet::on_exit(GameState::GameOver).with_system(exit_ui_despawn));
    }
}

fn exit_ui_despawn(mut commands: Commands, query: Query<Entity, With<Ui>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

#[cfg(target_family = "wasm")]
fn victory(mut commands: Commands, score: ResMut<Score>, asset_server: Res<AssetServer>) {
    let text_alignment = TextAlignment::CENTER;
    commands
        .spawn(Text2dBundle {
            text: Text::from_section(
                "You win!\nYour score: ".to_owned()
                    + &((*score).0.to_string()
                        + "\nEnter: replay same map\ns: new map\nEsc: return to main menu"),
                TextStyle {
        font_size: 60.0,
        color: Color::WHITE,
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
    },
            )
            .with_alignment(text_alignment),
            ..default()
        })
        .insert(Ui);
}

#[cfg(target_family = "wasm")]
fn main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let text_alignment = TextAlignment::CENTER;
    commands
        .spawn(Text2dBundle {
            text: Text::from_section(
                "GlodGET\n\nEnter: New Game\nS: set game seed",
                TextStyle {
        font_size: 60.0,
        color: Color::WHITE,
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
    },
            )
            .with_alignment(text_alignment),
            ..default()
        })
        .insert(Ui);
}

#[cfg(target_family = "wasm")]
fn main_menu_key_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_state: ResMut<State<AppState>>,
) {
    if keyboard_input.pressed(KeyCode::Return) {
        app_state.set(AppState::InGame).unwrap();
    } else if keyboard_input.pressed(KeyCode::S) {
        app_state.set(AppState::SeedMenu).unwrap();
    };
}

#[cfg(target_family = "wasm")]
fn seed_menu(mut commands: Commands, game_seed: ResMut<GameSeed>, asset_server: Res<AssetServer>) {
    let text_alignment = TextAlignment::CENTER;
    commands
        .spawn(Text2dBundle {
            text: Text::from_section(
                "Seed Menu\n\nHit enter to launch game\nSeed:\n ".to_owned()
                    + &((*game_seed).0.to_string()),
                TextStyle {
        font_size: 60.0,
        color: Color::WHITE,
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
    },
            )
            .with_alignment(text_alignment),
            ..default()
        })
        .insert(Ui);
}

#[cfg(target_family = "wasm")]
fn seed_menu_key_input(
    mut char_evr: EventReader<ReceivedCharacter>,
    keyboard_input: Res<Input<KeyCode>>,
    mut app_state: ResMut<State<AppState>>,
    mut game_seed: ResMut<GameSeed>,
) {
    for ev in char_evr.iter() {
        let GameSeed(mut string) = (*game_seed).clone();
        string.push(ev.char);
        *game_seed = GameSeed(string);
    }

    if keyboard_input.just_pressed(KeyCode::Back) {
        let GameSeed(mut string) = (*game_seed).clone();
        string.pop();
        *game_seed = GameSeed(string);
    };

    if keyboard_input.pressed(KeyCode::Return) {
        app_state.set(AppState::InGame).unwrap();
    };
}

#[cfg(target_family = "wasm")]
fn end_game_key_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_state: ResMut<State<AppState>>,
    mut game_state: ResMut<State<GameState>>,
) {
    if keyboard_input.pressed(KeyCode::Return) {
        game_state.set(GameState::Game).unwrap();
    } else if keyboard_input.pressed(KeyCode::Escape) {
        app_state.set(AppState::MainMenu).unwrap();
        game_state.set(GameState::OutOfGame).unwrap();
    } else if keyboard_input.pressed(KeyCode::S) {
        game_state.set(GameState::OutOfGame).unwrap();
    };
}

fn gameover(mut commands: Commands, asset_server: Res<AssetServer>) {
    let text_alignment = TextAlignment::CENTER;
    commands
        .spawn(Text2dBundle {
            text: Text::from_section(
                "You lose!\nEnter: replay same map\ns: new map\nEsc: return to main menu",
                TextStyle {
        font_size: 60.0,
        color: Color::WHITE,
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
    },
            )
            .with_alignment(text_alignment),
            ..default()
        })
        .insert(Ui);
}
