use crate::{CollisionFilters, CollisionMemberships};
use crate::{Drawn, Explosion, GameState, Playable, Torpedo, Weapon, WeaponPreLaunch, WEAPON_MASS};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude as lyon;
use bevy_rapier2d::prelude::*;

pub struct UIWeaponPlugin;

impl Plugin for UIWeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Game)
                .with_system(draw_inert_weapons)
                .with_system(standby_fire)
                .with_system(draw_explosions),
        );
    }
}

fn draw_inert_weapons(
    mut commands: Commands,
    weapons_q: Query<(Entity, &Transform), (With<WeaponPreLaunch>, Without<Drawn>)>,
) {
    for (weapon, trans) in weapons_q.iter() {
        commands
            .entity(weapon)
            .insert(Drawn)
            .insert_bundle(lyon::GeometryBuilder::build_as(
                &lyon::shapes::Rectangle {
                    extents: Vec2::new(5.0, 3.0),
                    origin: lyon::shapes::RectangleOrigin::Center,
                },
                lyon::DrawMode::Fill(lyon::FillMode::color(Color::WHITE)),
                *trans,
            ));
    }
}

fn standby_fire(
    keyboard_input: Res<Input<KeyCode>>,
    mut commands: Commands,
    weapons: Query<(Entity, &Transform), With<WeaponPreLaunch>>,
    unit: Query<(&Transform, &Velocity), With<Playable>>,
) {
    let (trans, velocity) = unit.single();
    match weapons.iter().last() {
        Some((child, c_trans)) => {
            if keyboard_input.just_released(KeyCode::Space) {
                commands.entity(child).despawn();

                commands
                    .spawn()
                    .insert(RigidBody::Dynamic)
                    .insert(Damping {
                        linear_damping: 0.2,
                        angular_damping: 1.5,
                    })
                    .insert(Collider::cuboid(5.0, 3.0))
                    .insert(CollisionGroups::new(
                        CollisionMemberships::KineticWeapon as u32,
                        CollisionFilters::KineticWeapon as u32,
                    ))
                    .insert(ColliderMassProperties::Mass(WEAPON_MASS))
                    .insert(ActiveEvents::COLLISION_EVENTS)
                    .insert_bundle(lyon::GeometryBuilder::build_as(
                        &lyon::shapes::Rectangle {
                            extents: Vec2::new(5.0, 3.0),
                            origin: lyon::shapes::RectangleOrigin::Center,
                        },
                        lyon::DrawMode::Fill(lyon::FillMode::color(Color::WHITE)),
                        (*trans) * (*c_trans),
                    ))
                    .insert(Weapon)
                    .insert(ExternalForce {
                        force: Vec2::ZERO,
                        torque: 0.0,
                    })
                    .insert(*velocity)
                    .insert(Torpedo::new());
            };
        }
        None => (),
    };
}

fn draw_explosions(
    mut commands: Commands,
    explosions: Query<(Entity, &Transform), (With<Explosion>, Without<Drawn>)>,
) {
    for (entity, trans) in explosions.iter() {
        commands
            .entity(entity)
            .insert(Drawn)
            .insert_bundle(lyon::GeometryBuilder::build_as(
                &lyon::shapes::Circle {
                    radius: 25.0,
                    center: Vec2::ZERO,
                },
                lyon::DrawMode::Fill(lyon::FillMode::color(Color::WHITE)),
                *trans,
            ));
    }
}
