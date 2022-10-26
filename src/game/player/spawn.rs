use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::components::*;

#[derive(Deref, DerefMut)]
pub struct Respawn(pub bool);

pub fn player_spawn(
    mut commands: Commands,

    kill_query: Query<Entity, Or<(With<Player>, With<WebPart>, With<WebShotVisual>)>>,

    asset_server: Res<AssetServer>,
    mut respawn: ResMut<Respawn>,
) {
    if !**respawn { return }
    **respawn = false;

    for entity in kill_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    let group = CollisionGroups::new(Group::from_bits_truncate(0b10), Group::ALL);

    let body = commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("body_capsule.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(40.0, 80.0)),
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, -10.0),
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
        transform: Transform::from_xyz(45.0, 20.0, 0.0),
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
        transform: Transform::from_xyz(-45.0, 20.0, 0.0),
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

    let joint = RevoluteJointBuilder::new()
        .local_anchor1(Vec2::new(-8.75, 21.25))
        .local_anchor2(Vec2::new(-3.125, 0.0));

    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("eye.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(11.0, 11.0)),
            ..default()
        },
        transform: Transform::from_xyz(-5.625, 21.25, -0.1),
        ..default()
    }).insert_bundle((
        Collider::ball(5.625),
        RigidBody::Dynamic,
        Damping { angular_damping: 2.0, linear_damping: 0.0 },
        ImpulseJoint::new(body, joint),
        CollisionGroups::new(Group::NONE, Group::NONE),
    ));

    let joint = RevoluteJointBuilder::new()
        .local_anchor1(Vec2::new(8.75, 21.25))
        .local_anchor2(Vec2::new(3.125, 0.0));

    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("eye.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(11.25, 11.25)),
            ..default()
        },
        transform: Transform::from_xyz(5.625, 21.25, -0.1),
        ..default()
    }).insert_bundle((
        Collider::ball(5.625),
        RigidBody::Dynamic,
        Damping { angular_damping: 2.0, linear_damping: 0.0 },
        ImpulseJoint::new(body, joint),
        CollisionGroups::new(Group::NONE, Group::NONE),
    ));

    commands.spawn().insert(Player { body, arm_r, arm_l, attached: None });
}