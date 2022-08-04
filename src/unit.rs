use crate::{BodyForce, ForceProfile};
use crate::{CollisionFilters, CollisionMemberships};
use crate::{
    Explodee, FoeStartingPoint, FriendStartingPoint, GameState, IsBase, IsGlod, Score, Weapon,
    WeaponPreLaunch, GLOD_MASS, WEAPON_MASS,
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(EnemyState::Undefined)
            .add_system_set(SystemSet::on_enter(GameState::Game).with_system(spawn_unit))
            .add_system_set(SystemSet::on_update(GameState::Game).with_system(display_events))
            .add_system_set(SystemSet::on_enter(GameState::Victory).with_system(despawn_units))
            .add_system_set(SystemSet::on_enter(GameState::GameOver).with_system(despawn_units));
    }
}

fn despawn_units(
    mut commands: Commands,
    unit: Query<Entity, With<Playable>>,
    enemy: Query<Entity, With<IsEnemy>>,
) {
    commands.entity(unit.single()).despawn_recursive();
    commands.entity(enemy.single()).despawn();
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum EnemyState {
    Undefined,
    PreStart,
    Released,
}

#[derive(Component)]
pub struct Playable;

#[derive(Component)]
pub struct IsEnemy;

pub const UNIT_FORCE_PROFILE: ForceProfile = ForceProfile {
    forward: 150.0,
    torque: 3000.0,
};
const UNIT_MASS: f32 = 3.0;

pub const UNIT_POINTS: [Vec2; 3] = [
    Vec2::new(-8.0, -8.0),
    Vec2::new(-8.0, 8.0),
    Vec2::new(14.0, 0.0),
];

#[derive(Component)]
struct PlayableCollider;

fn spawn_unit(
    mut commands: Commands,
    friend_start: Res<FriendStartingPoint>,
    foe_start: Res<FoeStartingPoint>,
    mut enemy_state: ResMut<State<EnemyState>>,
) {
    commands
        .spawn()
        .insert_bundle(SpatialBundle::visible_identity())
        .insert_bundle(TransformBundle::from_transform(
            Transform::from_translation(friend_start.0),
        ))
        .insert(Playable)
        .insert(Explodee)
        .insert(RigidBody::Dynamic)
        .with_children(|children| {
            children
                .spawn()
                .insert_bundle(SpatialBundle::visible_identity())
                .insert(PlayableCollider)
                .insert(Collider::triangle(
                    UNIT_POINTS[0],
                    UNIT_POINTS[1],
                    UNIT_POINTS[2],
                ))
                .insert(ActiveEvents::COLLISION_EVENTS)
                .insert(ColliderMassProperties::Mass(UNIT_MASS))
                .insert(CollisionGroups::new(
                    CollisionMemberships::Friend as u32,
                    CollisionFilters::Friend as u32,
                ));
            children
                .spawn()
                .insert(Collider::cuboid(5.0, 3.0))
                .insert(ColliderMassProperties::Mass(WEAPON_MASS))
                .insert(CollisionGroups::new(
                    CollisionMemberships::InertWeapon as u32,
                    CollisionFilters::InertWeapon as u32,
                ))
                .insert_bundle(TransformBundle::from_transform(Transform::from_xyz(
                    4.0, 10.0, 0.0,
                )))
                .insert(Weapon)
                .insert(WeaponPreLaunch);
            children
                .spawn()
                .insert(Collider::cuboid(5.0, 3.0))
                .insert(ColliderMassProperties::Mass(WEAPON_MASS))
                .insert(CollisionGroups::new(
                    CollisionMemberships::InertWeapon as u32,
                    CollisionFilters::InertWeapon as u32,
                ))
                .insert_bundle(TransformBundle::from_transform(Transform::from_xyz(
                    4.0, -10.0, 0.0,
                )))
                .insert(Weapon)
                .insert(WeaponPreLaunch);
        })
        .insert(Velocity::zero())
        .insert(Damping {
            linear_damping: 0.4,
            angular_damping: 1.0,
        })
        .insert(ExternalForce {
            force: Vec2::ZERO,
            torque: 0.0,
        })
        .insert(ExternalImpulse {
            impulse: Vec2::ZERO,
            torque_impulse: 0.0,
        });

    enemy_state.set(EnemyState::PreStart).unwrap();

    commands
        .spawn()
        .insert_bundle(TransformBundle::from_transform(
            Transform::from_translation(foe_start.0),
        ))
        .insert(IsEnemy)
        .insert(RigidBody::Dynamic)
        .insert(Collider::triangle(
            UNIT_POINTS[0],
            UNIT_POINTS[1],
            UNIT_POINTS[2],
        ))
        .insert(CollisionGroups::new(
            CollisionMemberships::Enemy as u32,
            CollisionFilters::Enemy as u32,
        ))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(ColliderMassProperties::Mass(UNIT_MASS))
        .insert(Explodee)
        .insert(Sensor)
        .insert(Velocity::zero())
        .insert(Damping {
            linear_damping: 0.4,
            angular_damping: 1.0,
        })
        .insert(ExternalForce {
            force: Vec2::ZERO,
            torque: 0.0,
        })
        .insert(ExternalImpulse {
            impulse: Vec2::ZERO,
            torque_impulse: 0.0,
        });
}

fn display_events(
    mut query: Query<(Entity, &mut ColliderMassProperties), With<PlayableCollider>>,
    //    mut query: Query<(Entity, &mut ColliderMassProperties), With<Playable>>,
    glods: Query<Entity, With<IsGlod>>,
    enemy: Query<Entity, With<IsEnemy>>,
    base: Query<Entity, With<IsBase>>,
    mut game_state: ResMut<State<GameState>>,
    mut enemy_state: ResMut<State<EnemyState>>,
    mut score: ResMut<Score>,
    mut collision_events: EventReader<CollisionEvent>,
    mut commands: Commands,
) {
    for collision_event in collision_events.iter() {
        for (entity, mut props) in query.iter_mut() {
            if let CollisionEvent::Started(h1, h2, _event_flag) = collision_event {
                let notme: &Entity;
                if *h1 == entity {
                    notme = h2;
                } else if *h2 == entity {
                    notme = h1;
                } else {
                    continue;
                }
                if glods.contains(*notme) {
                    commands.entity(*notme).despawn();
                    *score = match *score {
                        Score(x) => Score(x + 1),
                    };
                    match *props {
                        ColliderMassProperties::Mass(cur) => {
                            *props = ColliderMassProperties::Mass(GLOD_MASS + cur);
                        }
                        _ => panic!(),
                    }
                } else if enemy.contains(*notme) {
                    game_state.set(GameState::GameOver).unwrap();
                } else if base.contains(*notme) {
                    match enemy_state.current() {
                        EnemyState::Released => {
                            game_state.set(GameState::Victory).unwrap();
                        }
                        _ => continue,
                    };
                }
            } else if let CollisionEvent::Stopped(h1, h2, _event_flag) = collision_event {
                let notme: &Entity;
                if *h1 == entity {
                    notme = h2;
                } else if *h2 == entity {
                    notme = h1;
                } else {
                    continue;
                }

                if base.contains(*notme) {
                    enemy_state.set(EnemyState::Released).unwrap();
                }
            }
        }
    }
}

pub fn unit_movement_order<T: Component>(
    order: BodyForce,
    mut query: Query<(&Transform, &mut ExternalForce), With<T>>,
) {
    let (transform, mut ext_force) = query.single_mut();
    ext_force.torque = order.torque;
    ext_force.force = order.force_from_transform(transform);
}
