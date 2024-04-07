use bevy::prelude::*;
use nalgebra::{Isometry3, Point3, Vector3};
pub mod stats;
pub mod sweep;

#[derive(Debug)]
pub struct Intersection {
    pub entity: Entity,
    pub our_normal: Vec3,
    pub their_normal: Vec3,
    pub our_contact_point: Vec3,
    pub their_contact_point: Vec3,
    pub distance: f32,
    pub toi: f32,
}

#[derive(Clone)]
pub struct CollisionEntity<'a> {
    pub entity: Entity,
    pub collider: &'a Collider,
    pub transform: &'a Transform,
}

#[derive(Component)]
pub struct Collider {
    pub shape: Box<dyn parry3d::shape::Shape>,
    pub offset: Vec3,
}

impl Collider {
    pub fn cuboid(half_size: Vec3) -> Self {
        Collider {
            shape: Box::new(parry3d::shape::Cuboid::new(bevy_vec3_to_vector3(half_size))),
            offset: Vec3::ZERO,
        }
    }

    pub fn sphere(radius: f32) -> Self {
        Collider {
            shape: Box::new(parry3d::shape::Ball::new(radius)),
            offset: Vec3::ZERO,
        }
    }

    pub fn cylinder(radius: f32, half_height: f32) -> Self {
        Collider {
            shape: Box::new(parry3d::shape::Cylinder::new(half_height, radius)),
            offset: Vec3::ZERO,
        }
    }

    pub fn convex_hull(vertices: Vec<Vec3>) -> Option<Self> {
        let points: Vec<Point3<f32>> = vertices
            .iter()
            .map(|v| Point3::new(v.x, v.y, v.z).into())
            .collect();
        if let Some(convex) = parry3d::shape::ConvexPolyhedron::from_convex_hull(&points) {
            Some(Collider {
                shape: Box::new(convex),
                offset: Vec3::ZERO,
            })
        } else {
            None
        }
    }

    pub fn with_offset(mut self, offset: Vec3) -> Self {
        self.offset = offset;
        self
    }
}

pub(crate) fn bevy_transform_to_isometry(transform: Transform) -> Isometry3<f32> {
    let mut isometry = Isometry3::identity();
    isometry.translation = bevy_vec3_to_translation(transform.translation);
    isometry.rotation = bevy_quat_to_unit_quat(transform.rotation);
    isometry
}

pub(crate) fn bevy_vec3_to_translation(v: Vec3) -> nalgebra::Translation3<f32> {
    nalgebra::Translation3::new(v.x, v.y, v.z)
}

pub(crate) fn bevy_quat_to_unit_quat(quat: Quat) -> nalgebra::UnitQuaternion<f32> {
    nalgebra::UnitQuaternion::new_normalize(nalgebra::Quaternion::new(
        quat.x, quat.y, quat.z, quat.w,
    ))
}

pub(crate) fn bevy_vec3_to_vector3(v: Vec3) -> Vector3<f32> {
    Vector3::new(v.x, v.y, v.z)
}
