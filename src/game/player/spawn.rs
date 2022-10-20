use bevy::{prelude::*, sprite::Mesh2dHandle};
use bevy_rapier2d::prelude::*;

use super::{controls::{STEP_LENGTH, MAX_WEB_LENGTH}, components::*};

pub fn player_spawn(
    mut commands: Commands,

    asset_server: Res<AssetServer>,
) {
    let group = CollisionGroups::new(Group::from_bits_truncate(0b10), Group::ALL);

    let body = commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("black_capsule.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(40.0, 80.0)),
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    }).insert_bundle((
        Collider::capsule_y(20.0, 20.0),
        RigidBody::Dynamic,
        Friction::coefficient(0.1),
        Restitution::coefficient(0.7),
        group,
    )).id();

    let joint = RevoluteJointBuilder::new()
        .local_anchor1(Vec2::new(25.0, 20.0))
        .local_anchor2(Vec2::new(-20.0, 0.0));

    let arm_l = commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("arm_capsule.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(40.0, 10.0)),
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    }).insert_bundle((
        Collider::capsule_x(15.0, 5.0),
        RigidBody::Dynamic,
        Damping { angular_damping: 5.0, linear_damping: 0.0 },
        Friction::coefficient(0.5),
        Restitution::coefficient(0.4),
        ImpulseJoint::new(body, joint),
        group,
    )).id();

    let joint = RevoluteJointBuilder::new()
        .local_anchor1(Vec2::new(-25.0, 20.0))
        .local_anchor2(Vec2::new(20.0, 0.0));

    let arm_r = commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("arm_capsule.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(40.0, 10.0)),
            flip_x: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    }).insert_bundle((
        Collider::capsule_x(15.0, 5.0),
        RigidBody::Dynamic,
        Damping { angular_damping: 5.0, linear_damping: 0.0 },
        Friction::coefficient(0.5),
        Restitution::coefficient(0.4),
        ImpulseJoint::new(body, joint),
        group
    )).id();

    commands.spawn_bundle(TransformBundle::default())
        .insert_bundle(VisibilityBundle::default())
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