use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod random;
use random::*;

mod unit;
use unit::*;

mod weapon;
use weapon::*;

mod gui;
use gui::config::*;
use gui::glod::*;
use gui::menu::*;
use gui::start::*;
use gui::unit::*;
use gui::weapon::*;

mod start;
use start::*;

mod state;
use state::*;

mod physics;
use physics::*;

mod glod;
use glod::*;

fn main() {
    App::new()
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
            PIXELS_PER_METER,
        ))
        .add_startup_system(rapier_setup)
        .add_plugin(StatePlugin)
        .add_plugin(StartPointPlugin)
        .add_plugin(GlodPlugin)
        .add_plugin(RandPlugin)
        .add_plugin(UnitPlugin)
        .add_plugin(WeaponPlugin)
        .add_plugin(UIConfigPlugin)
        .add_plugin(UIWeaponPlugin)
        .add_plugin(UIStartPointPlugin)
        .add_plugin(UIGlodPlugin)
        .add_plugin(UIUnitPlugin)
        .add_plugin(UIMenuPlugin)
        .run();
}

fn rapier_setup(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::new(0.0, 0.0);
}
