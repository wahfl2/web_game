use bevy::{prelude::*, math::Vec3Swizzles};

use crate::util::EntityQuery;

use super::spawn::Player;

pub fn player_camera(
    camera_q: EntityQuery<Camera>,
    player_q: Query<&Player>,

    mut transform_query: Query<&mut Transform>,
) {
    let body_pos = transform_query.get(player_q.single().body).unwrap().translation.xy();
    let cam_pos = transform_query.get(camera_q.single()).unwrap().translation.xy();
    let new_cam_pos = body_pos.lerp(cam_pos, 0.9);

    let mut cam_transform = transform_query.get_mut(camera_q.single()).unwrap();
    cam_transform.translation = new_cam_pos.extend(0.0);
}