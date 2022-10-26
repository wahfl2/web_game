use bevy::{prelude::*, math::Vec3Swizzles, sprite::MaterialMesh2dBundle, input::mouse::MouseMotion, ecs::system::SystemParam};
use bevy_rapier2d::prelude::*;

use crate::{util::{Cursor, EntityQuery}, editor::components::EditorShape, game::player::components::*};

use super::raycast::handle_raycast;

pub const STEP_LENGTH: f32 = 200.0;
pub const MAX_WEB_LENGTH: f32 = 801.0;

pub struct WebPartEntities {
    pub entities: Vec<Entity>,
}

#[derive(SystemParam)]
pub struct PlayerControlsParam<'w, 's> {
    pub commands: Commands<'w, 's>,
 
    pub cursor: Res<'w, Cursor>,
    pub mouse: Res<'w, Input<MouseButton>>,
    pub failed_shot: ResMut<'w, FailedShot>,
    pub web_connection_entities: ResMut<'w, WebPartEntities>,
 
    pub mouse_motion_events: EventReader<'w, 's, MouseMotion>,
 
    pub rapier_context: Res<'w, RapierContext>,
    pub materials: ResMut<'w, Assets<ColorMaterial>>,
    pub meshes: ResMut<'w, Assets<Mesh>>,
}

#[derive(SystemParam)]
pub struct PlayerControlsQueries<'w, 's> {
    pub player: Query<'w, 's, (Entity, &'static mut Player)>,
    pub web_part_entities: EntityQuery<'w, 's, WebPart>,
    pub web_shot_entities: EntityQuery<'w, 's, WebShotVisual>,
    pub web_connections: EntityQuery<'w, 's, WebPartConnection>,

    pub transform: Query<'w, 's, &'static mut Transform>,
    pub visibility: Query<'w, 's, &'static mut Visibility>,
    pub shooting: Query<'w, 's, &'static mut ShootingWeb>,
    pub impulse_joint: Query<'w, 's, &'static mut ImpulseJoint>,
    pub editor_shape: Query<'w, 's, &'static EditorShape>,
    pub shape_with_joint: Query<'w, 's, Entity, (With<EditorShape>, With<ImpulseJoint>)>,
}

pub fn player_controls(
    mut p: PlayerControlsParam,
    mut query: PlayerControlsQueries,
) {
    if query.player.is_empty() { return }
    let (player_entity, player) = query.player.single();

    let arm_l_transform = query.transform.get(player.arm_l).unwrap();
    let hand_l_position = (arm_l_transform.rotation.mul_vec3(Vec3::X * 15.0) + arm_l_transform.translation).xy();

    if p.mouse.just_pressed(MouseButton::Left) {
        let ray_length = STEP_LENGTH;
        let ray_norm = (p.cursor.world_pos - hand_l_position).normalize();

        p.commands.entity(player_entity).insert(ShootingWeb {
            ray_norm,
            ray_length,
            max_length: MAX_WEB_LENGTH,
            steps: 0,
        });

        p.commands.spawn_bundle(MaterialMesh2dBundle {
            mesh: p.meshes.add(shape::Box::new(1.0, 1.0, 0.0).into()).into(),
            material: p.materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(hand_l_position.extend(0.0))
                .with_rotation(Quat::from_rotation_arc(Vec3::Y, ray_norm.extend(0.0)))
                .with_scale(Vec3::new(6.0, 6.0, 1.0)),
            
            ..default()
        }).insert(WebShotVisual);

    } else if p.mouse.pressed(MouseButton::Left) {
        if let Some(attached) = &player.attached {
            if !p.mouse_motion_events.is_empty() {
                let mut delta = Vec2::ZERO;
                p.mouse_motion_events.iter().for_each(|e| { 
                    delta.x -= e.delta.x; 
                    delta.y += e.delta.y; 
                });

                let move_dist = (hand_l_position - attached.hit_point).normalize().dot(-delta);
                let subtract = move_dist / (attached.num_segments * 8) as f32;
                let min_joint_length = attached.min_length / attached.num_segments as f32;

                query.web_part_entities.for_each(|e| {
                    let mut impulse_joint = query.impulse_joint.get_mut(e).unwrap();
                    let joint = impulse_joint.data.as_revolute_mut().unwrap();
                    
                    let anchor = joint.local_anchor2();
                    joint.set_local_anchor2((anchor.normalize() * (anchor.length() - subtract)).clamp_length_min(min_joint_length));
                });
            }
        } else {
            if **p.failed_shot {
                // Visually retract
                let web_shot_entity = query.web_shot_entities.iter().next().unwrap();
                let mut web_shot_transform = query.transform.get_mut(web_shot_entity).unwrap();

                let mut shooting = query.shooting.get_mut(player_entity).unwrap();
                if shooting.steps > 0 { shooting.steps -= 1; }

                let web_length = shooting.ray_length * shooting.steps as f32;
                let end_pos = hand_l_position + (shooting.ray_norm * web_length);
                web_shot_transform.translation = ((hand_l_position + end_pos) * 0.5).extend(0.0);
                web_shot_transform.scale.y = web_length;
            } else {
                // Try to attach
                let arm_l_transform = query.transform.get(player.arm_l).unwrap();
                let hand_l_position = (arm_l_transform.rotation.mul_vec3(Vec3::X * 15.0) + arm_l_transform.translation).xy();

                let mut shooting = query.shooting.get_mut(player_entity).unwrap();
                if (shooting.steps + 1) as f32 * shooting.ray_length <= shooting.max_length {
                    shooting.steps += 1;
                } else {
                    **p.failed_shot = true;
                }

                let raycast = p.rapier_context.cast_ray_and_get_normal(
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

                handle_raycast(
                    &mut p, 
                    &mut query, 
                    raycast,
                    hand_l_position,
                );
            }
        }
    }

    if p.mouse.just_released(MouseButton::Left) {
        let (_, mut player) = query.player.single_mut();

        **p.failed_shot = false;
        query.web_part_entities.for_each(|e| { p.commands.entity(e).despawn(); });
        query.web_shot_entities.for_each(|e| { p.commands.entity(e).despawn(); });
        query.web_connections.for_each(|e| { p.commands.entity(e).despawn(); });
        query.shape_with_joint.for_each(|e| { p.commands.entity(e).remove::<ImpulseJoint>(); });

        p.web_connection_entities.entities.clear();

        player.attached = None;
        p.commands.entity(player_entity).remove::<ShootingWeb>();
    }
}