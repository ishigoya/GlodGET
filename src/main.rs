use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[cfg(not(target_family = "wasm"))]
use bevy::render::settings::WgpuSettings;

#[cfg(not(target_family = "wasm"))]
use bevy_websocket_adapter::{
    bevy::{WebSocketClient, WebSocketServer, WsMessageInserter},
    client::Client,
    impl_message_type,
    server::Server,
    shared::NetworkEvent,
};

#[cfg(not(target_family = "wasm"))]
use serde::{Deserialize, Serialize};

#[cfg(not(target_family = "wasm"))]
#[derive(Serialize, Deserialize, Debug, Clone)]
struct DummyEvent {
    a: u32,
}

#[cfg(not(target_family = "wasm"))]
impl_message_type!(DummyEvent, "dummy");

// A macro to provide `println!(..)`-style syntax for `console.log` logging.

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

#[cfg(not(target_family = "wasm"))]
fn connect_to_server(mut ws: ResMut<Client>) {
    ws.connect("ws://127.0.0.1:12345".to_string());
}

#[cfg(not(target_family = "wasm"))]
fn send_dummies(client: Res<Client>) {
    client.send_message(&DummyEvent { a: 2 });
}

#[cfg(not(target_family = "wasm"))]
fn start_listen(mut ws: ResMut<Server>) {
    ws.listen("0.0.0.0:12346")
        .expect("failed to start websocket server");
}

#[cfg(not(target_family = "wasm"))]
fn listen_for_events(mut evs: EventReader<NetworkEvent>) {
    for ev in evs.iter() {
        println!("received NetworkEvent : {:?}", ev);
    }
}

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
