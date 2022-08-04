use bevy::prelude::*;
use bevy_prototype_lyon::prelude::ShapePlugin;
extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

pub(crate) use log;

pub struct UIConfigPlugin;

impl Plugin for UIConfigPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WindowDescriptor {
            title: "GlodGET".to_string(),
            width: 640.0,
            height: 640.0,
            ..Default::default()
        })
        .insert_resource(Msaa::default())
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_startup_system(setup.label("main_setup"))
        //        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn().insert_bundle(Camera2dBundle::default());

    commands.insert_resource(TextStyle {
        font_size: 60.0,
        color: Color::WHITE,
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
    })
}
