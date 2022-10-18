use bevy::{prelude::*, math::Vec3Swizzles};

use crate::util::{EntityQuery, Cursor};

use super::components::*;

pub fn selection_manipulation(
    mut commands: Commands,

    keyboard: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
    cursor: Res<Cursor>,

    selected: EntityQuery<Selected>,
    hovered: EntityQuery<Hovered>,
    
    mut transform_query: Query<&mut Transform>,
) {
    let single = selected.iter().len() == 1;
    let multiplier = match keyboard.pressed(KeyCode::LShift) {
        true => 0.2,
        false => 1.0,
    };

    if single {
        let entity = selected.single();
        let mut transform = transform_query.get_mut(entity).unwrap();

        if keyboard.pressed(KeyCode::Up) {
            transform.scale.y += 1.0 * multiplier;
        }

        if keyboard.pressed(KeyCode::Down) {
            transform.scale.y -= 1.0 * multiplier;
        }

        if keyboard.pressed(KeyCode::Right) {
            transform.scale.x += 1.0 * multiplier;
        }

        if keyboard.pressed(KeyCode::Left) {
            transform.scale.x -= 1.0 * multiplier;
        }

        if keyboard.pressed(KeyCode::E) {
            transform.rotation *= Quat::from_axis_angle(Vec3::Z, -0.05 * multiplier);
        }

        if keyboard.pressed(KeyCode::Q) {
            transform.rotation *= Quat::from_axis_angle(Vec3::Z, 0.05 * multiplier);
        }
    } else {
        if keyboard.any_pressed([KeyCode::Q, KeyCode::E]) {
            let mut center_pt = Vec2::ZERO;
            let avg_mul = 1.0 / selected.iter().len() as f32;

            for entity in selected.iter() {
                let translation = transform_query.get(entity).unwrap().translation;
                center_pt += translation.xy() * avg_mul;
            }

            let mut rot = Quat::from_axis_angle(Vec3::Z, 0.0);
            if keyboard.pressed(KeyCode::E) {
                rot *= Quat::from_axis_angle(Vec3::Z, -0.05 * multiplier);
            }
    
            if keyboard.pressed(KeyCode::Q) {
                rot *= Quat::from_axis_angle(Vec3::Z, 0.05 * multiplier);
            }

            for entity in selected.iter() {
                let mut transform = transform_query.get_mut(entity).unwrap();
                transform.rotate_around(center_pt.extend(0.0), rot);
            }
        }
    }

    if keyboard.just_pressed(KeyCode::W) {
        for entity in selected.iter() {
            let mut transform = transform_query.get_mut(entity).unwrap();
            transform.translation.y += 2.0 * multiplier;
        }
    }

    if keyboard.just_pressed(KeyCode::S) {
        for entity in selected.iter() {
            let mut transform = transform_query.get_mut(entity).unwrap();
            transform.translation.y -= 2.0 * multiplier;
        }
    }

    if keyboard.just_pressed(KeyCode::D) {
        for entity in selected.iter() {
            let mut transform = transform_query.get_mut(entity).unwrap();
            transform.translation.x += 2.0 * multiplier;
        }
    }

    if keyboard.just_pressed(KeyCode::A) {
        for entity in selected.iter() {
            let mut transform = transform_query.get_mut(entity).unwrap();
            transform.translation.x -= 2.0 * multiplier;
        }
    }

    if !hovered.is_empty() && mouse_input.just_pressed(MouseButton::Left) {
        let mut contains = false;
        for entity in hovered.iter() {
            if selected.contains(entity) {
                contains = true;
                break
            }
        }

        if !contains {
            for entity in selected.iter() {
                commands.entity(entity).remove::<Selected>();
            }

            for entity in hovered.iter() {
                commands.entity(entity).insert(Selected);
            }
        }
    }

    if mouse_input.pressed(MouseButton::Left) && !mouse_input.just_pressed(MouseButton::Left) {
        for entity in selected.iter() {
            let mut transform = transform_query.get_mut(entity).unwrap();
            transform.translation -= cursor.delta.extend(0.0);
        }
    }
}