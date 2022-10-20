use std::f32::consts::PI;

use bevy::{prelude::*, sprite::Mesh2dHandle};
use bevy_rapier2d::prelude::*;

use crate::util::DEGREES;

use super::controls::{STEP_LENGTH, MAX_WEB_LENGTH};

#[derive(Component)]
pub struct Player {
    pub body: Entity,
    pub arm_r: Entity,
    pub arm_l: Entity,
    pub attached: Option<Attached>,
}

#[derive(Component)]
pub struct Attached {
    pub hit_point: Vec2,
    pub start_cursor_pos: Vec2,
    pub num_segments: u32,
}

pub fn player_spawn(
    mut commands: Commands,
) {
    // Player scale, only necessary to create joints properly
    let p = 20.0;

    let group = CollisionGroups::new(Group::from_bits_truncate(0b10), Group::ALL);

    let body = commands.spawn_bundle(TransformBundle::default())
        .insert_bundle((
            Collider::capsule_y(1.0, 1.0),
            RigidBody::Dynamic,
            Friction::coefficient(0.1),
            Restitution::coefficient(0.7),
            group,
        )).id();

    let joint = RevoluteJointBuilder::new()
        .local_anchor1(Vec2::new(1.25 * p, p))
        .local_anchor2(Vec2::new(-p, 0.0));

    let arm_l = commands.spawn_bundle(TransformBundle::default())
        .insert_bundle((
            Collider::capsule_x(0.75, 0.25),
            RigidBody::Dynamic,
            Damping { angular_damping: 5.0, linear_damping: 0.0 },
            Friction::coefficient(0.5),
            Restitution::coefficient(0.4),
            ImpulseJoint::new(body, joint),
            group,
        )).id();

    let joint = RevoluteJointBuilder::new()
        .local_anchor1(Vec2::new(-1.25 * p, p))
        .local_anchor2(Vec2::new(p, 0.0));

    let arm_r = commands.spawn_bundle(TransformBundle::default())
        .insert_bundle((
            Collider::capsule_x(0.75, 0.25),
            RigidBody::Dynamic,
            Damping { angular_damping: 5.0, linear_damping: 0.0 },
            Friction::coefficient(0.5),
            Restitution::coefficient(0.4),
            ImpulseJoint::new(body, joint),
            group
        )).id();

    commands.spawn_bundle(TransformBundle::from_transform(Transform::from_scale(Vec3::splat(p))))
        .insert(Player { body, arm_r, arm_l, attached: None })
        .insert_children(0, &[body, arm_r, arm_l]);
}

pub struct WebMeshes {
    pub handles: Vec<Mesh2dHandle>,
}

pub fn preprocess_webs(
    mut commands: Commands,
    
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let mut handles = Vec::new();
    let num_meshes = (MAX_WEB_LENGTH / STEP_LENGTH).ceil() as u32 + 1;

    let mut capsule = shape::Capsule::default();
    capsule.radius = 10.0;
    capsule.latitudes = 4;
    capsule.longitudes = 8;

    for i in 0..num_meshes {
        capsule.depth = i as f32 * STEP_LENGTH;
        handles.push(meshes.add(capsule.clone().into()).into());
    }

    commands.insert_resource(WebMeshes { handles });
}