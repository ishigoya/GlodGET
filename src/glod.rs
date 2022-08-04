use crate::GlodPoints;
use crate::{CollisionFilters, CollisionMemberships};
use crate::{Explodee, GameState, IsGlod};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

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
            .spawn()
            .insert(RigidBody::Dynamic)
            .insert(Collider::ball(GLOD_RADIUS))
            .insert(CollisionGroups::new(
                CollisionMemberships::Glod as u32,
                CollisionFilters::WithFriend as u32,
            ))
            .insert(Damping {
                linear_damping: 2.0,
                angular_damping: 0.0,
            })
            .insert(Sensor)
            .insert(IsGlod)
            .insert(Explodee)
            .insert(ExternalForce {
                force: Vec2::ZERO,
                torque: 0.0,
            })
            .insert(ExternalImpulse {
                impulse: Vec2::ZERO,
                torque_impulse: 0.0,
            })
            .insert(ColliderMassProperties::Mass(GLOD_MASS))
            .insert(Transform::from_translation(*glod));
    }
}
