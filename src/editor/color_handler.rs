use bevy::sprite::ColorMaterial;

use bevy::prelude::*;

use crate::util::{EntityQuery, update_color_material};

use super::components::*;

pub fn color_handler(
    added_selected: Query<Entity, Added<Selected>>,
    added_hovered: Query<Entity, Added<Hovered>>,

    removed_selected: RemovedComponents<Selected>,
    removed_hovered: RemovedComponents<Hovered>,    

    selected: EntityQuery<Selected>,
    hovered: EntityQuery<Hovered>,

    mut materials: ResMut<Assets<ColorMaterial>>,
    material_query: Query<&Handle<ColorMaterial>>,
) {
    for entity in removed_hovered.iter() {
        if !selected.contains(entity) {
            update_color_material(
                material_query.get(entity).unwrap(), 
                &mut materials, 
                Color::RED
            );
        }
    }

    for entity in removed_selected.iter() {
        if hovered.contains(entity) {
            update_color_material(
                material_query.get(entity).unwrap(), 
                &mut materials, 
                Color::ORANGE_RED
            );
        } else {
            update_color_material(
                material_query.get(entity).unwrap(), 
                &mut materials, 
                Color::RED
            );
        }
    }

    for entity in added_hovered.iter() {
        if !selected.contains(entity) {
            update_color_material(
                material_query.get(entity).unwrap(), 
                &mut materials, 
                Color::ORANGE_RED
            );
        }
    }

    for entity in added_selected.iter() {
        update_color_material(
            material_query.get(entity).unwrap(), 
            &mut materials, 
            Color::YELLOW
        );
    }
}