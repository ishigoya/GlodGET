use crate::{CollisionFilters, CollisionMemberships};
use crate::{EnemyBase, EnemyState, FoeStartingPoint, FriendStartingPoint, GameState, IsBase};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::geometry::Group;

pub const START_RADIUS: f32 = 50.0;

pub struct StartPointPlugin;

impl Plugin for StartPointPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(EnemyState::Released).with_system(remove_enemy_base),
        )
        .add_system_set(SystemSet::on_enter(GameState::Game).with_system(spawn_startpoints))
        .add_system_set(SystemSet::on_enter(GameState::Victory).with_system(despawn_startpoints))
        .add_system_set(SystemSet::on_enter(GameState::GameOver).with_system(despawn_startpoints));
    }
}

fn remove_enemy_base(base: Query<Entity, With<EnemyBase>>, mut commands: Commands) {
    let entity = base.single();
    commands.entity(entity).despawn();
}

fn despawn_startpoints(mut commands: Commands, base: Query<Entity, With<IsBase>>) {
    for ent in base.iter() {
        commands.entity(ent).despawn();
    }
}

fn spawn_startpoints(
    mut commands: Commands,
    friend_start: Res<FriendStartingPoint>,
    foe_start: Res<FoeStartingPoint>,
) {
    commands
        .spawn((
        Collider::ball(START_RADIUS),
        Sensor,
        CollisionGroups::new(
            Group::from_bits(CollisionMemberships::FriendlyBase as u32).unwrap(),
            Group::from_bits(CollisionFilters::WithFriend as u32).unwrap(),
        ),
        IsBase,
        TransformBundle::from_transform(
            Transform::from_translation(friend_start.0),
        )));

    commands
        .spawn((
        TransformBundle::from_transform(
            Transform::from_translation(foe_start.0),
        ),
        EnemyBase));
}
