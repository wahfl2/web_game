use bevy::prelude::*;

use crate::util::EntityQuery;

use super::components::*;

pub fn selection_manipulation(
    keyboard: Res<Input<KeyCode>>,

    selected: EntityQuery<Selected>,
    
    mut transform_query: Query<&mut Transform>,
) {
    if selected.is_empty() { return }
    let single = selected.iter().len() == 1;

    if single {
        let entity = selected.single();
        let mut transform = transform_query.get_mut(entity).unwrap();

        if keyboard.pressed(KeyCode::Up) {
            transform.scale.y += 0.5;
        }

        if keyboard.pressed(KeyCode::Down) {
            transform.scale.y -= 0.5;
        }

        if keyboard.pressed(KeyCode::Right) {
            transform.scale.x += 0.5;
        }

        if keyboard.pressed(KeyCode::Left) {
            transform.scale.x -= 0.5;
        }
    } else {

    }
}