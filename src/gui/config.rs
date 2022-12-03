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
        app.insert_resource(Msaa::default())
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_startup_system(setup.label("main_setup"))
        //        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "GlodGET".to_string(),
                width: 640.0,
                height: 640.0,
                ..default()
            },
        ..default()
        }))
        .add_plugin(ShapePlugin);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
