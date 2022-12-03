use crate::{Drawn, EnemyBase, GameState, IsBase, START_RADIUS};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude as lyon;

pub struct UIStartPointPlugin;

impl Plugin for UIStartPointPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Game).with_system(draw_startpoints));
    }
}

fn draw_startpoints(
    mut commands: Commands,
    friend_q: Query<(Entity, &Transform), (With<IsBase>, Without<Drawn>)>,
    enemy_q: Query<(Entity, &Transform), (With<EnemyBase>, Without<Drawn>)>,
) {
    for ((friend, f_trans), (enemy, e_trans)) in friend_q.iter().zip(enemy_q.iter()) {
        commands
            .entity(friend)
            .insert(Drawn)
            .insert(lyon::GeometryBuilder::build_as(
                &lyon::shapes::Circle {
                    radius: START_RADIUS,
                    center: Vec2::ZERO,
                },
                lyon::DrawMode::Stroke(lyon::StrokeMode {
                    color: Color::TEAL,
                    options: lyon::StrokeOptions::default().with_line_width(4.0),
                }),
                *f_trans,
            ));

        commands
            .entity(enemy)
            .insert(Drawn)
            .insert(lyon::GeometryBuilder::build_as(
                &lyon::shapes::Rectangle {
                    extents: Vec2::new(-80.0, 80.0),
                    origin: lyon::RectangleOrigin::Center,
                },
                lyon::DrawMode::Stroke(lyon::StrokeMode {
                    color: Color::RED,
                    options: lyon::StrokeOptions::default().with_line_width(4.0),
                }),
                *e_trans,
            ));
    }
}
