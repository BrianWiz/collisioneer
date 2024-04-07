use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, window::PresentMode};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use collisioneer::{stats::FpsCounterPlugin, sweep, Collider, CollisionEntity};

const CHARACTER_WIDTH: f32 = 1.0; // chonker
const CHARACTER_HEIGHT: f32 = 1.7;
const CHARACTER_OVERCLIP: f32 = 0.001;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::Immediate,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(FpsCounterPlugin)
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, move_character)
        .run();
}

fn setup(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
) {
    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 5.0)),
            ..Default::default()
        },
        PanOrbitCamera::default(),
    ));

    // light
    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
        point_light: PointLight {
            shadows_enabled: true,
            ..Default::default()
        },
        ..Default::default()
    });

    // character
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cylinder {
                radius: CHARACTER_WIDTH * 0.5,
                half_height: CHARACTER_HEIGHT * 0.5,
            }),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.8, 0.7, 0.6),
                ..default()
            }),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 4.0)),
            ..Default::default()
        },
        Character {
            velocity: Vec3::new(0.0, 0.0, 0.0),
            move_speed: 12.0,
            move_accel: 10.0,
            move_friction: 10.0,
            grounded: false,
        },
        Collider::cylinder(CHARACTER_WIDTH * 0.5, CHARACTER_HEIGHT * 0.5),
    ));

    // floor
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid {
                half_size: Vec3::new(10.0, 0.1, 10.0),
                ..Default::default()
            }),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.5, 0.5, 0.5),
                ..default()
            }),
            transform: Transform::from_translation(Vec3::new(0.0, -1.2, 0.0)),
            ..Default::default()
        },
        Collider::cuboid(Vec3::new(10.0, 0.1, 10.0)),
    ));

    // small step
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid {
                half_size: Vec3::new(1.0, 0.1, 1.0),
                ..Default::default()
            }),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.5, 0.5, 0.5),
                ..default()
            }),
            transform: Transform::from_translation(Vec3::new(0.0, -1.0, 2.0)),
            ..Default::default()
        },
        Collider::cuboid(Vec3::new(1.0, 0.1, 1.0)),
    ));

    // pillar
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cylinder {
                radius: 1.0,
                half_height: 5.0,
                ..default()
            }),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.5, 0.5, 0.5),
                ..default()
            }),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..default()
        },
        Collider::cylinder(1.0, 5.0),
    ));

    // wall
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid {
                half_size: Vec3::new(1.0, 10.0, 10.0),
                ..default()
            }),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.5, 0.5, 0.5),
                ..default()
            }),
            transform: Transform::from_translation(Vec3::new(-10.0, 0.0, 0.0)),
            ..default()
        },
        Collider::cuboid(Vec3::new(1.0, 10.0, 10.0)),
    ));

    // wall
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid {
                half_size: Vec3::new(1.0, 10.0, 10.0),
                ..default()
            }),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.5, 0.5, 0.5),
                ..default()
            }),
            transform: Transform::from_translation(Vec3::new(10.0, 0.0, 0.0)),
            ..default()
        },
        Collider::cuboid(Vec3::new(1.0, 10.0, 10.0)),
    ));

    // wall
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid {
                half_size: Vec3::new(10.0, 10.0, 1.0),
                ..default()
            }),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.5, 0.5, 0.5),
                ..default()
            }),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, -10.0)),
            ..default()
        },
        Collider::cuboid(Vec3::new(10.0, 10.0, 1.0)),
    ));

    // wall
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid {
                half_size: Vec3::new(10.0, 10.0, 1.0),
                ..default()
            }),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.5, 0.5, 0.5),
                ..default()
            }),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
            ..default()
        },
        Collider::cuboid(Vec3::new(10.0, 10.0, 1.0)),
    ));

    // place a bunch of colliders to test performance
    for _ in 0..15000 {
        commands.spawn((
            TransformBundle {
                local: Transform::from_translation(Vec3::new(100.0, 100.0, 100.0)),
                ..Default::default()
            },
            Collider::cuboid(Vec3::new(0.5, 0.5, 0.5)),
        ));
    }
}

fn move_character(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut object_b: Query<(Entity, &mut Character, &Collider, &mut Transform)>,
    colliders: Query<(Entity, &Collider, &Transform), Without<Character>>,
) {
    let colliders = colliders
        .iter()
        .map(|(entity, collider, transform)| CollisionEntity {
            entity: entity,
            collider,
            transform,
        })
        .collect::<Vec<CollisionEntity>>();

    if let Ok((_, mut character, collider, mut transform)) = object_b.get_single_mut() {
        let character_input = CharacterInput {
            forward: keyboard_input.pressed(KeyCode::KeyW),
            backward: keyboard_input.pressed(KeyCode::KeyS),
            left: keyboard_input.pressed(KeyCode::KeyA),
            right: keyboard_input.pressed(KeyCode::KeyD),
            jump: keyboard_input.just_pressed(KeyCode::Space),
        };

        character.perform_movement(
            &time,
            &collider,
            &mut transform,
            &colliders,
            &character_input,
        );
    }
}

struct CharacterInput {
    forward: bool,
    backward: bool,
    left: bool,
    right: bool,
    jump: bool,
}

impl CharacterInput {
    pub fn compute_wish_dir(&self) -> Vec3 {
        let mut wish_dir = Vec3::ZERO;
        if self.forward {
            wish_dir.z -= 1.0;
        }
        if self.backward {
            wish_dir.z += 1.0;
        }
        if self.left {
            wish_dir.x -= 1.0;
        }
        if self.right {
            wish_dir.x += 1.0;
        }

        if wish_dir.length() > 0.0 {
            wish_dir = wish_dir.normalize();
        }
        wish_dir
    }
}

#[derive(Component)]
struct Character {
    velocity: Vec3,
    move_speed: f32,
    move_accel: f32,
    move_friction: f32,
    grounded: bool,
}

impl Character {
    pub fn perform_movement(
        &mut self,
        time: &Time,
        collider: &Collider,
        transform: &mut Transform,
        colliders: &Vec<CollisionEntity>,
        character_input: &CharacterInput,
    ) {
        let wish_dir = character_input.compute_wish_dir();
        self.apply_acceleration(wish_dir, time.delta_seconds());
        self.detect_ground(transform, time.delta_seconds(), colliders);
        if self.grounded {
            self.apply_friction(time.delta_seconds());

            if character_input.jump {
                self.velocity.y = 5.0;
                self.grounded = false;
            }
        } else {
            self.apply_gravity(time.delta_seconds());
        }
        self.move_and_slide(time, collider, transform, colliders);
    }

    fn apply_gravity(&mut self, delta_seconds: f32) {
        self.velocity += Vec3::Y * -9.81 * delta_seconds;
    }

    fn apply_acceleration(&mut self, wish_dir: Vec3, delta_seconds: f32) {
        let add_speed = self.move_speed - self.velocity.length();

        if add_speed <= 0.0 {
            return;
        }

        let mut accel_speed = self.move_accel * delta_seconds * self.move_speed;
        if accel_speed > add_speed {
            accel_speed = add_speed;
        }

        self.velocity += wish_dir * accel_speed;
    }

    fn apply_friction(&mut self, delta_seconds: f32) {
        let mut new_speed;
        let mut drop = 0.0;

        drop += self.velocity.length() * self.move_friction * delta_seconds;

        new_speed = self.velocity.length() - drop;
        if new_speed < 0.0 {
            new_speed = 0.0;
        }

        if new_speed != 0.0 {
            new_speed /= self.velocity.length();
        }

        self.velocity = self.velocity * new_speed;
    }

    fn detect_ground(
        &mut self,
        transform: &mut Transform,
        delta_seconds: f32,
        colliders: &Vec<CollisionEntity>,
    ) {
        let step_up_height = CHARACTER_HEIGHT * 0.5;
        let anticipated_position = transform.translation + self.velocity * delta_seconds;
        let detect_ground_at = anticipated_position;

        if let Some(intersection) = sweep::sweep_intersection_against(
            &parry3d::shape::Cylinder::new(0.01, CHARACTER_WIDTH * 0.5), // The shape used for detection
            detect_ground_at,
            Vec3::Y * -step_up_height,
            colliders,
        ) {
            // Check if the ground normal indicates a mostly upward-facing surface
            if intersection.their_normal.dot(Vec3::Y) > 0.7 {
                self.grounded = true;
                // If grounded, adjust the character's position to snap to the ground, considering step_up_height
                transform.translation.y =
                    intersection.their_contact_point.y + CHARACTER_HEIGHT * 0.5 + 0.01;
                self.velocity.y = 0.0; // Reset vertical velocity
            }
        } else {
            self.grounded = false;
        }
    }

    fn move_and_slide(
        &mut self,
        time: &Time,
        collider: &Collider,
        transform: &mut Transform,
        colliders: &Vec<CollisionEntity>,
    ) {
        let mut dt = time.delta_seconds();
        for _ in 0..4 {
            if let Some(intersection) = sweep::sweep_intersection_against(
                collider.shape.as_ref(),
                transform.translation,
                self.velocity * dt,
                &colliders,
            ) {
                // slide the velocity along the normal of the collider we hit
                self.velocity -=
                    self.velocity.dot(intersection.their_normal) * intersection.their_normal;

                // move the character to the point of intersection
                transform.translation += self.velocity.normalize() * intersection.distance.max(0.0);

                // nudge it back to avoid sticking to walls
                transform.translation += intersection.their_normal * CHARACTER_OVERCLIP;

                // dialate the time to avoid moving fast when sliding
                dt *= 1.0 - intersection.toi;
            } else {
                transform.translation += self.velocity * dt;
                break;
            }
        }
    }
}
