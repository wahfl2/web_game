use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::util::{Cursor, EntityQuery};

use super::components::*;

pub fn hover_shapes(
    mut commands: Commands,

    cursor: Res<Cursor>,
    rapier_context: Res<RapierContext>,
    mut materials: ResMut<Assets<ColorMaterial>>,

    material_query: Query<&Handle<ColorMaterial>>,
    current_hovered_query: EntityQuery<Hovered>,
) {
    if cursor.moved {
        let mut last = None;
        rapier_context.intersections_with_point(
            cursor.world_pos, 
            QueryFilter::default(), 
            |entity| {
                last = Some(entity);
                true
           }
        );

        if let Some(hovered) = last {
            if !current_hovered_query.contains(hovered) {
                commands.entity(hovered).insert(Hovered);
                let mat_handle = material_query.get(hovered).unwrap();
                let material = materials.get_mut(mat_handle).unwrap();

                material.color = Color::YELLOW;
            }
        }

        for entity in current_hovered_query.iter() {
            if Some(entity) == last { continue }

            commands.entity(entity).remove::<Hovered>();
            let mat_handle = material_query.get(entity).unwrap();
            let material = materials.get_mut(mat_handle).unwrap();

            material.color = Color::RED;
        }
    }
}