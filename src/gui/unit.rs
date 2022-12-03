use crate::{
    unit_movement_order, Body, BodyForce, Drawn, EnemyState, GameState, IsEnemy, Playable,
    TorqueDirection, UNIT_FORCE_PROFILE, UNIT_POINTS,
};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude as lyon;
use bevy_rapier2d::prelude::*;

pub struct UIUnitPlugin;

impl Plugin for UIUnitPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Game)
                .with_system(map_edge)
                .with_system(enemy_movement)
                .with_system(unit_movement)
                .with_system(draw_unit),
        );
    }
}

fn map_edge(mut game_state: ResMut<State<GameState>>, unit: Query<&Transform, With<Playable>>) {
    let pos = unit.single().translation;
    if (f32::abs(pos.x) > 342.0) || (f32::abs(pos.y) > 342.0) {
        game_state.set(GameState::GameOver).unwrap();
    };
}

fn draw_unit(
    mut commands: Commands,
    friend_q: Query<(Entity, &Transform), (With<Playable>, Without<Drawn>)>,
    enemy_q: Query<(Entity, &Transform), (With<IsEnemy>, Without<Drawn>)>,
) {
    for ((friend, f_trans), (enemy, e_trans)) in friend_q.iter().zip(enemy_q.iter()) {
        commands
            .entity(friend)
            .insert(Drawn)
            .insert(lyon::GeometryBuilder::build_as(
                &lyon::shapes::Polygon {
                    points: UNIT_POINTS.to_vec(),
                    closed: true,
                },
                lyon::DrawMode::Fill(lyon::FillMode::color(Color::BLUE)),
                *f_trans,
            ));

        commands
            .entity(enemy)
            .insert(Drawn)
            .insert(lyon::GeometryBuilder::build_as(
                &lyon::shapes::Polygon {
                    points: UNIT_POINTS.to_vec(),
                    closed: true,
                },
                lyon::DrawMode::Fill(lyon::FillMode::color(Color::RED)),
                *e_trans,
            ));
    }
}

fn enemy_movement(
    player: Query<(&Transform, Option<&Velocity>), With<Playable>>,
    enemy: Query<(&Transform, Option<&Velocity>), With<IsEnemy>>,
    query: Query<(&Transform, &mut ExternalForce), With<IsEnemy>>,
    enemy_state: ResMut<State<EnemyState>>,
) {
    let friend = Body::new(player.single());
    let enemy = Body::new(enemy.single());

    let (dir, angle) = enemy.bearing_of(&friend);

    let forward = match enemy_state.current() {
        EnemyState::Released => 1.0 / (1.0 + 2.0_f32.powf(angle)),
        _ => 0.0,
    };

    unit_movement_order(
        BodyForce::new(dir * f32::min(angle, 0.4), forward, UNIT_FORCE_PROFILE),
        query,
    );
}

fn unit_movement(
    keyboard_input: Res<Input<KeyCode>>,
    query: Query<(&Transform, &mut ExternalForce), With<Playable>>,
) {
    let mut torque = 0.0;
    if keyboard_input.pressed(KeyCode::Left) {
        torque = TorqueDirection::Left * 1.0;
    } else if keyboard_input.pressed(KeyCode::Right) {
        torque = TorqueDirection::Right * 1.0;
    };

    unit_movement_order(
        BodyForce::new(
            torque,
            if keyboard_input.pressed(KeyCode::Up) {
                1.0
            } else {
                0.0
            },
            UNIT_FORCE_PROFILE,
        ),
        query,
    );
}
