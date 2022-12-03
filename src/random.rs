use crate::{AppState, GameState};
use crate::{FoeStartingPoint, FriendStartingPoint, START_RADIUS};
use crate::{GLOD_QUANTITY, GLOD_RADIUS};
use bevy::prelude::*;
use rand::distributions::{Alphanumeric, DistString};
use rand::prelude::*;
use rand_pcg::Pcg64;
use rand_seeder::Seeder;

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
#[derive(Resource)]
pub struct GameSeed(pub String);

impl Default for GameSeed {
    fn default() -> GameSeed {
        GameSeed(Alphanumeric.sample_string(&mut rand::thread_rng(), 8))
    }
}

#[derive(Resource)]
pub struct GlodPoints {
    pub glods: [Vec3; GLOD_QUANTITY],
}

struct GlodCoord {
    rng: Pcg64,
}

impl Iterator for GlodCoord {
    type Item = Vec3;

    fn next(&mut self) -> Option<Self::Item> {
        Some(Vec3::new(
            self.rng
                .gen_range(-320.0 + GLOD_RADIUS..320.0 - GLOD_RADIUS),
            self.rng
                .gen_range(-320.0 + GLOD_RADIUS..320.0 - GLOD_RADIUS),
            0.0,
        ))
    }
}

pub struct RandPlugin;

impl Plugin for RandPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameSeed::default())
            .insert_resource(GlodPoints {
                glods: [Vec3::ZERO; GLOD_QUANTITY],
            })
            .insert_resource(FriendStartingPoint(Vec3::ZERO))
            .insert_resource(FoeStartingPoint(Vec3::ZERO))
            .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(gen_random))
            .add_system_set(
                SystemSet::on_enter(GameState::SampleRandom).with_system(update_game_resources),
            );
    }
}

fn update_game_resources(
    game_seed: Res<GameSeed>,
    mut friend: ResMut<FriendStartingPoint>,
    mut foe: ResMut<FoeStartingPoint>,
    mut glod_points: ResMut<GlodPoints>,
) {
    let mut rng: Pcg64 = Seeder::from((*game_seed).0.clone()).make_rng();
    *friend = FriendStartingPoint(Vec3::new(
        rng.gen_range(-320.0 + START_RADIUS..0.0 - START_RADIUS),
        rng.gen_range(-320.0 + START_RADIUS..0.0 - START_RADIUS),
        0.0,
    ));
    *foe = FoeStartingPoint(Vec3::new(
        rng.gen_range(0.0 + START_RADIUS..320.0 - START_RADIUS),
        rng.gen_range(0.0 + START_RADIUS..320.0 - START_RADIUS),
        0.0,
    ));

    let g = GlodCoord { rng };
    *glod_points = GlodPoints {
        glods: g
            .into_iter()
            .take(GLOD_QUANTITY)
            .collect::<Vec<Vec3>>()
            .try_into()
            .unwrap(),
    };
}

fn gen_random(mut game_state: ResMut<State<GameState>>) {
    game_state.set(GameState::SampleRandom).unwrap();
}
