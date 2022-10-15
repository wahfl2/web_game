use bevy::{prelude::*, input::mouse::{MouseWheel, MouseScrollUnit}};

use crate::util::Cursor;

const MAX_ZOOM: f32 = 0.5;

pub fn camera_movement(
    mut c_query: Query<(&mut Transform, &mut Camera)>,

    cursor: Res<Cursor>,
    mouse_input: Res<Input<MouseButton>>,
    mut mouse_wheel: EventReader<MouseWheel>,
) {
    if c_query.is_empty() { return }
    let (mut transform, mut camera) = c_query.single_mut();

    if mouse_input.pressed(MouseButton::Middle) && !mouse_input.just_pressed(MouseButton::Middle) {
        transform.translation += cursor.delta().extend(0.0);
    }
}