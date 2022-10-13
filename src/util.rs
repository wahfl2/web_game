use bevy::prelude::*;

pub type EntityQuery<'w, 's, T> = Query<'w, 's, Entity, With<T>>;

pub struct Cursor {
    pub pos: Vec2,
}

pub fn cursor_pos(
    mut cursor_pos: ResMut<Cursor>,
    mut cursor_input: EventReader<CursorMoved>,
) {
    if let Some(cursor) = cursor_input.iter().last() {
        cursor_pos.pos = cursor.position;
    }
}