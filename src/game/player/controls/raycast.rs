use bevy::{prelude::*, math::Vec3Swizzles, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

use crate::game::player::components::*;

use super::controls::{PlayerControlsParam, PlayerControlsQueries};

pub fn handle_raycast(
    p: &mut PlayerControlsParam,
    query: &mut PlayerControlsQueries,
    raycast: Option<(Entity, RayIntersection)>,
    hand_l_position: Vec2,
) {
    let (player_entity, mut player) = query.player.single_mut();
    let shooting = query.shooting.get(player_entity).unwrap();

    if let Some((hit_entity, intersection)) = raycast {
        if !query.editor_shape.get(hit_entity).unwrap().stickable {
            **p.failed_shot = true;
        } else {
            **p.failed_shot = false;
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
    
            let hit_entity_transform = query.transform.get(hit_entity).unwrap();
            let local_raw_stick_point = (intersection.point - hit_entity_transform.translation.xy()) + (intersection.normal * (ball_diameter * 0.5));
            let local_stick_point = hit_entity_transform.rotation.inverse().mul_vec3(local_raw_stick_point.extend(0.0)).xy();
    
            let middle_joint = RevoluteJointBuilder::new()
                .local_anchor1(Vec2::ZERO)
                .local_anchor2(ball_t * -shooting.ray_norm);
    
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
    
                prev_entity = Some(p.commands.spawn_bundle(MaterialMesh2dBundle {
                    transform: Transform::from_translation(pos.extend(0.0)),
                    mesh: p.meshes.add(shape::Circle::new(ball_diameter * 0.5).into()).into(),
                    material: p.materials.add(ColorMaterial::from(Color::WHITE)),
                    ..default()
                })
                    .insert_bundle(bundle.clone())
                    .insert_bundle((
                        RigidBody::Dynamic,
                        joint, 
                        WebPart
                    )).id());
            }
    
            let hit_joint = ImpulseJoint::new(
                prev_entity.unwrap(), 
                RevoluteJointBuilder::new()
                    .local_anchor1(Vec2::ZERO)
                    .local_anchor2(local_stick_point)
            );
    
            p.commands.entity(hit_entity).insert(hit_joint);
            query.web_shot_entities.for_each(|e| { p.commands.entity(e).despawn(); });
    
            player.attached = Some(Attached {
                hit_point: intersection.point,
                start_cursor_pos: p.cursor.pos,
                num_segments: num_balls,
                min_length: (intersection.toi - 150.0).max(40.0),
            });
        }
    } else {
        // Extend line that represents web
        let web_shot_entity = query.web_shot_entities.iter().next().unwrap();
        let mut web_shot_transform = query.transform.get_mut(web_shot_entity).unwrap();

        let web_length = shooting.ray_length * shooting.steps as f32;
        let end_pos = hand_l_position + (shooting.ray_norm * web_length);
        web_shot_transform.translation = ((hand_l_position + end_pos) * 0.5).extend(0.0);
        web_shot_transform.scale.y = web_length;
    }
}