use bevy::prelude::*;

pub type EntityQuery<'w, 's, T> = Query<'w, 's, Entity, With<T>>;

pub struct Cursor {
    pub pos: Vec2,
}

pub fn cursor_pos(
    windows: Res<Windows>,
    mut cursor_pos: ResMut<Cursor>,
    mut cursor_input: EventReader<CursorMoved>,
) {
    if let Some(cursor) = cursor_input.iter().last() {        
        let window = windows.get_primary().unwrap();
        let window_size = Vec2::new(window.width(), window.height());

        cursor_pos.pos = cursor.position - (window_size / 2.0);
    }
}