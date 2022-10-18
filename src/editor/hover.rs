use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::rapier::geometry::Group;

use crate::util::{Cursor, EntityQuery, update_color_material};

use super::components::*;

pub fn hover_shapes(
    mut commands: Commands,

    cursor: Res<Cursor>,
    mouse_button_input: Res<Input<MouseButton>>,
    rapier_context: Res<RapierContext>,
    mut materials: ResMut<Assets<ColorMaterial>>,

    material_query: Query<&Handle<ColorMaterial>>,
    current_hovered_query: EntityQuery<Hovered>,
) {
    if mouse_button_input.pressed(MouseButton::Left) { return }

    if cursor.moved {
        let mut last = None;
        rapier_context.intersections_with_point(
            cursor.world_pos, 
            QueryFilter::default().groups(
                InteractionGroups::new(Group::ALL, Group::from_bits_truncate(0b1))
            ), 
            |entity| {
                last = Some(entity);
                true
           }
        );

        if let Some(hovered) = last {
            if !current_hovered_query.contains(hovered) {
                commands.entity(hovered).insert(Hovered);
            }
        }

        for entity in current_hovered_query.iter() {
            if Some(entity) == last { continue }
            commands.entity(entity).remove::<Hovered>();
        }
    }
}