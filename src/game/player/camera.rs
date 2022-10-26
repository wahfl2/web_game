use bevy::{prelude::*, math::Vec3Swizzles};

use crate::util::{EntityQuery, Cursor};

use crate::game::Player;

pub fn player_camera(
    camera_q: EntityQuery<Camera>,
    player_q: Query<&Player>,
    
    cursor: Res<Cursor>,

    mut transform_query: Query<&mut Transform>,
) {
    if player_q.is_empty() { return }

    let body_pos = transform_query.get(player_q.single().body).unwrap().translation.xy();
    let goal_pos = body_pos + (cursor.pos * 0.15);

    let mut cam_transform = transform_query.get_mut(camera_q.single()).unwrap();
    cam_transform.translation = cam_transform.translation.lerp(goal_pos.extend(0.0), 0.5);
}