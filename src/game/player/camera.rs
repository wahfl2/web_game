use bevy::{prelude::*, math::Vec3Swizzles};

use crate::util::{EntityQuery, Cursor};

use crate::game::Player;

#[derive(Component)]
pub struct PlayerFollow;

pub fn player_camera(
    camera_q: EntityQuery<Camera>,
    player_follow_q: EntityQuery<PlayerFollow>,
    player_q: Query<&Player>,
    
    cursor: Res<Cursor>,

    mut transform_query: Query<&mut Transform>,
) {
    let body_pos = transform_query.get(player_q.single().body).unwrap().translation.xy();
    let follow_pos = transform_query.get(player_follow_q.single()).unwrap().translation.xy();
    let new_follow_pos = body_pos.lerp(follow_pos, 0.75);

    let mut follow_transform = transform_query.get_mut(player_follow_q.single()).unwrap();
    follow_transform.translation = new_follow_pos.extend(0.0);

    let mut cam_transform = transform_query.get_mut(camera_q.single()).unwrap();
    cam_transform.translation = (cursor.pos * 0.15).extend(0.0);
}