use crate::GLOD_RADIUS;
use crate::{Drawn, GameState, IsGlod};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude as lyon;

pub struct UIGlodPlugin;

impl Plugin for UIGlodPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Game).with_system(draw_glod));
    }
}

fn draw_glod(
    mut commands: Commands,
    glods: Query<(Entity, &Transform), (With<IsGlod>, Without<Drawn>)>,
) {
    for (glod, trans) in glods.iter() {
        commands
            .entity(glod)
            .insert(Drawn)
            .insert(lyon::GeometryBuilder::build_as(
                &lyon::shapes::Circle {
                    radius: GLOD_RADIUS,
                    center: Vec2::ZERO,
                },
                lyon::DrawMode::Fill(lyon::FillMode::color(Color::YELLOW)),
                *trans,
            ));
    }
}
