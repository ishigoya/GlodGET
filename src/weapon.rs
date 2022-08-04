use crate::{Body, BodyForce, Drawn, ForceProfile, GameState};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::time::Duration;

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Game)
                .with_system(weapon_movement.before("impact"))
                .with_system(torpedo_impact.before("explosion").label("impact"))
                .with_system(explosion_impact.before("despawn").label("explosion"))
                .with_system(despawn_explosions.label("despawn")),
        )
        .add_system_set(SystemSet::on_enter(GameState::Victory).with_system(despawn_weapons))
        .add_system_set(SystemSet::on_enter(GameState::GameOver).with_system(despawn_weapons));
    }
}

fn despawn_weapons(mut commands: Commands, weapons: Query<Entity, With<Weapon>>) {
    for weapon in weapons.iter() {
        commands.entity(weapon).despawn();
    }
}

#[derive(Component)]
pub struct Weapon;

#[derive(Component)]
pub struct WeaponPreLaunch;

#[derive(Component)]
pub struct WeaponLaunch;

const WEAPON_FORCE_PROFILE: ForceProfile = ForceProfile {
    forward: 15.0,
    torque: 0.0,
};
pub const WEAPON_MASS: f32 = 0.1;

#[derive(Component)]
pub struct Explodee;

#[derive(Component)]
pub struct Explosion {
    radius: f32,
    origin_impulse: f32,
}

impl Explosion {
    fn impulse_vector(&self, source: &Body, target: &Body) -> Vec2 {
        let vec = source.norm_vec_to(target);
        let effect = 1.0 / (1.0 + source.distance(target).sqrt());
        effect * self.origin_impulse * vec
    }
}

#[derive(Component)]
pub struct Torpedo {
    explosion_timer: Timer,
    active_timer: Timer,
}

impl Torpedo {
    pub fn new() -> Torpedo {
        Torpedo {
            explosion_timer: Timer::new(Duration::from_millis(2500), false),
            active_timer: Timer::new(Duration::from_millis(0), false),
        }
    }

    fn tick(&mut self, delta: Duration) {
        self.explosion_timer.tick(delta);
        self.active_timer.tick(delta);
    }

    fn can_explode(&self) -> bool {
        self.active_timer.finished()
    }

    fn explode(&self) -> Explosion {
        Explosion {
            radius: 50.0,
            origin_impulse: 3500.0,
        }
    }
}

fn torpedo_impact(
    mut commands: Commands,
    weapon: Query<(Entity, &Torpedo, &Transform), With<Torpedo>>,
    mut collision_events: EventReader<CollisionEvent>,
) {
    for collision_event in collision_events.iter() {
        for (entity, torp, trans) in weapon.iter() {
            if let CollisionEvent::Started(h1, h2, _event_flag) = collision_event {
                if torp.can_explode() && ((*h1 == entity) || (*h2 == entity)) {
                    spawn_explosion(&mut commands, entity, torp, trans);
                };
            };
        }
    }
}

pub fn spawn_explosion(commands: &mut Commands, entity: Entity, source: &Torpedo, loc: &Transform) {
    let trans = TransformBundle::from_transform(*loc);
    commands
        .spawn()
        .insert_bundle(trans)
        .insert(source.explode());
    commands.entity(entity).despawn();
}

fn weapon_movement(
    mut commands: Commands,
    mut weapons: Query<(Entity, &mut Torpedo, &Transform, &mut ExternalForce), With<Torpedo>>,
    time: Res<Time>,
) {
    for (entity, mut torp, trans, mut force) in weapons.iter_mut() {
        torp.tick(time.delta());
        if torp.explosion_timer.finished() {
            spawn_explosion(&mut commands, entity, &*torp, trans);
        } else {
            let bf = BodyForce::new(0.0, 1.0, WEAPON_FORCE_PROFILE);
            force.force = bf.force_from_transform(trans);
        };
    }
}

fn explosion_impact(
    explosions: Query<(&Transform, &Explosion), With<Explosion>>,
    mut query: Query<((&Transform, Option<&Velocity>), &mut ExternalImpulse), With<Explodee>>,
) {
    for (trans, explosion) in explosions.iter() {
        let source = Body::new((trans, None));
        for (pos, mut impulse) in query.iter_mut() {
            let target = &Body::new(pos);
            if source.distance(target) < explosion.radius {
                impulse.impulse = explosion.impulse_vector(&source, target);
            };
        }
    }
}

fn despawn_explosions(
    mut commands: Commands,
    explosions: Query<Entity, (With<Explosion>, With<Drawn>)>,
) {
    for entity in explosions.iter() {
        commands.entity(entity).despawn();
    }
}
