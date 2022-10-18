use bevy::{prelude::*, math::Vec3Swizzles};
use bevy_rapier2d::prelude::*;

use crate::{util::{Cursor, EntityQuery}, editor::components::EditorShape, game::player::spawn::Attached};

use super::spawn::Player;

#[derive(Component)]
pub struct ShootingWeb {
    ray_norm: Vec2,
    ray_length: f32,
    max_length: f32,
    pub steps: u32,
}

#[derive(Component)]
pub struct WebPart;

pub fn player_controls(
    mut commands: Commands,

    keyboard: Res<Input<KeyCode>>,
    cursor: Res<Cursor>,
    mouse: Res<Input<MouseButton>>,

    rapier_context: Res<RapierContext>,

    mut player_q: Query<(Entity, &mut Player)>,
    web_part_q: EntityQuery<WebPart>,

    transform_query: Query<&Transform>,
    mut shooting_query: Query<&mut ShootingWeb>,
    mut impulse_joint_query: Query<&mut ImpulseJoint>,
    shape_with_joint: Query<Entity, (With<EditorShape>, With<ImpulseJoint>)>,
) {
    let (player_entity, mut player) = player_q.single_mut();

    let arm_l_transform = transform_query.get(player.arm_l).unwrap();
    let hand_l_position = (arm_l_transform.rotation.mul_vec3(Vec3::X * 15.0) + arm_l_transform.translation).xy();

    if mouse.just_pressed(MouseButton::Left) {
        let ray_length = 100.0;
        let ray_norm = (cursor.world_pos - hand_l_position).normalize();

        commands.entity(player_entity).insert(ShootingWeb {
            ray_norm,
            ray_length,
            max_length: 900.0,
            steps: 0,
        });
    } else if mouse.pressed(MouseButton::Left) {
        if let Some(attached) = &player.attached {
            let move_dist = (attached.hit_point - cursor.world_pos).normalize().dot(cursor.delta);
            let subtract = move_dist / (attached.num_segments * 8) as f32;

            web_part_q.for_each(|e| {
                let mut impulse_joint = impulse_joint_query.get_mut(e).unwrap();
                let mut joint = impulse_joint.data.as_revolute_mut().unwrap();
                
                let anchor = joint.local_anchor2();
                joint.set_local_anchor2(anchor.clamp_length(0.0, anchor.length() - subtract));
            });
        } else {
            // Try to attach
            let arm_l_transform = transform_query.get(player.arm_l).unwrap();
            let hand_l_position = (arm_l_transform.rotation.mul_vec3(Vec3::X * 15.0) + arm_l_transform.translation).xy();

            let mut shooting = shooting_query.get_mut(player_entity).unwrap();
            shooting.steps += 1;

            let raycast = rapier_context.cast_ray_and_get_normal(
                hand_l_position, 
                shooting.ray_norm, 
                shooting.ray_length * shooting.steps as f32, 
                true, 
                QueryFilter::default().groups({
                    // ripperoni
                    use bevy_rapier2d::rapier::prelude::Group;

                    InteractionGroups::new(
                        Group::ALL,
                        Group::from_bits_truncate(0b1)
                )}),
            );

            if let Some((hit_entity, intersection)) = raycast {
                let ball_diameter = 8.0;
                let length = (intersection.point - hand_l_position).length();
                let num_balls = (length / ball_diameter).ceil() as u32;
                let ball_t = length / num_balls as f32;

                let bundle = (
                    Collider::ball(ball_diameter * 0.5),
                    ColliderMassProperties::Density(0.1),
                    Damping { linear_damping: 5.0, angular_damping: 5.0 },
                    CollisionGroups::new(
                        Group::from_bits_truncate(0b10),
                        Group::from_bits_truncate(0b11111101),
                    )
                );

                let first_joint = ImpulseJoint::new(
                    player.arm_l, 
                    RevoluteJointBuilder::new()
                        .local_anchor1(Vec2::new(15.0, 0.0))
                        .local_anchor2(Vec2::new(-ball_diameter, 0.0))
                );

                let hit_entity_transform = transform_query.get(hit_entity).unwrap();
                let local_stick_point = (intersection.point - hit_entity_transform.translation.xy()) + (intersection.normal * (ball_diameter * 0.5));

                let middle_joint = RevoluteJointBuilder::new()
                    .local_anchor1(Vec2::ZERO)
                    .local_anchor2(ball_t * -shooting.ray_norm);

                let mut prev_pos = Vec2::ZERO;
                let mut prev_entity = None;

                for i in 1..num_balls {
                    let t = ball_t * i as f32;
                    let pos = hand_l_position + (shooting.ray_norm * t);
                    let joint;

                    if i == 1 {
                        joint = first_joint;
                    } else {
                        joint = ImpulseJoint::new(
                            prev_entity.unwrap(),
                            middle_joint
                        );
                    }

                    prev_entity = Some(commands.spawn_bundle(TransformBundle::from_transform(Transform::from_translation(pos.extend(0.0))))
                        .insert_bundle(bundle.clone())
                        .insert_bundle((
                            RigidBody::Dynamic,
                            joint, 
                            WebPart
                        )).id());

                    prev_pos = pos;
                }

                info!("{:?}", intersection);

                let hit_joint = ImpulseJoint::new(
                    prev_entity.unwrap(), 
                    RevoluteJointBuilder::new()
                        .local_anchor1(Vec2::ZERO)
                        .local_anchor2(local_stick_point)
                );

                commands.entity(hit_entity).insert(hit_joint);

                player.attached = Some(Attached {
                    hit_point: intersection.point,
                    start_cursor_pos: cursor.pos,
                    num_segments: num_balls,
                });
            }
        }
    }

    if mouse.just_released(MouseButton::Left) {
        for entity in web_part_q.iter() {
            commands.entity(entity).despawn();
        }

        player.attached = None;
        commands.entity(player_entity).remove::<ShootingWeb>();
        shape_with_joint.for_each(|entity| { commands.entity(entity).remove::<ImpulseJoint>(); });
    }
}