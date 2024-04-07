use crate::CollisionEntity;
use crate::Intersection;
use bevy::prelude::*;
use nalgebra::Isometry3;
use parry3d::bounding_volume::BoundingVolume;
use parry3d::query::contact;
use parry3d::shape::Shape;

const STEP_SIZE: f32 = 0.01;

/// Sweeps the collider against the provided colliders and returns the closest intersection.
///
/// # Arguments
/// * `collider` - The collider we are sweeping with.
/// * `transform` - The current location of the collider we are sweeping with.
/// * `movement` - The movement vector for the sweep, typically velocity multiplied by `delta_time`.
/// * `colliders` - A reference to a vector containing tuples of entity, collider, and transform for each collider to sweep against.
///
/// # Returns
/// An `Option<Intersection>` which is `Some(Intersection)` if an intersection occurs, or `None` if there is no intersection.
pub fn sweep_intersection_against(
    collider_shape: &dyn Shape,
    mut translation: Vec3,
    movement: Vec3,
    colliders: &Vec<CollisionEntity>,
) -> Option<Intersection> {
    let direction = movement.normalize();
    let total_movement_length = movement.length();
    let mut total_movement_distance = 0.0;

    let end_translation = translation + movement;

    // Computes the AABB which represents the swept shape of the collider
    let moving_collider_swept_aabb = collider_shape.compute_swept_aabb(
        &crate::bevy_transform_to_isometry(Transform::from_translation(translation)),
        &crate::bevy_transform_to_isometry(Transform::from_translation(end_translation)),
    );

    // Perform broad-phase collision check to find potentially colliding entities
    let potentially_colliding_entities: Vec<&CollisionEntity> =
        colliders
            .iter()
            .filter(|collision_entity| {
                let other_collider_aabb = collision_entity.collider.shape.compute_aabb(
                    &crate::bevy_transform_to_isometry(*collision_entity.transform),
                );
                moving_collider_swept_aabb.intersects(&other_collider_aabb)
            })
            .collect();

    // Perform step-wise movement and detailed collision checks
    let mut closest_intersection: Option<Intersection> = None;

    while total_movement_distance < total_movement_length {
        let step = (total_movement_length - total_movement_distance).min(STEP_SIZE);
        translation += direction * step;
        total_movement_distance += step;

        for other_collision_entity in potentially_colliding_entities.iter() {
            if let Some(intersection) = perform_intersection(
                collider_shape,
                crate::bevy_transform_to_isometry(Transform::from_translation(translation)),
                other_collision_entity.entity,
                other_collision_entity.collider.shape.as_ref(),
                crate::bevy_transform_to_isometry(*other_collision_entity.transform),
            ) {
                let is_closer = closest_intersection
                    .as_ref()
                    .map_or(true, |closest| intersection.distance < closest.distance);
                if is_closer {
                    closest_intersection = Some(intersection);
                }
            }
        }

        if closest_intersection.is_some() {
            break;
        }
    }

    // Calculate TOI for the closest intersection found, if any
    if let Some(mut intersection) = closest_intersection {
        intersection.toi = total_movement_distance / total_movement_length;
        return Some(intersection);
    }

    None
}

fn perform_intersection(
    collider_shape: &dyn Shape,
    transform: Isometry3<f32>,
    other_entity: Entity,
    other_collider_shape: &dyn Shape,
    other_transform: Isometry3<f32>,
) -> Option<Intersection> {
    match contact(
        &transform,
        collider_shape,
        &other_transform,
        other_collider_shape,
        0.00,
    ) {
        Ok(Some(contact)) => Some(Intersection {
            entity: other_entity,
            our_normal: Vec3::new(contact.normal1.x, contact.normal1.y, contact.normal1.z),
            their_normal: Vec3::new(contact.normal2.x, contact.normal2.y, contact.normal2.z),
            distance: contact.dist,
            our_contact_point: Vec3::new(
                contact.point1.coords.x,
                contact.point1.coords.y,
                contact.point1.coords.z,
            ),
            their_contact_point: Vec3::new(
                contact.point2.coords.x,
                contact.point2.coords.y,
                contact.point2.coords.z,
            ),
            toi: 0.0, // get's filled in
        }),
        Ok(None) => None,
        Err(e) => {
            error!("Error: {}", e);
            None
        }
    }
}
