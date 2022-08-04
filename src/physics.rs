use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::f32::consts::PI;
use std::ops::{Mul, Neg};
extern crate web_sys;

pub const PIXELS_PER_METER: f32 = 1.0;

#[repr(u32)]
pub enum CollisionMemberships {
    FriendlyBase = 0b1,
    Friend = 0b10,
    Enemy = 0b100,
    InertWeapon = 0b1000,
    KineticWeapon = 0b10000,
    Glod = 0b100000,
}

#[repr(u32)]
pub enum CollisionFilters {
    Friend = CollisionMemberships::FriendlyBase as u32
        | CollisionMemberships::Glod as u32
        | CollisionMemberships::Enemy as u32
        | CollisionMemberships::KineticWeapon as u32,
    Enemy = CollisionMemberships::Friend as u32 | CollisionMemberships::KineticWeapon as u32,
    InertWeapon = CollisionMemberships::KineticWeapon as u32,
    KineticWeapon = CollisionMemberships::Friend as u32
        | CollisionMemberships::InertWeapon as u32
        | CollisionMemberships::Enemy as u32,
    // FriendlyBase, Glod
    WithFriend = CollisionMemberships::Friend as u32,
}

pub enum TorqueDirection {
    Left,
    Right,
}

impl TorqueDirection {
    fn from<T>(x: T) -> TorqueDirection
    where
        T: PartialOrd + From<u8>,
    {
        if x < T::from(0) {
            TorqueDirection::Right
        } else {
            TorqueDirection::Left
        }
    }
}

impl<T> Mul<T> for TorqueDirection
where
    T: Mul<Output = T> + Neg<Output = T>,
{
    type Output = T;

    fn mul(self, rhs: T) -> T {
        match self {
            TorqueDirection::Left => rhs,
            TorqueDirection::Right => rhs.neg(),
        }
    }
}

pub struct Body {
    pos: Vec2,
    heading: f32,
    linvel: Vec2,
    angvel: f32,
}

impl Body {
    pub fn new((trans, vel): (&Transform, Option<&Velocity>)) -> Body {
        let passed_vel = match vel {
            Some(x) => x,
            None => &Velocity {
                linvel: Vec2::ZERO,
                angvel: 0.0,
            },
        };
        let (raxis, rot) = trans.rotation.to_axis_angle();

        Body {
            pos: trans.translation.truncate(),
            heading: f32::signum(raxis.z) * rot,
            linvel: passed_vel.linvel,
            angvel: passed_vel.angvel,
        }
    }

    pub fn angle_to(&self, other: &Body) -> f32 {
        let diff = (*other).pos - self.pos;
        diff.y.atan2(diff.x)
    }

    pub fn bearing_of(&self, other: &Body) -> (TorqueDirection, f32) {
        let mut angle_diff = self.angle_to(other) - self.heading;
        let mut dir = TorqueDirection::from(angle_diff);
        angle_diff = f32::abs(angle_diff);
        if angle_diff > PI {
            angle_diff = dir * (angle_diff - 2.0 * PI);
            dir = TorqueDirection::from(angle_diff);
            angle_diff = f32::abs(angle_diff);
        };
        (dir, angle_diff)
    }

    pub fn distance(&self, other: &Body) -> f32 {
        self.pos.distance((*other).pos)
    }

    pub fn norm_vec_to(&self, other: &Body) -> Vec2 {
        Vec2::from_angle(self.angle_to(other))
    }
}

pub struct ForceProfile {
    pub forward: f32,
    pub torque: f32,
}

pub struct BodyForce {
    pub torque: f32,
    pub forward: f32,
}

impl BodyForce {
    pub fn new(torque: f32, forward: f32, fp: ForceProfile) -> BodyForce {
        let _ = (-1.0..=1.0).contains(&torque) && (0.0..=1.0).contains(&forward) || panic!();

        BodyForce {
            torque: torque * fp.torque,
            forward: forward * fp.forward,
        }
    }

    pub fn force_from_transform(&self, trans: &Transform) -> Vec2 {
        let vec = trans.local_x();
        Vec2::new(self.forward * vec.x, self.forward * vec.y)
    }
}
