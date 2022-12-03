use crate::GlodPoints;
use crate::{CollisionFilters, CollisionMemberships};
use crate::{Explodee, GameState, IsGlod};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::geometry::Group;

pub struct GlodPlugin;

impl Plugin for GlodPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Game).with_system(spawn_glod))
            .add_system_set(SystemSet::on_enter(GameState::Victory).with_system(despawn_glod))
            .add_system_set(SystemSet::on_enter(GameState::GameOver).with_system(despawn_glod));
    }
}

pub const GLOD_RADIUS: f32 = 2.0;
pub const GLOD_QUANTITY: usize = 50;
pub const GLOD_MASS: f32 = 2.0;

fn despawn_glod(mut commands: Commands, glod: Query<Entity, With<IsGlod>>) {
    for ent in glod.iter() {
        commands.entity(ent).despawn();
    }
}

fn spawn_glod(mut commands: Commands, glods: Res<GlodPoints>) {
    for glod in (*glods).glods.iter() {
        commands
            .spawn((
            RigidBody::Dynamic,
            Collider::ball(GLOD_RADIUS),
            CollisionGroups::new(
                Group::from_bits(CollisionMemberships::Glod as u32).unwrap(),
                Group::from_bits(CollisionFilters::WithFriend as u32).unwrap(),
            ),
            Damping {
                linear_damping: 2.0,
                angular_damping: 0.0,
            },
            Sensor, 
            IsGlod,
            Explodee,
            ExternalForce {
                force: Vec2::ZERO,
                torque: 0.0,
            },
            ExternalImpulse {
                impulse: Vec2::ZERO,
                torque_impulse: 0.0,
            },
            ColliderMassProperties::Mass(GLOD_MASS),
            Transform::from_translation(*glod)));
    }
}
