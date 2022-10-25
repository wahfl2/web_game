use bevy::sprite::ColorMaterial;

use bevy::prelude::*;

use crate::{util::{EntityQuery, update_color_material, ColorUpdate, ZipAll, ZipAllTrait}, constants};

use super::components::*;

#[derive(Clone, Copy)]
pub enum ColorStateChange {
    AddHover,
    AddSelect,
    RemHover,
    RemSelect,
    ChangeSticky,
}

pub fn color_handler(
    added_selected: Query<Entity, Added<Selected>>,
    added_hovered: Query<Entity, Added<Hovered>>,

    removed_selected: RemovedComponents<Selected>,
    removed_hovered: RemovedComponents<Hovered>,

    changed_stickable: Query<Entity, Changed<EditorShape>>,

    editor_shape_query: Query<&EditorShape>,

    selected: EntityQuery<Selected>,
    hovered: EntityQuery<Hovered>,

    mut materials: ResMut<Assets<ColorMaterial>>,
    material_query: Query<&Handle<ColorMaterial>>,
) {
    let mut updated_entities = Vec::with_capacity(
        added_selected.iter().size_hint().1.unwrap_or(0) + 
        added_hovered.iter().size_hint().1.unwrap_or(0) + 
        removed_selected.iter().size_hint().1.unwrap_or(0) + 
        removed_hovered.iter().size_hint().1.unwrap_or(0) + 
        changed_stickable.iter().size_hint().1.unwrap_or(0)
    );

    updated_entities.append(&mut added_selected.iter().zip_all(ColorStateChange::AddSelect).collect());
    updated_entities.append(&mut added_hovered.iter().zip_all(ColorStateChange::AddHover).collect());
    updated_entities.append(&mut removed_selected.iter().zip_all(ColorStateChange::RemSelect).collect());
    updated_entities.append(&mut removed_hovered.iter().zip_all(ColorStateChange::RemHover).collect());
    updated_entities.append(&mut changed_stickable.iter().zip_all(ColorStateChange::ChangeSticky).collect());

    for (entity, state) in updated_entities.into_iter() {
        let (sel, hov) = match state {
            ColorStateChange::AddHover => (None, Some(true)),
            ColorStateChange::AddSelect => (Some(true), None),
            ColorStateChange::RemHover => (None, Some(false)),
            ColorStateChange::RemSelect => (Some(false), None),
            ColorStateChange::ChangeSticky => (None, None),
        };

        update_color_material(
            material_query.get(entity).unwrap(), 
            &mut materials, 
            ColorUpdate {
                selected: sel.unwrap_or(selected.contains(entity)),
                hovered: hov.unwrap_or(hovered.contains(entity)),
                stickable: editor_shape_query.get(entity).unwrap().stickable,
            }
        );
    }
}